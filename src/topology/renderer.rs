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
            if zoom < 0.5 { LODLevel::Minimal }
            else if zoom < 1.0 { LODLevel::Low }
            else if zoom < 2.0 { LODLevel::Medium }
            else { LODLevel::High }
        } else if node_count > 100 {
            if zoom < 0.3 { LODLevel::Low }
            else if zoom < 0.7 { LODLevel::Medium }
            else if zoom < 1.5 { LODLevel::High }
            else { LODLevel::Full }
        } else {
            if zoom < 0.4 { LODLevel::Medium }
            else if zoom < 1.0 { LODLevel::High }
            else { LODLevel::Full }
        }
    }

    pub fn show_labels(&self) -> bool {
        !matches!(self, LODLevel::Minimal | LODLevel::Low)
    }

    pub fn show_ports(&self) -> bool {
        matches!(self, LODLevel::High | LODLevel::Full)
    }

    pub fn show_icons(&self) -> bool {
        !matches!(self, LODLevel::Minimal)
    }

    pub fn node_radius(&self) -> f32 {
        match self {
            LODLevel::Minimal => 4.0,
            LODLevel::Low => 6.0,
            LODLevel::Medium => 10.0,
            LODLevel::High => 14.0,
            LODLevel::Full => 18.0,
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
            hovered_node_color: Color32::from_rgb(255, 255, 255),
            text_color: Color32::from_rgb(230, 230, 230),
            label_background: Color32::from_rgba_premultiplied(30, 30, 40, 200),
            show_tooltips: true,
            show_minimap: true,
            show_stats_panel: true,
            enable_animations: true,
        }
    }
}

pub struct TopologyRenderer {
    pub graph: TopologyGraph,
    positions: HashMap<NodeIndex, Pos2>,
    config: RenderConfig,
    layout_config: LayoutConfig,
    tooltip_data: Option<HoveredNodeInfo>,
    dragging_node: Option<NodeIndex>,
}

impl std::fmt::Debug for TopologyRenderer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TopologyRenderer")
            .field("node_count", &self.graph.graph.node_count())
            .field("edge_count", &self.graph.graph.edge_count())
            .finish()
    }
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
        Self {
            graph,
            positions: HashMap::new(),
            config: RenderConfig::default(),
            layout_config: LayoutConfig::default(),
            tooltip_data: None,
            dragging_node: None,
        }
    }

    pub fn set_graph(&mut self, graph: TopologyGraph) {
        self.graph = graph;
        self.positions.clear();
    }

    pub fn compute_layout(&mut self) {
        let mut engine = LayoutEngine::new(self.layout_config.clone());
        let positions: HashMap<NodeIndex, Vec2> = self.positions.iter()
            .map(|(&idx, pos)| (idx, Vec2::new(pos.x, pos.y)))
            .collect();

        let new_positions = engine.compute(&self.graph, if positions.is_empty() { None } else { Some(&positions) });

        self.positions = new_positions.into_iter()
            .map(|(idx, pos)| (idx, Pos2::new(pos.x, pos.y)))
            .collect();
    }

    fn screen_to_world(&self, screen_pos: Pos2, viewport_rect: Rect, view_state: &TopologyViewState) -> Pos2 {
        let center = viewport_rect.center();
        let world_x = (screen_pos.x - center.x - view_state.pan_offset.x) / view_state.zoom;
        let world_y = (screen_pos.y - center.y - view_state.pan_offset.y) / view_state.zoom;
        Pos2::new(world_x, world_y)
    }

    fn world_to_screen(&self, world_pos: Pos2, viewport_rect: Rect, view_state: &TopologyViewState) -> Pos2 {
        let center = viewport_rect.center();
        let screen_x = center.x + view_state.pan_offset.x + world_pos.x * view_state.zoom;
        let screen_y = center.y + view_state.pan_offset.y + world_pos.y * view_state.zoom;
        Pos2::new(screen_x, screen_y)
    }

    pub fn render(&mut self, ui: &mut egui::Ui, viewport_rect: Rect, view_state: &mut TopologyViewState) {
        let painter = ui.painter_at(viewport_rect);

        // Draw background
        painter.rect_filled(viewport_rect, 0.0, self.config.background_color);

        if self.graph.graph.node_count() == 0 {
            self.render_empty_state(&painter, viewport_rect);
            return;
        }

        if self.positions.is_empty() {
            self.initialize_layout();
        }

        view_state.update_lod(viewport_rect.size(), &self.graph);

        // Handle interactions before rendering to have correct hovered/selected states
        self.handle_interaction(ui, viewport_rect, view_state);

        // Render layers
        if view_state.show_edges {
            self.render_edges(&painter, viewport_rect, view_state);
        }
        self.render_nodes(&painter, viewport_rect, view_state);

        if self.config.show_stats_panel {
            self.render_stats_panel(&painter, viewport_rect);
        }
    }

    fn initialize_layout(&mut self) {
        let node_count = self.graph.graph.node_count();
        let radius = 200.0;
        for (i, node_idx) in self.graph.graph.node_indices().enumerate() {
            let angle = (i as f32 / node_count as f32) * std::f32::consts::TAU;
            self.positions.insert(node_idx, Pos2::new(angle.cos() * radius, angle.sin() * radius));
        }
        self.compute_layout();
    }

    fn render_empty_state(&self, painter: &egui::Painter, viewport_rect: Rect) {
        painter.text(
            viewport_rect.center(),
            egui::Align2::CENTER_CENTER,
            "No topology data available.\nRun a scan to see network structure.",
            egui::FontId::proportional(18.0),
            Color32::from_gray(120),
        );
    }

    fn render_edges(&self, painter: &egui::Painter, viewport_rect: Rect, view_state: &TopologyViewState) {
        for edge_idx in self.graph.graph.edge_indices() {
            let (u, v) = self.graph.graph.edge_endpoints(edge_idx).unwrap();
            let (Some(&pos_u), Some(&pos_v)) = (self.positions.get(&u), self.positions.get(&v)) else { continue };

            let screen_u = self.world_to_screen(pos_u, viewport_rect, view_state);
            let screen_v = self.world_to_screen(pos_v, viewport_rect, view_state);

            // Simple frustum culling
            if !viewport_rect.intersects(Rect::from_two_pos(screen_u, screen_v).expand(10.0)) {
                continue;
            }

            painter.line_segment([screen_u, screen_v], Stroke::new(self.config.edge_width * view_state.zoom.sqrt(), self.config.edge_color));
        }
    }

    fn render_nodes(&self, painter: &egui::Painter, viewport_rect: Rect, view_state: &TopologyViewState) {
        let lod = view_state.lod_level;
        let base_radius = lod.node_radius();

        for node_idx in self.graph.graph.node_indices() {
            let Some(&world_pos) = self.positions.get(&node_idx) else { continue };
            let screen_pos = self.world_to_screen(world_pos, viewport_rect, view_state);

            let radius = base_radius * view_state.zoom.sqrt();
            if !viewport_rect.intersects(Rect::from_center_size(screen_pos, EguiVec2::splat(radius * 2.0))) {
                continue;
            }

            let node_data = &self.graph.graph[node_idx];
            let is_selected = view_state.selected_node.as_ref().map_or(false, |id| id.0 == node_data.ip);
            let is_hovered = view_state.hovered_node.as_ref().map_or(false, |id| id.0 == node_data.ip);

            let fill_color = self.get_device_color(node_data.device_type);
            let mut stroke_color = self.config.node_stroke_color;
            let mut stroke_width = 1.0;

            if is_selected {
                stroke_color = self.config.selected_node_color;
                stroke_width = 3.0;
            } else if is_hovered {
                stroke_color = self.config.hovered_node_color;
                stroke_width = 2.0;
            }

            painter.circle_filled(screen_pos, radius, fill_color);
            painter.circle_stroke(screen_pos, radius, Stroke::new(stroke_width, stroke_color));

            if lod.show_labels() {
                let label = self.get_node_label(node_data, lod);
                painter.text(
                    screen_pos + EguiVec2::new(0.0, radius + 4.0),
                    egui::Align2::CENTER_TOP,
                    label,
                    egui::FontId::proportional(lod.label_font_size() * view_state.zoom.sqrt().max(0.8)),
                    self.config.text_color,
                );
            }
        }
    }

    fn handle_interaction(&mut self, ui: &mut egui::Ui, viewport_rect: Rect, view_state: &mut TopologyViewState) {
        let response = ui.interact(viewport_rect, ui.id(), egui::Sense::click_and_drag());
        let pointer = ui.input(|i| i.pointer.clone());

        // Zooming
        let zoom_delta = ui.input(|i| i.zoom_delta());
        if zoom_delta != 1.0 {
            let _old_zoom = view_state.zoom;
            view_state.zoom = (view_state.zoom * zoom_delta).clamp(0.05, 5.0);

            if let Some(mouse_pos) = pointer.hover_pos() {
                let _world_mouse = self.screen_to_world(mouse_pos, viewport_rect, view_state);
                // Adjust pan to keep world_mouse under the cursor
                // This logic is a bit tricky, often handled by panned world coordinates
                // For now, simple zoom is okay.
            }
        }

        // Hover detection
        if let Some(mouse_pos) = pointer.hover_pos() {
            let world_mouse = self.screen_to_world(mouse_pos, viewport_rect, view_state);
            let mut closest_node = None;
            let mut min_dist = 20.0 / view_state.zoom; // Threshold in world space

            for node_idx in self.graph.graph.node_indices() {
                if let Some(&pos) = self.positions.get(&node_idx) {
                    let dist = pos.distance(world_mouse);
                    if dist < min_dist {
                        min_dist = dist;
                        closest_node = Some(node_idx);
                    }
                }
            }

            if let Some(idx) = closest_node {
                let node_data = &self.graph.graph[idx];
                view_state.hovered_node = Some(crate::topology::NodeId(node_data.ip));

                if response.clicked() {
                    view_state.selected_node = Some(crate::topology::NodeId(node_data.ip));
                }

                if response.drag_started() {
                    self.dragging_node = Some(idx);
                }
            } else {
                view_state.hovered_node = None;
                if response.clicked() {
                    view_state.selected_node = None;
                }
            }
        }

        // Dragging
        if let Some(idx) = self.dragging_node {
            if pointer.any_down() {
                if let Some(mouse_pos) = pointer.hover_pos() {
                    let world_mouse = self.screen_to_world(mouse_pos, viewport_rect, view_state);
                    self.positions.insert(idx, world_mouse);
                }
            } else {
                self.dragging_node = None;
            }
        } else if response.dragged() {
            // Panning the whole view
            view_state.pan_offset += response.drag_delta();
        }
    }

    fn render_stats_panel(&self, painter: &egui::Painter, viewport_rect: Rect) {
        let stats = self.graph.get_stats();
                let text = format!(
                    "Nodes: {}\nEdges: {}\nRouters: {}\nServers: {}",
                    stats.node_count, stats.edge_count, stats.router_count, stats.server_count
                );
        

        let rect = Rect::from_min_size(
            viewport_rect.min + EguiVec2::new(10.0, 10.0),
            EguiVec2::new(120.0, 80.0),
        );

        painter.rect_filled(rect, 4.0, Color32::from_black_alpha(150));
        painter.text(
            rect.min + EguiVec2::new(8.0, 8.0),
            egui::Align2::LEFT_TOP,
            text,
            egui::FontId::proportional(12.0),
            Color32::WHITE,
        );
    }

    fn get_device_color(&self, device_type: DeviceType) -> Color32 {
        match device_type {
            DeviceType::Router => Color32::from_rgb(100, 200, 100),
            DeviceType::Server => Color32::from_rgb(80, 150, 220),
            DeviceType::Firewall => Color32::from_rgb(220, 80, 80),
            DeviceType::IoT => Color32::from_rgb(100, 220, 220),
            DeviceType::Workstation => Color32::from_rgb(120, 120, 150),
            _ => Color32::from_rgb(150, 150, 150),
        }
    }

    fn get_node_label(&self, node_data: &NodeData, lod: LODLevel) -> String {
        match lod {
            LODLevel::Minimal | LODLevel::Low => String::new(),
            LODLevel::Medium => node_data.ip.to_string(),
            _ => node_data.hostname.as_ref().cloned().unwrap_or_else(|| node_data.ip.to_string()),
        }
    }
}