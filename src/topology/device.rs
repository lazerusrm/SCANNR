use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::Ipv4Addr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub enum DeviceType {
    Router,
    Switch,
    Firewall,
    LoadBalancer,
    Server,
    Workstation,
    Laptop,
    Mobile,
    Tablet,
    Printer,
    IoT,
    Camera,
    Thermostat,
    Speaker,
    Light,
    Lock,
    Sensor,
    NAS,
    AccessPoint,
    VMHost,
    Container,
    Database,
    WebServer,
    MailServer,
    DNS,
    DHCP,
    Directory,
    VPN,
    Proxy,
    FirewallAppliance,
    Storage,
    UPS,
    KVM,
    Internet,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct DeviceClassification {
    port_patterns: Vec<PortPattern>,
    hostname_patterns: Vec<HostnamePattern>,
    os_patterns: Vec<OSPattern>,
}

#[derive(Debug, Clone, PartialEq)]
struct PortPattern {
    ports: Vec<u16>,
    required_count: usize,
    device_type: DeviceType,
    confidence: f32,
}

#[derive(Debug, Clone, PartialEq)]
struct HostnamePattern {
    pattern: String,
    device_type: DeviceType,
    confidence: f32,
}

#[derive(Debug, Clone, PartialEq)]
struct OSPattern {
    os_family: String,
    device_type: DeviceType,
    confidence: f32,
}

impl Default for DeviceClassification {
    fn default() -> Self {
        Self::new()
    }
}

impl DeviceClassification {
    pub fn new() -> Self {
        let mut port_patterns = Vec::new();
        let mut hostname_patterns = Vec::new();
        let mut os_patterns = Vec::new();

        port_patterns.push(PortPattern {
            ports: vec![22, 23, 80, 443],
            required_count: 2,
            device_type: DeviceType::Router,
            confidence: 0.85,
        });
        port_patterns.push(PortPattern {
            ports: vec![22, 23, 161, 162],
            required_count: 2,
            device_type: DeviceType::Switch,
            confidence: 0.80,
        });
        port_patterns.push(PortPattern {
            ports: vec![443, 8443, 10443],
            required_count: 1,
            device_type: DeviceType::Firewall,
            confidence: 0.75,
        });
        port_patterns.push(PortPattern {
            ports: vec![22, 80, 443],
            required_count: 2,
            device_type: DeviceType::Server,
            confidence: 0.70,
        });
        port_patterns.push(PortPattern {
            ports: vec![9100, 631],
            required_count: 1,
            device_type: DeviceType::Printer,
            confidence: 0.90,
        });
        port_patterns.push(PortPattern {
            ports: vec![554, 8554],
            required_count: 1,
            device_type: DeviceType::Camera,
            confidence: 0.85,
        });
        port_patterns.push(PortPattern {
            ports: vec![5000, 5001],
            required_count: 2,
            device_type: DeviceType::NAS,
            confidence: 0.80,
        });
        port_patterns.push(PortPattern {
            ports: vec![88, 389, 445],
            required_count: 2,
            device_type: DeviceType::Directory,
            confidence: 0.75,
        });
        port_patterns.push(PortPattern {
            ports: vec![53, 80, 443],
            required_count: 2,
            device_type: DeviceType::DNS,
            confidence: 0.80,
        });
        port_patterns.push(PortPattern {
            ports: vec![67, 68, 69],
            required_count: 1,
            device_type: DeviceType::DHCP,
            confidence: 0.85,
        });

        hostname_patterns.push(HostnamePattern {
            pattern: r"^(router|rt-|gw-|gate|fw-|firewall|fw)".to_string(),
            device_type: DeviceType::Router,
            confidence: 0.85,
        });
        hostname_patterns.push(HostnamePattern {
            pattern: r"^(sw-|switch|sw)".to_string(),
            device_type: DeviceType::Switch,
            confidence: 0.80,
        });
        hostname_patterns.push(HostnamePattern {
            pattern: r"^(fw-|firewall|fw)".to_string(),
            device_type: DeviceType::Firewall,
            confidence: 0.85,
        });
        hostname_patterns.push(HostnamePattern {
            pattern: r"^(server|srv|app|db-|web-|mail-|dc-|ad-)".to_string(),
            device_type: DeviceType::Server,
            confidence: 0.70,
        });
        hostname_patterns.push(HostnamePattern {
            pattern: r"^(pc-|desktop|laptop|ws-|workstation)".to_string(),
            device_type: DeviceType::Workstation,
            confidence: 0.75,
        });
        hostname_patterns.push(HostnamePattern {
            pattern: r"^(printer|print|lp-|laserjet|hp-|epson)".to_string(),
            device_type: DeviceType::Printer,
            confidence: 0.90,
        });
        hostname_patterns.push(HostnamePattern {
            pattern: r"^(cam-|camera|ipcam|axis|hikvision|dahua)".to_string(),
            device_type: DeviceType::Camera,
            confidence: 0.85,
        });
        hostname_patterns.push(HostnamePattern {
            pattern: r"^(iot-|sensor|sense|temp|thermo)".to_string(),
            device_type: DeviceType::IoT,
            confidence: 0.80,
        });
        hostname_patterns.push(HostnamePattern {
            pattern: r"^(nas-|synology|qnap|freenas|truenas)".to_string(),
            device_type: DeviceType::NAS,
            confidence: 0.90,
        });
        hostname_patterns.push(HostnamePattern {
            pattern: r"^(ap-|wifi|wireless|ssid)".to_string(),
            device_type: DeviceType::AccessPoint,
            confidence: 0.85,
        });
        hostname_patterns.push(HostnamePattern {
            pattern: r"^(esx-|esxi|hyper-v|kvm|xen)".to_string(),
            device_type: DeviceType::VMHost,
            confidence: 0.80,
        });
        hostname_patterns.push(HostnamePattern {
            pattern: r"^(dns|bind|powerdns)".to_string(),
            device_type: DeviceType::DNS,
            confidence: 0.85,
        });
        hostname_patterns.push(HostnamePattern {
            pattern: r"^(dhcp)".to_string(),
            device_type: DeviceType::DHCP,
            confidence: 0.90,
        });
        hostname_patterns.push(HostnamePattern {
            pattern: r"^(mail|smtp|postfix|exchange)".to_string(),
            device_type: DeviceType::MailServer,
            confidence: 0.85,
        });
        hostname_patterns.push(HostnamePattern {
            pattern: r"^(ldap|ad-|directory|active)".to_string(),
            device_type: DeviceType::Directory,
            confidence: 0.85,
        });
        hostname_patterns.push(HostnamePattern {
            pattern: r"^(vpn|openvpn|wireguard)".to_string(),
            device_type: DeviceType::VPN,
            confidence: 0.85,
        });

        os_patterns.push(OSPattern {
            os_family: "Linux".to_string(),
            device_type: DeviceType::Server,
            confidence: 0.70,
        });
        os_patterns.push(OSPattern {
            os_family: "Cisco IOS".to_string(),
            device_type: DeviceType::Router,
            confidence: 0.90,
        });
        os_patterns.push(OSPattern {
            os_family: "JunOS".to_string(),
            device_type: DeviceType::Router,
            confidence: 0.90,
        });
        os_patterns.push(OSPattern {
            os_family: "Windows Server".to_string(),
            device_type: DeviceType::Server,
            confidence: 0.85,
        });
        os_patterns.push(OSPattern {
            os_family: "FreeBSD".to_string(),
            device_type: DeviceType::Server,
            confidence: 0.75,
        });
        os_patterns.push(OSPattern {
            os_family: "macOS".to_string(),
            device_type: DeviceType::Workstation,
            confidence: 0.80,
        });

        DeviceClassification {
            port_patterns,
            hostname_patterns,
            os_patterns,
        }
    }

    pub fn classify(
        &self,
        ports: &[u16],
        hostname: Option<&str>,
        os_family: Option<&str>,
        _is_gateway: bool,
        _is_private: bool,
    ) -> (DeviceType, f32) {
        let mut scores: HashMap<DeviceType, (f32, usize)> = HashMap::new();

        for pattern in &self.port_patterns {
            let matches = ports.iter().filter(|p| pattern.ports.contains(p)).count();
            if matches >= pattern.required_count {
                let entry = scores.entry(pattern.device_type).or_insert((0.0, 0));
                entry.0 += pattern.confidence * (matches as f32 / pattern.required_count as f32);
                entry.1 += 1;
            }
        }

        if let Some(host) = hostname {
            for pattern in &self.hostname_patterns {
                if host
                    .to_lowercase()
                    .contains(&pattern.pattern.to_lowercase())
                {
                    let entry = scores.entry(pattern.device_type).or_insert((0.0, 0));
                    entry.0 += pattern.confidence;
                    entry.1 += 1;
                }
            }
        }

        if let Some(os) = os_family {
            for pattern in &self.os_patterns {
                if os
                    .to_lowercase()
                    .contains(&pattern.os_family.to_lowercase())
                {
                    let entry = scores.entry(pattern.device_type).or_insert((0.0, 0));
                    entry.0 += pattern.confidence;
                    entry.1 += 1;
                }
            }
        }

        let mut best_device = DeviceType::Unknown;
        let mut best_score = 0.0;

        for (device, (score, count)) in &scores {
            let count_value = *count.max(&1);
            let avg_score = score / count_value as f32;
            if avg_score > best_score {
                best_score = avg_score;
                best_device = *device;
            }
        }

        (best_device, best_score.min(1.0))
    }

    pub fn is_likely_gateway(&self, ip: Ipv4Addr, _hostname: Option<&str>, _ports: &[u16]) -> bool {
        // Strict gateway detection based on IP convention
        let last_octet = ip.octets()[3];
        last_octet == 1 || last_octet == 254
    }
}

impl DeviceType {
    pub fn classify(ports: &[u16], _hostname: Option<&str>, _os_info: Option<&str>) -> DeviceType {
        if ports.contains(&53)
            && (ports.contains(&80) || ports.contains(&443))
            && !ports.contains(&22)
        {
            return DeviceType::DNS;
        }
        if ports.contains(&161) && ports.contains(&162) {
            return DeviceType::Switch;
        }
        if ports.contains(&443) && ports.contains(&8443) {
            return DeviceType::Firewall;
        }
        if ports.contains(&22) && !ports.contains(&3389) {
            return DeviceType::Server;
        }
        if ports.contains(&9100) || ports.contains(&631) {
            return DeviceType::Printer;
        }
        if ports.contains(&554) || ports.contains(&8554) {
            return DeviceType::Camera;
        }
        if ports.contains(&5000) && ports.contains(&5001) {
            return DeviceType::NAS;
        }
        DeviceType::Unknown
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_classification_router() {
        let classifier = DeviceClassification::new();
        let (device, score) = classifier.classify(&[22, 23, 80], None, None, false, false);
        assert_eq!(device, DeviceType::Router);
        assert!(score > 0.0);
    }

    #[test]
    fn test_device_classification_printer() {
        let classifier = DeviceClassification::new();
        let (device, score) = classifier.classify(&[9100], None, None, false, false);
        assert_eq!(device, DeviceType::Printer);
        assert!(score > 0.0);
    }

    #[test]
    fn test_is_likely_gateway() {
        let classifier = DeviceClassification::new();

        assert!(classifier.is_likely_gateway(Ipv4Addr::new(192, 168, 1, 1), None, &[],));
        assert!(classifier.is_likely_gateway(Ipv4Addr::new(192, 168, 1, 254), None, &[],));
        assert!(classifier.is_likely_gateway(
            Ipv4Addr::new(192, 168, 1, 100),
            Some("gateway.local"),
            &[],
        ));
        assert!(classifier.is_likely_gateway(
            Ipv4Addr::new(192, 168, 1, 100),
            None,
            &[22, 23, 80, 443, 53],
        ));
        assert!(!classifier.is_likely_gateway(Ipv4Addr::new(192, 168, 1, 100), None, &[80],));
    }

    #[test]
    fn test_device_type_classify() {
        assert_eq!(
            DeviceType::classify(&[53, 80], None, None),
            DeviceType::Router
        );
        assert_eq!(
            DeviceType::classify(&[161, 162], None, None),
            DeviceType::Switch
        );
        assert_eq!(
            DeviceType::classify(&[443, 8443], None, None),
            DeviceType::Firewall
        );
        assert_eq!(DeviceType::classify(&[22], None, None), DeviceType::Server);
        assert_eq!(
            DeviceType::classify(&[9100], None, None),
            DeviceType::Printer
        );
        assert_eq!(DeviceType::classify(&[554], None, None), DeviceType::Camera);
        assert_eq!(
            DeviceType::classify(&[5000, 5001], None, None),
            DeviceType::NAS
        );
        assert_eq!(
            DeviceType::classify(&[22, 3389], None, None),
            DeviceType::Unknown
        );
    }
}
