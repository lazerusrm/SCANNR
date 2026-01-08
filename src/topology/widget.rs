use crate::topology::{ConnectionType, DeviceType, NodeData, TopologyGraph};
use egui::Color32;
use egui::Painter;
use egui::Pos2;
use egui::Rect;
use egui::Stroke;
use egui::Ui;
use egui::Vec2;
use petgraph::graph::NodeIndex;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutType {
    ForceDirected,
    Circular,
    Hierarchical,
}

impl Default for LayoutType {
    fn default() -> Self {
        LayoutType::ForceDirected
    }
}

#[derive(Debug, Clone)]
pub struct ForceDirectedConfig {
    pub repulsion_strength: f32,
    pub spring_length: f32,
    pub spring_strength: f32,
    pub damping: f32,
    pub theta: f32,
    pub max_iterations: usize,
    pub convergence_threshold: f32,
}

impl Default for ForceDirectedConfig {
    fn default() -> Self {
        Self {
            repulsion_strength: -300.0,
            spring_length: 100.0,
            spring_strength: 0.1,
            damping: 0.85,
            theta: 0.5,
            max_iterations: 300,
            convergence_threshold: 0.01,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct InteractionState {
    pub selected_nodes: HashSet<NodeIndex>,
    pub hovered_node: Option<NodeIndex>,
    pub pan_offset: Vec2,
    pub zoom: f32,
    pub is_dragging: bool,
    pub drag_start: Option<Pos2>,
    pub search_query: String,
}

impl InteractionState {
    pub fn new() -> Self {
        Self {
            selected_nodes: HashSet::new(),
            hovered_node: None,
            pan_offset: Vec2::ZERO,
            zoom: 1.0,
            is_dragging: false,
            drag_start: None,
            search_query: String::new(),
        }
    }
}

#[derive(Debug)]
pub struct TopologyWidget {
    graph: TopologyGraph,
    positions: HashMap<NodeIndex, Pos2>,
    velocities: HashMap<NodeIndex, Vec2>,
    interaction: InteractionState,
    layout_type: LayoutType,
    force_config: ForceDirectedConfig,
    visible_nodes: HashSet<NodeIndex>,
}

impl TopologyWidget {
    pub fn new(graph: TopologyGraph) -> Self {
        let mut positions = HashMap::new();
        let mut velocities = HashMap::new();

        for node_idx in graph.graph.node_indices() {
            positions.insert(node_idx, Pos2::new(400.0, 300.0));
            velocities.insert(node_idx, Vec2::ZERO);
        }

        Self {
            graph,
            positions,
            velocities,
            interaction: InteractionState::new(),
            layout_type: LayoutType::ForceDirected,
            force_config: ForceDirectedConfig::default(),
            visible_nodes: HashSet::new(),
        }
    }

    pub fn set_graph(&mut self, graph: TopologyGraph) {
        self.graph = graph;
        self.positions.clear();
        self.velocities.clear();

        for node_idx in self.graph.graph.node_indices() {
            self.positions.insert(node_idx, Pos2::new(400.0, 300.0));
            self.velocities.insert(node_idx, Vec2::ZERO);
        }

        self.compute_circular_layout(150.0);
    }

    pub fn show(&mut self, ui: &mut Ui, viewport_rect: Rect) {
        let painter = ui.painter_at(viewport_rect);

        painter.rect_filled(viewport_rect, 0.0, Color32::from_rgb(25, 25, 35));
        painter.rect_stroke(
            viewport_rect,
            0.0,
            Stroke::new(2.0, Color32::from_rgb(80, 80, 100)),
        );

        if self.graph.graph.node_count() == 0 {
            self.render_empty_state(ui, viewport_rect);
            return;
        }

        if self.positions.is_empty() {
            self.compute_circular_layout(150.0);
        }

        self.handle_interactions(ui, viewport_rect);
        self.render_edges(&painter);
        self.render_nodes(&painter, viewport_rect);
    }

    fn render_empty_state(&self, ui: &mut Ui, viewport_rect: Rect) {
        let center = viewport_rect.center();
        ui.painter_at(viewport_rect).text(
            center,
            egui::Align2::CENTER_CENTER,
            "No topology data",
            egui::FontId::proportional(24.0),
            Color32::from_rgb(150, 150, 150),
        );
    }

    fn handle_interactions(&mut self, ui: &mut Ui, _viewport_rect: Rect) {
        let pointer = ui.input(|i| i.pointer.clone());

        if pointer.any_down() && pointer.delta().length() > 0.5 {
            let delta = pointer.delta();
            self.interaction.pan_offset -= delta;
            self.interaction.is_dragging = true;

            for pos in self.positions.values_mut() {
                pos.x -= delta.x;
                pos.y -= delta.y;
            }
        }

        if !pointer.any_down() {
            self.interaction.is_dragging = false;
        }

        let hover_pos = ui.input(|i| i.pointer.hover_pos());
        if let Some(mouse_pos) = hover_pos {
            let mut closest: Option<(NodeIndex, f32)> = None;

            for (node_idx, pos) in &self.positions {
                let dist = (mouse_pos - *pos).length();
                if dist < 20.0 {
                    if closest.map(|(_, d)| dist < d).unwrap_or(true) {
                        closest = Some((*node_idx, dist));
                    }
                }
            }

            if let Some((node_idx, _)) = closest {
                self.interaction.hovered_node = Some(node_idx);
            } else {
                self.interaction.hovered_node = None;
            }
        }

        let zoom_delta = ui.input(|i| i.zoom_delta());
        if zoom_delta != 1.0 {
            let new_zoom = (self.interaction.zoom * zoom_delta).clamp(0.2, 3.0);
            self.interaction.zoom = new_zoom;
        }
    }

    fn render_edges(&self, painter: &Painter) {
        for edge_idx in self.graph.graph.edge_indices() {
            let (source, target) = match self.graph.graph.edge_endpoints(edge_idx) {
                Some(e) => e,
                None => continue,
            };

            let Some(source_pos) = self.positions.get(&source) else {
                continue;
            };
            let Some(target_pos) = self.positions.get(&target) else {
                continue;
            };

            let edge_data = self.graph.graph.edge_weight(edge_idx);

            let color = match edge_data.map(|e| e.connection_type) {
                Some(ConnectionType::LocalSubnet) => Color32::from_rgb(100, 180, 255),
                Some(ConnectionType::TracerouteHop) => Color32::from_rgb(100, 255, 150),
                Some(ConnectionType::Inferred) => Color32::from_rgb(255, 200, 100),
                _ => Color32::from_rgb(120, 120, 140),
            };

            painter.line_segment([*source_pos, *target_pos], Stroke::new(1.5, color));
        }
    }

    fn render_nodes(&self, painter: &Painter, viewport_rect: Rect) {
        let zoom = self.interaction.zoom;
        let radius = (12.0 * zoom.sqrt()).max(6.0);

        for node_idx in self.graph.graph.node_indices() {
            let Some(pos) = self.positions.get(&node_idx) else {
                continue;
            };

            if !viewport_rect.contains(*pos) {
                continue;
            }

            let node_data = match self.graph.graph.node_weight(node_idx) {
                Some(n) => n,
                None => continue,
            };

            let is_selected = self.interaction.selected_nodes.contains(&node_idx);
            let is_hovered = self.interaction.hovered_node == Some(node_idx);

            let fill_color = self.get_device_color(node_data.device_type);
            let stroke_color = if is_selected {
                Color32::from_rgb(255, 200, 0)
            } else if is_hovered {
                Color32::from_rgb(255, 255, 255)
            } else {
                Color32::from_rgb(200, 200, 200)
            };

            painter.circle_filled(*pos, radius, fill_color);
            painter.circle_stroke(*pos, radius, Stroke::new(2.0, stroke_color));

            if is_selected {
                painter.circle_stroke(
                    *pos,
                    radius + 4.0,
                    Stroke::new(2.0, Color32::from_rgb(255, 200, 0)),
                );
            }

            if zoom > 0.6 {
                let label = self.get_node_label(node_data);
                let text_pos = Pos2::new(pos.x, pos.y + radius + 14.0);
                painter.text(
                    text_pos,
                    egui::Align2::CENTER_TOP,
                    label,
                    egui::FontId::proportional(11.0 * zoom.sqrt()),
                    Color32::from_rgb(220, 220, 220),
                );
            }
        }
    }

    fn get_device_color(&self, device_type: DeviceType) -> Color32 {
        match device_type {
            DeviceType::Router | DeviceType::Switch | DeviceType::AccessPoint => {
                Color32::from_rgb(100, 200, 100)
            }
            DeviceType::Server
            | DeviceType::WebServer
            | DeviceType::Database
            | DeviceType::MailServer => Color32::from_rgb(80, 150, 220),
            DeviceType::Firewall | DeviceType::FirewallAppliance => Color32::from_rgb(220, 80, 80),
            DeviceType::IoT | DeviceType::Camera | DeviceType::Thermostat | DeviceType::Light => {
                Color32::from_rgb(100, 220, 220)
            }
            DeviceType::Workstation
            | DeviceType::Laptop
            | DeviceType::Mobile
            | DeviceType::Tablet => Color32::from_rgb(100, 160, 220),
            DeviceType::Printer => Color32::from_rgb(200, 180, 100),
            DeviceType::NAS | DeviceType::Storage => Color32::from_rgb(100, 150, 200),
            _ => Color32::from_rgb(120, 120, 130),
        }
    }

    fn get_node_label(&self, node_data: &NodeData) -> String {
        if let Some(hostname) = &node_data.hostname {
            if hostname.len() > 15 {
                hostname.chars().take(15).collect::<String>() + "..."
            } else {
                hostname.clone()
            }
        } else {
            node_data.ip.to_string()
        }
    }

    pub fn compute_layout(&mut self, layout_type: LayoutType) {
        match layout_type {
            LayoutType::ForceDirected => {
                self.compute_force_directed_layout();
            }
            LayoutType::Circular => {
                self.compute_circular_layout(150.0);
            }
            LayoutType::Hierarchical => {
                self.compute_circular_layout(150.0);
            }
        }
        self.layout_type = layout_type;
    }

    pub fn compute_force_directed_layout(&mut self) {
        let config = self.force_config.clone();
        let node_count = self.graph.graph.node_count();

        if node_count == 0 {
            return;
        }

        let center_x = 400.0;
        let center_y = 300.0;

        for (i, node_idx) in self.graph.graph.node_indices().enumerate() {
            let angle = (i as f32 / node_count as f32) * 2.0 * std::f32::consts::PI;
            let dist = 100.0;
            self.positions.insert(
                node_idx,
                Pos2::new(center_x + angle.cos() * dist, center_y + angle.sin() * dist),
            );
            self.velocities.insert(node_idx, Vec2::ZERO);
        }

        let mut positions: HashMap<NodeIndex, Pos2> = self.positions.clone();
        let mut velocities: HashMap<NodeIndex, Vec2> = HashMap::new();

        for _ in 0..config.max_iterations {
            let mut total_delta = 0.0;

            for (node_idx, pos) in &positions {
                let mut force = Vec2::ZERO;

                for (other_idx, other_pos) in &positions {
                    if node_idx == other_idx {
                        continue;
                    }

                    let delta = *pos - *other_pos;
                    let dist = delta.length().max(1.0);

                    let repulsion = config.repulsion_strength / (dist * dist);
                    force += delta.normalized() * repulsion;
                }

                velocities.insert(
                    *node_idx,
                    velocities.get(node_idx).copied().unwrap_or_default() * config.damping
                        + force * 0.1,
                );
            }

            for (node_idx, vel) in &velocities {
                if let Some(pos) = positions.get_mut(node_idx) {
                    let delta = *vel;
                    pos.x += delta.x;
                    pos.y += delta.y;
                    total_delta += delta.length();
                }
            }

            if total_delta < config.convergence_threshold * node_count as f32 {
                break;
            }
        }

        self.positions = positions;
        self.velocities = velocities;
    }

    pub fn compute_circular_layout(&mut self, radius: f32) {
        let node_count = self.graph.graph.node_count();

        if node_count == 0 {
            return;
        }

        let center_x = 400.0;
        let center_y = 300.0;

        let mut nodes: Vec<NodeIndex> = self.graph.graph.node_indices().collect();

        nodes.sort_by_key(|idx| {
            self.graph
                .graph
                .node_weight(*idx)
                .map(|n| format!("{:?}", n.device_type))
                .unwrap_or_default()
        });

        for (i, node_idx) in nodes.iter().enumerate() {
            let angle = (i as f32 / node_count as f32) * 2.0 * std::f32::consts::PI;
            let pos = Pos2::new(
                center_x + angle.cos() * radius,
                center_y + angle.sin() * radius,
            );
            self.positions.insert(*node_idx, pos);
        }
    }

    pub fn zoom_to_fit(&mut self) {
        if self.positions.is_empty() {
            return;
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

        let _width = max_x - min_x;
        let _height = max_y - min_y;
        let center_x = (min_x + max_x) / 2.0;
        let center_y = (min_y + max_y) / 2.0;

        let target_center = Pos2::new(400.0, 300.0);
        let offset = target_center - Pos2::new(center_x, center_y);

        for pos in self.positions.values_mut() {
            pos.x += offset.x;
            pos.y += offset.y;
        }

        self.interaction.zoom = 1.0;
        self.interaction.pan_offset = Vec2::ZERO;
    }

    pub fn reset_view(&mut self) {
        self.interaction.zoom = 1.0;
        self.interaction.pan_offset = Vec2::ZERO;
        self.interaction.selected_nodes.clear();
        self.compute_circular_layout(150.0);
    }

    pub fn node_count(&self) -> usize {
        self.graph.graph.node_count()
    }

    pub fn edge_count(&self) -> usize {
        self.graph.graph.edge_count()
    }

    pub fn get_stats(&self) -> WidgetTopologyStats {
        let mut router_count = 0;
        let mut server_count = 0;
        let mut iot_count = 0;
        let mut firewall_count = 0;
        let mut unknown_count = 0;

        for node in self.graph.graph.node_weights() {
            match node.device_type {
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
        }

        WidgetTopologyStats {
            node_count: self.graph.graph.node_count(),
            edge_count: self.graph.graph.edge_count(),
            router_count,
            server_count,
            iot_count,
            firewall_count,
            unknown_count,
        }
    }

    pub fn search(&mut self, query: &str) {
        self.interaction.search_query = query.to_string();

        if query.is_empty() {
            self.visible_nodes = self.graph.graph.node_indices().collect();
            return;
        }

        self.visible_nodes = self
            .graph
            .graph
            .node_indices()
            .filter(|idx| {
                if let Some(node) = self.graph.graph.node_weight(*idx) {
                    node.ip.to_string().contains(query)
                        || node
                            .hostname
                            .as_ref()
                            .map(|s| s.contains(query))
                            .unwrap_or(false)
                        || node
                            .vendor
                            .as_ref()
                            .map(|s| s.contains(query))
                            .unwrap_or(false)
                } else {
                    false
                }
            })
            .collect();
    }

    pub fn set_selection(&mut self, nodes: HashSet<NodeIndex>) {
        self.interaction.selected_nodes = nodes;
    }

    pub fn clear_selection(&mut self) {
        self.interaction.selected_nodes.clear();
    }
}

#[derive(Debug, Clone, Default)]
pub struct WidgetTopologyStats {
    pub node_count: usize,
    pub edge_count: usize,
    pub router_count: usize,
    pub server_count: usize,
    pub iot_count: usize,
    pub firewall_count: usize,
    pub unknown_count: usize,
}
