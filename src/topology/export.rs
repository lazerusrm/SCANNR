use crate::topology::{ConnectionType, DeviceType, EdgeData, NodeData, TopologyGraph};
use glam::Vec2;
use petgraph::graph::NodeIndex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportOptions {
    pub include_geo: bool,
    pub include_risk: bool,
    pub include_ports: bool,
    pub include_hostnames: bool,
    pub edge_labels: bool,
    pub format_positions: bool,
}

impl Default for ExportOptions {
    fn default() -> Self {
        ExportOptions {
            include_geo: true,
            include_risk: true,
            include_ports: true,
            include_hostnames: true,
            edge_labels: true,
            format_positions: true,
        }
    }
}

pub struct Exporter;

impl Exporter {
    pub fn export_dot(
        graph: &TopologyGraph,
        positions: &HashMap<NodeIndex, Vec2>,
        options: &ExportOptions,
    ) -> String {
        let mut output = String::new();

        output.push_str("digraph network {\n");
        output.push_str("  rankdir=TB;\n");
        output.push_str("  node [shape=box, style=filled, fontname=\"Helvetica\", fontsize=10];\n");
        output.push_str("  edge [fontname=\"Helvetica\", fontsize=8];\n\n");

        for idx in graph.graph.node_indices() {
            let node_data = match graph.graph.node_weight(idx) {
                Some(n) => n,
                None => continue,
            };
            let node_id = node_data.ip.to_string();
            let label = Self::format_node_label(node_data, options);

            let fillcolor = Self::get_device_color(node_data.device_type);
            let risk_color = if options.include_risk {
                Self::get_risk_color(node_data.risk_score)
            } else {
                None
            };

            let color = risk_color.unwrap_or_else(|| fillcolor.clone());

            output.push_str(&format!("  \"{}\" [\n", node_id));
            output.push_str(&format!("    label=\"{}\",\n", label));
            output.push_str(&format!("    fillcolor=\"{}\";\n", fillcolor));
            output.push_str(&format!("    color=\"{}\";\n", color));

            if let Some(pos) = positions.get(&idx) {
                output.push_str(&format!(
                    "    pos=\"{:.2},{:.2}\";\n",
                    pos.x + 500.0,
                    pos.y + 500.0
                ));
            }

            output.push_str("  ];\n\n");
        }

        for edge_idx in graph.graph.edge_indices() {
            let (src, dst) = match graph.graph.edge_endpoints(edge_idx) {
                Some(e) => e,
                None => continue,
            };
            let edge_data = match graph.graph.edge_weight(edge_idx) {
                Some(d) => d,
                None => continue,
            };
            let src_data = match graph.graph.node_weight(src) {
                Some(d) => d,
                None => continue,
            };
            let dst_data = match graph.graph.node_weight(dst) {
                Some(d) => d,
                None => continue,
            };

            let label = if options.edge_labels {
                Self::format_edge_label(edge_data)
            } else {
                String::new()
            };

            output.push_str(&format!("  \"{}\" -> \"{}\" [\n", src_data.ip, dst_data.ip));
            output.push_str(&format!("    label=\"{}\";\n", label));

            let edge_color = Self::get_edge_color(edge_data.connection_type);
            output.push_str(&format!("    color=\"{}\";\n", edge_color));
            output.push_str("  ];\n\n");
        }

        output.push_str("}\n");

        output
    }

    pub fn export_json(
        graph: &TopologyGraph,
        positions: &HashMap<NodeIndex, Vec2>,
        options: &ExportOptions,
    ) -> Result<String, serde_json::Error> {
        #[derive(Serialize)]
        struct JsonNode {
            id: String,
            label: String,
            ip: String,
            device_type: String,
            vendor: Option<String>,
            hostname: Option<String>,
            port_count: usize,
            risk_score: u8,
            risk_level: String,
            geo: Option<JsonGeo>,
            position: Option<JsonPosition>,
            ports: Vec<JsonPort>,
        }

        #[derive(Serialize)]
        struct JsonGeo {
            country: Option<String>,
            city: Option<String>,
            latitude: Option<f64>,
            longitude: Option<f64>,
        }

        #[derive(Serialize)]
        struct JsonPosition {
            x: f32,
            y: f32,
        }

        #[derive(Serialize)]
        struct JsonPort {
            port: u16,
            protocol: String,
            service: Option<String>,
        }

        #[derive(Serialize)]
        struct JsonEdge {
            source: String,
            target: String,
            connection_type: String,
            latency_ms: Option<u32>,
            hop_count: Option<u8>,
        }

        #[derive(Serialize)]
        struct JsonGraph {
            node_count: usize,
            edge_count: usize,
            nodes: Vec<JsonNode>,
            edges: Vec<JsonEdge>,
        }

        let mut nodes = Vec::new();
        let mut edges = Vec::new();

        for idx in graph.graph.node_indices() {
            if let Some(node_data) = graph.graph.node_weight(idx) {
                let node = JsonNode {
                    id: node_data.ip.to_string(),
                    label: Self::format_node_label(node_data, options),
                    ip: node_data.ip.to_string(),
                    device_type: format!("{:?}", node_data.device_type),
                    vendor: node_data.vendor.clone(),
                    hostname: node_data.hostname.clone(),
                    port_count: node_data.ports.len(),
                    risk_score: node_data.risk_score,
                    risk_level: Self::risk_level(node_data.risk_score),
                    geo: if options.include_geo {
                        node_data.geo_location.as_ref().map(|g| JsonGeo {
                            country: g.country.clone(),
                            city: g.city.clone(),
                            latitude: g.latitude,
                            longitude: g.longitude,
                        })
                    } else {
                        None
                    },
                    position: positions.get(&idx).map(|p| JsonPosition { x: p.x, y: p.y }),
                    ports: if options.include_ports {
                        node_data
                            .ports
                            .iter()
                            .map(|p| JsonPort {
                                port: p.port,
                                protocol: format!("{:?}", p.protocol),
                                service: p.service.clone(),
                            })
                            .collect()
                    } else {
                        Vec::new()
                    },
                };
                nodes.push(node);
            }
        }

        for edge_idx in graph.graph.edge_indices() {
            let (src, dst) = match graph.graph.edge_endpoints(edge_idx) {
                Some(e) => e,
                None => continue,
            };
            let edge_data = match graph.graph.edge_weight(edge_idx) {
                Some(d) => d,
                None => continue,
            };
            let src_data = match graph.graph.node_weight(src) {
                Some(d) => d,
                None => continue,
            };
            let dst_data = match graph.graph.node_weight(dst) {
                Some(d) => d,
                None => continue,
            };

            edges.push(JsonEdge {
                source: src_data.ip.to_string(),
                target: dst_data.ip.to_string(),
                connection_type: format!("{:?}", edge_data.connection_type),
                latency_ms: edge_data.latency_ms,
                hop_count: edge_data.hop_count,
            });
        }

        let json_graph = JsonGraph {
            node_count: nodes.len(),
            edge_count: edges.len(),
            nodes,
            edges,
        };

        serde_json::to_string_pretty(&json_graph)
    }

    pub fn export_svg(
        graph: &TopologyGraph,
        positions: &HashMap<NodeIndex, Vec2>,
        options: &ExportOptions,
        width: u32,
        height: u32,
    ) -> String {
        let mut output = String::new();

        let scale = 1.0;
        let offset_x = width as f32 / 2.0;
        let offset_y = height as f32 / 2.0;

        output.push_str(&format!(
            r##"<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" viewBox="0 0 {} {}">
  <defs>
    <marker id="arrowhead" markerWidth="10" markerHeight="7" refX="9" refY="3.5" orient="auto">
      <polygon points="0 0, 10 3.5, 0 7" fill="#666"/>
    </marker>
  </defs>
  <rect width="100%" height="100%" fill="#fafafa"/>
</svg>
"##,
            width, height, width, height
        ));

        for edge_idx in graph.graph.edge_indices() {
            let (src, dst) = match graph.graph.edge_endpoints(edge_idx) {
                Some(e) => e,
                None => continue,
            };
            let edge_data = match graph.graph.edge_weight(edge_idx) {
                Some(d) => d,
                None => continue,
            };
            let src_pos = match (positions.get(&src), positions.get(&dst)) {
                (Some(s), Some(d)) => (s, d),
                _ => continue,
            };

            let x1 = src_pos.0.x * scale + offset_x;
            let y1 = src_pos.0.y * scale + offset_y;
            let x2 = src_pos.1.x * scale + offset_x;
            let y2 = src_pos.1.y * scale + offset_y;

            let edge_color = Self::get_edge_color(edge_data.connection_type);
            let stroke_width = match edge_data.connection_type {
                ConnectionType::TracerouteHop => 2.0,
                ConnectionType::LocalSubnet => 1.5,
                _ => 1.0,
            };

            output.push_str(&format!(
                r##"  <line x1="{:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}" 
       stroke="{}" stroke-width="{:.1}" marker-end="url(#arrowhead)"/>
"##,
                x1, y1, x2, y2, edge_color, stroke_width
            ));

            if options.edge_labels {
                if let Some(latency) = edge_data.latency_ms {
                    let mid_x = (x1 + x2) / 2.0;
                    let mid_y = (y1 + y2) / 2.0;
                    output.push_str(&format!(
                        r##"  <text x="{:.1}" y="{:.1}" font-size="8" fill="#666" text-anchor="middle">{}ms</text>
"##,
                        mid_x, mid_y, latency
                    ));
                }
            }
        }

        for idx in graph.graph.node_indices() {
            let node_data = match graph.graph.node_weight(idx) {
                Some(d) => d,
                None => continue,
            };
            let pos = match positions.get(&idx) {
                Some(p) => p,
                None => continue,
            };

            let x = pos.x * scale + offset_x;
            let y = pos.y * scale + offset_y;
            let radius = 15.0 + (node_data.risk_score as f32 / 100.0 * 10.0);

            let fill_color = Self::get_device_color(node_data.device_type);
            let risk_color = Self::get_risk_color(node_data.risk_score);
            let stroke_color = risk_color.unwrap_or_else(|| fill_color.clone());

            output.push_str(&format!(
                r##"  <circle cx="{:.1}" cy="{:.1}" r="{:.1}" 
       fill="{}" stroke="{}" stroke-width="2"/>
"##,
                x, y, radius, fill_color, stroke_color
            ));

            let label_x = x;
            let label_y = y + radius + 12.0;
            let label_text = node_data.ip.to_string();

            output.push_str(&format!(
                r##"  <text x="{:.1}" y="{:.1}" font-size="9"
       font-family="Helvetica" text-anchor="middle" fill="#333">{}</text>
"##,
                label_x, label_y, label_text
            ));

            if options.include_hostnames {
                if let Some(hostname) = &node_data.hostname {
                    let hostname_y = label_y + 12.0;
                    output.push_str(&format!(
                        r##"  <text x="{:.1}" y="{:.1}" font-size="7"
       font-family="Helvetica" text-anchor="middle" fill="#666">{}</text>
"##,
                        label_x, hostname_y, hostname
                    ));
                }
            }
        }

        output.push_str("</svg>\n");

        output
    }

    pub fn save_to_file(content: &str, path: &Path) -> std::io::Result<()> {
        fs::write(path, content)
    }

    fn format_node_label(node_data: &NodeData, options: &ExportOptions) -> String {
        let mut label = node_data.ip.to_string();

        if options.include_hostnames {
            if let Some(hostname) = &node_data.hostname {
                label = format!("{}\n{}", hostname, label);
            }
        }

        if options.include_risk {
            let risk_level = Self::risk_level(node_data.risk_score);
            label = format!("{} [{}]", label, risk_level);
        }

        label
    }

    fn format_edge_label(edge_data: &EdgeData) -> String {
        let mut parts = Vec::new();

        if let Some(latency) = edge_data.latency_ms {
            parts.push(format!("{}ms", latency));
        }

        if let Some(hop_count) = edge_data.hop_count {
            if hop_count > 1 {
                parts.push(format!("{} hops", hop_count));
            }
        }

        parts.join(", ")
    }

    fn get_device_color(device_type: DeviceType) -> String {
        match device_type {
            DeviceType::Router => "#4CAF50".to_string(),
            DeviceType::Switch => "#8BC34A".to_string(),
            DeviceType::Firewall => "#F44336".to_string(),
            DeviceType::LoadBalancer => "#FF9800".to_string(),
            DeviceType::Server => "#2196F3".to_string(),
            DeviceType::Workstation => "#9C27B0".to_string(),
            DeviceType::Laptop => "#9C27B0".to_string(),
            DeviceType::Mobile => "#E91E63".to_string(),
            DeviceType::Tablet => "#E91E63".to_string(),
            DeviceType::Printer => "#607D8B".to_string(),
            DeviceType::IoT => "#00BCD4".to_string(),
            DeviceType::Camera => "#00BCD4".to_string(),
            DeviceType::Thermostat => "#00BCD4".to_string(),
            DeviceType::Speaker => "#00BCD4".to_string(),
            DeviceType::Light => "#00BCD4".to_string(),
            DeviceType::Lock => "#00BCD4".to_string(),
            DeviceType::Sensor => "#00BCD4".to_string(),
            DeviceType::NAS => "#3F51B5".to_string(),
            DeviceType::AccessPoint => "#4CAF50".to_string(),
            DeviceType::VMHost => "#673AB7".to_string(),
            DeviceType::Container => "#673AB7".to_string(),
            DeviceType::Database => "#2196F3".to_string(),
            DeviceType::WebServer => "#2196F3".to_string(),
            DeviceType::MailServer => "#2196F3".to_string(),
            DeviceType::DNS => "#2196F3".to_string(),
            DeviceType::DHCP => "#2196F3".to_string(),
            DeviceType::Directory => "#2196F3".to_string(),
            DeviceType::VPN => "#F44336".to_string(),
            DeviceType::Proxy => "#F44336".to_string(),
            DeviceType::FirewallAppliance => "#F44336".to_string(),
            DeviceType::Storage => "#3F51B5".to_string(),
            DeviceType::UPS => "#795548".to_string(),
            DeviceType::KVM => "#795548".to_string(),
            DeviceType::Internet => "#FFD700".to_string(), // Gold
            DeviceType::Unknown => "#9E9E9E".to_string(),
        }
    }

    fn get_risk_color(risk_score: u8) -> Option<String> {
        if risk_score >= 70 {
            Some("#D32F2F".to_string())
        } else if risk_score >= 40 {
            Some("#FF9800".to_string())
        } else if risk_score >= 20 {
            Some("#FFEB3B".to_string())
        } else {
            None
        }
    }

    fn get_edge_color(connection_type: ConnectionType) -> String {
        match connection_type {
            ConnectionType::LocalSubnet => "#2196F3".to_string(),
            ConnectionType::TracerouteHop => "#4CAF50".to_string(),
            ConnectionType::Inferred => "#9E9E9E".to_string(),
            ConnectionType::Manual => "#FF9800".to_string(),
            ConnectionType::Unknown => "#9E9E9E".to_string(),
        }
    }

    fn risk_level(score: u8) -> String {
        if score >= 70 {
            "HIGH".to_string()
        } else if score >= 40 {
            "MEDIUM".to_string()
        } else if score >= 20 {
            "LOW".to_string()
        } else {
            "NONE".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::topology::ConnectionType;
    use crate::topology::DeviceType;
    use crate::topology::EdgeData;
    use crate::topology::NodeData;
    use crate::topology::TopologyGraph;
    use glam::Vec2;
    use petgraph::Graph;
    use petgraph::Undirected;
    use std::collections::HashMap;
    use std::net::Ipv4Addr;
    use std::time::SystemTime;

    fn create_test_graph() -> (TopologyGraph, HashMap<NodeIndex, Vec2>) {
        let mut graph: Graph<NodeData, EdgeData, Undirected> = Graph::with_capacity(0, 0);
        let mut ip_to_node = HashMap::new();
        let mut positions = HashMap::new();

        let ips = [Ipv4Addr::new(192, 168, 1, 1), Ipv4Addr::new(192, 168, 1, 2)];

        for (i, &ip) in ips.iter().enumerate() {
            let node_data = NodeData {
                ip: std::net::IpAddr::V4(ip),
                mac: Some(format!("00:00:00:00:00:0{}", i + 1)),
                hostname: Some(format!("host-{}", i + 1)),
                vendor: Some("Test Vendor".to_string()),
                device_type: if i == 0 {
                    DeviceType::Router
                } else {
                    DeviceType::Server
                },
                os_fingerprint: None,
                ports: Vec::new(),
                risk_score: if i == 1 { 45 } else { 10 },
                geo_location: None,
                traceroute_hops: Vec::new(),
                first_seen: SystemTime::UNIX_EPOCH,
                last_seen: SystemTime::UNIX_EPOCH,
            };

            let idx = graph.add_node(node_data);
            ip_to_node.insert(std::net::IpAddr::V4(ip), idx);
            positions.insert(idx, Vec2::new(i as f32 * 100.0, i as f32 * 50.0));
        }

        if let (Some(&n0), Some(&n1)) = (
            ip_to_node.get(&std::net::IpAddr::V4(ips[0])),
            ip_to_node.get(&std::net::IpAddr::V4(ips[1])),
        ) {
            graph.update_edge(
                n0,
                n1,
                EdgeData {
                    connection_type: ConnectionType::LocalSubnet,
                    latency_ms: Some(5),
                    hop_count: Some(1),
                    bandwidth_estimate: None,
                },
            );
        }

        (TopologyGraph { graph }, positions)
    }

    #[test]
    fn test_export_dot_basic() {
        let (graph, positions) = create_test_graph();
        let options = ExportOptions::default();

        let dot = Exporter::export_dot(&graph, &positions, &options);

        assert!(dot.contains("digraph network"));
        assert!(dot.contains("192.168.1.1"));
        assert!(dot.contains("192.168.1.2"));
        assert!(dot.contains("->"));
    }

    #[test]
    fn test_export_dot_includes_risk() {
        let (graph, positions) = create_test_graph();
        let options = ExportOptions::default();

        let dot = Exporter::export_dot(&graph, &positions, &options);

        assert!(dot.contains("HIGH") || dot.contains("MEDIUM") || dot.contains("LOW"));
    }

    #[test]
    fn test_export_dot_node_attributes() {
        let (graph, positions) = create_test_graph();
        let options = ExportOptions::default();

        let dot = Exporter::export_dot(&graph, &positions, &options);

        assert!(dot.contains("shape=box"));
        assert!(dot.contains("style=filled"));
        assert!(dot.contains("fillcolor="));
    }

    #[test]
    fn test_export_json_basic() {
        let (graph, positions) = create_test_graph();
        let options = ExportOptions::default();

        let json = Exporter::export_json(&graph, &positions, &options).unwrap();

        assert!(json.contains("\"nodes\""));
        assert!(json.contains("\"edges\""));
        assert!(json.contains("192.168.1.1"));
    }

    #[test]
    fn test_export_json_valid_structure() {
        let (graph, positions) = create_test_graph();
        let options = ExportOptions::default();

        let json = Exporter::export_json(&graph, &positions, &options).unwrap();

        let parsed: serde_json::Value = serde_json::from_str(&json).expect("Valid JSON");
        assert!(parsed.get("node_count").is_some());
        assert!(parsed.get("edge_count").is_some());
        assert!(parsed.get("nodes").is_some());
        assert!(parsed.get("edges").is_some());
    }

    #[test]
    fn test_export_svg_basic() {
        let (graph, positions) = create_test_graph();
        let options = ExportOptions::default();

        let svg = Exporter::export_svg(&graph, &positions, &options, 800, 600);

        assert!(svg.contains("<svg"));
        assert!(svg.contains("</svg>"));
        assert!(svg.contains("<circle"));
        assert!(svg.contains("192.168.1.1"));
    }

    #[test]
    fn test_export_svg_with_edge_labels() {
        let (graph, positions) = create_test_graph();
        let options = ExportOptions::default();

        let svg = Exporter::export_svg(&graph, &positions, &options, 800, 600);

        assert!(svg.contains("5ms"));
    }

    #[test]
    fn test_save_to_file() {
        let temp_dir = std::env::temp_dir();
        let temp_file = temp_dir.join("test_topology.dot");

        let content = "digraph test { }";
        let result = Exporter::save_to_file(content, &temp_file);

        assert!(result.is_ok());
        assert!(temp_file.exists());

        let read_content = std::fs::read_to_string(&temp_file).unwrap();
        assert_eq!(read_content, content);

        let _ = std::fs::remove_file(temp_file);
    }

    #[test]
    fn test_device_color_router() {
        let color = Exporter::get_device_color(DeviceType::Router);
        assert_eq!(color, "#4CAF50");
    }

    #[test]
    fn test_device_color_server() {
        let color = Exporter::get_device_color(DeviceType::Server);
        assert_eq!(color, "#2196F3");
    }

    #[test]
    fn test_device_color_firewall() {
        let color = Exporter::get_device_color(DeviceType::Firewall);
        assert_eq!(color, "#F44336");
    }

    #[test]
    fn test_edge_color_local_subnet() {
        let color = Exporter::get_edge_color(ConnectionType::LocalSubnet);
        assert_eq!(color, "#2196F3");
    }

    #[test]
    fn test_edge_color_traceroute_hop() {
        let color = Exporter::get_edge_color(ConnectionType::TracerouteHop);
        assert_eq!(color, "#4CAF50");
    }

    #[test]
    fn test_risk_level_high() {
        assert_eq!(Exporter::risk_level(75), "HIGH");
    }

    #[test]
    fn test_risk_level_medium() {
        assert_eq!(Exporter::risk_level(50), "MEDIUM");
    }

    #[test]
    fn test_risk_level_low() {
        assert_eq!(Exporter::risk_level(25), "LOW");
    }

    #[test]
    fn test_risk_level_none() {
        assert_eq!(Exporter::risk_level(10), "NONE");
    }
}
