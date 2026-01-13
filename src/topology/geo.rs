use maxminddb;
use serde::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr};
use std::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GeoInfo {
    pub country: Option<String>,
    pub country_iso: Option<String>,
    pub city: Option<String>,
    pub subdivision: Option<String>,
    pub subdivision_iso: Option<String>,
    pub metro_code: Option<u32>,
    pub time_zone: Option<String>,
    pub is_in_european_union: bool,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub accuracy_radius: Option<u32>,
}

fn get_english_name(names: &maxminddb::geoip2::Names) -> Option<String> {
    names
        .english
        .or(names.brazilian_portuguese)
        .or(names.french)
        .or(names.german)
        .or(names.japanese)
        .or(names.russian)
        .or(names.spanish)
        .map(|s| s.to_string())
}

pub struct GeoLookup {
    reader: Option<maxminddb::Reader<std::vec::Vec<u8>>>,
    fallback_cache: std::collections::HashMap<String, GeoInfo>,
}

impl Default for GeoLookup {
    fn default() -> Self {
        Self::new()
    }
}

impl GeoLookup {
    pub fn new() -> Self {
        let data = std::fs::read("assets/GeoIP-City.mmdb").ok();
        let reader = data.and_then(|d| maxminddb::Reader::from_source(d).ok());

        GeoLookup {
            reader,
            fallback_cache: std::collections::HashMap::new(),
        }
    }

    pub fn lookup(&mut self, ip: IpAddr) -> GeoInfo {
        self.database_lookup(&ip)
            .unwrap_or_else(|| self.fallback_lookup(&ip))
    }

    fn database_lookup(&mut self, ip: &IpAddr) -> Option<GeoInfo> {
        if let Some(ref reader) = self.reader {
            let result = reader.lookup(*ip).ok()?;
            let city = result.decode::<maxminddb::geoip2::City>().ok()??;

            let location = city.location;

            let country_names = get_english_name(&city.country.names);
            let country_code = city.country.iso_code.map(|s| s.to_string());

            let city_name = get_english_name(&city.city.names);

            let subdivision = city
                .subdivisions
                .first()
                .and_then(|s| get_english_name(&s.names));
            let subdivision_iso = city
                .subdivisions
                .first()
                .and_then(|s| s.iso_code.map(|s| s.to_string()));

            Some(GeoInfo {
                country: country_code.clone().or(country_names),
                country_iso: country_code,
                city: city_name,
                subdivision,
                subdivision_iso,
                metro_code: location.metro_code.map(|m| m as u32),
                time_zone: location.time_zone.map(|t| t.to_string()),
                is_in_european_union: city.country.is_in_european_union.unwrap_or(false),
                latitude: location.latitude,
                longitude: location.longitude,
                accuracy_radius: location.accuracy_radius.map(|a| a as u32),
            })
        } else {
            None
        }
    }

    fn fallback_lookup(&mut self, ip: &IpAddr) -> GeoInfo {
        let cache_key = ip.to_string();
        if let Some(cached) = self.fallback_cache.get(&cache_key) {
            return cached.clone();
        }

        let fallback = self.compute_fallback(ip);
        self.fallback_cache.insert(cache_key, fallback.clone());

        fallback
    }

    fn compute_fallback(&self, ip: &IpAddr) -> GeoInfo {
        match ip {
            IpAddr::V4(ipv4) => self.fallback_ipv4(ipv4),
            IpAddr::V6(_) => GeoInfo::default(),
        }
    }

    fn fallback_ipv4(&self, ip: &Ipv4Addr) -> GeoInfo {
        let octets = ip.octets();

        if octets[0] == 127 {
            return GeoInfo {
                country: Some("Loopback".to_string()),
                country_iso: None,
                city: Some("Loopback".to_string()),
                subdivision: None,
                subdivision_iso: None,
                metro_code: None,
                time_zone: None,
                is_in_european_union: false,
                latitude: None,
                longitude: None,
                accuracy_radius: None,
            };
        }

        if octets[0] == 0 || octets[0] == 255 {
            return GeoInfo::default();
        }

        GeoInfo::default()
    }

    pub fn is_available(&self) -> bool {
        self.reader.is_some()
    }
}

#[allow(dead_code)]
fn is_private_ip(ip: &IpAddr) -> bool {
    match ip {
        IpAddr::V4(ipv4) => {
            let octets = ipv4.octets();
            octets[0] == 10
                || (octets[0] == 172 && (octets[1] >= 16 && octets[1] <= 31))
                || (octets[0] == 192 && octets[1] == 168)
                || octets[0] == 127
        }
        IpAddr::V6(ipv6) => {
            let segments = ipv6.segments();
            (segments[0] == 0xfe80)
                || (segments[0] == 0xfc00)
                || (segments[0] == 0x0000
                    && segments[1] == 0x0000
                    && segments[2] == 0x0000
                    && segments[3] == 0x0000
                    && segments[4] == 0x0000
                    && segments[5] == 0x0000
                    && segments[6] == 0x0000
                    && segments[7] == 0x0001)
        }
    }
}

pub fn geo_lookup(ip: IpAddr) -> GeoInfo {
    use once_cell::sync::Lazy;

    static GEO_LOOKUP: Lazy<Mutex<GeoLookup>> = Lazy::new(|| Mutex::new(GeoLookup::new()));

    let mut geo_guard = GEO_LOOKUP.lock().unwrap();
    geo_guard.lookup(ip)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{Ipv4Addr, Ipv6Addr};

    #[test]
    fn test_private_ip_detection() {
        assert!(is_private_ip(&IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1))));
        assert!(is_private_ip(&IpAddr::V4(Ipv4Addr::new(172, 16, 0, 1))));
        assert!(is_private_ip(&IpAddr::V4(Ipv4Addr::new(172, 31, 255, 255))));
        assert!(is_private_ip(&IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1))));
        assert!(is_private_ip(&IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))));
        assert!(is_private_ip(&IpAddr::V6(Ipv6Addr::new(
            0xfe80, 0, 0, 0, 0, 0, 0, 1
        ))));
        assert!(is_private_ip(&IpAddr::V6(Ipv6Addr::new(
            0xfc00, 0, 0, 0, 0, 0, 0, 1
        ))));

        assert!(!is_private_ip(&IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8))));
        assert!(!is_private_ip(&IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1))));
        assert!(!is_private_ip(&IpAddr::V6(Ipv6Addr::new(
            0x2001, 0xdb8, 0, 0, 0, 0, 0, 1
        ))));
    }

    #[test]
    fn test_localhost_geo() {
        let mut geo = GeoLookup::new();
        let result = geo.lookup(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));

        assert_eq!(result.country, Some("Loopback".to_string()));
        assert_eq!(result.city, Some("Loopback".to_string()));
    }

    #[test]
    fn test_private_range_returns_default() {
        let mut geo = GeoLookup::new();
        let result = geo.lookup(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100)));

        assert!(result.country.is_none());
        assert!(result.city.is_none());
    }

    #[test]
    fn test_geo_info_default() {
        let geo_info = GeoInfo::default();

        assert!(geo_info.country.is_none());
        assert!(geo_info.city.is_none());
        assert!(geo_info.latitude.is_none());
        assert!(geo_info.longitude.is_none());
        assert!(!geo_info.is_in_european_union);
    }

    #[test]
    fn test_geo_lookup_utility_function() {
        let result = geo_lookup(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));

        assert_eq!(result.country, Some("Loopback".to_string()));
    }

    #[test]
    fn test_database_availability() {
        let geo = GeoLookup::new();

        assert!(geo.is_available());
    }

    #[test]
    fn test_fallback_cache() {
        let mut geo = GeoLookup::new();

        let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        let result1 = geo.lookup(ip);
        let result2 = geo.lookup(ip);

        assert_eq!(result1.country, result2.country);
    }
}
