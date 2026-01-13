use std::collections::HashMap;

use egui::{Color32, Pos2, Rect, Shape, Stroke, Vec2 as EguiVec2};
use glam::Vec2;
use petgraph::graph::NodeIndex;

use crate::topology::device::DeviceType;
use crate::topology::graph::TopologyGraph;
use crate::topology::layout::{LayoutConfig, LayoutEngine};
use crate::topology::{NodeData, TopologyViewState};

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
        let effective_zoom = zoom;
        if node_count > 1000 {
            if effective_zoom < 0.5 {
                LODLevel::Minimal
            } else if effective_zoom < 1.0 {
                LODLevel::Low
            } else if effective_zoom < 2.0 {
                LODLevel::Medium
            } else {
                LODLevel::High
            }
        } else if node_count > 100 {
            if effective_zoom < 0.3 {
                LODLevel::Low
            } else if effective_zoom < 0.7 {
                LODLevel::Medium
            } else if effective_zoom < 1.5 {
                LODLevel::High
            } else {
                LODLevel::Full
            }
        } else if effective_zoom < 0.4 {
            LODLevel::Medium
        } else if effective_zoom < 1.0 {
            LODLevel::High
        } else {
            LODLevel::Full
        }
    }

    pub fn show_labels(&self) -> bool {
        match self {
            LODLevel::Minimal | LODLevel::Low | LODLevel::Medium => false,
            LODLevel::High | LODLevel::Full => true,
        }
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
    pub grid_color: Color32,
    pub node_fill_color: Color32,
    pub node_stroke_color: Color32,
    pub edge_color: Color32,
    pub edge_width: f32,
    pub selected_node_color: Color32,
    pub hovered_node_color: Color32,
    pub text_color: Color32,
    pub show_grid: bool,
    pub show_stats_panel: bool,
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            background_color: Color32::from_rgb(10, 10, 15),
            grid_color: Color32::from_rgb(25, 25, 35),
            node_fill_color: Color32::from_rgb(60, 120, 180),
            node_stroke_color: Color32::from_rgb(100, 160, 220),
            edge_color: Color32::from_rgba_premultiplied(100, 150, 255, 80),
            edge_width: 1.5,
            selected_node_color: Color32::from_rgb(255, 215, 0),
            hovered_node_color: Color32::from_rgb(255, 255, 255),
            text_color: Color32::from_rgb(200, 200, 210),
            show_grid: true,
            show_stats_panel: true,
        }
    }
}

pub struct TopologyRenderer {
    pub graph: TopologyGraph,
    positions: HashMap<NodeIndex, Pos2>,
    config: RenderConfig,
    layout_config: LayoutConfig,
    dragging_node: Option<NodeIndex>,
    last_layout_step: std::time::Instant,
    layout_engine: LayoutEngine,
    layout_stable: bool,
}

impl std::fmt::Debug for TopologyRenderer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TopologyRenderer")
            .field("node_count", &self.graph.graph.node_count())
            .field("edge_count", &self.graph.graph.edge_count())
            .finish()
    }
}

impl TopologyRenderer {
    pub fn new(graph: TopologyGraph) -> Self {
        Self {
            graph,
            positions: HashMap::new(),
            config: RenderConfig::default(),
            layout_config: LayoutConfig::default(),
            dragging_node: None,
            last_layout_step: std::time::Instant::now(),
            layout_engine: LayoutEngine::default(),
            layout_stable: false,
        }
    }

    pub fn set_graph(&mut self, graph: TopologyGraph) {
        self.graph = graph;
        self.positions.clear();
        self.layout_stable = false;
        // Force re-initialization of layout engine positions
        self.layout_engine = LayoutEngine::new(self.layout_config.clone());
    }

    pub fn compute_layout(&mut self) {
        // Full recompute
        self.layout_engine = LayoutEngine::new(self.layout_config.clone());
        let positions = self.layout_engine.compute(&self.graph, None);

        self.positions = positions
            .into_iter()
            .map(|(idx, pos)| (idx, Pos2::new(pos.x, pos.y)))
            .collect();
        self.layout_stable = true; // Assumes compute() ran to convergence
    }

    fn screen_to_world(
        &self,
        screen_pos: Pos2,
        viewport_rect: Rect,
        view_state: &TopologyViewState,
    ) -> Pos2 {
        let center = viewport_rect.center();
        let world_x = (screen_pos.x - center.x - view_state.pan_offset.x) / view_state.zoom;
        let world_y = (screen_pos.y - center.y - view_state.pan_offset.y) / view_state.zoom;
        Pos2::new(world_x, world_y)
    }

    fn world_to_screen(
        &self,
        world_pos: Pos2,
        viewport_rect: Rect,
        view_state: &TopologyViewState,
    ) -> Pos2 {
        let center = viewport_rect.center();
        let screen_x = center.x + view_state.pan_offset.x + world_pos.x * view_state.zoom;
        let screen_y = center.y + view_state.pan_offset.y + world_pos.y * view_state.zoom;
        Pos2::new(screen_x, screen_y)
    }

    pub fn apply_theme(&mut self, is_dark: bool) {
        if is_dark {
            self.config.background_color = Color32::from_rgb(10, 10, 15);
            self.config.grid_color = Color32::from_rgb(25, 25, 35);
            self.config.text_color = Color32::from_rgb(200, 200, 210);
            self.config.edge_color = Color32::from_rgba_premultiplied(100, 150, 255, 80);
        } else {
            self.config.background_color = Color32::from_rgb(240, 240, 245);
            self.config.grid_color = Color32::from_rgb(220, 220, 230);
            self.config.text_color = Color32::from_rgb(20, 20, 30);
            // Stronger blue for light mode visibility
            self.config.edge_color = Color32::from_rgba_premultiplied(40, 100, 255, 120);
        }
    }

    pub fn render(
        &mut self,
        ui: &mut egui::Ui,
        viewport_rect: Rect,
        view_state: &mut TopologyViewState,
        public_ip: Option<&str>,
    ) {
        // Allocate space for the widget to capture input robustly
        let response = ui.allocate_rect(viewport_rect, egui::Sense::click_and_drag());
        
        // Create a painter on top of the allocated rect
        let mut painter = ui.painter_at(viewport_rect);

        // Clip drawing to the viewport
        painter.set_layer_id(ui.layer_id());

        // Draw background
        painter.rect_filled(viewport_rect, 0.0, self.config.background_color);

        if self.config.show_grid {
            self.draw_grid(&painter, viewport_rect, view_state);
        }

        if self.graph.graph.node_count() == 0 {
            self.render_empty_state(&painter, viewport_rect);
            return;
        }

        if self.positions.is_empty() {
            self.compute_layout(); // Initial compute
        }

        // Always request repaint to drive data flow animations if we have data
        if self.graph.graph.node_count() > 0 {
            ui.ctx().request_repaint();
        }

        // Incremental layout step for animation (only if not stable)
        if !self.layout_stable && self.dragging_node.is_none() && self.last_layout_step.elapsed().as_millis() > 16 {
            self.step_layout();
            ui.ctx().request_repaint();
        }

        view_state.update_lod(viewport_rect.size(), &self.graph);

        // Handle interactions using the allocated response
        self.handle_interaction(ui, &response, viewport_rect, view_state);

        // Render layers
        if view_state.show_edges {
            self.render_edges(ui, &painter, viewport_rect, view_state);
        }
        self.render_nodes(ui, &painter, viewport_rect, view_state, public_ip);

        if self.config.show_stats_panel {
            self.render_stats_panel(&painter, viewport_rect);
            self.render_legend(&painter, viewport_rect);
        }
    }

    fn step_layout(&mut self) {
        let delta = self.layout_engine.step(&self.graph);
        
        // Sync positions
        let engine_positions = self.layout_engine.get_positions();
        self.positions = engine_positions
            .iter()
            .map(|(&idx, &pos)| (idx, Pos2::new(pos.x, pos.y)))
            .collect();
            
        // Check for stability (stop simulating if movement is tiny)
        if delta < 0.1 {
            self.layout_stable = true;
        }
        
        self.last_layout_step = std::time::Instant::now();
    }

    fn draw_grid(
        &self,
        painter: &egui::Painter,
        viewport_rect: Rect,
        view_state: &TopologyViewState,
    ) {
        let grid_spacing = 100.0 * view_state.zoom;
        if grid_spacing < 10.0 {
            return;
        }

        let center = viewport_rect.center();
        let offset_x = (view_state.pan_offset.x + center.x) % grid_spacing;
        let offset_y = (view_state.pan_offset.y + center.y) % grid_spacing;

        let dot_color = self.config.grid_color.gamma_multiply(0.5);
        
        // Draw a dot grid for a more professional "Blueprint" look
        let mut x = offset_x;
        while x < viewport_rect.width() {
            let mut y = offset_y;
            while y < viewport_rect.height() {
                painter.circle_filled(
                    Pos2::new(viewport_rect.min.x + x, viewport_rect.min.y + y),
                    1.5 * view_state.zoom.sqrt().max(0.5),
                    dot_color,
                );
                y += grid_spacing;
            }
            x += grid_spacing;
        }
    }

    fn render_empty_state(&self, painter: &egui::Painter, viewport_rect: Rect) {
        painter.text(
            viewport_rect.center(),
            egui::Align2::CENTER_CENTER,
            "No topology data available.\nRun a network discovery scan.",
            egui::FontId::proportional(18.0),
            Color32::from_gray(100),
        );
    }

    fn render_edges(
        &self,
        ui: &egui::Ui,
        painter: &egui::Painter,
        viewport_rect: Rect,
        view_state: &TopologyViewState,
    ) {
        for edge_idx in self.graph.graph.edge_indices() {
            let (u, v) = self.graph.graph.edge_endpoints(edge_idx).unwrap();
            let (Some(&pos_u), Some(&pos_v)) = (self.positions.get(&u), self.positions.get(&v))
            else {
                continue;
            };

            let screen_u = self.world_to_screen(pos_u, viewport_rect, view_state);
            let screen_v = self.world_to_screen(pos_v, viewport_rect, view_state);

            // Culling
            if !viewport_rect.intersects(Rect::from_two_pos(screen_u, screen_v).expand(50.0)) {
                continue;
            }

            let mut edge_color = self.config.edge_color;

            if view_state.highlight_mode == crate::topology::HighlightMode::Latency {
                if let Some(edge_data) = self.graph.graph.edge_weight(edge_idx) {
                    if let Some(ms) = edge_data.latency_ms {
                        if ms < 2 {
                            edge_color = Color32::from_rgb(0, 255, 150);
                        } else if ms < 50 {
                            edge_color = Color32::from_rgb(255, 200, 0);
                        } else {
                            edge_color = Color32::from_rgb(255, 50, 50);
                        }
                    }
                }
            }

            let width = self.config.edge_width * view_state.zoom.sqrt();
            
            // Quadratic Bezier for a more organic look
            let mid = screen_u.lerp(screen_v, 0.5);
            let dist = screen_u.distance(screen_v);
            let vec = screen_v - screen_u;
            let normal = EguiVec2::new(-vec.y, vec.x).normalized();
            let control = mid + normal * (dist * 0.1);

            // Edge Glow
            painter.add(Shape::QuadraticBezier(egui::epaint::QuadraticBezierShape {
                points: [screen_u, control, screen_v],
                closed: false,
                fill: Color32::TRANSPARENT,
                stroke: Stroke::new(width * 2.0, edge_color.gamma_multiply(0.1)),
            }));

            // Main Edge
            painter.add(Shape::QuadraticBezier(egui::epaint::QuadraticBezierShape {
                points: [screen_u, control, screen_v],
                closed: false,
                fill: Color32::TRANSPARENT,
                stroke: Stroke::new(width, edge_color.gamma_multiply(0.5)),
            }));

            // Data Flow Animation (Moving Pulses)
            let time = ui.input(|i| i.time);
            let speed = 0.45; // Slowed down by 70% from 1.5
            let pulse_count = 3;
            
            for i in 0..pulse_count {
                // Offset each pulse
                let t_offset = i as f64 / pulse_count as f64;
                let t = ((time * speed + t_offset) % 1.0) as f32;
                
                // Sample the bezier curve at t
                let pulse_pos = self.sample_quadratic_bezier(screen_u, control, screen_v, t);
                
                // Draw glowing pulse
                painter.circle_filled(
                    pulse_pos, 
                    2.0 * view_state.zoom.sqrt(), 
                    edge_color.gamma_multiply(0.8)
                );
                // Extra glow layer
                painter.circle_filled(
                    pulse_pos, 
                    4.0 * view_state.zoom.sqrt(), 
                    edge_color.gamma_multiply(0.2)
                );
            }

            // Show latency label on edge if zoomed in and in latency mode
            if view_state.highlight_mode == crate::topology::HighlightMode::Latency && view_state.zoom > 1.2 {
                if let Some(edge_data) = self.graph.graph.edge_weight(edge_idx) {
                    if let Some(ms) = edge_data.latency_ms {
                        let label = format!("{}ms", ms);
                        let font_size = 10.0 * view_state.zoom.sqrt();
                        
                        painter.rect_filled(
                            Rect::from_center_size(control, EguiVec2::new(30.0, 14.0) * view_state.zoom.sqrt()),
                            2.0,
                            Color32::from_black_alpha(200)
                        );
                        
                        painter.text(
                            control,
                            egui::Align2::CENTER_CENTER,
                            label,
                            egui::FontId::proportional(font_size),
                            edge_color,
                        );
                    }
                }
            }
        }
    }

    fn render_nodes(
        &self,
        ui: &egui::Ui,
        painter: &egui::Painter,
        viewport_rect: Rect,
        view_state: &TopologyViewState,
        public_ip: Option<&str>,
    ) {
        let lod = view_state.lod_level;
        let base_radius = lod.node_radius();

        for node_idx in self.graph.graph.node_indices() {
            let Some(&world_pos) = self.positions.get(&node_idx) else {
                continue;
            };
            let screen_pos = self.world_to_screen(world_pos, viewport_rect, view_state);

            let node_data = &self.graph.graph[node_idx];
            let is_internet = node_data.device_type == DeviceType::Internet;

            let mut radius = base_radius * view_state.zoom.sqrt();
            if is_internet {
                radius *= 1.4;
            }
            
            radius = radius.max(10.0);

            if !viewport_rect.intersects(Rect::from_center_size(
                screen_pos,
                EguiVec2::splat(radius * 5.0),
            )) {
                continue;
            }

            let is_selected = view_state
                .selected_node
                .as_ref()
                .is_some_and(|id| id.0 == node_data.ip);
            let is_hovered = view_state
                .hovered_node
                .as_ref()
                .is_some_and(|id| id.0 == node_data.ip);

            let type_color = self.get_node_color(node_data, view_state.highlight_mode);

            // Modern selection ring
            if is_selected {
                painter.circle_stroke(screen_pos, radius * 1.6, Stroke::new(2.0, self.config.selected_node_color));
                painter.circle_filled(screen_pos, radius * 2.0, self.config.selected_node_color.gamma_multiply(0.1));
            } else if is_hovered {
                painter.circle_stroke(screen_pos, radius * 1.4, Stroke::new(1.0, self.config.hovered_node_color.gamma_multiply(0.5)));
            }

            // Draw Node Card (Glass effect)
            self.draw_node_card(painter, screen_pos, radius, node_data.device_type, type_color);

            // Risk Indicator (Pulsing ring for all levels > 0)
            if node_data.risk_score > 0 && view_state.show_risk_levels {
                let risk_color = self.get_risk_color(node_data.risk_score);
                let time = ui.input(|i| i.time);
                let pulse = (time * 2.0).sin() as f32 * 0.15 + 1.25;
                painter.circle_stroke(
                    screen_pos,
                    radius * pulse,
                    Stroke::new(1.5, risk_color.gamma_multiply(0.6)),
                );
            }

            if lod.show_labels() {
                let label = self.get_node_label(node_data, lod, public_ip);
                let font_size = lod.label_font_size() * view_state.zoom.sqrt().max(0.8);

                let galley = ui.fonts(|f| {
                    f.layout_no_wrap(
                        label.clone(),
                        egui::FontId::proportional(font_size),
                        Color32::WHITE,
                    )
                });
                
                let label_pos = screen_pos + EguiVec2::new(0.0, radius + 10.0);
                
                let label_bg = if ui.visuals().dark_mode {
                    Color32::from_black_alpha(180)
                } else {
                    Color32::from_white_alpha(150) // More transparent light bg for light mode
                };

                painter.rect_filled(
                    Rect::from_center_size(label_pos + EguiVec2::new(0.0, galley.size().y/2.0), galley.size() + EguiVec2::new(8.0, 4.0)), 
                    4.0, 
                    label_bg
                );

                painter.text(
                    label_pos,
                    egui::Align2::CENTER_TOP,
                    label,
                    egui::FontId::proportional(font_size),
                    self.config.text_color,
                );
            }
        }
    }

    fn draw_node_card(
        &self,
        painter: &egui::Painter,
        center: Pos2,
        radius: f32,
        device_type: DeviceType,
        color: Color32,
    ) {
        // Glass container
        painter.circle_filled(center, radius, Color32::from_black_alpha(220));
        painter.circle_stroke(center, radius, Stroke::new(1.5, Color32::from_gray(100)));
        
        // Inner accent
        painter.circle_filled(center, radius * 0.8, color.gamma_multiply(0.2));
        
        self.draw_icon(painter, center, radius * 0.6, device_type, color);
    }

    fn draw_icon(
        &self,
        painter: &egui::Painter,
        center: Pos2,
        radius: f32,
        device_type: DeviceType,
        color: Color32,
    ) {
        let stroke = Stroke::new(1.5, Color32::WHITE);
        let fill = color; // Base color
        
        // Draw base shape container
        painter.circle_filled(center, radius, Color32::from_black_alpha(100)); // Dark background for contrast
        
        match device_type {
            DeviceType::Router | DeviceType::Switch | DeviceType::AccessPoint => {
                // Router: Circle with arrows or stylized box
                painter.circle_filled(center, radius, fill);
                painter.circle_stroke(center, radius, stroke);
                
                // Draw cross arrows (simple representation of routing)
                let r = radius * 0.5;
                painter.line_segment([center - EguiVec2::new(r, 0.0), center + EguiVec2::new(r, 0.0)], Stroke::new(1.5, Color32::WHITE));
                painter.line_segment([center - EguiVec2::new(0.0, r), center + EguiVec2::new(0.0, r)], Stroke::new(1.5, Color32::WHITE));
                // Arrow heads (simplified as dots for robustness at small scale)
                painter.circle_filled(center + EguiVec2::new(r, 0.0), 1.5, Color32::WHITE);
                painter.circle_filled(center - EguiVec2::new(r, 0.0), 1.5, Color32::WHITE);
                painter.circle_filled(center + EguiVec2::new(0.0, r), 1.5, Color32::WHITE);
                painter.circle_filled(center - EguiVec2::new(0.0, r), 1.5, Color32::WHITE);
            }
            DeviceType::Server | DeviceType::WebServer | DeviceType::MailServer => {
                // Server: Rect with lines (rack mount)
                let rect_size = EguiVec2::new(radius * 1.4, radius * 1.8);
                let rect = Rect::from_center_size(center, rect_size);
                painter.rect_filled(rect, 2.0, fill);
                painter.rect_stroke(rect, 2.0, stroke);
                
                // Rack lines
                let line_y_step = rect.height() / 4.0;
                for i in 1..4 {
                     let y = rect.min.y + line_y_step * i as f32;
                     painter.line_segment([Pos2::new(rect.min.x + 2.0, y), Pos2::new(rect.max.x - 2.0, y)], Stroke::new(1.0, Color32::from_black_alpha(100)));
                }
                
                // Blinking lights
                painter.circle_filled(rect.min + EguiVec2::new(4.0, 4.0), 1.5, Color32::GREEN);
            }
            DeviceType::Database => {
                 // Database: Cylinder shape (approx with rounded rect + ellipse lines)
                 let rect_size = EguiVec2::new(radius * 1.6, radius * 1.8);
                 let rect = Rect::from_center_size(center, rect_size);
                 painter.rect_filled(rect, 4.0, fill);
                 painter.rect_stroke(rect, 2.0, stroke);
                 
                 // Cylinder bands
                 painter.line_segment([Pos2::new(rect.min.x, rect.min.y + rect.height()*0.3), Pos2::new(rect.max.x, rect.min.y + rect.height()*0.3)], Stroke::new(1.0, Color32::BLACK));
                 painter.line_segment([Pos2::new(rect.min.x, rect.min.y + rect.height()*0.6), Pos2::new(rect.max.x, rect.min.y + rect.height()*0.6)], Stroke::new(1.0, Color32::BLACK));
            }
            DeviceType::Internet => {
                // Internet: Cloud-like or Globe
                // Globe: Circle with latitude/longitude lines
                let r = radius * 1.2;
                painter.circle_filled(center, r, fill);
                painter.circle_stroke(center, r, stroke);
                
                // Grid lines
                painter.line_segment([center - EguiVec2::new(r, 0.0), center + EguiVec2::new(r, 0.0)], Stroke::new(1.0, Color32::from_white_alpha(150)));
                painter.line_segment([center - EguiVec2::new(0.0, r), center + EguiVec2::new(0.0, r)], Stroke::new(1.0, Color32::from_white_alpha(150)));
                // Ellipse approximations (curves) are hard with basic painter, stick to crosshair grid + diagonal
                painter.line_segment([center - EguiVec2::new(r*0.7, r*0.7), center + EguiVec2::new(r*0.7, r*0.7)], Stroke::new(1.0, Color32::from_white_alpha(100)));
                 painter.line_segment([center - EguiVec2::new(r*0.7, -r*0.7), center + EguiVec2::new(r*0.7, -r*0.7)], Stroke::new(1.0, Color32::from_white_alpha(100)));
            }
            DeviceType::Firewall | DeviceType::FirewallAppliance => {
                // Firewall: Shield
                // Draw a simple shield shape using convex polygon
                let points = vec![
                    center + EguiVec2::new(-radius, -radius*0.5), // Top Left
                    center + EguiVec2::new(radius, -radius*0.5),  // Top Right
                    center + EguiVec2::new(radius*0.8, radius*0.5), // Bot Right
                    center + EguiVec2::new(0.0, radius), // Bot Tip
                    center + EguiVec2::new(-radius*0.8, radius*0.5), // Bot Left
                ];
                painter.add(Shape::convex_polygon(points.clone(), fill, stroke));
                
                // Flame/Icon inside? Just a cross for "Block"
                 painter.line_segment([center - EguiVec2::new(radius*0.4, radius*0.4), center + EguiVec2::new(radius*0.4, radius*0.4)], Stroke::new(1.5, Color32::WHITE));
                 painter.line_segment([center - EguiVec2::new(radius*0.4, -radius*0.4), center + EguiVec2::new(-radius*0.4, radius*0.4)], Stroke::new(1.5, Color32::WHITE));
            }
            DeviceType::IoT | DeviceType::Camera | DeviceType::Thermostat => {
                // IoT: Hexagon or Chip
                let points = (0..6).map(|i| {
                    let angle = std::f32::consts::TAU * i as f32 / 6.0;
                    center + EguiVec2::new(angle.cos() * radius, angle.sin() * radius)
                }).collect();
                 painter.add(Shape::convex_polygon(points, fill, stroke));
                 
                 // Chip legs (implied)
                 painter.circle_filled(center, radius*0.4, Color32::from_black_alpha(100));
            }
            _ => {
                // Generic Device: Screen / Monitor shape
                 let rect_size = EguiVec2::new(radius * 1.8, radius * 1.2);
                 let rect = Rect::from_center_size(center - EguiVec2::new(0.0, radius*0.2), rect_size);
                 painter.rect_filled(rect, 2.0, fill);
                 painter.rect_stroke(rect, 1.5, stroke);
                 
                 // Stand
                 painter.line_segment([center + EguiVec2::new(0.0, radius*0.4), center + EguiVec2::new(0.0, radius)], Stroke::new(2.0, Color32::GRAY));
                 painter.line_segment([center + EguiVec2::new(-radius*0.6, radius), center + EguiVec2::new(radius*0.6, radius)], Stroke::new(2.0, Color32::GRAY));
            }
        }
    }

    fn get_node_color(&self, node: &NodeData, mode: crate::topology::HighlightMode) -> Color32 {
        match mode {
            crate::topology::HighlightMode::DeviceType => self.get_device_color(node.device_type),
            crate::topology::HighlightMode::RiskScore => self.get_risk_color(node.risk_score),
            crate::topology::HighlightMode::Latency => {
                 // Nodes are neutral in latency mode
                 Color32::from_gray(100)
            }
        }
    }

    fn get_risk_color(&self, risk_score: u8) -> Color32 {
        if risk_score > 70 {
            Color32::from_rgb(255, 80, 80) // Critical - Red
        } else if risk_score > 30 {
            Color32::from_rgb(255, 200, 50) // Medium - Yellow/Orange
        } else if risk_score > 0 {
            Color32::from_rgb(80, 255, 100) // Low - Green
        } else {
            Color32::from_gray(100) // None
        }
    }

    fn get_device_color(&self, device_type: DeviceType) -> Color32 {
        match device_type {
            DeviceType::Internet => Color32::from_rgb(255, 215, 0), // Gold
            DeviceType::Router => Color32::from_rgb(46, 204, 113),
            DeviceType::Server => Color32::from_rgb(52, 152, 219),
            DeviceType::Firewall => Color32::from_rgb(231, 76, 60),
            DeviceType::IoT => Color32::from_rgb(155, 89, 182),
            DeviceType::Workstation => Color32::from_rgb(149, 165, 166),
            _ => Color32::from_rgb(127, 140, 141),
        }
    }

    fn handle_interaction(
        &mut self,
        ui: &mut egui::Ui,
        response: &egui::Response,
        viewport_rect: Rect,
        view_state: &mut TopologyViewState,
    ) {
        // Handle Zooming (Mouse Wheel) - Only if hovering
        if response.hovered() {
            let mut zoom_factor = ui.input(|i| i.zoom_delta());
            
            // Support scroll-to-zoom without Ctrl key (standard for maps)
            if zoom_factor == 1.0 {
                let raw_scroll = ui.input(|i| i.raw_scroll_delta);
                let dy = raw_scroll.y;
                
                if dy != 0.0 {
                    // Normalize scroll speed across platforms - Reduced sensitivity
                    let delta = (dy / 200.0).clamp(-0.5, 0.5);
                    zoom_factor = 1.0 + delta;
                }
            }

            if zoom_factor != 1.0 {
                if let Some(mouse_pos) = ui.input(|i| i.pointer.hover_pos()) {
                    let world_mouse = self.screen_to_world(mouse_pos, viewport_rect, view_state);
                    view_state.zoom = (view_state.zoom * zoom_factor).clamp(0.05, 10.0);
                    let new_screen_pos = self.world_to_screen(world_mouse, viewport_rect, view_state);
                    view_state.pan_offset += mouse_pos - new_screen_pos;
                }
            }
        }

        // Handle Hover & Selection
        if let Some(mouse_pos) = ui.input(|i| i.pointer.hover_pos()) {
            // Check if mouse is within viewport
            if !viewport_rect.contains(mouse_pos) {
                return;
            }

            let world_mouse = self.screen_to_world(mouse_pos, viewport_rect, view_state);
            let mut closest_node = None;
            let mut min_dist = 25.0 / view_state.zoom;

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

                if response.clicked() || response.secondary_clicked() {
                    view_state.selected_node = Some(crate::topology::NodeId(node_data.ip));
                }

                if response.drag_started() {
                    self.dragging_node = Some(idx);
                    self.layout_stable = false; // Wake up simulation
                }
            } else {
                view_state.hovered_node = None;
                if response.clicked() {
                    view_state.selected_node = None;
                }
            }
        }

        // Handle Dragging
        if let Some(idx) = self.dragging_node {
            if ui.input(|i| i.pointer.any_down()) {
                if let Some(mouse_pos) = ui.input(|i| i.pointer.hover_pos()) {
                    let world_mouse = self.screen_to_world(mouse_pos, viewport_rect, view_state);
                    self.positions.insert(idx, world_mouse);
                    
                    // Update engine with new position so physics reacts
                    self.layout_engine.set_position(idx, Vec2::new(world_mouse.x, world_mouse.y));
                    self.layout_stable = false;
                    
                    ui.ctx().request_repaint();
                }
            } else {
                self.dragging_node = None;
            }
        } else if response.dragged() {
            // Panning: Move the view WITH the mouse (Standard/Natural)
            view_state.pan_offset += response.drag_delta();
        }
    }

    fn render_stats_panel(&self, painter: &egui::Painter, viewport_rect: Rect) {
        let stats = self.graph.get_stats();
        let stats_text = format!(
            "Nodes: {}\nEdges: {}\nRouters: {}\nServers: {}",
            stats.node_count, stats.edge_count, stats.router_count, stats.server_count
        );

        let stats_rect = Rect::from_min_size(
            viewport_rect.min + EguiVec2::new(15.0, 60.0),
            EguiVec2::new(130.0, 85.0),
        );

        painter.rect_filled(stats_rect, 6.0, Color32::from_black_alpha(200));
        painter.rect_stroke(stats_rect, 6.0, Stroke::new(1.0, Color32::from_gray(80)));

        painter.text(
            stats_rect.min + EguiVec2::new(10.0, 10.0),
            egui::Align2::LEFT_TOP,
            stats_text,
            egui::FontId::monospace(12.0),
            Color32::WHITE,
        );
    }

    fn render_legend(&self, painter: &egui::Painter, viewport_rect: Rect) {
        let legend_width = 200.0;
        let legend_height = 240.0;
        let margin = 15.0;
        
        let legend_rect = Rect::from_min_size(
            viewport_rect.left_bottom() + EguiVec2::new(margin, -legend_height - margin),
            EguiVec2::new(legend_width, legend_height),
        );

        painter.rect_filled(legend_rect, 8.0, Color32::from_black_alpha(230));
        painter.rect_stroke(legend_rect, 8.0, Stroke::new(1.0, Color32::from_gray(70)));

        let mut y = legend_rect.min.y + 15.0;
        let x_offset = legend_rect.min.x + 15.0;
        let line_height = 24.0;

        // Header
        painter.text(
            Pos2::new(x_offset, y),
            egui::Align2::LEFT_CENTER,
            "NETWORK LEGEND",
            egui::FontId::monospace(14.0),
            Color32::WHITE,
        );
        y += line_height * 1.2;

        // Device Types
        let types = [
            (DeviceType::Internet, "Internet Hub"),
            (DeviceType::Router, "Router / Gateway"),
            (DeviceType::Server, "Server / Infrastructure"),
            (DeviceType::Unknown, "Workstation / Host"),
        ];

        for (dtype, label) in types {
            let icon_pos = Pos2::new(x_offset + 10.0, y);
            self.draw_icon(painter, icon_pos, 7.0, dtype, Color32::WHITE);
            
            painter.text(
                Pos2::new(x_offset + 30.0, y),
                egui::Align2::LEFT_CENTER,
                label,
                egui::FontId::proportional(13.0),
                Color32::from_gray(200),
            );
            y += line_height;
        }

        y += 10.0;
        // Risk Levels
        painter.text(
            Pos2::new(x_offset, y),
            egui::Align2::LEFT_CENTER,
            "SECURITY RISK",
            egui::FontId::monospace(12.0),
            Color32::from_gray(160),
        );
        y += line_height;

        let risks = [
            (Color32::from_rgb(80, 255, 100), "Low Risk"),
            (Color32::from_rgb(255, 200, 50), "Medium Risk"),
            (Color32::from_rgb(255, 80, 80), "Critical Risk"),
        ];

        for (color, label) in risks {
            painter.circle_filled(Pos2::new(x_offset + 10.0, y), 5.0, color);
            painter.text(
                Pos2::new(x_offset + 30.0, y),
                egui::Align2::LEFT_CENTER,
                label,
                egui::FontId::proportional(13.0),
                Color32::from_gray(200),
            );
            y += line_height;
        }
    }

    fn get_node_label(&self, node_data: &NodeData, lod: LODLevel, public_ip: Option<&str>) -> String {
        let mut base = match lod {
            LODLevel::Minimal | LODLevel::Low => String::new(),
            LODLevel::Medium | LODLevel::High | LODLevel::Full => {
                let ip_str = node_data.ip.to_string();
                match node_data.device_type {
                    DeviceType::Internet => {
                        if let Some(pub_ip) = public_ip {
                            format!("{} ({:?})\nvia {}", ip_str, node_data.device_type, pub_ip)
                        } else {
                            format!("{} ({:?})", ip_str, node_data.device_type)
                        }
                    }
                    DeviceType::Router | DeviceType::Firewall | DeviceType::Switch => {
                        format!("{} ({:?})", ip_str, node_data.device_type)
                    }
                    _ => {
                        node_data.hostname.as_ref().cloned().unwrap_or(ip_str)
                    }
                }
            }
        };

        if !base.is_empty() {
            if let Some(ref geo) = node_data.geo_location {
                if let Some(ref country) = geo.country {
                    if !base.contains('\n') {
                         base = format!("{} [{}]", base, country);
                    }
                }
            }
        }
        base
    }

    fn sample_quadratic_bezier(&self, p0: Pos2, p1: Pos2, p2: Pos2, t: f32) -> Pos2 {
        let t_inv = 1.0 - t;
        let x = t_inv * t_inv * p0.x + 2.0 * t_inv * t * p1.x + t * t * p2.x;
        let y = t_inv * t_inv * p0.y + 2.0 * t_inv * t * p1.y + t * t * p2.y;
        Pos2::new(x, y)
    }
}
