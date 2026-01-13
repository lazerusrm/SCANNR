use crate::topology::graph::TopologyGraph;
use crate::topology::NodeData;
use glam::Vec2;
use petgraph::graph::NodeIndex;
use petgraph::visit::EdgeRef;
use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};

static CANCEL_LAYOUT_FLAG: AtomicBool = AtomicBool::new(false);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutConfig {
    pub repulsion_strength: f32,
    pub attraction_strength: f32,
    pub damping: f32,
    pub max_iterations: usize,
    pub convergence_threshold: f32,
    pub initial_step_size: f32,
    pub center_gravity: f32,
    pub prevent_overlap: bool,
    pub overlap_distance: f32,
}

impl Default for LayoutConfig {
    fn default() -> Self {
        LayoutConfig {
            repulsion_strength: 1200.0, // Increased
            attraction_strength: 0.005, // Lowered
            damping: 0.2, // Very high damping for stability
            max_iterations: 1500,
            convergence_threshold: 0.1,
            initial_step_size: 0.01,
            center_gravity: 0.002,
            prevent_overlap: true,
            overlap_distance: 100.0, // More space between nodes
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutProgress {
    pub iteration: usize,
    pub total_iterations: usize,
    pub energy: f32,
    pub delta_max: f32,
    pub converged: bool,
}

impl LayoutProgress {
    pub fn new(iteration: usize, total_iterations: usize, energy: f32, delta_max: f32) -> Self {
        LayoutProgress {
            iteration,
            total_iterations,
            energy,
            delta_max,
            converged: false,
        }
    }
}

#[derive(Debug)]
pub struct LayoutEngine {
    config: LayoutConfig,
    velocities: HashMap<NodeIndex, Vec2>,
    positions: HashMap<NodeIndex, Vec2>,
    quadtree: Option<QuadTree>,
}

impl Default for LayoutEngine {
    fn default() -> Self {
        Self::new(LayoutConfig::default())
    }
}

impl LayoutEngine {
    pub fn new(config: LayoutConfig) -> Self {
        LayoutEngine {
            config,
            velocities: HashMap::new(),
            positions: HashMap::new(),
            quadtree: None,
        }
    }

    pub fn compute(
        &mut self,
        graph: &TopologyGraph,
        existing_positions: Option<&HashMap<NodeIndex, Vec2>>,
    ) -> HashMap<NodeIndex, Vec2> {
        CANCEL_LAYOUT_FLAG.store(false, Ordering::SeqCst);

        let node_count = graph.graph.node_count();
        if node_count == 0 {
            return HashMap::new();
        }

        // Initialize positions if needed
        if self.positions.is_empty() || self.positions.len() != node_count {
             self.velocities.clear();
             self.positions.clear();
             
             // Check if we have existing positions to preserve
             if let Some(existing) = existing_positions {
                 if !existing.is_empty() {
                     self.positions = existing.clone();
                     // Fill missing
                     let mut rng = StdRng::from_entropy();
                     for idx in graph.graph.node_indices() {
                         self.positions.entry(idx).or_insert_with(|| {
                            let angle = rng.gen::<f32>() * std::f32::consts::TAU;
                            let radius = rng.gen::<f32>() * 500.0;
                            Vec2::new(angle.cos() * radius, angle.sin() * radius)
                         });
                         self.velocities.entry(idx).or_insert(Vec2::ZERO);
                     }
                 } else {
                     // No existing positions, use structured initialization
                     self.initialize_structured(graph);
                 }
             } else {
                 self.initialize_structured(graph);
             }
        }

        // Run iterations
        for _ in 0..self.config.max_iterations {
            if CANCEL_LAYOUT_FLAG.load(Ordering::SeqCst) {
                break;
            }
            if self.step(graph) < self.config.convergence_threshold {
                break;
            }
        }

        self.positions.clone()
    }

    pub fn apply_layout(&mut self, graph: &TopologyGraph, layout_type: crate::topology::LayoutType) {
        self.velocities.clear();
        self.positions.clear();
        
        match layout_type {
            crate::topology::LayoutType::Hierarchical => self.initialize_structured(graph),
            crate::topology::LayoutType::Circular => self.apply_circular(graph),
            crate::topology::LayoutType::ForceDirected => self.apply_random(graph),
        }
    }

    fn apply_random(&mut self, graph: &TopologyGraph) {
        let mut rng = StdRng::from_entropy();
        for idx in graph.graph.node_indices() {
            let angle = rng.gen::<f32>() * std::f32::consts::TAU;
            let radius = rng.gen::<f32>() * 500.0;
            self.positions.insert(idx, Vec2::new(angle.cos() * radius, angle.sin() * radius));
            self.velocities.insert(idx, Vec2::ZERO);
        }
    }

    fn apply_circular(&mut self, graph: &TopologyGraph) {
        let node_count = graph.graph.node_count();
        if node_count == 0 { return; }
        
        let mut sorted_nodes: Vec<_> = graph.graph.node_indices().collect();
        // Sort by device type for some grouping
        sorted_nodes.sort_by_key(|&idx| {
            graph.graph.node_weight(idx).map(|n| n.device_type)
        });

        let radius = (node_count as f32 * 50.0).max(300.0);
        
        for (i, &idx) in sorted_nodes.iter().enumerate() {
            let angle = (i as f32 / node_count as f32) * std::f32::consts::TAU;
            let pos = Vec2::new(angle.cos() * radius, angle.sin() * radius);
            self.positions.insert(idx, pos);
            self.velocities.insert(idx, Vec2::ZERO);
        }
    }

    fn initialize_structured(&mut self, graph: &TopologyGraph) {
        let _center = Vec2::ZERO;
        
        // 1. Identify Root (Internet) and Infrastructure
        let mut root_nodes = Vec::new();
        let mut infrastructure = Vec::new();
        let mut leaf_nodes = Vec::new();
        
        for idx in graph.graph.node_indices() {
            if let Some(node) = graph.graph.node_weight(idx) {
                match node.device_type {
                    crate::topology::DeviceType::Internet => root_nodes.push(idx),
                    crate::topology::DeviceType::Router | 
                    crate::topology::DeviceType::Firewall | 
                    crate::topology::DeviceType::FirewallAppliance |
                    crate::topology::DeviceType::Switch |
                    crate::topology::DeviceType::AccessPoint |
                    crate::topology::DeviceType::LoadBalancer => infrastructure.push(idx),
                    _ => leaf_nodes.push(idx),
                }
            }
            self.velocities.insert(idx, Vec2::ZERO);
        }

        // 2. Assign Levels
        let mut visited = std::collections::HashSet::new();
        
        // Level 0: Roots (Internet) - Top Center
        for &idx in &root_nodes {
            visited.insert(idx);
            self.positions.insert(idx, Vec2::new(0.0, -400.0));
        }

        // Level 1: Infrastructure (Routers) - Directly Under Internet
        // If no infrastructure found, fallback to IP convention
        if infrastructure.is_empty() && root_nodes.is_empty() {
             let mut potential_gateways = Vec::new();
             let mut others = Vec::new();
             for &idx in &leaf_nodes {
                 if let Some(node) = graph.graph.node_weight(idx) {
                     if let std::net::IpAddr::V4(ipv4) = node.ip {
                         if ipv4.octets()[3] == 1 || ipv4.octets()[3] == 254 {
                             potential_gateways.push(idx);
                             continue;
                         }
                     }
                 }
                 others.push(idx);
             }
             if !potential_gateways.is_empty() {
                 infrastructure = potential_gateways;
                 leaf_nodes = others;
             }
        }

        let l1_count = infrastructure.len();
        let l1_width = l1_count as f32 * 200.0;
        let l1_start_x = -l1_width / 2.0;
        
        for (i, &idx) in infrastructure.iter().enumerate() {
            let x = if l1_count == 1 { 0.0 } else { l1_start_x + i as f32 * 200.0 };
            let pos = Vec2::new(x, -200.0);
            self.positions.insert(idx, pos);
            visited.insert(idx);
        }
        
        // Level 2: Hosts - Fan out below
        let hub_idx = if !infrastructure.is_empty() {
            Some(infrastructure[0])
        } else if !root_nodes.is_empty() {
            Some(root_nodes[0])
        } else {
            None
        };

        // Sort leaf nodes by IP (last octet) for logical ordering
        leaf_nodes.sort_by(|&a, &b| {
            let ip_a = graph.graph.node_weight(a).map(|n| n.ip);
            let ip_b = graph.graph.node_weight(b).map(|n| n.ip);
            
            match (ip_a, ip_b) {
                (Some(std::net::IpAddr::V4(ipa)), Some(std::net::IpAddr::V4(ipb))) => {
                    ipa.octets()[3].cmp(&ipb.octets()[3])
                },
                (Some(std::net::IpAddr::V6(_)), Some(std::net::IpAddr::V4(_))) => std::cmp::Ordering::Greater,
                (Some(std::net::IpAddr::V4(_)), Some(std::net::IpAddr::V6(_))) => std::cmp::Ordering::Less,
                _ => std::cmp::Ordering::Equal,
            }
        });

        if let Some(hub) = hub_idx {
            let hub_pos = *self.positions.get(&hub).unwrap();
            
            // Fan out in a semi-circle or arc below the router
            let mut current_row = 1;
            let mut nodes_in_row = 0;
            let mut max_in_row = 8;
            let row_height = 150.0;
            
            for &idx in &leaf_nodes {
                if nodes_in_row >= max_in_row {
                    current_row += 1;
                    nodes_in_row = 0;
                    max_in_row = (current_row * 8).max(1);
                }
                
                let row_width = max_in_row as f32 * 100.0;
                let row_start_x = hub_pos.x - row_width / 2.0;
                
                let x = row_start_x + (nodes_in_row as f32 * 100.0);
                let y = hub_pos.y + (current_row as f32 * row_height);
                
                self.positions.insert(idx, Vec2::new(x, y));
                nodes_in_row += 1;
            }
        } else {
            // Grid fallback
            let cols = (leaf_nodes.len() as f32).sqrt().ceil() as usize;
            for (i, &idx) in leaf_nodes.iter().enumerate() {
                let c = i % cols;
                let r = i / cols;
                self.positions.insert(idx, Vec2::new(c as f32 * 100.0, r as f32 * 100.0));
            }
        }
    }    
    // Returns max_delta (movement)
    pub fn step(&mut self, graph: &TopologyGraph) -> f32 {
        self.build_quadtree(graph);

        let mut max_delta = 0.0f32;
        let mut new_positions = self.positions.clone();

        for (idx, pos) in &self.positions {
            let repulsion = self.calculate_repulsion(*idx, *pos);
            let attraction = self.calculate_attraction(graph, *idx, *pos);
            let gravity = self.calculate_center_gravity(*pos);

            let mut total_force_vec = repulsion + attraction + gravity;
            
            // Hard Collision / Overlap Prevention
            if self.config.prevent_overlap {
                let mut collision_force = Vec2::ZERO;
                // Check neighbors (using quadtree would be faster but for small N simple check is ok for now)
                // Actually we can use the quadtree we just built if we added a query method,
                // but let's just do a simple check against nearby nodes or use the repulsion loop logic.
                // For simplicity/robustness here: iterate all. (Optimization: Use Quadtree query)
                for (&other_idx, &other_pos) in &self.positions {
                    if *idx == other_idx { continue; }
                    let diff = *pos - other_pos;
                    let dist_sq = diff.length_squared();
                    let min_dist_sq = self.config.overlap_distance * self.config.overlap_distance;
                    
                    if dist_sq < min_dist_sq && dist_sq > 0.0001 {
                        let dist = dist_sq.sqrt();
                        let penetration = self.config.overlap_distance - dist;
                        // Strong push back
                        collision_force += diff.normalize() * (penetration * 10.0); 
                    }
                }
                total_force_vec += collision_force;
            }

            let velocity = self.velocities.get(idx).copied().unwrap_or(Vec2::ZERO);
            let new_velocity = (velocity + total_force_vec * self.config.initial_step_size)
                * self.config.damping;
            let new_pos = *pos + new_velocity;

            let delta = (new_pos - *pos).length();
            max_delta = max_delta.max(delta);

            new_positions.insert(*idx, new_pos);
            self.velocities.insert(*idx, new_velocity);
        }

        self.positions = new_positions;
        max_delta
    }
    
    // Helper to update a single position (e.g. dragging)
    pub fn set_position(&mut self, idx: NodeIndex, pos: Vec2) {
        self.positions.insert(idx, pos);
        self.velocities.insert(idx, Vec2::ZERO); // Stop momentum when dragging
    }
    
    pub fn get_positions(&self) -> &HashMap<NodeIndex, Vec2> {
        &self.positions
    }

    fn build_quadtree(&mut self, graph: &TopologyGraph) {
        let bounds = self.calculate_bounds();

        self.quadtree = Some(QuadTree::new(bounds));

        for (&idx, &pos) in &self.positions {
            if let Some(node_data) = graph.graph.node_weight(idx) {
                self.quadtree.as_mut().unwrap().insert(pos, idx, node_data);
            }
        }
    }

    fn calculate_bounds(&self) -> Bounds {
        if self.positions.is_empty() {
            return Bounds::new(-1000.0, -1000.0, 2000.0, 2000.0);
        }

        let mut min_x = f32::MAX;
        let mut min_y = f32::MAX;
        let mut max_x = f32::MIN;
        let mut max_y = f32::MIN;

        for pos in self.positions.values() {
            min_x = min_x.min(pos.x);
            min_y = min_y.min(pos.y);
            max_x = max_x.max(pos.x);
            max_y = max_y.max(pos.y);
        }

        let padding = 100.0;
        Bounds::new(
            min_x - padding,
            min_y - padding,
            (max_x - min_x) + padding * 2.0,
            (max_y - min_y) + padding * 2.0,
        )
    }

    fn calculate_repulsion(&self, _idx: NodeIndex, pos: Vec2) -> Vec2 {
        if self.quadtree.is_none() {
            let mut repulsion = Vec2::ZERO;

            for (&_other_idx, &other_pos) in &self.positions {
                if other_pos != pos {
                    let diff = pos - other_pos;
                    let dist_sq = diff.length_squared();
                    if dist_sq > 0.001 {
                        let _dist = dist_sq.sqrt();
                        let force = self.config.repulsion_strength / dist_sq;
                        repulsion += diff.normalize_or_zero() * force;
                    }
                }
            }

            return repulsion;
        }

        let quadtree = self.quadtree.as_ref().unwrap();
        self.calculate_repulsion_barnes_hut(quadtree, pos)
    }

    fn calculate_repulsion_barnes_hut(&self, quadtree: &QuadTree, pos: Vec2) -> Vec2 {
        let mut repulsion = Vec2::ZERO;

        self.calculate_repulsion_recursive(quadtree, pos, &mut repulsion, None);

        repulsion
    }

    fn calculate_repulsion_recursive(
        &self,
        node: &QuadTree,
        pos: Vec2,
        total_force: &mut Vec2,
        _parent_center: Option<Vec2>,
    ) {
        let center = node.center();
        let diff = pos - center;
        let dist = diff.length();

        let node_width = node.bounds.width;

        if node.is_leaf() {
            if let Some((node_pos, _)) = node.get_point() {
                let diff = node_pos - pos;
                if diff.length() > 0.001 {
                    let dist_sq = diff.length_squared();
                    if dist_sq > 0.001 {
                        let force = self.config.repulsion_strength / dist_sq;
                        *total_force += diff.normalize_or_zero() * force;
                    }
                }
            }
        } else if dist > 0.0 && (node_width / dist) < 0.5 {
            let force =
                self.config.repulsion_strength / dist * self.config.repulsion_strength / dist;
            *total_force += diff.normalize_or_zero() * force;
        } else if let Some(children_iter) = node.children() {
            for child in children_iter {
                self.calculate_repulsion_recursive(child, pos, total_force, Some(center));
            }
        }
    }

    fn calculate_attraction(&self, graph: &TopologyGraph, idx: NodeIndex, pos: Vec2) -> Vec2 {
        let mut attraction = Vec2::ZERO;

        for edge in graph.graph.edges(idx) {
            let edge_id = edge.id();
            let endpoints = graph.graph.edge_endpoints(edge_id);
            if endpoints.is_none() {
                continue;
            }
            let (src, dst) = endpoints.unwrap();
            let neighbor_idx = if src == idx { dst } else { src };

            if let Some(&neighbor_pos) = self.positions.get(&neighbor_idx) {
                let diff = neighbor_pos - pos;
                let dist = diff.length();

                if dist > 0.001 {
                    let force = dist * self.config.attraction_strength;
                    attraction += diff.normalize_or_zero() * force;
                }
            }
        }

        attraction
    }

    fn calculate_center_gravity(&self, pos: Vec2) -> Vec2 {
        Vec2::new(-pos.x, -pos.y) * self.config.center_gravity
    }

    pub fn cancel(&mut self) {
        CANCEL_LAYOUT_FLAG.store(true, Ordering::SeqCst);
    }

    pub fn reset_cancel(&mut self) {
        CANCEL_LAYOUT_FLAG.store(false, Ordering::SeqCst);
    }
}

#[derive(Debug, Clone)]
pub struct Bounds {
    pub min_x: f32,
    pub min_y: f32,
    pub width: f32,
    pub height: f32,
}

impl Bounds {
    pub fn new(min_x: f32, min_y: f32, width: f32, height: f32) -> Self {
        Bounds {
            min_x,
            min_y,
            width,
            height,
        }
    }

    pub fn center(&self) -> Vec2 {
        Vec2::new(
            self.min_x + self.width / 2.0,
            self.min_y + self.height / 2.0,
        )
    }

    pub fn contains(&self, pos: Vec2) -> bool {
        pos.x >= self.min_x
            && pos.x <= self.min_x + self.width
            && pos.y >= self.min_y
            && pos.y <= self.min_y + self.height
    }
}

#[derive(Debug)]
struct QuadTree {
    bounds: Bounds,
    points: Vec<(Vec2, NodeIndex)>,
    children: Option<Box<[QuadTree; 4]>>,
    capacity: usize,
    subdivided: bool,
}

impl QuadTree {
    pub fn new(bounds: Bounds) -> Self {
        QuadTree {
            bounds,
            points: Vec::new(),
            children: None,
            capacity: 4,
            subdivided: false,
        }
    }

    pub fn insert(&mut self, pos: Vec2, idx: NodeIndex, _node_data: &NodeData) {
        if !self.bounds.contains(pos) {
            return;
        }

        if self.children.is_none() {
            if self.points.len() < self.capacity {
                self.points.push((pos, idx));
                return;
            }

            self.subdivide();
        }

        for child in self.children.as_mut().unwrap().iter_mut() {
            child.insert(pos, idx, _node_data);
        }
    }

    fn subdivide(&mut self) {
        let half_w = self.bounds.width / 2.0;
        let half_h = self.bounds.height / 2.0;

        let mut children = Box::new([
            QuadTree::new(Bounds::new(
                self.bounds.min_x,
                self.bounds.min_y,
                half_w,
                half_h,
            )),
            QuadTree::new(Bounds::new(
                self.bounds.min_x + half_w,
                self.bounds.min_y,
                half_w,
                half_h,
            )),
            QuadTree::new(Bounds::new(
                self.bounds.min_x,
                self.bounds.min_y + half_h,
                half_w,
                half_h,
            )),
            QuadTree::new(Bounds::new(
                self.bounds.min_x + half_w,
                self.bounds.min_y + half_h,
                half_w,
                half_h,
            )),
        ]);

        for (pos, idx) in self.points.drain(..) {
            for child in children.iter_mut() {
                if child.bounds.contains(pos) {
                    child.insert(
                        pos,
                        idx,
                        &NodeData {
                            ip: std::net::IpAddr::V4(std::net::Ipv4Addr::new(0, 0, 0, 0)),
                            mac: None,
                            hostname: None,
                            vendor: None,
                            device_type: crate::topology::DeviceType::Unknown,
                            os_fingerprint: None,
                            ports: Vec::new(),
                            risk_score: 0,
                            geo_location: None,
                            traceroute_hops: Vec::new(),
                            first_seen: std::time::SystemTime::UNIX_EPOCH,
                            last_seen: std::time::SystemTime::UNIX_EPOCH,
                        },
                    );
                    break;
                }
            }
        }

        self.children = Some(children);
        self.subdivided = true;
    }

    pub fn is_leaf(&self) -> bool {
        self.children.is_none()
    }

    pub fn center(&self) -> Vec2 {
        self.bounds.center()
    }

    pub fn get_point(&self) -> Option<(Vec2, NodeIndex)> {
        if self.points.len() == 1 {
            Some(self.points[0])
        } else {
            None
        }
    }

    pub fn children(&self) -> Option<std::slice::Iter<'_, QuadTree>> {
        self.children.as_ref().map(|c| c.iter())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::topology::device::DeviceType;
    use crate::topology::ConnectionType;
    use crate::topology::EdgeData;
    use crate::topology::NodeData;
    use petgraph::Graph;
    use petgraph::Undirected;
    use std::net::Ipv4Addr;
    use std::time::SystemTime;

    fn create_test_graph() -> TopologyGraph {
        let mut graph: Graph<NodeData, EdgeData, Undirected> = Graph::with_capacity(0, 0);
        let mut ip_to_node = HashMap::new();

        let ips = [
            Ipv4Addr::new(192, 168, 1, 1),
            Ipv4Addr::new(192, 168, 1, 2),
            Ipv4Addr::new(192, 168, 1, 3),
            Ipv4Addr::new(192, 168, 1, 4),
        ];

        for (i, &ip) in ips.iter().enumerate() {
            let node_data = NodeData {
                ip: std::net::IpAddr::V4(ip),
                mac: None,
                hostname: None,
                vendor: None,
                device_type: if i == 0 {
                    DeviceType::Router
                } else {
                    DeviceType::Server
                },
                os_fingerprint: None,
                ports: Vec::new(),
                risk_score: 0,
                geo_location: None,
                traceroute_hops: Vec::new(),
                first_seen: SystemTime::UNIX_EPOCH,
                last_seen: SystemTime::UNIX_EPOCH,
            };

            let idx = graph.add_node(node_data);
            ip_to_node.insert(std::net::IpAddr::V4(ip), idx);
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
                    latency_ms: Some(2),
                    hop_count: Some(1),
                    bandwidth_estimate: None,
                },
            );
        }

        if let (Some(&n0), Some(&n2)) = (
            ip_to_node.get(&std::net::IpAddr::V4(ips[0])),
            ip_to_node.get(&std::net::IpAddr::V4(ips[2])),
        ) {
            graph.update_edge(
                n0,
                n2,
                EdgeData {
                    connection_type: ConnectionType::LocalSubnet,
                    latency_ms: Some(3),
                    hop_count: Some(1),
                    bandwidth_estimate: None,
                },
            );
        }

        if let (Some(&n1), Some(&n3)) = (
            ip_to_node.get(&std::net::IpAddr::V4(ips[1])),
            ip_to_node.get(&std::net::IpAddr::V4(ips[3])),
        ) {
            graph.update_edge(
                n1,
                n3,
                EdgeData {
                    connection_type: ConnectionType::LocalSubnet,
                    latency_ms: Some(1),
                    hop_count: Some(1),
                    bandwidth_estimate: None,
                },
            );
        }

        TopologyGraph { graph }
    }

    #[test]
    fn test_layout_compute_empty_graph() {
        let mut engine = LayoutEngine::new(LayoutConfig::default());
        let empty_graph = TopologyGraph::new();

        let positions = engine.compute(&empty_graph, None);

        assert!(positions.is_empty());
    }

    #[test]
    fn test_layout_compute_basic_graph() {
        let mut engine = LayoutEngine::new(LayoutConfig::default());
        let graph = create_test_graph();

        let positions = engine.compute(&graph, None);

        assert_eq!(positions.len(), 4);

        for (_idx, pos) in &positions {
            assert!(!pos.x.is_nan());
            assert!(!pos.y.is_nan());
            assert!(pos.x.is_finite());
            assert!(pos.y.is_finite());
        }
    }

    #[test]
    fn test_layout_preserves_existing_positions() {
        let mut engine = LayoutEngine::new(LayoutConfig::default());
        let graph = create_test_graph();

        let mut existing = HashMap::new();
        for idx in graph.graph.node_indices() {
            existing.insert(idx, Vec2::new(1000.0, 1000.0));
        }

        let positions = engine.compute(&graph, Some(&existing));

        for pos in positions.values() {
            assert!((pos.x - 1000.0).abs() > 1.0 || (pos.y - 1000.0).abs() > 1.0);
        }
    }

    #[test]
    fn test_layout_config_defaults() {
        let config = LayoutConfig::default();

        assert_eq!(config.repulsion_strength, 500.0);
        assert_eq!(config.attraction_strength, 0.1);
        assert_eq!(config.damping, 0.9);
        assert_eq!(config.max_iterations, 300);
        assert!(config.convergence_threshold > 0.0);
    }

    #[test]
    fn test_bounds_creation() {
        let bounds = Bounds::new(0.0, 0.0, 100.0, 100.0);

        assert_eq!(bounds.min_x, 0.0);
        assert_eq!(bounds.min_y, 0.0);
        assert_eq!(bounds.width, 100.0);
        assert_eq!(bounds.height, 100.0);
    }

    #[test]
    fn test_bounds_center() {
        let bounds = Bounds::new(0.0, 0.0, 100.0, 100.0);
        let center = bounds.center();

        assert!((center.x - 50.0).abs() < 0.001);
        assert!((center.y - 50.0).abs() < 0.001);
    }

    #[test]
    fn test_bounds_contains() {
        let bounds = Bounds::new(0.0, 0.0, 100.0, 100.0);

        assert!(bounds.contains(Vec2::new(50.0, 50.0)));
        assert!(bounds.contains(Vec2::new(0.0, 0.0)));
        assert!(bounds.contains(Vec2::new(100.0, 100.0)));
        assert!(!bounds.contains(Vec2::new(-1.0, 50.0)));
        assert!(!bounds.contains(Vec2::new(50.0, -1.0)));
        assert!(!bounds.contains(Vec2::new(101.0, 50.0)));
        assert!(!bounds.contains(Vec2::new(50.0, 101.0)));
    }

    #[test]
    fn test_quadtree_insert_and_query() {
        let bounds = Bounds::new(0.0, 0.0, 100.0, 100.0);
        let mut quadtree = QuadTree::new(bounds);

        let node_data = NodeData {
            ip: std::net::IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)),
            mac: None,
            hostname: None,
            vendor: None,
            device_type: DeviceType::Router,
            os_fingerprint: None,
            ports: Vec::new(),
            risk_score: 0,
            geo_location: None,
            traceroute_hops: Vec::new(),
            first_seen: SystemTime::UNIX_EPOCH,
            last_seen: SystemTime::UNIX_EPOCH,
        };

        quadtree.insert(Vec2::new(25.0, 25.0), NodeIndex::new(0), &node_data);
        quadtree.insert(Vec2::new(75.0, 75.0), NodeIndex::new(1), &node_data);
        quadtree.insert(Vec2::new(25.0, 75.0), NodeIndex::new(2), &node_data);
        quadtree.insert(Vec2::new(75.0, 25.0), NodeIndex::new(3), &node_data);
        quadtree.insert(Vec2::new(50.0, 50.0), NodeIndex::new(4), &node_data);

        assert!(!quadtree.is_leaf());
    }

    #[test]
    fn test_layout_cancel() {
        let mut engine = LayoutEngine::new(LayoutConfig::default());

        engine.cancel();
        assert!(CANCEL_LAYOUT_FLAG.load(Ordering::SeqCst));

        engine.reset_cancel();
        assert!(!CANCEL_LAYOUT_FLAG.load(Ordering::SeqCst));
    }
}
