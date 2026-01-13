use std::collections::HashMap;
use std::net::IpAddr;
use std::time::SystemTime;

use petgraph::visit::EdgeRef;

use serde::{Deserialize, Serialize};

pub mod device;
pub mod discovery;
pub mod export;
pub mod geo;
pub mod graph;
pub mod layout;
pub mod renderer;
pub mod widget;
pub mod mdns;

pub use device::*;
pub use discovery::*;
pub use geo::GeoInfo;
pub use graph::{TopologyGraph, TopologyGraphBuilder};
pub use layout::{LayoutConfig, LayoutEngine};
pub use renderer::{LODLevel, RenderConfig, TopologyRenderer};
pub use widget::{TopologyWidget, TopologyAction};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum LayoutType {
    #[default]
    ForceDirected,
    Circular,
    Hierarchical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HighlightMode {
    #[default]
    DeviceType,
    RiskScore,
    Latency,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeId(pub IpAddr);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeData {
    pub ip: IpAddr,
    pub mac: Option<String>,
    pub hostname: Option<String>,
    pub vendor: Option<String>,
    pub device_type: DeviceType,
    pub os_fingerprint: Option<OSInfo>,
    pub ports: Vec<PortInfo>,
    pub risk_score: u8,
    pub geo_location: Option<GeoInfo>,
    pub traceroute_hops: Vec<Hop>,
    pub first_seen: SystemTime,
    pub last_seen: SystemTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeData {
    pub connection_type: ConnectionType,
    pub latency_ms: Option<u32>,
    pub hop_count: Option<u8>,
    pub bandwidth_estimate: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortInfo {
    pub port: u16,
    pub protocol: PortProtocol,
    pub service: Option<String>,
    pub version: Option<String>,
    pub banner: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OSInfo {
    pub os_family: String,
    pub os_gen: Option<String>,
    pub vendor: Option<String>,
    pub device_type: Option<DeviceType>,
    pub accuracy: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PortProtocol {
    TCP,
    UDP,
    SCTP,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConnectionType {
    LocalSubnet,
    TracerouteHop,
    Inferred,
    Manual,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopologyStats {
    pub node_count: usize,
    pub edge_count: usize,
    pub device_type_counts: HashMap<DeviceType, usize>,
    pub router_count: usize,
    pub server_count: usize,
    pub iot_count: usize,
    pub firewall_count: usize,
    pub unknown_count: usize,
    pub average_latency_ms: f32,
    pub highest_risk_node: Option<NodeId>,
    pub highest_risk_score: u8,
}

impl TopologyStats {
    pub fn from_graph(graph: &TopologyGraph) -> Self {
        let mut device_type_counts = HashMap::new();
        let mut router_count = 0;
        let mut server_count = 0;
        let mut iot_count = 0;
        let mut firewall_count = 0;
        let mut unknown_count = 0;
        let mut highest_risk_node = None;
        let mut highest_risk_score = 0u8;
        let mut total_latency = 0u64;
        let mut latency_count = 0;

        for node_idx in graph.graph.node_indices() {
            if let Some(node_data) = graph.graph.node_weight(node_idx) {
                let count = device_type_counts.entry(node_data.device_type).or_insert(0);
                *count += 1;

                match node_data.device_type {
                    DeviceType::Router | DeviceType::Switch | DeviceType::AccessPoint => {
                        router_count += 1
                    }
                    DeviceType::Server
                    | DeviceType::WebServer
                    | DeviceType::Database
                    | DeviceType::MailServer => server_count += 1,
                    DeviceType::Firewall | DeviceType::FirewallAppliance => firewall_count += 1,
                    DeviceType::IoT
                    | DeviceType::Camera
                    | DeviceType::Thermostat
                    | DeviceType::Light => iot_count += 1,
                    DeviceType::Unknown => unknown_count += 1,
                    _ => {}
                }

                if node_data.risk_score > highest_risk_score {
                    highest_risk_score = node_data.risk_score;
                    highest_risk_node = Some(NodeId(node_data.ip));
                }

                for edge in graph.graph.edges(node_idx) {
                    let edge_idx = edge.id();
                    if let Some(edge_data) = graph.graph.edge_weight(edge_idx) {
                        if let Some(latency) = edge_data.latency_ms {
                            total_latency += latency as u64;
                            latency_count += 1;
                        }
                    }
                }
            }
        }

        let average_latency = if latency_count > 0 {
            (total_latency as f32) / (latency_count as f32)
        } else {
            0.0
        };

        TopologyStats {
            node_count: graph.graph.node_count(),
            edge_count: graph.graph.edge_count(),
            device_type_counts,
            router_count,
            server_count,
            iot_count,
            firewall_count,
            unknown_count,
            average_latency_ms: average_latency,
            highest_risk_node,
            highest_risk_score,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TopologyViewState {
    pub zoom: f32,
    pub pan_offset: egui::Vec2,
    pub selected_node: Option<NodeId>,
    pub hovered_node: Option<NodeId>,
    pub show_labels: bool,
    pub show_edges: bool,
    pub show_risk_levels: bool,
    pub lod_level: LODLevel,
    pub highlight_mode: HighlightMode,
}

impl Default for TopologyViewState {
    fn default() -> Self {
        TopologyViewState {
            zoom: 1.0,
            pan_offset: egui::Vec2::ZERO,
            selected_node: None,
            hovered_node: None,
            show_labels: true,
            show_edges: true,
            show_risk_levels: true,
            lod_level: LODLevel::Medium,
            highlight_mode: HighlightMode::default(),
        }
    }
}

impl TopologyViewState {
    pub fn update_lod(&mut self, _viewport_size: egui::Vec2, graph: &TopologyGraph) {
        self.lod_level = LODLevel::from_zoom_and_count(self.zoom, graph.graph.node_count());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    #[test]
    fn test_node_data_creation() {
        let node = NodeData {
            ip: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)),
            mac: Some("00:11:22:33:44:55".to_string()),
            hostname: Some("router.local".to_string()),
            vendor: Some("Cisco".to_string()),
            device_type: DeviceType::Router,
            os_fingerprint: Some(OSInfo {
                os_family: "Cisco IOS".to_string(),
                os_gen: Some("15.2".to_string()),
                vendor: Some("Cisco".to_string()),
                device_type: Some(DeviceType::Router),
                accuracy: 95,
            }),
            ports: vec![PortInfo {
                port: 443,
                protocol: PortProtocol::TCP,
                service: Some("https".to_string()),
                version: None,
                banner: None,
            }],
            risk_score: 25,
            geo_location: None,
            traceroute_hops: Vec::new(),
            first_seen: SystemTime::UNIX_EPOCH,
            last_seen: SystemTime::UNIX_EPOCH,
        };

        assert_eq!(node.device_type, DeviceType::Router);
        assert_eq!(node.ports.len(), 1);
        assert!(node.risk_score < 100);
    }

    #[test]
    fn test_topology_stats() {
        let mut graph = TopologyGraph::new();

        let node1 = graph.add_node(NodeData {
            ip: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)),
            mac: None,
            hostname: None,
            vendor: None,
            device_type: DeviceType::Router,
            os_fingerprint: None,
            ports: Vec::new(),
            risk_score: 50,
            geo_location: None,
            traceroute_hops: Vec::new(),
            first_seen: SystemTime::UNIX_EPOCH,
            last_seen: SystemTime::UNIX_EPOCH,
        });

        let node2 = graph.add_node(NodeData {
            ip: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 10)),
            mac: None,
            hostname: None,
            vendor: None,
            device_type: DeviceType::Server,
            os_fingerprint: None,
            ports: Vec::new(),
            risk_score: 30,
            geo_location: None,
            traceroute_hops: Vec::new(),
            first_seen: SystemTime::UNIX_EPOCH,
            last_seen: SystemTime::UNIX_EPOCH,
        });

        graph.add_edge(
            node1,
            node2,
            EdgeData {
                connection_type: ConnectionType::LocalSubnet,
                latency_ms: Some(2),
                hop_count: Some(1),
                bandwidth_estimate: None,
            },
        );

        let stats = TopologyStats::from_graph(&graph);

        assert_eq!(stats.node_count, 2);
        assert_eq!(stats.edge_count, 1);
        assert_eq!(stats.router_count, 1);
        assert_eq!(stats.server_count, 1);
    }
}
