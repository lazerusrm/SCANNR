use crate::topology::{NodeData, TopologyGraph, TopologyViewState};
use crate::topology::renderer::{TopologyRenderer};
use egui::{Rect, Ui, Layout, Align, Color32, RichText};

#[derive(Debug, Clone, PartialEq, Default)]
pub enum LayoutType {
    #[default]
    ForceDirected,
    Circular,
    Hierarchical,
}

#[derive(Debug)]
pub struct TopologyWidget {
    renderer: TopologyRenderer,
    view_state: TopologyViewState,
}

impl TopologyWidget {
    pub fn new(graph: TopologyGraph) -> Self {
        let mut renderer = TopologyRenderer::new(graph);
        renderer.compute_layout();
        Self {
            renderer,
            view_state: TopologyViewState::default(),
        }
    }

    pub fn show(&mut self, ui: &mut Ui, viewport_rect: Rect) {
        // Main rendering area
        self.renderer.render(ui, viewport_rect, &mut self.view_state);

        // Selection details panel (overlay on the right)
        if let Some(selected_id) = &self.view_state.selected_node {
            // Find node data
            let node_data = self.renderer.graph.graph.node_weights()
                .find(|n| n.ip == selected_id.0);

            if let Some(node) = node_data {
                self.draw_details_panel(ui, viewport_rect, node);
            }
        }
    }

    fn draw_details_panel(&self, ui: &mut Ui, viewport_rect: Rect, node: &NodeData) {
        let panel_width = 280.0;
        let margin = 15.0;
        
        let panel_rect = Rect::from_min_max(
            viewport_rect.right_top() + egui::vec2(-panel_width - margin, margin),
            viewport_rect.right_bottom() + egui::vec2(-margin, -margin),
        );

        let mut ui = ui.child_ui(panel_rect, Layout::top_down(Align::Min));
        
        egui::Frame::none()
            .fill(Color32::from_black_alpha(220))
            .stroke(egui::Stroke::new(1.0, Color32::from_gray(80)))
            .rounding(8.0)
            .inner_margin(12.0)
            .show(&mut ui, |ui| {
                ui.set_width(panel_width - 24.0);
                
                ui.horizontal(|ui| {
                    ui.heading(RichText::new("Node Details").strong().color(Color32::WHITE));
                });
                
                ui.add_space(8.0);
                ui.separator();
                ui.add_space(8.0);
                
                egui::Grid::new("node_details_grid")
                    .num_columns(2)
                    .spacing([10.0, 8.0])
                    .show(ui, |ui| {
                        ui.label(RichText::new("IP:").weak());
                        ui.label(RichText::new(node.ip.to_string()).strong());
                        ui.end_row();
                        
                        if let Some(host) = &node.hostname {
                            ui.label(RichText::new("Host:").weak());
                            ui.label(host);
                            ui.end_row();
                        }
                        
                        if let Some(mac) = &node.mac {
                            ui.label(RichText::new("MAC:").weak());
                            ui.label(mac);
                            ui.end_row();
                        }

                        if let Some(vendor) = &node.vendor {
                            ui.label(RichText::new("Vendor:").weak());
                            ui.label(vendor);
                            ui.end_row();
                        }
                        
                        ui.label(RichText::new("Type:").weak());
                        ui.colored_label(self.get_type_color(node.device_type), format!("{:?}", node.device_type));
                        ui.end_row();
                    });
                
                ui.add_space(15.0);
                ui.label(RichText::new(format!("Risk Score: {}", node.risk_score)).strong());
                let risk_color = if node.risk_score > 70 { Color32::RED } else if node.risk_score > 30 { Color32::GOLD } else { Color32::GREEN };
                ui.add(egui::ProgressBar::new(node.risk_score as f32 / 100.0).fill(risk_color));
                
                if !node.ports.is_empty() {
                    ui.add_space(15.0);
                    ui.label(RichText::new("Open Ports").underline());
                    ui.add_space(5.0);
                    egui::ScrollArea::vertical()
                        .max_height(200.0)
                        .auto_shrink([false, true])
                        .show(ui, |ui| {
                            for port in &node.ports {
                                ui.horizontal(|ui| {
                                    ui.label(RichText::new(port.port.to_string()).color(Color32::from_rgb(100, 255, 100)));
                                    ui.label(RichText::new(format!("/{:?}", port.protocol)).small().weak());
                                    if let Some(service) = &port.service {
                                        ui.add_space(5.0);
                                        ui.label(service);
                                    }
                                });
                            }
                        });
                }
                
                if let Some(geo) = &node.geo_location {
                    ui.add_space(15.0);
                    ui.label(RichText::new("Location").underline());
                    if let (Some(city), Some(country)) = (&geo.city, &geo.country) {
                        ui.label(format!("{}, {}", city, country));
                    }
                }
            });
    }

    fn get_type_color(&self, device_type: crate::topology::DeviceType) -> Color32 {
        match device_type {
            crate::topology::DeviceType::Router => Color32::from_rgb(46, 204, 113),
            crate::topology::DeviceType::Server => Color32::from_rgb(52, 152, 219),
            crate::topology::DeviceType::Firewall => Color32::from_rgb(231, 76, 60),
            _ => Color32::from_gray(180),
        }
    }

    pub fn compute_layout(&mut self, _layout_type: LayoutType) {
        self.renderer.compute_layout();
    }

    pub fn zoom_to_fit(&mut self) {
        self.view_state.zoom = 1.0;
        self.view_state.pan_offset = egui::Vec2::ZERO;
    }

    pub fn reset_view(&mut self) {
        self.view_state = TopologyViewState::default();
        self.renderer.compute_layout();
    }

    pub fn search(&mut self, query: &str) {
        if query.is_empty() {
            self.view_state.selected_node = None;
            return;
        }

        let query = query.to_lowercase();
        let found = self.renderer.graph.graph.node_weights()
            .find(|n| {
                n.ip.to_string().contains(&query) || 
                n.hostname.as_ref().map_or(false, |h| h.to_lowercase().contains(&query))
            });

        if let Some(node) = found {
            self.view_state.selected_node = Some(crate::topology::NodeId(node.ip));
        }
    }

    pub fn get_stats(&self) -> crate::topology::TopologyStats {
        crate::topology::TopologyStats::from_graph(&self.renderer.graph)
    }

    pub fn set_selection(&mut self, nodes: std::collections::HashSet<petgraph::graph::NodeIndex>) {
        if let Some(first) = nodes.iter().next() {
            if let Some(node) = self.renderer.graph.graph.node_weight(*first) {
                self.view_state.selected_node = Some(crate::topology::NodeId(node.ip));
            }
        }
    }

    pub fn clear_selection(&mut self) {
        self.view_state.selected_node = None;
    }
}