use mdns_sd::{ServiceDaemon, ServiceEvent};
use std::collections::HashMap;
use std::net::IpAddr;
use std::time::Duration;

pub fn resolve_mdns_hostnames(timeout: Duration) -> HashMap<IpAddr, String> {
    let mdns = match ServiceDaemon::new() {
        Ok(m) => m,
        Err(e) => {
            log::error!("Failed to create mdns daemon: {}", e);
            return HashMap::new();
        }
    };
    
    let mut resolved_hosts = HashMap::new();

    // Browse for common service types to find devices
    let service_types = [
        "_http._tcp.local.",
        "_workstation._tcp.local.",
        "_ssh._tcp.local.",
        "_printer._tcp.local.",
        "_ipp._tcp.local.",
        "_googlecast._tcp.local.",
        "_spotify-connect._tcp.local.",
        "_smb._tcp.local.",
        "_afpovertcp._tcp.local.",
    ];

    let receiver = match mdns.browse("_services._dns-sd._udp.local.") {
        Ok(r) => r,
        Err(e) => {
            log::error!("Failed to browse mdns services: {}", e);
            return HashMap::new();
        }
    };

    for service_type in service_types {
        let _ = mdns.browse(service_type);
    }

    let start = std::time::Instant::now();
    
    // Process events for the duration of the timeout
    while start.elapsed() < timeout {
        while let Ok(event) = receiver.recv_timeout(Duration::from_millis(100)) {
            match event {
                ServiceEvent::ServiceResolved(info) => {
                    for addr in info.get_addresses() {
                        let hostname = info.get_fullname().trim_end_matches('.');
                        let simple_name = hostname.split('.').next().unwrap_or(hostname).to_string();
                        resolved_hosts.insert(*addr, simple_name);
                    }
                }
                _ => {}
            }
            if start.elapsed() >= timeout {
                break;
            }
        }
    }
    
    // Shutdown the daemon gracefully before returning
    let _ = mdns.shutdown();
    
    resolved_hosts
}
