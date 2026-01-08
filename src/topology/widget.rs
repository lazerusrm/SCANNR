use crate::topology::{NodeData, TopologyGraph, TopologyViewState};
use crate::topology::renderer::{TopologyRenderer};
use egui::{Rect, Ui, Layout, Align};

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
        let panel_width = 250.0;
        let panel_rect = Rect::from_min_max(
            viewport_rect.right_top() - egui::vec2(panel_width + 10.0, -10.0),
            viewport_rect.right_bottom() - egui::vec2(10.0, 10.0),
        );

        let mut ui = ui.child_ui(panel_rect, Layout::top_down(Align::Min));
        
        egui::Frame::window(ui.style()).show(&mut ui, |ui| {
            ui.set_width(panel_width);
            ui.heading("Node Details");
            ui.separator();
            
            ui.label(format!("IP: {}", node.ip));
            if let Some(host) = &node.hostname {
                ui.label(format!("Hostname: {}", host));
            }
            if let Some(vendor) = &node.vendor {
                ui.label(format!("Vendor: {}", vendor));
            }
            ui.label(format!("Type: {:?}", node.device_type));
            
            ui.add_space(10.0);
            ui.label(format!("Risk Score: {}", node.risk_score));
            ui.add(egui::ProgressBar::new(node.risk_score as f32 / 100.0));
            
            if !node.ports.is_empty() {
                ui.add_space(10.0);
                ui.label("Open Ports:");
                egui::ScrollArea::vertical().max_height(100.0).show(ui, |ui| {
                    for port in &node.ports {
                        ui.label(format!("• {} ({:?})", port.port, port.service.as_deref().unwrap_or("unknown")));
                    }
                });
            }
        });
    }

    pub fn compute_layout(&mut self, _layout_type: LayoutType) {
        // Current renderer supports force-directed
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

    