use crate::topology::renderer::TopologyRenderer;
use crate::topology::{NodeData, TopologyGraph, TopologyViewState, LayoutType};
use egui::{Align, Color32, Layout, Rect, RichText, Ui};

#[derive(Debug, Clone, PartialEq)]
pub enum TopologyAction {
    None,
    ScanHost(std::net::IpAddr),
    OpenWeb(std::net::IpAddr, u16),
}

#[derive(Debug)]
pub struct TopologyWidget {
    renderer: TopologyRenderer,
    pub view_state: TopologyViewState,
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

    pub fn show(&mut self, ui: &mut Ui, viewport_rect: Rect, public_ip_override: Option<&str>, public_ports: Option<&[u16]>) -> TopologyAction {
        let mut action = TopologyAction::None;

        // Apply theme
        self.renderer.apply_theme(ui.visuals().dark_mode);

        // Controls Overlay (Top Left)
        let controls_rect = Rect::from_min_size(
            viewport_rect.min + egui::vec2(15.0, 15.0),
            egui::vec2(200.0, 40.0),
        );
        
        let mut controls_ui = ui.child_ui(controls_rect, Layout::top_down(Align::Min));
        egui::Frame::none()
            .fill(Color32::from_black_alpha(180))
            .stroke(egui::Stroke::new(1.0, Color32::from_gray(80)))
            .rounding(4.0)
            .inner_margin(6.0)
            .show(&mut controls_ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Color Mode:").size(14.0));
                    egui::ComboBox::from_id_source("highlight_mode")
                        .selected_text(RichText::new(match self.view_state.highlight_mode {
                            crate::topology::HighlightMode::DeviceType => "Device Type",
                            crate::topology::HighlightMode::RiskScore => "Risk Score",
                            crate::topology::HighlightMode::Latency => "Latency",
                        }).size(14.0))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut self.view_state.highlight_mode,
                                crate::topology::HighlightMode::DeviceType,
                                RichText::new("Device Type").size(14.0),
                            );
                            ui.selectable_value(
                                &mut self.view_state.highlight_mode,
                                crate::topology::HighlightMode::RiskScore,
                                RichText::new("Risk Score").size(14.0),
                            );
                            ui.selectable_value(
                                &mut self.view_state.highlight_mode,
                                crate::topology::HighlightMode::Latency,
                                RichText::new("Latency").size(14.0),
                            );
                        });
                });
            });

        // Main rendering area
        self.renderer
            .render(ui, viewport_rect, &mut self.view_state, public_ip_override);

        // Handle interactions
        if let Some(hovered) = &self.view_state.hovered_node {
             if ui.input(|i| i.pointer.button_clicked(egui::PointerButton::Secondary)) {
                 self.view_state.selected_node = Some(hovered.clone());
             }
        }

        // Selection details panel (overlay on the right)
        if let Some(selected_id) = &self.view_state.selected_node {
            // Find node data
            let node_data = self
                .renderer
                .graph
                .graph
                .node_weights()
                .find(|n| n.ip == selected_id.0);

            if let Some(node) = node_data {
                if node.device_type == crate::topology::DeviceType::Internet && public_ip_override.is_some() {
                    self.draw_internet_details_panel(ui, viewport_rect, public_ip_override.unwrap(), public_ports);
                } else {
                    if let Some(act) = self.draw_details_panel(ui, viewport_rect, node) {
                        action = act;
                    }
                }
            }
        }

        action
    }

    fn draw_internet_details_panel(&self, ui: &mut Ui, viewport_rect: Rect, public_ip: &str, ports: Option<&[u16]>) {
        let panel_width = 300.0;
        let margin = 20.0;

        let panel_rect = Rect::from_min_max(
            viewport_rect.right_top() + egui::vec2(-panel_width - margin, margin),
            viewport_rect.right_bottom() + egui::vec2(-margin, -margin),
        );

        let mut ui = ui.child_ui(panel_rect, Layout::top_down(Align::Min));

        egui::Frame::none()
            .fill(Color32::from_black_alpha(200))
            .stroke(egui::Stroke::new(1.0, Color32::from_gray(60)))
            .rounding(12.0)
            .inner_margin(16.0)
            .show(&mut ui, |ui| {
                ui.set_width(panel_width - 32.0);

                ui.vertical_centered(|ui| {
                    ui.horizontal(|ui| {
                        ui.add_space(ui.available_width() / 4.0);
                        ui.add(egui::Image::new(egui::include_image!("../../assets/logo.png")).max_width(24.0).rounding(4.0));
                        ui.heading(RichText::new("EXTERNAL NETWORK").strong().color(Color32::WHITE).size(16.0));
                    });
                    ui.add_space(10.0);
                    ui.label(RichText::new("Public IP Address").weak().size(14.0));
                    ui.heading(RichText::new(public_ip).color(Color32::GREEN).size(18.0).monospace());
                });

                ui.add_space(15.0);
                ui.separator();
                ui.add_space(15.0);

                if draw_polished_button(ui, RichText::new("📋 Copy to Clipboard").size(14.0), Color32::from_rgb(60, 100, 140)).clicked() {
                    ui.output_mut(|o| o.copied_text = public_ip.to_string());
                }
                
                ui.add_space(20.0);
                
                if let Some(ports) = ports {
                    if !ports.is_empty() {
                        ui.label(RichText::new("OPEN PORTS").strong().size(14.0));
                        ui.add_space(4.0);
                        egui::ScrollArea::vertical().max_height(100.0).show(ui, |ui| {
                            for port in ports {
                                ui.horizontal(|ui| {
                                    ui.label(RichText::new(port.to_string()).monospace().color(Color32::from_rgb(255, 100, 100)).size(14.0));
                                });
                            }
                        });
                        ui.add_space(4.0);
                        ui.label(RichText::new("Source: Shodan InternetDB (Data may be up to 7 days old)").weak().size(10.0));
                        ui.add_space(15.0);
                    } else {
                        ui.label(RichText::new("No open ports found.").weak().size(14.0));
                        ui.add_space(4.0);
                        ui.label(RichText::new("Source: Shodan InternetDB (Data may be up to 7 days old)").weak().size(10.0));
                        ui.add_space(15.0);
                    }
                } else {
                    ui.horizontal(|ui| {
                        ui.spinner();
                        ui.label(RichText::new("Scanning ports...").weak().size(14.0));
                    });
                    ui.add_space(15.0);
                }

                ui.label(RichText::new("This node represents your network's gateway to the public internet. The IP address shown is visible to external servers.").weak().size(12.0));
            });
    }

    fn draw_details_panel(&self, ui: &mut Ui, viewport_rect: Rect, node: &NodeData) -> Option<TopologyAction> {
        let mut action = None;
        let panel_width = 300.0;
        let margin = 20.0;

        let panel_rect = Rect::from_min_max(
            viewport_rect.right_top() + egui::vec2(-panel_width - margin, margin),
            viewport_rect.right_bottom() + egui::vec2(-margin, -margin),
        );

        let mut ui = ui.child_ui(panel_rect, Layout::top_down(Align::Min));

        egui::Frame::none()
            .fill(Color32::from_black_alpha(200))
            .stroke(egui::Stroke::new(1.0, Color32::from_gray(60)))
            .rounding(12.0)
            .inner_margin(16.0)
            .show(&mut ui, |ui| {
                ui.set_width(panel_width - 32.0);

                ui.vertical_centered(|ui| {
                    ui.horizontal(|ui| {
                        ui.add_space(ui.available_width() / 4.0); // Simple centering trick
                        ui.add(egui::Image::new(egui::include_image!("../../assets/logo.png")).max_width(24.0).rounding(4.0));
                        ui.heading(RichText::new("HOST INFORMATION").strong().color(Color32::WHITE).size(16.0));
                    });
                    ui.label(RichText::new(node.ip.to_string()).color(Color32::from_rgb(100, 200, 255)).monospace().size(14.0));
                });

                ui.add_space(12.0);
                ui.separator();
                ui.add_space(12.0);

                egui::Grid::new("node_details_grid")
                    .num_columns(2)
                    .spacing([12.0, 10.0])
                    .show(ui, |ui| {
                        ui.label(RichText::new("Hostname:").weak().size(14.0));
                        ui.label(RichText::new(node.hostname.as_deref().unwrap_or("N/A")).size(14.0));
                        ui.end_row();

                        ui.label(RichText::new("Device Type:").weak().size(14.0));
                        let type_color = self.get_type_color(node.device_type);
                        ui.colored_label(type_color, RichText::new(format!("{:?}", node.device_type)).size(14.0));
                        ui.end_row();

                        ui.label(RichText::new("Vendor:").weak().size(14.0));
                        ui.label(RichText::new(node.vendor.as_deref().unwrap_or("Unknown")).size(14.0));
                        ui.end_row();

                        if let Some(mac) = &node.mac {
                            ui.label(RichText::new("MAC Addr:").weak().size(14.0));
                            ui.label(RichText::new(mac).monospace().size(12.0));
                            ui.end_row();
                        }
                    });

                ui.add_space(20.0);
                
                // Security section
                ui.label(RichText::new("SECURITY STATUS").strong().size(14.0));
                ui.add_space(4.0);
                
                let risk_color = if node.risk_score > 70 {
                    Color32::from_rgb(255, 80, 80)
                } else if node.risk_score > 30 {
                    Color32::from_rgb(255, 200, 50)
                } else {
                    Color32::from_rgb(80, 255, 100)
                };

                ui.horizontal(|ui| {
                    ui.label(RichText::new(format!("Risk Score: {}", node.risk_score)).size(14.0));
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        let level = if node.risk_score > 70 { "CRITICAL" } else if node.risk_score > 30 { "MEDIUM" } else { "LOW" };
                        ui.label(RichText::new(level).color(risk_color).strong().size(12.0));
                    });
                });
                ui.add(egui::ProgressBar::new(node.risk_score as f32 / 100.0).fill(risk_color).show_percentage());

                ui.add_space(12.0);
                if draw_polished_button(ui, RichText::new("📋 Copy IP Address").size(14.0), Color32::from_gray(60)).clicked() {
                    ui.output_mut(|o| o.copied_text = node.ip.to_string());
                }
                
                ui.add_space(8.0);
                if draw_polished_button(ui, RichText::new("🔍 Targeted Port Scan").size(14.0), Color32::from_rgb(0, 120, 215)).clicked() {
                    action = Some(TopologyAction::ScanHost(node.ip));
                }
                
                let has_web = node.ports.iter().any(|p| p.port == 80 || p.port == 443 || p.port == 8080);
                if has_web {
                    ui.add_space(8.0);
                    if draw_polished_button(ui, RichText::new("🌐 Open Web Interface").size(14.0), Color32::from_rgb(50, 150, 80)).clicked() {
                        action = Some(TopologyAction::OpenWeb(node.ip, 80));
                    }
                }

                if !node.ports.is_empty() {
                    ui.add_space(20.0);
                    ui.label(RichText::new(format!("OPEN SERVICES ({})", node.ports.len())).strong().size(14.0));
                    ui.add_space(4.0);
                    
                    egui::ScrollArea::vertical()
                        .max_height(150.0)
                        .auto_shrink([false, true])
                        .show(ui, |ui| {
                            for port in &node.ports {
                                ui.horizontal(|ui| {
                                    ui.label(RichText::new(format!("{:>5}", port.port)).color(Color32::from_rgb(80, 255, 100)).monospace().size(14.0));
                                    ui.label(RichText::new(format!("/{:?}", port.protocol)).weak().size(12.0));
                                    if let Some(service) = &port.service {
                                        ui.add_space(8.0);
                                        ui.label(RichText::new(service).size(14.0));
                                    }
                                });
                            }
                        });
                }

                if let Some(geo) = &node.geo_location {
                    ui.add_space(20.0);
                    ui.label(RichText::new("GEOLOCATION").strong().size(14.0));
                    ui.add_space(4.0);
                    
                    if let Some(country) = &geo.country {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("Location:").weak().size(14.0));
                            ui.label(RichText::new(format!("{}, {}", geo.city.as_deref().unwrap_or("Unknown"), country)).size(14.0));
                        });
                    }
                }
            });
            
        action
    }

    fn get_type_color(&self, device_type: crate::topology::DeviceType) -> Color32 {
        match device_type {
            crate::topology::DeviceType::Internet => Color32::from_rgb(255, 215, 0),
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
        let found = self.renderer.graph.graph.node_weights().find(|n| {
            n.ip.to_string().contains(&query)
                || n.hostname
                    .as_ref()
                    .is_some_and(|h| h.to_lowercase().contains(&query))
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

/// Helper to draw a polished, Apple-style button with glass effect
fn draw_polished_button(ui: &mut egui::Ui, text: impl Into<egui::WidgetText>, fill_color: Color32) -> egui::Response {
    let padding = egui::vec2(10.0, 4.0); // Further reduced padding
    let rounding = 8.0;
    
    let wrap_width = ui.available_width();
    let text = text.into().fallback_text_style(egui::TextStyle::Button);
    let galley = text.into_galley(ui, None, wrap_width, egui::TextStyle::Button);
    
    let desired_size = galley.size() + padding * 2.0;
    let (rect, response) = ui.allocate_at_least(desired_size, egui::Sense::click());
    
    if ui.is_rect_visible(rect) {
        let _visuals = ui.style().interact(&response);
        
        let mut base_color = fill_color;
        if response.hovered() {
            let rgb = base_color.to_array();
            base_color = Color32::from_rgba_unmultiplied(
                rgb[0].saturating_add(20),
                rgb[1].saturating_add(20),
                rgb[2].saturating_add(20),
                rgb[3]
            );
        } else if response.clicked() {
            let rgb = base_color.to_array();
            base_color = Color32::from_rgba_unmultiplied(
                rgb[0].saturating_sub(20),
                rgb[1].saturating_sub(20),
                rgb[2].saturating_sub(20),
                rgb[3]
            );
        }
        
        // Background
        ui.painter().rect_filled(rect, rounding, base_color);
        
        // Liquid Glass Highlight (Top 35% even more subtle)
        let highlight_rect = egui::Rect::from_min_max(
            rect.min,
            rect.center_top() + egui::vec2(0.0, rect.height() * 0.35)
        ).intersect(rect);
        
        ui.painter().rect_filled(
            highlight_rect, 
            egui::Rounding {
                nw: rounding,
                ne: rounding,
                sw: 1.0,
                se: 1.0,
            },
            Color32::from_white_alpha(8) // Reduced from 10
        );
        
        // Inner stroke for depth (more subtle)
        ui.painter().rect_stroke(rect, rounding, egui::Stroke::new(1.0, Color32::from_white_alpha(11))); // Reduced from 14
        
        let text_pos = rect.center() - galley.size() / 2.0;
        ui.painter().galley(text_pos, galley, Color32::WHITE);
    }
    
    response
}
