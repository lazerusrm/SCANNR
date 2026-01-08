use std::collections::HashMap;

use egui::{Color32, Pos2, Rect, Stroke, Vec2 as EguiVec2};
use glam::Vec2;
use petgraph::graph::NodeIndex;

use crate::topology::device::DeviceType;
use crate::topology::geo::GeoInfo;
use crate::topology::graph::TopologyGraph;
use crate::topology::layout::{LayoutConfig, LayoutEngine};
use crate::topology::{Hop, NodeData, PortInfo, TopologyViewState};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LODLevel {
    #[default]
    Minimal,
    Low,
    Medium,
    High,
    Full,
}

impl LODLevel {
    pub fn from_zoom_and_count(zoom: f32, node_count: usize) -> Self {
        if node_count > 1000 {
            if zoom < 0.05 {
                LODLevel::Minimal
            } else if zoom < 0.15 {
                LODLevel::Low
            } else if zoom < 0.3 {
                LODLevel::Medium
            } else {
                LODLevel::High
            }
        } else if node_count > 100 {
            if zoom < 0.1 {
                LODLevel::Low
            } else if zoom < 0.25 {
                LODLevel::Medium
            } else if zoom < 0.5 {
                LODLevel::High
            } else {
                LODLevel::Full
            }
        } else {
            if zoom < 0.2 {
                LODLevel::Medium
            } else if zoom < 0.5 {
                LODLevel::High
            } else {
                LODLevel::Full
            }
        }
    }

    pub fn show_labels(&self) -> bool {
        match self {
            LODLevel::Minimal => false,
            LODLevel::Low => false,
            LODLevel::Medium => true,
            LODLevel::High => true,
            LODLevel::Full => true,
        }
    }

    pub fn show_ports(&self) -> bool {
        match self {
            LODLevel::Minimal => false,
            LODLevel::Low => false,
            LODLevel::Medium => false,
            LODLevel::High => true,
            LODLevel::Full => true,
        }
    }

    pub fn show_icons(&self) -> bool {
        match self {
            LODLevel::Minimal => false,
            LODLevel::Low => true,
            LODLevel::Medium => true,
            LODLevel::High => true,
            LODLevel::Full => true,
        }
    }

    pub fn node_radius(&self) -> f32 {
        match self {
            LODLevel::Minimal => 3.0,
            LODLevel::Low => 5.0,
            LODLevel::Medium => 8.0,
            LODLevel::High => 12.0,
            LODLevel::Full => 16.0,
        }
    }

    pub fn label_font_size(&self) -> f32 {
        match self {
            LODLevel::Minimal => 8.0,
            LODLevel::Low => 10.0,
            LODLevel::Medium => 12.0,
            LODLevel::High => 14.0,
            LODLevel::Full => 16.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RenderConfig {
    pub background_color: Color32,
    pub node_fill_color: Color32,
    pub node_stroke_color: Color32,
    pub edge_color: Color32,
    pub edge_width: f32,
    pub selected_node_color: Color32,
    pub hovered_node_color: Color32,
    pub text_color: Color32,
    pub label_background: Color32,
    pub show_tooltips: bool,
    pub show_minimap: bool,
    pub show_stats_panel: bool,
    pub enable_animations: bool,
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            background_color: Color32::from_rgb(15, 15, 20),
            node_fill_color: Color32::from_rgb(60, 120, 180),
            node_stroke_color: Color32::from_rgb(100, 160, 220),
            edge_color: Color32::from_rgba_premultiplied(100, 150, 200, 120),
            edge_width: 1.5,
            selected_node_color: Color32::from_rgb(255, 200, 50),
            hovered_node_color: Color32::from_rgb(150, 220, 255),
            text_color: Color32::from_rgb(230, 230, 230),
            label_background: Color32::from_rgba_premultiplied(30, 30, 40, 200),
            show_tooltips: true,
            show_minimap: true,
            show_stats_panel: true,
            enable_animations: true,
        }
    }
}

struct QuadTree {
    bounds: Rect,
    points: Vec<(Pos2, NodeIndex)>,
    children: Option<Box<[QuadTree; 4]>>,
    capacity: usize,
}

impl QuadTree {
    fn new(bounds: Rect) -> Self {
        Self {
            bounds,
            points: Vec::new(),
            children: None,
            capacity: 4,
        }
    }

    fn insert(&mut self, point: Pos2, node_idx: NodeIndex) -> bool {
        if !self.bounds.contains(point) {
            return false;
        }

        if self.children.is_none() {
            if self.points.len() < self.capacity {
                self.points.push((point, node_idx));
                return true;
            }
            self.subdivide();
        }

        if let Some(ref mut children) = self.children {
            for child in children.iter_mut() {
                if child.insert(point, node_idx) {
                    return true;
                }
            }
        }
        false
    }

    fn subdivide(&mut self) {
        let half_width = self.bounds.width() / 2.0;
        let half_height = self.bounds.height() / 2.0;
        let min = self.bounds.min;

        self.children = Some(Box::new([
            QuadTree::new(Rect::from_min_size(
                min,
                EguiVec2::new(half_width, half_height),
            )),
            QuadTree::new(Rect::from_min_size(
                min + EguiVec2::new(half_width, 0.0),
                EguiVec2::new(half_width, half_height),
            )),
            QuadTree::new(Rect::from_min_size(
                min + EguiVec2::new(0.0, half_height),
                EguiVec2::new(half_width, half_height),
            )),
            QuadTree::new(Rect::from_min_size(
                min + EguiVec2::new(half_width, half_height),
                EguiVec2::new(half_width, half_height),
            )),
        ]));

        let old_points = std::mem::take(&mut self.points);
        for (point, node_idx) in old_points {
            if let Some(ref mut children) = self.children {
                for child in children.iter_mut() {
                    if child.insert(point, node_idx) {
                        break;
                    }
                }
            }
        }
    }

    fn query(&self, range: Rect) -> Vec<(Pos2, NodeIndex)> {
        let mut results = Vec::new();
        self.query_recursive(range, &mut results);
        results
    }

    fn query_recursive(&self, range: Rect, results: &mut Vec<(Pos2, NodeIndex)>) {
        if !self.bounds.intersects(range) {
            return;
        }

        for (point, node_idx) in &self.points {
            if range.contains(*point) {
                results.push((*point, *node_idx));
            }
        }

        if let Some(ref children) = self.children {
            for child in children.iter() {
                child.query_recursive(range, results);
            }
        }
    }
}

pub struct TopologyRenderer {
    pub graph: TopologyGraph,
    positions: HashMap<NodeIndex, Pos2>,
    velocities: HashMap<NodeIndex, Vec2>,
    quadtree: Option<QuadTree>,
    config: RenderConfig,
    layout_config: LayoutConfig,
    is_layout_computing: bool,
    layout_iteration: usize,
    tooltip_data: Option<HoveredNodeInfo>,
    minimap_rect: Option<Rect>,
    stats_panel_rect: Option<Rect>,
}

#[derive(Clone, Debug)]
pub struct HoveredNodeInfo {
    pub node_index: NodeIndex,
    pub node_data: NodeData,
    pub position: Pos2,
    pub ports: Vec<PortInfo>,
    pub geo_location: Option<GeoInfo>,
    pub traceroute_hops: Vec<Hop>,
}

impl TopologyRenderer {
    pub fn new(graph: TopologyGraph) -> Self {
        let positions = HashMap::new();
        let velocities = HashMap::new();

        Self {
            graph,
            positions,
            velocities,
            quadtree: None,
            config: RenderConfig::default(),
            layout_config: LayoutConfig::default(),
            is_layout_computing: false,
            layout_iteration: 0,
            tooltip_data: None,
            minimap_rect: None,
            stats_panel_rect: None,
        }
    }

    pub fn set_graph(&mut self, graph: TopologyGraph) {
        self.graph = graph;
        self.positions.clear();
        self.velocities.clear();
        self.quadtree = None;
        self.layout_iteration = 0;
    }

    pub fn compute_layout(&mut self) {
        let mut engine = LayoutEngine::new(self.layout_config.clone());

        let positions: HashMap<NodeIndex, Vec2> = self
            .positions
            .iter()
            .map(|(&idx, pos)| (idx, Vec2::new(pos.x, pos.y)))
            .collect();

        let new_positions = engine.compute(&self.graph, Some(&positions));

        self.positions = new_positions
            .into_iter()
            .map(|(idx, pos)| (idx, Pos2::new(pos.x, pos.y)))
            .collect();
    }

    fn build_quadtree(&mut self) {
        if self.positions.is_empty() {
            self.quadtree = None;
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

        let padding = 50.0;
        let bounds = Rect::from_min_max(
            Pos2::new(min_x - padding, min_y - padding),
            Pos2::new(max_x + padding, max_y + padding),
        );

        let mut quadtree = QuadTree::new(bounds);

        for (node_idx, pos) in &self.positions {
            quadtree.insert(*pos, *node_idx);
        }

        self.quadtree = Some(quadtree);
    }

    pub fn render(
        &mut self,
        ui: &mut egui::Ui,
        viewport_rect: Rect,
        view_state: &mut TopologyViewState,
    ) {
        let painter = ui.painter_at(viewport_rect);

        painter.rect_filled(viewport_rect, 0.0, Color32::from_rgb(30, 30, 40));
        painter.rect_stroke(
            viewport_rect,
            0.0,
            Stroke::new(2.0, Color32::from_rgb(100, 100, 120)),
        );

        let node_count = self.graph.graph.node_count();
        let _edge_count = self.graph.graph.edge_count();

        if node_count == 0 {
            self.render_empty_state(ui, viewport_rect);
            return;
        }

        if self.positions.is_empty() {
            self.initialize_layout(viewport_rect.size());
        }

        self.update_lod(view_state, viewport_rect.size());

        self.render_edges(&painter, view_state);
        self.render_nodes(&painter, view_state);

        if view_state.show_edges {
            self.render_edge_labels(&painter, view_state);
        }

        self.handle_interaction(ui, viewport_rect, view_state);

        if self.config.show_minimap {
            self.render_minimap(ui, viewport_rect);
        }

        if self.config.show_stats_panel {
            self.render_stats_panel(ui, viewport_rect);
        }

        if self.config.show_tooltips {
            self.render_tooltip(ui);
        }
    }

    fn initialize_layout(&mut self, viewport_size: EguiVec2) {
        let center = Pos2::new(viewport_size.x / 2.0, viewport_size.y / 2.0);
        let radius = (viewport_size.x.min(viewport_size.y) / 2.0).min(100.0);

        for (idx, node_idx) in self.graph.graph.node_indices().enumerate() {
            let angle = (idx as f32 / self.graph.graph.node_count().max(1) as f32)
                * 2.0
                * std::f32::consts::PI;
            let dist =
                radius * (0.3 + 0.7 * (idx as f32 / self.graph.graph.node_count().max(1) as f32));
            let x = center.x + angle.cos() * dist;
            let y = center.y + angle.sin() * dist;
            let pos = Pos2::new(x, y);
            self.positions.insert(node_idx, pos);
            self.velocities.insert(node_idx, Vec2::ZERO);
        }

        self.build_quadtree();
    }

    fn update_lod(&mut self, view_state: &mut TopologyViewState, viewport_size: EguiVec2) {
        view_state.update_lod(viewport_size, &self.graph);
    }

    fn render_empty_state(&self, ui: &mut egui::Ui, viewport_rect: Rect) {
        let center = viewport_rect.center();
        ui.painter_at(viewport_rect).text(
            center,
            egui::Align2::CENTER_CENTER,
            "No topology data",
            egui::FontId::proportional(24.0),
            self.config.text_color,
        );
    }

    fn render_edges(&self, painter: &egui::Painter, _view_state: &TopologyViewState) {
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

            painter.line_segment(
                [*source_pos, *target_pos],
                Stroke::new(2.0, Color32::from_rgba_premultiplied(150, 150, 200, 200)),
            );
        }
    }

    fn render_nodes(&self, painter: &egui::Painter, view_state: &TopologyViewState) {
        let lod = view_state.lod_level;
        let radius = lod.node_radius().max(8.0);

        for node_idx in self.graph.graph.node_indices() {
            let Some(pos) = self.positions.get(&node_idx) else {
                continue;
            };

            let node_data = match self.graph.graph.node_weight(node_idx) {
                Some(n) => n,
                None => continue,
            };

            let fill_color = self.get_device_color(node_data.device_type);
            let stroke_color = Color32::from_rgb(255, 255, 255);

            painter.circle_filled(*pos, radius, fill_color);
            painter.circle_stroke(*pos, radius, Stroke::new(2.0, stroke_color));
        }
    }

    fn render_edge_labels(&self, painter: &egui::Painter, view_state: &TopologyViewState) {
        if view_state.lod_level != LODLevel::Full {
            return;
        }

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

            let edge_data = match self.graph.graph.edge_weight(edge_idx) {
                Some(e) => e,
                None => continue,
            };

            let mid_point = Pos2::new(
                (source_pos.x + target_pos.x) / 2.0,
                (source_pos.y + target_pos.y) / 2.0,
            );

            if let Some(latency) = edge_data.latency_ms {
                let label = format!("{}ms", latency);
                painter.text(
                    mid_point,
                    egui::Align2::CENTER_CENTER,
                    label,
                    egui::FontId::proportional(10.0),
                    Color32::from_rgb(180, 180, 180),
                );
            }
        }
    }

    fn handle_interaction(
        &mut self,
        _ui: &mut egui::Ui,
        _viewport_rect: Rect,
        _view_state: &mut TopologyViewState,
    ) {
        let pointer = _ui.input(|i| i.pointer.clone());

        let is_dragging = pointer.any_down();
        let drag_delta = pointer.delta();

        if is_dragging && drag_delta.length() > 0.5 {
            for pos in self.positions.values_mut() {
                pos.x -= drag_delta.x;
                pos.y -= drag_delta.y;
            }
        }
    }

    fn render_minimap(&mut self, ui: &mut egui::Ui, viewport_rect: Rect) {
        let minimap_size = EguiVec2::new(180.0, 120.0);
        let margin = 10.0;
        let minimap_rect = Rect::from_min_size(
            Pos2::new(viewport_rect.max.x - minimap_size.x - margin, margin),
            minimap_size,
        );

        self.minimap_rect = Some(minimap_rect);

        let painter = ui.painter_at(viewport_rect);

        painter.rect_filled(
            minimap_rect,
            0.0,
            Color32::from_rgba_premultiplied(20, 20, 30, 220),
        );
        painter.rect_stroke(
            minimap_rect,
            2.0,
            Stroke::new(1.0, Color32::from_rgb(80, 80, 100)),
        );

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

        let graph_bounds = Rect::from_min_max(Pos2::new(min_x, min_y), Pos2::new(max_x, max_y));

        let scale_x = minimap_size.x / graph_bounds.width().max(1.0);
        let scale_y = minimap_size.y / graph_bounds.height().max(1.0);
        let scale = scale_x.min(scale_y) * 0.9;

        let offset_x = minimap_rect.min.x + (minimap_size.x - graph_bounds.width() * scale) / 2.0;
        let offset_y = minimap_rect.min.y + (minimap_size.y - graph_bounds.height() * scale) / 2.0;

        for node_idx in self.graph.graph.node_indices() {
            let Some(pos) = self.positions.get(&node_idx) else {
                continue;
            };

            let minimap_pos = Pos2::new(
                offset_x + (pos.x - min_x) * scale,
                offset_y + (pos.y - min_y) * scale,
            );

            let node_data = match self.graph.graph.node_weight(node_idx) {
                Some(n) => n,
                None => continue,
            };

            let color = self.get_device_color(node_data.device_type);
            painter.circle_filled(minimap_pos, 2.0, color);
        }

        let view_rect = Rect::from_min_size(
            Pos2::new(offset_x + (-min_x) * scale, offset_y + (-min_y) * scale),
            EguiVec2::new(
                viewport_rect.width() * scale * 0.5,
                viewport_rect.height() * scale * 0.5,
            ),
        );

        painter.rect_stroke(
            view_rect,
            0.0,
            Stroke::new(1.0, Color32::from_rgb(255, 255, 255)),
        );
    }

    fn render_stats_panel(&mut self, ui: &mut egui::Ui, viewport_rect: Rect) {
        let panel_width = 200.0;
        let panel_height = 150.0;
        let margin = 10.0;

        let stats_rect = Rect::from_min_size(
            Pos2::new(margin, viewport_rect.max.y - panel_height - margin),
            EguiVec2::new(panel_width, panel_height),
        );

        self.stats_panel_rect = Some(stats_rect);

        let painter = ui.painter_at(viewport_rect);

        painter.rect_filled(
            stats_rect,
            0.0,
            Color32::from_rgba_premultiplied(20, 20, 30, 220),
        );
        painter.rect_stroke(
            stats_rect,
            2.0,
            Stroke::new(1.0, Color32::from_rgb(80, 80, 100)),
        );

        let stats = self.graph.get_stats();

        let texts = vec![
            format!("Nodes: {}", stats.node_count),
            format!("Edges: {}", stats.edge_count),
            format!("Routers: {}", stats.router_count),
            format!("Servers: {}", stats.server_count),
            format!("IoT Devices: {}", stats.iot_count),
        ];

        let mut y_offset = stats_rect.min.y + 10.0;
        for text in texts {
            painter.text(
                Pos2::new(stats_rect.min.x + 10.0, y_offset),
                egui::Align2::LEFT_TOP,
                text,
                egui::FontId::proportional(12.0),
                self.config.text_color,
            );
            y_offset += 18.0;
        }
    }

    fn render_tooltip(&self, ui: &mut egui::Ui) {
        let Some(info) = &self.tooltip_data else {
            return;
        };

        let tooltip_text = self.format_tooltip(info);

        let text_size = ui.fonts(|f| {
            f.layout_no_wrap(
                tooltip_text.clone(),
                egui::FontId::proportional(13.0),
                self.config.text_color,
            )
        });

        let padding = 10.0;
        let galley = &text_size;
        let _tooltip_rect = Rect::from_min_size(
            info.position + EguiVec2::new(20.0, -20.0),
            EguiVec2::new(
                galley.size().x + padding * 2.0,
                galley.size().y + padding * 2.0,
            ),
        );

        ui.painter_at(ui.max_rect()).text(
            info.position + EguiVec2::new(25.0, -20.0),
            egui::Align2::LEFT_TOP,
            tooltip_text,
            egui::FontId::proportional(13.0),
            self.config.text_color,
        );
    }

    fn format_tooltip(&self, info: &HoveredNodeInfo) -> String {
        let mut text = String::new();

        text.push_str(&format!("IP: {}\n", info.node_data.ip));

        if let Some(hostname) = &info.node_data.hostname {
            text.push_str(&format!("Host: {}\n", hostname));
        }

        if let Some(vendor) = &info.node_data.vendor {
            text.push_str(&format!("Vendor: {}\n", vendor));
        }

        text.push_str(&format!("Type: {:?}\n", info.node_data.device_type));
        text.push_str(&format!("Risk: {}\n", info.node_data.risk_score));

        if !info.ports.is_empty() {
            let port_list: Vec<String> = info
                .ports
                .iter()
                .take(5)
                .map(|p| p.port.to_string())
                .collect();
            text.push_str(&format!("Ports: {}\n", port_list.join(", ")));
        }

        if let Some(geo) = &info.geo_location {
            text.push_str(&format!(
                "Location: {}, {}\n",
                geo.city.as_deref().unwrap_or("Unknown"),
                geo.country.as_deref().unwrap_or("Unknown")
            ));
        }

        if !info.traceroute_hops.is_empty() {
            text.push_str(&format!("Hops: {}\n", info.traceroute_hops.len()));
        }

        text
    }

    fn get_device_color(&self, device_type: DeviceType) -> Color32 {
        match device_type {
            DeviceType::Router => Color32::from_rgb(100, 200, 100),
            DeviceType::Switch => Color32::from_rgb(100, 180, 100),
            DeviceType::Firewall => Color32::from_rgb(220, 80, 80),
            DeviceType::LoadBalancer => Color32::from_rgb(180, 100, 200),
            DeviceType::Server => Color32::from_rgb(80, 150, 220),
            DeviceType::Workstation => Color32::from_rgb(100, 160, 220),
            DeviceType::Laptop => Color32::from_rgb(120, 180, 240),
            DeviceType::Mobile => Color32::from_rgb(160, 200, 240),
            DeviceType::Tablet => Color32::from_rgb(140, 190, 240),
            DeviceType::Printer => Color32::from_rgb(200, 180, 100),
            DeviceType::IoT => Color32::from_rgb(100, 220, 220),
            DeviceType::Camera => Color32::from_rgb(220, 180, 80),
            DeviceType::Thermostat => Color32::from_rgb(180, 220, 180),
            DeviceType::Speaker => Color32::from_rgb(160, 200, 200),
            DeviceType::Light => Color32::from_rgb(255, 255, 150),
            DeviceType::Lock => Color32::from_rgb(200, 150, 150),
            DeviceType::Sensor => Color32::from_rgb(150, 220, 180),
            DeviceType::NAS => Color32::from_rgb(100, 150, 200),
            DeviceType::AccessPoint => Color32::from_rgb(120, 200, 120),
            DeviceType::VMHost => Color32::from_rgb(150, 100, 200),
            DeviceType::Container => Color32::from_rgb(200, 150, 220),
            DeviceType::Database => Color32::from_rgb(80, 180, 140),
            DeviceType::WebServer => Color32::from_rgb(100, 160, 220),
            DeviceType::MailServer => Color32::from_rgb(120, 170, 230),
            DeviceType::DNS => Color32::from_rgb(140, 180, 100),
            DeviceType::DHCP => Color32::from_rgb(160, 190, 120),
            DeviceType::Directory => Color32::from_rgb(180, 160, 200),
            DeviceType::VPN => Color32::from_rgb(200, 120, 100),
            DeviceType::Proxy => Color32::from_rgb(180, 140, 180),
            DeviceType::FirewallAppliance => Color32::from_rgb(220, 80, 80),
            DeviceType::Storage => Color32::from_rgb(100, 140, 180),
            DeviceType::UPS => Color32::from_rgb(180, 200, 100),
            DeviceType::KVM => Color32::from_rgb(160, 160, 180),
            DeviceType::Unknown => Color32::from_rgb(120, 120, 120),
        }
    }

    fn get_risk_color(&self, risk_score: u8) -> Color32 {
        if risk_score >= 70 {
            Color32::from_rgb(220, 60, 60)
        } else if risk_score >= 40 {
            Color32::from_rgb(220, 160, 60)
        } else {
            Color32::from_rgb(160, 220, 80)
        }
    }

    fn get_node_label(&self, node_data: &NodeData, lod: LODLevel) -> String {
        match lod {
            LODLevel::Minimal => node_data.ip.to_string(),
            LODLevel::Low => {
                if let Some(hostname) = &node_data.hostname {
                    hostname.clone()
                } else {
                    node_data.ip.to_string()
                }
            }
            LODLevel::Medium => {
                let label = if let Some(hostname) = &node_data.hostname {
                    format!("{}\n{}", hostname, node_data.ip)
                } else {
                    node_data.ip.to_string()
                };
                label
            }
            LODLevel::High | LODLevel::Full => {
                let mut label = String::new();
                if let Some(hostname) = &node_data.hostname {
                    label.push_str(hostname);
                }
                label.push_str(&format!("\n{}", node_data.ip));
                if let Some(vendor) = &node_data.vendor {
                    label.push_str(&format!("\n{}", vendor));
                }
                label
            }
        }
    }

    pub fn get_selected_node_data(&self) -> Option<&NodeData> {
        for node_idx in self.graph.graph.node_indices() {
            let node_data = match self.graph.graph.node_weight(node_idx) {
                Some(n) => n,
                None => continue,
            };

            if node_data.ip.is_loopback() {
                continue;
            }
        }
        None
    }

    pub fn node_count(&self) -> usize {
        self.graph.graph.node_count()
    }

    pub fn edge_count(&self) -> usize {
        self.graph.graph.edge_count()
    }

    pub fn is_layout_computing(&self) -> bool {
        self.is_layout_computing
    }
}
