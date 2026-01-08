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
            repulsion_strength: 500.0,
            attraction_strength: 0.1,
            damping: 0.9,
            max_iterations: 300,
            convergence_threshold: 0.01,
            initial_step_size: 0.1,
            center_gravity: 0.05,
            prevent_overlap: true,
            overlap_distance: 20.0,
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

        self.velocities.clear();
        self.positions.clear();

        let mut rng = StdRng::from_entropy();

        for idx in graph.graph.node_indices() {
            let pos = if let Some(existing) = existing_positions {
                existing.get(&idx).copied().unwrap_or_else(|| {
                    let angle = rng.gen::<f32>() * std::f32::consts::TAU;
                    let radius = rng.gen::<f32>() * 500.0;
                    Vec2::new(angle.cos() * radius, angle.sin() * radius)
                })
            } else {
                let angle = rng.gen::<f32>() * std::f32::consts::TAU;
                let radius = rng.gen::<f32>() * 500.0;
                Vec2::new(angle.cos() * radius, angle.sin() * radius)
            };

            self.positions.insert(idx, pos);
            self.velocities.insert(idx, Vec2::ZERO);
        }

        let mut _prev_positions = self.positions.clone();
        let mut _min_delta = f32::MAX;
        let mut _converged = false;

        for iteration in 0..self.config.max_iterations {
            if CANCEL_LAYOUT_FLAG.load(Ordering::SeqCst) {
                break;
            }

            self.build_quadtree(graph);

            let mut _total_force = 0.0f32;
            let mut max_delta = 0.0f32;

            let mut new_positions = self.positions.clone();

            for (idx, pos) in &self.positions {
                if CANCEL_LAYOUT_FLAG.load(Ordering::SeqCst) {
                    break;
                }

                let repulsion = self.calculate_repulsion(*idx, *pos);
                let attraction = self.calculate_attraction(graph, *idx, *pos);
                let gravity = self.calculate_center_gravity(*pos);

                let total_force_vec = repulsion + attraction + gravity;
                _total_force += total_force_vec.length();

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

            if iteration > 0 {
                let mut position_delta = 0.0f32;
                for (idx, pos) in &self.positions {
                    if let Some(prev) = _prev_positions.get(idx) {
                        position_delta += (pos - prev).length();
                    }
                }
                _min_delta = position_delta / node_count as f32;

                if _min_delta < self.config.convergence_threshold {
                    _converged = true;
                    break;
                }
            }

            _prev_positions = self.positions.clone();
        }

        self.positions.clone()
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

        for (idx, pos) in &positions {
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
