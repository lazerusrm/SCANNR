use crate::topology::{DeviceType, GeoInfo};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use tokio::net::TcpSocket;
use tokio::net::UdpSocket;
use tokio::time::timeout;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArpEntry {
    pub ip: Ipv4Addr,
    pub mac: String,
    pub iface: String,
    pub entry_type: ArpEntryType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArpEntryType {
    Dynamic,
    Static,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TracerouteResult {
    pub target: IpAddr,
    pub hops: Vec<Hop>,
    pub completed: bool,
    pub total_time_ms: u128,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryResult {
    pub arp_entries: Vec<ArpEntry>,
    pub probed_hosts: HashMap<IpAddr, ProbedHost>,
    pub traceroutes: Vec<TracerouteResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProbedHost {
    pub ip: IpAddr,
    pub ports: Vec<u16>,
    pub hostname: Option<String>,
    pub mac: Option<String>,
    pub vendor: Option<String>,
    pub os_info: Option<OSInfo>,
    pub device_type: DeviceType,
    pub is_gateway: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OSInfo {
    pub os_family: String,
    pub os_gen: Option<String>,
    pub vendor: Option<String>,
    pub accuracy: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hop {
    pub hop_number: u8,
    pub ip: Option<IpAddr>,
    pub hostname: Option<String>,
    pub latency_us: Option<u64>,
    pub rtt_samples_us: Vec<u64>,
    pub geo_location: Option<GeoInfo>,
    pub is_gateway: bool,
    pub is_private: bool,
    pub is_timeout: bool,
}

const COMMON_PORTS: [u16; 20] = [
    21, 22, 23, 25, 53, 80, 110, 143, 443, 445, 993, 995, 1723, 3306, 3389, 5432, 5900, 8080, 8443,
    27017,
];

pub async fn get_arp_entries() -> Vec<ArpEntry> {
    let mut entries = Vec::new();

    #[cfg(target_os = "linux")]
    {
        if let Ok(content) = tokio::fs::read_to_string("/proc/net/arp").await {
            for line in content.lines().skip(1) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 6 {
                    if let Ok(ip) = parts[0].parse::<Ipv4Addr>() {
                        let mac = parts[3];
                        if mac != "00:00:00:00:00:00" && mac.len() == 17 {
                            let entry_type = if parts[5] == "0x2" {
                                ArpEntryType::Static
                            } else {
                                ArpEntryType::Dynamic
                            };
                            entries.push(ArpEntry {
                                ip,
                                mac: mac.to_uppercase(),
                                iface: String::new(),
                                entry_type,
                            });
                        }
                    }
                }
            }
        }
    }

    #[cfg(target_os = "windows")]
    {
        const CREATE_NO_WINDOW: u32 = 0x08000000;

        let output = tokio::process::Command::new("arp")
            .args(["-a"])
            .creation_flags(CREATE_NO_WINDOW)
            .output()
            .await
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok());

        if let Some(content) = output {
            for line in content.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    if let Ok(ip) = parts[0].parse::<Ipv4Addr>() {
                        let mac = parts[1];
                        if mac.contains('-') && mac.len() == 17 {
                            entries.push(ArpEntry {
                                ip,
                                mac: mac.to_uppercase(),
                                iface: String::new(),
                                entry_type: ArpEntryType::Dynamic,
                            });
                        }
                    }
                }
            }
        }
    }

    #[cfg(target_os = "macos")]
    {
        let output = tokio::process::Command::new("arp")
            .args(&["-a", "-n"])
            .output()
            .await
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok());

        if let Some(content) = output {
            for line in content.lines() {
                if let Some(start) = line.find('(') {
                    if let Some(end) = line.find(')') {
                        let ip_str = &line[start + 1..end];
                        if let Ok(ip) = ip_str.parse::<Ipv4Addr>() {
                            if let Some(mac_start) = line.find("at ") {
                                let mac_line = &line[mac_start + 3..];
                                let mac_parts: Vec<&str> = mac_line.split_whitespace().collect();
                                if let Some(mac) = mac_parts.first() {
                                    if mac.contains(':') && mac.len() == 17 {
                                        entries.push(ArpEntry {
                                            ip,
                                            mac: mac.to_uppercase(),
                                            iface: String::new(),
                                            entry_type: ArpEntryType::Dynamic,
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    entries
}

pub async fn trace_route(
    target: IpAddr,
    max_hops: u8,
    timeout_duration: Duration,
    cancel_flag: Arc<AtomicBool>,
) -> TracerouteResult {
    let mut hops = Vec::new();
    let start_time = Instant::now();
    let mut completed = false;
    let mut ttl: u8 = 1;

    while ttl <= max_hops && !cancel_flag.load(Ordering::Relaxed) {
        let hop = perform_hop(target, ttl, timeout_duration).await;

        if hop.ip == Some(target) {
            completed = true;
        }
        hops.push(hop);

        if completed {
            break;
        }

        ttl += 1;
    }

    let total_time_ms = start_time.elapsed().as_millis();

    TracerouteResult {
        target,
        hops,
        completed,
        total_time_ms,
    }
}

async fn perform_hop(target: IpAddr, _ttl: u8, timeout_duration: Duration) -> Hop {
    let start_time = Instant::now();

    match target {
        IpAddr::V4(target_ipv4) => {
            perform_ipv4_hop(target_ipv4, timeout_duration, start_time).await
        }
        IpAddr::V6(target_ipv6) => {
            perform_ipv6_hop(target_ipv6, timeout_duration, start_time).await
        }
    }
}

async fn perform_ipv4_hop(
    target: Ipv4Addr,
    timeout_duration: Duration,
    start_time: Instant,
) -> Hop {
    let socket = UdpSocket::bind("0.0.0.0:0").await.unwrap();

    let timeout_result = timeout(
        timeout_duration,
        socket.connect(SocketAddr::new(IpAddr::V4(target), 33434)),
    )
    .await;

    let latency_us = start_time.elapsed().as_micros() as u64;

    Hop {
        hop_number: 0,
        ip: Some(IpAddr::V4(target)),
        hostname: None,
        latency_us: Some(latency_us),
        rtt_samples_us: vec![latency_us],
        geo_location: None,
        is_gateway: false,
        is_private: target.is_private(),
        is_timeout: timeout_result.is_err(),
    }
}

async fn perform_ipv6_hop(
    target: Ipv6Addr,
    timeout_duration: Duration,
    start_time: Instant,
) -> Hop {
    let socket = UdpSocket::bind("[::]:0").await.unwrap();

    let timeout_result = timeout(
        timeout_duration,
        socket.connect(SocketAddr::new(IpAddr::V6(target), 33434)),
    )
    .await;

    let latency_us = start_time.elapsed().as_micros() as u64;

    Hop {
        hop_number: 0,
        ip: Some(IpAddr::V6(target)),
        hostname: None,
        latency_us: Some(latency_us),
        rtt_samples_us: vec![latency_us],
        geo_location: None,
        is_gateway: false,
        is_private: target.is_unicast_link_local(),
        is_timeout: timeout_result.is_err(),
    }
}

async fn check_port_fast(ip: IpAddr, port: u16, timeout_duration: Duration) -> bool {
    let addr = SocketAddr::new(ip, port);
    let socket = match TcpSocket::new_v4() {
        Ok(s) => s,
        Err(_) => return false,
    };

    socket.set_nodelay(true).ok();
    let stream = timeout(timeout_duration, socket.connect(addr)).await;
    stream.is_ok()
}

async fn check_ports_batch(ip: IpAddr, ports: &[u16], timeout_duration: Duration) -> Vec<u16> {
    let mut open_ports = Vec::new();
    let mut handles = Vec::new();
    let ip_clone = ip;

    for &port in ports {
        let handle = tokio::spawn(check_port_fast(ip_clone, port, timeout_duration));
        handles.push((port, handle));
    }

    for (port, handle) in handles {
        if handle.await.unwrap_or(false) {
            open_ports.push(port);
        }
    }

    open_ports
}

pub async fn probe_host(
    ip: IpAddr,
    ports: &[u16],
    timeout_duration: Duration,
) -> Option<ProbedHost> {
    let open_ports = check_ports_batch(ip, ports, timeout_duration).await;

    if open_ports.is_empty() && !ports.is_empty() {
        return None;
    }

    let os_info = detect_os(&open_ports);
    let device_type = DeviceType::classify(open_ports.as_slice(), None, None);

    Some(ProbedHost {
        ip,
        ports: open_ports,
        hostname: None,
        mac: None,
        vendor: None,
        os_info,
        device_type,
        is_gateway: false,
    })
}

fn detect_os(ports: &[u16]) -> Option<OSInfo> {
    if ports.contains(&22) {
        return Some(OSInfo {
            os_family: "Linux/Unix".to_string(),
            os_gen: None,
            vendor: None,
            accuracy: 70,
        });
    }
    if ports.contains(&3389) {
        return Some(OSInfo {
            os_family: "Windows".to_string(),
            os_gen: None,
            vendor: Some("Microsoft".to_string()),
            accuracy: 80,
        });
    }
    None
}

pub async fn discover_network_fast(
    subnet: &str,
    max_concurrent: usize,
    timeout_ms: u64,
    cancel_flag: Arc<AtomicBool>,
    on_progress: Option<Arc<dyn Fn(f32) + Send + Sync>>,
) -> DiscoveryResult {
    let arp_entries = get_arp_entries().await;
    let mut probed_hosts = HashMap::new();
    let mut traceroutes = Vec::new();

    let targets: Vec<IpAddr> = parse_subnet(subnet).into_iter().take(256).collect();

    let total_targets = targets.len();
    let common_ports: Vec<u16> = COMMON_PORTS.to_vec();
    let common_ports = Arc::new(common_ports);
    let timeout = Duration::from_millis(timeout_ms);
    let traceroute_timeout = Duration::from_millis(timeout_ms / 2);

    let semaphore = Arc::new(tokio::sync::Semaphore::new(max_concurrent));
    let mut handles = Vec::new();
    
    // Spawn MDNS discovery in the background
    let mdns_handle = tokio::task::spawn_blocking(move || {
        crate::topology::mdns::resolve_mdns_hostnames(Duration::from_millis(timeout_ms))
    });

    for target in targets {
        if cancel_flag.load(Ordering::Relaxed) {
            break;
        }

        let permit = semaphore.clone().acquire_owned().await.unwrap();
        let cancel_flag = cancel_flag.clone();
        let common_ports = common_ports.clone();

        let handle = tokio::spawn(async move {
            let _permit = permit;
            if cancel_flag.load(Ordering::Relaxed) {
                return None;
            }

            probe_host(target, &common_ports, timeout).await
        });

        handles.push(handle);
    }

    let mut completed_targets = 0;
    for handle in handles {
        if let Ok(Some(host)) = handle.await {
            probed_hosts.insert(host.ip, host);
        }

        completed_targets += 1;
        if let Some(ref cb) = on_progress {
            if total_targets > 0 {
                cb(completed_targets as f32 / total_targets as f32);
            }
        }
    }

    // Merge MDNS results
    if let Ok(mdns_map) = mdns_handle.await {
        for (ip, hostname) in mdns_map {
            // Update existing host or create new one if active
            if let Some(host) = probed_hosts.get_mut(&ip) {
                if host.hostname.is_none() {
                    host.hostname = Some(hostname);
                }
            } else {
                // Should we add hosts found via MDNS that weren't probed? 
                // Yes, they are clearly on the network.
                let host = ProbedHost {
                    ip,
                    ports: Vec::new(), // We didn't scan it yet, but it exists
                    hostname: Some(hostname),
                    mac: None, // MDNS doesn't give MAC directly usually
                    vendor: None,
                    os_info: None,
                    device_type: crate::topology::DeviceType::Unknown,
                    is_gateway: false,
                };
                probed_hosts.insert(ip, host);
            }
        }
    }

    let hosts_with_ports: Vec<IpAddr> = probed_hosts
        .iter()
        .filter(|(_, h)| !h.ports.is_empty())
        .map(|(ip, _)| *ip)
        .collect();

    let mut traceroute_handles = Vec::new();
    let traceroute_semaphore = Arc::new(tokio::sync::Semaphore::new(max_concurrent / 2));

    for ip in hosts_with_ports {
        let permit = traceroute_semaphore.clone().acquire_owned().await.unwrap();
        let cancel_flag = cancel_flag.clone();

        let handle = tokio::spawn(async move {
            let _permit = permit;
            if cancel_flag.load(Ordering::Relaxed) {
                return None;
            }

            Some(trace_route(ip, 30, traceroute_timeout, cancel_flag).await)
        });

        traceroute_handles.push(handle);
    }

    for handle in traceroute_handles {
        if let Ok(Some(traceroute)) = handle.await {
            traceroutes.push(traceroute);
        }
    }

    DiscoveryResult {
        arp_entries,
        probed_hosts,
        traceroutes,
    }
}

pub async fn discover_network(
    _subnet: &str,
    _common_ports: &[u16],
    max_concurrent: usize,
    timeout: Duration,
    cancel_flag: Arc<AtomicBool>,
) -> DiscoveryResult {
    discover_network_fast(
        _subnet,
        max_concurrent,
        timeout.as_millis() as u64,
        cancel_flag,
        None,
    )
    .await
}

fn parse_subnet(subnet: &str) -> Vec<IpAddr> {
    let mut targets = Vec::new();

    if let Some((ip_str, prefix_str)) = subnet.split_once('/') {
        if let Ok(prefix) = prefix_str.parse::<u8>() {
            if let Ok(base_ip) = ip_str.parse::<Ipv4Addr>() {
                let num_hosts = 2_u32.pow((32 - prefix) as u32);

                for i in 0..num_hosts.min(256) {
                    let octets = base_ip.octets();
                    let last_octet = octets[3] as u32 + i;
                    if last_octet <= 255 {
                        let new_ip =
                            Ipv4Addr::new(octets[0], octets[1], octets[2], last_octet as u8);
                        targets.push(IpAddr::V4(new_ip));
                    }
                }
            }
        }
    }

    targets
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_subnet() {
        let ips = parse_subnet("192.168.1.0/29");
        assert_eq!(ips.len(), 8);
        assert_eq!(ips[0], IpAddr::V4(Ipv4Addr::new(192, 168, 1, 0)));
        assert_eq!(ips[7], IpAddr::V4(Ipv4Addr::new(192, 168, 1, 7)));
    }

    #[test]
    fn test_common_ports_count() {
        assert_eq!(COMMON_PORTS.len(), 20);
    }
}
