use crate::topology::device::DeviceClassification;
use crate::topology::discovery;
use crate::topology::discovery::DiscoveryResult;
use crate::topology::geo::geo_lookup;
use crate::topology::{ConnectionType, DeviceType, EdgeData, NodeData, PortInfo, TopologyStats};
use petgraph::graph::NodeIndex;
use petgraph::Graph;
use petgraph::Undirected;
use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct TopologyGraph {
    pub graph: Graph<NodeData, EdgeData, Undirected>,
}

impl Default for TopologyGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl TopologyGraph {
    pub fn new() -> Self {
        TopologyGraph {
            graph: Graph::with_capacity(0, 0),
        }
    }

    pub fn add_node(&mut self, node_data: NodeData) -> NodeIndex {
        self.graph.add_node(node_data)
    }

    pub fn add_edge(
        &mut self,
        source: NodeIndex,
        target: NodeIndex,
        edge_data: EdgeData,
    ) -> Option<petgraph::graph::EdgeIndex> {
        self.graph
            .find_edge(source, target)
            .or_else(|| Some(self.graph.add_edge(source, target, edge_data)))
    }

    pub fn get_node(&self, idx: NodeIndex) -> Option<&NodeData> {
        self.graph.node_weight(idx)
    }

    pub fn node_count(&self) -> usize {
        self.graph.node_count()
    }

    pub fn edge_count(&self) -> usize {
        self.graph.edge_count()
    }

    pub fn get_stats(&self) -> TopologyStats {
        TopologyStats::from_graph(self)
    }
}

pub struct TopologyGraphBuilder {
    graph: TopologyGraph,
    ip_to_node: HashMap<IpAddr, NodeIndex>,
}

impl TopologyGraphBuilder {
    pub fn new() -> Self {
        TopologyGraphBuilder {
            graph: TopologyGraph::new(),
            ip_to_node: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, ip: IpAddr, node_data: NodeData) -> NodeIndex {
        if let Some(&existing_idx) = self.ip_to_node.get(&ip) {
            return existing_idx;
        }
        let idx = self.graph.add_node(node_data);
        self.ip_to_node.insert(ip, idx);
        idx
    }

    pub fn add_edge(
        &mut self,
        source: IpAddr,
        target: IpAddr,
        edge_data: EdgeData,
    ) -> Option<petgraph::graph::EdgeIndex> {
        let source_idx = self.ip_to_node.get(&source).copied()?;
        let target_idx = self.ip_to_node.get(&target).copied()?;
        self.graph.add_edge(source_idx, target_idx, edge_data)
    }

    pub fn from_discovery_result(
        &mut self,
        result: &DiscoveryResult,
        device_classifier: &DeviceClassification,
    ) {
        let mut mac_cache: HashMap<String, String> = HashMap::new();

        // Create Internet node first
        let internet_ip = "0.0.0.0".parse::<IpAddr>().unwrap();
        let internet_data = NodeData {
            ip: internet_ip,
            mac: None,
            hostname: Some("External Internet".to_string()),
            vendor: None,
            device_type: DeviceType::Internet,
            os_fingerprint: None,
            ports: Vec::new(),
            risk_score: 0,
            geo_location: None,
            traceroute_hops: Vec::new(),
            first_seen: SystemTime::now(),
            last_seen: SystemTime::now(),
        };
        let internet_node_idx = self.add_node(internet_ip, internet_data);

        for arp in &result.arp_entries {
            if let Some(mac) = self.lookup_vendor(&arp.mac, &mut mac_cache) {
                let node_data = NodeData {
                    ip: IpAddr::V4(arp.ip),
                    mac: Some(arp.mac.clone()),
                    hostname: None,
                    vendor: Some(mac),
                    device_type: DeviceType::Unknown,
                    os_fingerprint: None,
                    ports: Vec::new(),
                    risk_score: 10,
                    geo_location: None,
                    traceroute_hops: Vec::new(),
                    first_seen: SystemTime::UNIX_EPOCH,
                    last_seen: SystemTime::UNIX_EPOCH,
                };
                self.add_node(IpAddr::V4(arp.ip), node_data);
            }
        }

        for (ip, host) in &result.probed_hosts {
            let mac = if let Some(mac_str) = &host.mac {
                Some(mac_str.clone())
            } else {
                result
                    .arp_entries
                    .iter()
                    .find(|a| a.ip == *ip)
                    .map(|arp_entry| arp_entry.mac.clone())
            };
            let vendor = mac
                .as_ref()
                .and_then(|m| self.lookup_vendor(m, &mut mac_cache));

            let hostname = host.hostname.clone();
            let os_family = host.os_info.as_ref().map(|info| info.os_family.as_str());

            let (mut device_type, _confidence) = device_classifier.classify(
                &host.ports,
                hostname.as_deref(),
                os_family,
                host.is_gateway,
                is_private_ip(ip),
            );

            // Double check if it's a gateway
            if device_type == DeviceType::Unknown && host.is_gateway {
                device_type = DeviceType::Router;
            }

            let geo = geo_lookup(*ip);

            let ports: Vec<PortInfo> = host
                .ports
                .iter()
                .map(|&p| PortInfo {
                    port: p,
                    protocol: crate::topology::PortProtocol::TCP,
                    service: None,
                    version: None,
                    banner: None,
                })
                .collect();

            let node_data = NodeData {
                ip: *ip,
                mac,
                hostname,
                vendor,
                device_type,
                os_fingerprint: None,
                ports,
                risk_score: calculate_risk_score(&host.ports),
                geo_location: if geo.country.is_some() {
                    Some(geo)
                } else {
                    None
                },
                traceroute_hops: Vec::new(),
                first_seen: SystemTime::UNIX_EPOCH,
                last_seen: SystemTime::UNIX_EPOCH,
            };

            let node_idx = self.add_node(*ip, node_data);

            // If it's a gateway, connect it to the internet
            if host.is_gateway
                || device_type == DeviceType::Router
                || device_type == DeviceType::Firewall
            {
                self.graph.add_edge(
                    internet_node_idx,
                    node_idx,
                    EdgeData {
                        connection_type: ConnectionType::Inferred,
                        latency_ms: None,
                        hop_count: None,
                        bandwidth_estimate: None,
                    },
                );
            }
        }

        for traceroute in &result.traceroutes {
            let mut previous_ip: Option<IpAddr> = None;
            let mut connected_to_internet = false;

            for hop in &traceroute.hops {
                if let Some(ip) = hop.ip {
                    let is_private = is_private_ip(&ip);

                    // If we encounter a public IP, connect the internet node to the first public IP if not already connected
                    if !is_private && !connected_to_internet {
                        let geo = geo_lookup(ip);
                        let node_data = NodeData {
                            ip,
                            mac: None,
                            hostname: hop.hostname.clone(),
                            vendor: None,
                            device_type: DeviceType::Unknown,
                            os_fingerprint: None,
                            ports: Vec::new(),
                            risk_score: 0,
                            geo_location: if geo.country.is_some() {
                                Some(geo)
                            } else {
                                None
                            },
                            traceroute_hops: Vec::new(),
                            first_seen: SystemTime::now(),
                            last_seen: SystemTime::now(),
                        };
                        let hop_node_idx = self.add_node(ip, node_data);
                        self.graph.add_edge(
                            internet_node_idx,
                            hop_node_idx,
                            EdgeData {
                                connection_type: ConnectionType::TracerouteHop,
                                latency_ms: hop.latency_us.map(|us| (us / 1000) as u32),
                                hop_count: Some(1),
                                bandwidth_estimate: None,
                            },
                        );
                        connected_to_internet = true;
                    }

                    if let Some(prev) = previous_ip {
                        if prev != ip {
                            self.add_edge(
                                prev,
                                ip,
                                EdgeData {
                                    connection_type: ConnectionType::TracerouteHop,
                                    latency_ms: hop.latency_us.map(|us| (us / 1000) as u32),
                                    hop_count: Some(hop.hop_number),
                                    bandwidth_estimate: None,
                                },
                            );
                        }
                    }
                    previous_ip = Some(ip);
                }
            }
        }

        // Pre-calculate MAC prefixes to avoid repeated string manipulation in O(N^2) loop
        let arp_entries_with_prefix: Vec<(
            IpAddr,
            &crate::topology::discovery::ArpEntry,
            Option<String>,
        )> = result
            .arp_entries
            .iter()
            .map(|a| {
                let prefix = a.mac.get(0..8).map(|s| s.to_uppercase().replace('-', ":"));
                (IpAddr::V4(a.ip), a, prefix)
            })
            .collect();

        for (i, (ip1, _entry1, prefix1)) in arp_entries_with_prefix.iter().enumerate() {
            for (j, (ip2, _entry2, prefix2)) in arp_entries_with_prefix.iter().enumerate() {
                // Avoid self-comparison and duplicate checks (check only upper triangle)
                if i >= j {
                    continue;
                }

                // Check invalid prefixes
                if prefix1.as_ref().map(|s| s.as_str()) == Some("")
                    || prefix2.as_ref().map(|s| s.as_str()) == Some("")
                    || prefix1.as_ref().map(|s| s.starts_with("00:00:00")) == Some(true)
                    || prefix2.as_ref().map(|s| s.starts_with("00:00:00")) == Some(true)
                {
                    continue;
                }

                if prefix1 == prefix2 {
                    self.add_edge(
                        *ip1,
                        *ip2,
                        EdgeData {
                            connection_type: ConnectionType::LocalSubnet,
                            latency_ms: None,
                            hop_count: None,
                            bandwidth_estimate: None,
                        },
                    );
                }
            }
        }
    }

    fn lookup_vendor(&self, mac: &str, cache: &mut HashMap<String, String>) -> Option<String> {
        let prefix = mac.get(0..8)?.to_uppercase().replace('-', ":");

        if let Some(cached) = cache.get(&prefix) {
            return Some(cached.clone());
        }

        let vendor = lookup_oui_vendor(&prefix);
        if let Some(ref v) = vendor {
            cache.insert(prefix.clone(), v.clone());
        }
        vendor
    }

    pub fn build(self) -> TopologyGraph {
        self.graph
    }
}

impl Default for TopologyGraphBuilder {
    fn default() -> Self {
        Self::new()
    }
}

fn lookup_oui_vendor(mac: &str) -> Option<String> {
    crate::oui::lookup_vendor(mac)
}

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

fn calculate_risk_score(ports: &[u16]) -> u8 {
    let mut score = 0;

    let critical_ports = [21, 23, 445, 3389, 5900, 22];
    let high_ports = [25, 53, 80, 443, 3306, 5432, 8080, 8443];
    let medium_ports = [110, 143, 993, 995, 1723, 27017];

    for &port in ports {
        if critical_ports.contains(&port) {
            score += 20;
        } else if high_ports.contains(&port) {
            score += 10;
        } else if medium_ports.contains(&port) {
            score += 5;
        }
    }

    score.min(100)
}

pub async fn discover_and_build_fast(
    subnet: &str,
    max_concurrent: usize,
    timeout_ms: u64,
    cancel_flag: Arc<AtomicBool>,
    on_progress: Option<Arc<dyn Fn(f32) + Send + Sync>>,
) -> TopologyGraph {
    let result = discovery::discover_network_fast(
        subnet,
        max_concurrent,
        timeout_ms,
        cancel_flag.clone(),
        on_progress,
    )
    .await;

    let device_classifier = DeviceClassification::new();
    let mut builder = TopologyGraphBuilder::new();

    builder.from_discovery_result(&result, &device_classifier);

    builder.build()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_topology_graph_new() {
        let graph = TopologyGraph::new();
        assert_eq!(graph.node_count(), 0);
        assert_eq!(graph.edge_count(), 0);
    }

    #[test]
    fn test_topology_graph_add_node() {
        let mut graph = TopologyGraph::new();
        let node_data = NodeData {
            ip: "192.168.1.1".parse().unwrap(),
            mac: Some("00:11:22:33:44:55".to_string()),
            hostname: Some("router.local".to_string()),
            vendor: Some("Test Vendor".to_string()),
            device_type: DeviceType::Router,
            os_fingerprint: None,
            ports: vec![],
            risk_score: 10,
            geo_location: None,
            traceroute_hops: vec![],
            first_seen: SystemTime::UNIX_EPOCH,
            last_seen: SystemTime::UNIX_EPOCH,
        };

        let idx = graph.add_node(node_data.clone());
        assert_eq!(graph.node_count(), 1);

        let retrieved = graph.get_node(idx);
        assert!(retrieved.is_some());
        assert_eq!(
            retrieved.unwrap().mac,
            Some("00:11:22:33:44:55".to_string())
        );
    }

    #[test]
    fn test_calculate_risk_score() {
        assert_eq!(calculate_risk_score(&[22, 445]), 40);
        assert_eq!(calculate_risk_score(&[80, 443]), 20);
        assert_eq!(calculate_risk_score(&[22]), 20);
        assert_eq!(calculate_risk_score(&[]), 0);
    }
}
