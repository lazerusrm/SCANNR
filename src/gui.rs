//! Modern GUI for SCANNR with light/dark/auto theme support
use crate::input::ScanOrder;
use crate::port_strategy::PortStrategy;
use crate::scanner::Scanner;
use crate::topology::widget::TopologyWidget;
use crate::topology::{LayoutType, TopologyStats};
use egui::{Color32, RichText, Visuals};
use ipnetwork::IpNetwork;
use serde::Serialize;
use std::collections::{HashMap, HashSet};
use std::net::IpAddr;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x08000000;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThemeMode {
    Light,
    Dark,
    Auto,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ViewMode {
    List,
    Topology,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SortOption {
    IpAddress,
    PortCount,
    Vendor,
    Hostname,
}

impl SortOption {
    fn label(&self) -> &'static str {
        match self {
            SortOption::IpAddress => "IP Address",
            SortOption::PortCount => "Port Count",
            SortOption::Vendor => "Vendor",
            SortOption::Hostname => "Hostname",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScanMode {
    Quick,
    Standard,
    Full,
}

impl ScanMode {
    fn label(&self) -> &'static str {
        match self {
            ScanMode::Quick => "Quick (15 ports)",
            ScanMode::Standard => "Standard (55 ports)",
            ScanMode::Full => "Full (65535 ports)",
        }
    }

    fn ports(&self) -> Option<Vec<u16>> {
        match self {
            ScanMode::Quick => Some(QUICK_PORTS.to_vec()),
            ScanMode::Standard => Some(STANDARD_PORTS.to_vec()),
            ScanMode::Full => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ExportFeedback {
    pub show: bool,
    pub message: String,
    pub is_error: bool,
    pub timestamp: Option<std::time::Instant>,
}

impl Default for ExportFeedback {
    fn default() -> Self {
        Self {
            show: false,
            message: String::new(),
            is_error: false,
            timestamp: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SshDialogState {
    pub show: bool,
    pub ip: Option<IpAddr>,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub connecting: bool,
}

impl Default for SshDialogState {
    fn default() -> Self {
        Self {
            show: false,
            ip: None,
            port: 22,
            username: String::new(),
            password: String::new(),
            connecting: false,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct HostInfo {
    pub ip: IpAddr,
    pub hostname: Option<String>,
    pub mac: Option<String>,
    pub vendor: Option<String>,
    pub ports: Vec<u16>,
    pub os: Option<String>,
    pub service_names: HashMap<u16, String>,
    pub user_alias: Option<String>,
}

impl HostInfo {
    pub fn from_result(
        ip: IpAddr,
        ports: Vec<u16>,
        hostname: Option<String>,
        mac: Option<String>,
    ) -> Self {
        Self {
            ip,
            hostname,
            mac,
            vendor: None,
            ports,
            os: None,
            service_names: HashMap::new(),
            user_alias: None,
        }
    }

    pub fn port_count(&self) -> usize {
        self.ports.len()
    }
}

#[derive(Debug)]
pub struct AppState {
    pub detected_subnet: Option<IpNetwork>,
    pub subnet_input: String,
    pub is_scanning: bool,
    pub is_discovering_topology: bool,
    pub scan_progress: f32,
    pub scan_status: String,
    pub scanned_ports: usize,
    pub topology_discovery_status: String,
    pub start_topology_discovery: bool,
    pub results: Vec<HostInfo>,
    pub theme_mode: ThemeMode,
    pub selected_result: Option<usize>,
    pub scanned_ips: usize,
    pub total_ips: usize,
    pub available_interfaces: Vec<NetworkInterface>,
    pub selected_interface_idx: usize,
    pub ssh_dialog: SshDialogState,
    pub view_mode: ViewMode,
    pub selected_topology_node: Option<IpAddr>,
    pub topology_node_positions: HashMap<IpAddr, egui::Pos2>,
    pub dragging_node: Option<IpAddr>,
    pub sort_by: SortOption,
    pub sort_ascending: bool,
    pub filter_text: String,
    pub filter_by_port: Option<u16>,
    pub rescan_single_ip: Option<IpAddr>,
    pub export_feedback: ExportFeedback,
    pub topology_widget: Option<TopologyWidget>,
    pub layout_type: LayoutType,
    pub topology_stats: Option<TopologyStats>,
    pub scan_mode: ScanMode,
    pub advanced_settings_open: bool,
    pub batch_size: u16,
    pub timeout_ms: u32,
    pub udp_scan: bool,
    pub public_ip: Option<String>,
    pub public_ports: Option<Vec<u16>>,
    pub editing_alias: Option<(IpAddr, String)>,
}

#[derive(Debug, Clone)]
pub struct NetworkInterface {
    pub name: String,
    pub subnet: String,
    pub gateway: Option<IpAddr>,
    pub priority: u8,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            detected_subnet: None,
            subnet_input: String::new(),
            is_scanning: false,
            is_discovering_topology: false,
            scan_progress: 0.0,
            scan_status: String::from("Ready to scan"),
            scanned_ports: 0,
            topology_discovery_status: String::from("Ready to discover"),
            start_topology_discovery: false,
            results: Vec::new(),
            theme_mode: ThemeMode::Auto,
            selected_result: None,
            scanned_ips: 0,
            total_ips: 0,
            available_interfaces: Vec::new(),
            selected_interface_idx: 0,
            ssh_dialog: SshDialogState::default(),
            view_mode: ViewMode::List,
            selected_topology_node: None,
            topology_node_positions: HashMap::new(),
            dragging_node: None,
            sort_by: SortOption::IpAddress,
            sort_ascending: true,
            filter_text: String::new(),
            filter_by_port: None,
            rescan_single_ip: None,
            export_feedback: ExportFeedback::default(),
            topology_widget: None,
            layout_type: LayoutType::ForceDirected,
            topology_stats: None,
            scan_mode: ScanMode::Quick,
            advanced_settings_open: false,
            batch_size: 3000,
            timeout_ms: 100,
            udp_scan: false,
            public_ip: None,
            public_ports: None,
            editing_alias: None,
        }
    }
}

// Quick port list - essential services
const QUICK_PORTS: [u16; 15] = [
    21, 22, 23, 25, 53, 80, 110, 143, 443, 993, 995, 3389, 8080, 8443, 554,
];

// Standard port list - extended for IoT/network devices
const STANDARD_PORTS: [u16; 55] = [
    // Standard services
    21, 22, 23, 25, 53, 80, 110, 143, 443, 993, 995, 3389, // IP Cameras
    81, 88, 554, 8000, 8554, 9000, 10554, // Network equipment
    161, 162, 8291, 8728, 8729, 10001, 20561, // Video/NVR
    37777, 37778, 37810, 8090, // IoT/Industrial
    1883, 502, 5683, 47808, 623, // NAS/Storage
    139, 445, 2049, 5000, // Management
    5900, 62078, 8443, 8883, 1900, // Additional common
    5555, 5080, 5300, 5200, 10000, // Databases & Cache
    1433, 1521, 3306, 5432, 6379, 27017,
];

fn detect_device_type(ports: &[u16]) -> String {
    let ports_set: HashSet<u16> = ports.iter().copied().collect();

    if ports_set.contains(&554) && (ports_set.contains(&8000) || ports_set.contains(&37777)) {
        return "IP Camera (Dahua)".to_string();
    }
    if ports_set.contains(&34567) || (ports_set.contains(&37777) && !ports_set.contains(&8000)) {
        return "IP Camera (Hikvision)".to_string();
    }
    if ports_set.contains(&554) {
        return "IP Camera (RTSP)".to_string();
    }
    if ports_set.contains(&81) || ports_set.contains(&88) {
        return "IP Camera (Web)".to_string();
    }
    if ports_set.contains(&8291) || ports_set.contains(&8728) || ports_set.contains(&20561) {
        return "Router (Mikrotik)".to_string();
    }
    if ports_set.contains(&10001) || ports_set.contains(&10000) || ports_set.contains(&41121) {
        return "Ubiquiti Device".to_string();
    }
    if ports_set.contains(&139) && ports_set.contains(&445) {
        return "NAS/Storage".to_string();
    }
    if ports_set.contains(&2049) || ports_set.contains(&111) {
        return "NAS/Network Storage".to_string();
    }
    if ports_set.contains(&80) && !ports_set.contains(&443) && ports_set.contains(&22) {
        return "Linux Server".to_string();
    }
    if ports_set.contains(&443) && ports_set.contains(&3389) && ports_set.contains(&22) {
        return "Windows Server".to_string();
    }
    if ports_set.contains(&53) && (ports_set.contains(&80) || ports_set.contains(&443)) {
        return "DNS Server".to_string();
    }
    if ports_set.contains(&25) || ports_set.contains(&587) || ports_set.contains(&465) {
        return "Mail Server".to_string();
    }
    if ports_set.contains(&1883) || ports_set.contains(&8883) {
        return "IoT Gateway (MQTT)".to_string();
    }
    if ports_set.contains(&502) {
        return "Industrial/PLC".to_string();
    }
    if ports_set.contains(&5683) {
        return "IoT Device (CoAP)".to_string();
    }
    if ports_set.contains(&47808) {
        return "Building Automation (BACnet)".to_string();
    }
    if ports_set.contains(&623) || ports_set.contains(&664) {
        return "IPMI/Remote Management".to_string();
    }
    if ports_set.contains(&161) && !ports_set.contains(&162) {
        return "Network Device (SNMP)".to_string();
    }
    if ports_set.contains(&5000) && ports_set.contains(&1900) {
        return "UPnP Media Server".to_string();
    }
    if ports_set.contains(&5900) {
        return "VNC Remote".to_string();
    }
    if ports_set.contains(&22) && !ports_set.contains(&80) && !ports_set.contains(&443) {
        return "SSH Server".to_string();
    }
    if ports_set.contains(&80) || ports_set.contains(&443) {
        return "Web Server".to_string();
    }
    if ports_set.contains(&22) && ports_set.contains(&80) {
        return "Web Server (SSH)".to_string();
    }
    if ports_set.contains(&23) {
        return "Telnet Device".to_string();
    }
    "Unknown Device".to_string()
}

fn get_device_icon(device_type: &str) -> egui::ImageSource<'static> {
    if device_type.contains("Camera") {
        egui::include_image!("../assets/icons/camera.svg")
    } else if device_type.contains("Router") || device_type.contains("Gateway") {
        egui::include_image!("../assets/icons/router.svg")
    } else if device_type.contains("Server") {
        egui::include_image!("../assets/icons/server.svg")
    } else if device_type.contains("IoT") || device_type.contains("Automation") {
        egui::include_image!("../assets/icons/iot.svg")
    } else if device_type.contains("NAS") || device_type.contains("Storage") {
        egui::include_image!("../assets/icons/nas.svg")
    } else if device_type.contains("Printer") {
        egui::include_image!("../assets/icons/printer.svg")
    } else if device_type.contains("TV") || device_type.contains("Media") {
        egui::include_image!("../assets/icons/tv.svg")
    } else if device_type.contains("Phone") || device_type.contains("Android") || device_type.contains("iOS") {
        egui::include_image!("../assets/icons/phone.svg")
    } else if device_type.contains("Workstation") || device_type.contains("Windows") || device_type.contains("Linux") {
        egui::include_image!("../assets/icons/computer.svg")
    } else {
        egui::include_image!("../assets/icons/default.svg")
    }
}

pub struct ScannrApp {
    state: Arc<Mutex<AppState>>,
    runtime: tokio::runtime::Runtime,
}

impl ScannrApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);

        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed to create tokio runtime");

        let state = Arc::new(Mutex::new(AppState::default()));
        let state_clone = state.clone();
        let ctx = cc.egui_ctx.clone();

        runtime.spawn(async move {
            let interfaces = enumerate_network_interfaces().await;

            let mut state = state_clone.lock().unwrap();
            state.available_interfaces = interfaces;

            if !state.available_interfaces.is_empty() {
                let primary = state.available_interfaces.first().unwrap();
                // Extract network address for subnet input (e.g., "192.168.1.0/24")
                if let Some(pos) = primary.subnet.find(' ') {
                    // Format is "192.168.1.0 (eth0)" - extract just the IP/CIDR part
                    state.subnet_input = format!("{}/24", &primary.subnet[..pos]);
                } else {
                    state.subnet_input = format!("{}/24", primary.subnet);
                }
                state.selected_interface_idx = 0;
            }

            drop(state);
            ctx.request_repaint();
        });

        Self { state, runtime }
    }
}

async fn enumerate_network_interfaces() -> Vec<NetworkInterface> {
    let mut interfaces = Vec::new();

    let active_interface_name = detect_default_route_interface();

    if let Ok(ifaces) = if_addrs::get_if_addrs() {
        let mut subnet_map: HashMap<String, (String, Option<IpAddr>)> = HashMap::new();

        for iface in &ifaces {
            let name = &iface.name;

            let lower_name = name.to_lowercase();
            if lower_name.starts_with("lo")
                || lower_name.starts_with("loopback")
                || lower_name.starts_with("docker")
                || lower_name.starts_with("veth")
                || lower_name.starts_with("csi")
                || lower_name.starts_with("hsr")
                || lower_name.starts_with("ifb")
                || lower_name.starts_with("nm")
                || lower_name.starts_with("br")
                || lower_name.starts_with("tun")
                || lower_name.starts_with("tap")
                || lower_name == "lo0"
            {
                continue;
            }

            let addr = &iface.addr;
            let ip = addr.ip();

            if ip.is_loopback() {
                continue;
            }

            if let IpAddr::V4(ipv4) = ip {
                let net = ipnetwork::Ipv4Network::new(ipv4, 24).unwrap();
                let ip_str = net.network().to_string();
                let prefix = net.prefix();
                let gateway: IpAddr = net.broadcast().into();

                let existing = subnet_map
                    .entry(format!("{}/{}", ip_str, prefix))
                    .or_insert_with(|| (format!("{} ({})", ip_str, name), Some(gateway)));

                if name.starts_with("eth") || name.starts_with("en") || name.starts_with("wi") {
                    existing.0 = format!("{} ({})", ip_str, name);
                }
            }
        }

        for (_, (name, gateway)) in subnet_map {
            let name_for_check = name.clone();
            let is_active = active_interface_name
                .as_ref()
                .map(|active| name_for_check.contains(active))
                .unwrap_or(false);

            let priority = if is_active {
                0
            } else if name_for_check.contains("eth")
                || name_for_check.contains("en")
                || name_for_check.contains("wi")
                || name_for_check.contains("wlan")
            {
                1
            } else {
                2
            };

            interfaces.push(NetworkInterface {
                name: name.clone(),
                subnet: name,
                gateway,
                priority,
            });
        }
    }

    interfaces.sort_by_key(|i| i.priority);
    interfaces
}

#[cfg(target_os = "windows")]
fn detect_default_route_interface() -> Option<String> {
    use std::process::Command;

    let mut interfaces = Vec::new();

    if let Ok(output) = Command::new("powershell")
        .args([
            "-Command",
            "Get-NetIPConfiguration | Where-Object { $_.IPv4DefaultGateway -ne $null } | Select-Object -ExpandProperty InterfaceAlias"
        ])
        .creation_flags(CREATE_NO_WINDOW)
        .output()
    {
        let output_str = String::from_utf8_lossy(&output.stdout);
        for line in output_str.lines() {
            let interface_name = line.trim();
            if !interface_name.is_empty() {
                interfaces.push(interface_name.to_string());
            }
        }
    }

    if interfaces.is_empty() {
        if let Ok(output) = Command::new("powershell")
            .args([
                "-Command",
                "Get-NetRoute -DestinationPrefix '0.0.0.0/0' | Select-Object -First 1 -ExpandProperty InterfaceAlias"
            ])
            .creation_flags(CREATE_NO_WINDOW)
            .output()
        {
            let output_str = String::from_utf8_lossy(&output.stdout);
            let interface_name = output_str.trim();
            if !interface_name.is_empty() {
                return Some(interface_name.to_string());
            }
        }
    } else {
        return interfaces.first().cloned();
    }

    None
}

#[cfg(target_family = "unix")]
fn detect_default_route_interface() -> Option<String> {
    use std::process::Command;

    if let Ok(output) = Command::new("ip")
        .args(["route", "show", "default"])
        .output()
    {
        let output_str = String::from_utf8_lossy(&output.stdout);
        for line in output_str.lines() {
            if line.contains("default") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if let Some(&"dev") = parts.get(4) {
                    if let Some(interface) = parts.get(5) {
                        return Some(interface.to_string());
                    }
                }
            }
        }
    }

    if let Ok(output) = Command::new("netstat").args(["-nr"]).output() {
        let output_str = String::from_utf8_lossy(&output.stdout);
        for line in output_str.lines() {
            if line.contains("default") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if let Some(interface) = parts.last() {
                    return Some(interface.to_string());
                }
            }
        }
    }

    None
}

impl eframe::App for ScannrApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut state = self.state.lock().unwrap();

        // --- GLOBAL POLISH ---
        let mut visuals = if state.theme_mode == ThemeMode::Light {
            let mut vis = Visuals::light();
            vis.override_text_color = Some(Color32::BLACK);
            vis.panel_fill = Color32::from_rgb(245, 246, 250);
            vis
        } else {
            let mut vis = Visuals::dark();
            vis.panel_fill = Color32::from_rgb(15, 17, 20);
            vis.widgets.noninteractive.bg_fill = Color32::from_rgb(22, 24, 28);
            vis
        };
        
        // Apple-style refinements
        visuals.widgets.noninteractive.rounding = 10.0.into();
        visuals.widgets.inactive.rounding = 8.0.into();
        visuals.widgets.hovered.rounding = 8.0.into();
        visuals.widgets.active.rounding = 8.0.into();
        
        // Add subtle shadows to windows
        visuals.window_shadow.blur = 20.0;
        visuals.window_shadow.spread = 2.0;
        visuals.window_shadow.color = Color32::from_black_alpha(80);
        
        ctx.set_visuals(visuals);

        // Global font size unification
        let mut style = (*ctx.style()).clone();
        style.text_styles.insert(
            egui::TextStyle::Body,
            egui::FontId::new(18.0, egui::FontFamily::Proportional),
        );
        style.text_styles.insert(
            egui::TextStyle::Button,
            egui::FontId::new(18.0, egui::FontFamily::Proportional),
        );
        style.text_styles.insert(
            egui::TextStyle::Heading,
            egui::FontId::new(24.0, egui::FontFamily::Proportional),
        );
        ctx.set_style(style);
        // ----------------------

        // Request repaint during active operations
        if state.is_scanning || state.is_discovering_topology {
            ctx.request_repaint();
        }

        // Handle topology discovery trigger
        let should_start_topology =
            state.start_topology_discovery && !state.is_discovering_topology;
        if should_start_topology {
            state.start_topology_discovery = false;
            state.is_discovering_topology = true;
            state.topology_discovery_status = "Starting discovery...".to_string();
            state.scan_progress = 0.0;

            let subnet = state.subnet_input.clone();
            let cancel_flag = Arc::new(AtomicBool::new(false));
            let state_clone = self.state.clone();
            let ctx_clone = ctx.clone();

            let on_progress = std::sync::Arc::new(move |progress: f32| {
                if let Ok(mut guard) = state_clone.lock() {
                    guard.scan_progress = progress;
                    guard.topology_discovery_status =
                        format!("Discovering... {:.0}%", progress * 100.0);
                }
                ctx_clone.request_repaint();
            });

            let state_clone = self.state.clone();
            let ctx_clone = ctx.clone();

            self.runtime.spawn(async move {
                let cancel = cancel_flag;

                let graph = crate::topology::graph::discover_and_build_fast(
                    &subnet,
                    128,
                    150,
                    cancel,
                    Some(on_progress),
                )
                .await;

                let mut state = state_clone.lock().unwrap();
                let mut widget = TopologyWidget::new(graph);
                widget.compute_layout(LayoutType::ForceDirected);
                state.topology_widget = Some(widget);
                state.is_discovering_topology = false;
                state.topology_discovery_status = "Discovery complete".to_string();
                state.scan_progress = 1.0;

                ctx_clone.request_repaint();
            });
        }

        // Clone values needed for async operations before dropping the lock
        let ssh_dialog_show = state.ssh_dialog.show;
        let export_feedback_show = state.export_feedback.show;

        // Top panel - header bar
        egui::TopBottomPanel::top("top_panel")
            .frame(egui::Frame::none().fill(Color32::from_rgb(20, 22, 25)).inner_margin(12.0))
            .show(ctx, |ui| {
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                ui.add(egui::Image::new(egui::include_image!("../assets/logo.png"))
                    .max_width(32.0)
                    .rounding(8.0));
                ui.add_space(8.0);
                ui.heading(RichText::new("SCANNR").size(28.0).strong().color(Color32::from_rgb(100, 200, 255)));
                ui.add_space(10.0);
                ui.label(RichText::new("NETWORK DISCOVERY").size(12.0).weak().color(Color32::from_rgb(150, 160, 170)));

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // Theme selector (right-to-left so Light appears last/rightmost)
                    let light_color = if state.theme_mode == ThemeMode::Light { Color32::from_rgb(0, 120, 215) } else { Color32::from_gray(60) };
                    if draw_polished_button(ui, RichText::new("Light").size(14.0), light_color).clicked() {
                        state.theme_mode = ThemeMode::Light;
                    }
                    
                    let dark_color = if state.theme_mode == ThemeMode::Dark { Color32::from_rgb(0, 120, 215) } else { Color32::from_gray(60) };
                    if draw_polished_button(ui, RichText::new("Dark").size(14.0), dark_color).clicked() {
                        state.theme_mode = ThemeMode::Dark;
                    }
                    
                    let auto_color = if state.theme_mode == ThemeMode::Auto { Color32::from_rgb(0, 120, 215) } else { Color32::from_gray(60) };
                    if draw_polished_button(ui, RichText::new("Auto").size(14.0), auto_color).clicked() {
                        state.theme_mode = ThemeMode::Auto;
                    }

                    ui.add_space(15.0);
                    ui.separator();
                    ui.add_space(15.0);

                    // View mode selector with icons (Topology right of List)
                    let topo_color = if state.view_mode == ViewMode::Topology { Color32::from_rgb(0, 120, 215) } else { Color32::from_gray(60) };
                    if draw_polished_button(ui, RichText::new("🕸 Topology").size(14.0), topo_color).clicked() {
                        state.view_mode = ViewMode::Topology;
                    }
                    
                    let list_color = if state.view_mode == ViewMode::List { Color32::from_rgb(0, 120, 215) } else { Color32::from_gray(60) };
                    if draw_polished_button(ui, RichText::new("📄 List").size(14.0), list_color).clicked() {
                        state.view_mode = ViewMode::List;
                    }
                });
            });
        });

        // Central panel - main content
        egui::CentralPanel::default().show(ctx, |ui| {
            // Apply a global style tweak for this panel
            let mut style = (*ctx.style()).clone();
            style.spacing.item_spacing = egui::vec2(10.0, 10.0);
            ui.set_style(style);

            ui.add_space(10.0);

            // Network selection row
            egui::Frame::group(ui.style())
                .fill(Color32::from_rgb(25, 27, 30))
                .stroke(egui::Stroke::new(1.0, Color32::from_gray(40)))
                .inner_margin(12.0)
                .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Target:").strong().color(Color32::LIGHT_GRAY).size(14.0));

                    // Network interface dropdown
                    let current_selection = if state.available_interfaces.is_empty() {
                        "Detecting...".to_string()
                    } else if state.selected_interface_idx < state.available_interfaces.len() {
                        state.available_interfaces[state.selected_interface_idx]
                            .name
                            .clone()
                    } else {
                        "Select Network".to_string()
                    };

                    // Clone interfaces for iteration to avoid borrow issues
                    let interfaces: Vec<_> = state
                        .available_interfaces
                        .iter()
                        .enumerate()
                        .map(|(idx, iface)| (idx, iface.name.clone(), iface.subnet.clone()))
                        .collect();

                    let mut new_selection: Option<(usize, String)> = None;

                    let combo = egui::ComboBox::from_id_source("network_interface")
                        .width(220.0)
                        .selected_text(RichText::new(&current_selection).size(14.0));
                    
                    combo.show_ui(ui, |ui| {
                            for (idx, name, subnet) in &interfaces {
                                let selected = state.selected_interface_idx == *idx;
                                if ui.selectable_label(selected, RichText::new(name).size(14.0)).clicked() {
                                    new_selection = Some((*idx, subnet.clone()));
                                }
                            }
                        });

                    // Apply selection change after iteration
                    if let Some((idx, subnet)) = new_selection {
                        state.selected_interface_idx = idx;
                        if let Some(pos) = subnet.find(' ') {
                            state.subnet_input = format!("{}/24", &subnet[..pos]);
                        } else {
                            state.subnet_input = format!("{}/24", &subnet);
                        }
                    }

                    ui.add_space(5.0);
                    ui.label(RichText::new("CIDR:").size(14.0));
                    ui.add(egui::TextEdit::singleline(&mut state.subnet_input).desired_width(140.0).font(egui::FontId::proportional(14.0)));

                    ui.add_space(15.0);
                    ui.label(RichText::new("Profile:").size(14.0));
                    egui::ComboBox::from_id_source("scan_mode")
                        .width(160.0)
                        .selected_text(RichText::new(state.scan_mode.label()).size(14.0))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut state.scan_mode,
                                ScanMode::Quick,
                                RichText::new(ScanMode::Quick.label()).size(14.0),
                            );
                            ui.selectable_value(
                                &mut state.scan_mode,
                                ScanMode::Standard,
                                RichText::new(ScanMode::Standard.label()).size(14.0),
                            );
                            ui.selectable_value(
                                &mut state.scan_mode,
                                ScanMode::Full,
                                RichText::new(ScanMode::Full.label()).size(14.0),
                            );
                        });

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if draw_polished_button(ui, RichText::new(if state.advanced_settings_open {
                                "⚙ Advanced"
                            } else {
                                "⚙ Advanced..."
                            }).size(14.0), Color32::from_gray(60)).clicked()
                        {
                            state.advanced_settings_open = !state.advanced_settings_open;
                        }
                    });
                });
            });

            ui.add_space(5.0);

            // Large scan button row
            ui.horizontal(|ui| {
                let button_text = if state.is_scanning {
                    "ABORT SCAN"
                } else {
                    "START SCAN"
                };
                let button_color = if state.is_scanning {
                    Color32::from_rgb(200, 60, 60)
                } else {
                    Color32::from_rgb(0, 120, 215)
                };

                let enabled = !state.subnet_input.is_empty() || state.is_scanning;
                
                let response = ui.add_enabled_ui(enabled, |ui| {
                    draw_polished_button(ui, RichText::new(button_text).size(18.0).strong(), button_color)
                }).inner;

                if response.clicked() {
                    if state.is_scanning {
                        state.is_scanning = false;
                        state.scan_status = "Scan cancelled".to_string();
                    } else {
                        state.is_scanning = true;
                        state.scan_status = "Initializing scan...".to_string();
                        state.scan_progress = 0.0;
                        state.results.clear();

                        let subnet = state.subnet_input.clone();
                        let scan_mode = state.scan_mode;
                        let state_clone = self.state.clone();
                        let ctx_clone = ctx.clone();

                        self.runtime.spawn(async move {
                            run_scan(&subnet, state_clone, ctx_clone, scan_mode).await;
                        });
                    }
                }

                // Passive Scan Button (Visible only for public subnets)
                let is_public = if let Ok(net) = state.subnet_input.parse::<IpNetwork>() {
                    match net {
                        IpNetwork::V4(v4) => !v4.ip().is_private() && !v4.ip().is_loopback() && !v4.ip().is_link_local(),
                        IpNetwork::V6(v6) => !v6.ip().is_loopback() && !v6.ip().is_unicast_link_local(),
                    }
                } else {
                    false
                };

                if is_public && !state.is_scanning {
                    ui.add_space(10.0);
                    if draw_polished_button(ui, RichText::new("Passive Scan (Shodan)").size(18.0).strong(), Color32::from_rgb(100, 60, 160)).clicked() {
                        state.is_scanning = true;
                        state.scan_status = "Initializing passive scan...".to_string();
                        state.scan_progress = 0.0;
                        state.results.clear();

                        let subnet = state.subnet_input.clone();
                        let state_clone = self.state.clone();
                        let ctx_clone = ctx.clone();

                        self.runtime.spawn(async move {
                            run_passive_scan(&subnet, state_clone, ctx_clone).await;
                        });
                    }
                }

                ui.add_space(20.0);

                // Show scan info
                if !state.is_scanning && !state.results.is_empty() {
                    ui.label(RichText::new(format!(
                        "✔ Last scan: {} hosts, {} open ports",
                        state.results.len(),
                        state.scanned_ports
                    )).color(Color32::GREEN).size(14.0));
                }
            });

            // Advanced settings panel
            if state.advanced_settings_open {
                ui.add_space(5.0);
                egui::Frame::none()
                    .fill(Color32::from_rgb(30, 32, 35))
                    .inner_margin(8.0)
                    .rounding(4.0)
                    .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Batch Size:").size(14.0));
                        ui.add(
                            egui::Slider::new(&mut state.batch_size, 100..=10000).logarithmic(true),
                        );

                        ui.add_space(20.0);

                        ui.label(RichText::new("Timeout (ms):").size(14.0));
                        ui.add(
                            egui::Slider::new(&mut state.timeout_ms, 10..=5000).logarithmic(true),
                        );

                        ui.add_space(20.0);

                        ui.checkbox(&mut state.udp_scan, RichText::new("UDP Scan").size(14.0));
                        
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(RichText::new("v1.1.0").weak().size(12.0));
                            ui.add(egui::Image::new(egui::include_image!("../assets/logo.png")).max_width(20.0).rounding(4.0));
                        });
                    });
                });
            }

            ui.add_space(10.0);
            
            // Scan progress/status bar
            let progress_color = if state.is_scanning { Color32::from_rgb(0, 120, 215) } else { Color32::from_rgb(60, 60, 60) };
            let progress_bg = Color32::from_rgb(30, 30, 30);
            
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    if state.is_scanning {
                        ui.spinner();
                    }
                    ui.label(RichText::new(&state.scan_status).strong().color(Color32::LIGHT_GRAY).size(14.0));
                });
                
                let rect = ui.available_rect_before_wrap();
                let bar_rect = egui::Rect::from_min_size(rect.min, egui::vec2(rect.width(), 6.0));
                ui.painter().rect_filled(bar_rect, 3.0, progress_bg);
                if state.scan_progress > 0.0 {
                    let fill_width = rect.width() * state.scan_progress.clamp(0.0, 1.0);
                    let fill_rect = egui::Rect::from_min_size(rect.min, egui::vec2(fill_width, 6.0));
                    ui.painter().rect_filled(fill_rect, 3.0, progress_color);
                }
                ui.allocate_rect(bar_rect, egui::Sense::hover());
            });
            
            ui.add_space(15.0);

            // Results view
            if !state.results.is_empty() || state.view_mode == ViewMode::Topology {
                match state.view_mode {
                    ViewMode::List => draw_list_view(ui, &mut state),
                    ViewMode::Topology => {
                        draw_topology_view(ui, &mut state, &self.state, &self.runtime)
                    }
                }
            } else {
                // Empty state placeholder
                ui.centered_and_justified(|ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(40.0);
                        ui.add(egui::Image::new(egui::include_image!("../assets/logo.png"))
                            .max_width(128.0)
                            .rounding(16.0));
                        ui.add_space(20.0);
                        ui.label(RichText::new("Ready to explore.").size(22.0).strong());
                        ui.label(RichText::new("Enter a network range and click Start Scan.").size(18.0).weak());
                    });
                });
            }
        });

        // Export feedback modal (outside panels)
        if export_feedback_show {
            let feedback = state.export_feedback.clone();
            
            // Auto-dismiss logic
            if let Some(timestamp) = feedback.timestamp {
                if timestamp.elapsed().as_secs() >= 3 {
                    state.export_feedback.show = false;
                }
                ctx.request_repaint(); // Keep updating until dismissed
            }

            egui::Area::new(egui::Id::new("export_toast"))
                .anchor(egui::Align2::CENTER_BOTTOM, [0.0, -40.0])
                .show(ctx, |ui| {
                    let is_error = feedback.is_error;
                    let bg_color = if is_error {
                        Color32::from_rgb(180, 50, 50)
                    } else {
                        Color32::from_rgb(50, 150, 80)
                    };

                    egui::Frame::none()
                        .fill(bg_color)
                        .rounding(8.0)
                        .inner_margin(12.0)
                        .stroke(egui::Stroke::new(1.0, Color32::from_white_alpha(100)))
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(RichText::new(if is_error { "❌" } else { "✔" }).size(18.0));
                                ui.add_space(8.0);
                                ui.vertical(|ui| {
                                    ui.label(RichText::new(if is_error { "Export Failed" } else { "Success" }).strong().color(Color32::WHITE).size(16.0));
                                    ui.label(RichText::new(&feedback.message).color(Color32::from_gray(220)).size(14.0));
                                });
                            });
                        });
                });
        }

        // SSH dialog modal (outside panels)
        if ssh_dialog_show {
            egui::Window::new("SSH Connection")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    if let Some(ip) = state.ssh_dialog.ip {
                        ui.label(RichText::new(format!("Connecting to: {}:{}", ip, state.ssh_dialog.port)).size(14.0));
                    }
                    ui.add_space(10.0);

                    egui::Grid::new("ssh_form")
                        .num_columns(2)
                        .spacing([10.0, 8.0])
                        .show(ui, |ui| {
                            ui.label(RichText::new("Username:").size(14.0));
                            let user_resp = ui.add(
                                egui::TextEdit::singleline(&mut state.ssh_dialog.username)
                                    .desired_width(200.0)
                                    .font(egui::FontId::proportional(14.0)),
                            );
                            if ui.memory(|m| m.focused().is_none()) {
                                user_resp.request_focus();
                            }
                            ui.end_row();

                            ui.label(RichText::new("Password:").size(14.0));
                            let pwd_resp = ui.add(
                                egui::TextEdit::singleline(&mut state.ssh_dialog.password)
                                    .password(true)
                                    .desired_width(200.0)
                                    .font(egui::FontId::proportional(14.0)),
                            );
                            
                            // Handle Enter key to submit
                            if pwd_resp.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                                if !state.ssh_dialog.username.is_empty() && !state.ssh_dialog.password.is_empty() {
                                    let ip = state.ssh_dialog.ip;
                                    let port = state.ssh_dialog.port;
                                    let username = state.ssh_dialog.username.clone();
                                    let password = state.ssh_dialog.password.clone();
                                    spawn_ssh_terminal(ip.unwrap(), port, &username, &password);
                                    state.ssh_dialog = SshDialogState::default();
                                }
                            }
                            ui.end_row();
                        });

                    ui.add_space(10.0);

                    if state.ssh_dialog.connecting {
                        ui.horizontal(|ui| {
                            ui.spinner();
                            ui.label(RichText::new("Connecting...").size(14.0));
                        });
                    } else {
                        ui.horizontal(|ui| {
                            if draw_polished_button(ui, RichText::new("Connect").size(14.0).strong(), Color32::from_rgb(0, 120, 215)).clicked()
                                && !state.ssh_dialog.username.is_empty()
                                && !state.ssh_dialog.password.is_empty()
                            {
                                let ip = state.ssh_dialog.ip;
                                let port = state.ssh_dialog.port;
                                let username = state.ssh_dialog.username.clone();
                                let password = state.ssh_dialog.password.clone();
                                
                                // Hand off to system terminal
                                spawn_ssh_terminal(ip.unwrap(), port, &username, &password);
                                
                                // Close dialog immediately
                                state.ssh_dialog = SshDialogState::default();
                            }

                            if draw_polished_button(ui, RichText::new("Cancel").size(14.0), Color32::from_gray(60)).clicked() {
                                state.ssh_dialog = SshDialogState::default();
                            }
                        });
                    }
                });
        }
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
                rgb[0].saturating_add(20), // Reduced from 30
                rgb[1].saturating_add(20),
                rgb[2].saturating_add(20),
                rgb[3]
            );
        } else if response.clicked() {
            let rgb = base_color.to_array();
            base_color = Color32::from_rgba_unmultiplied(
                rgb[0].saturating_sub(20), // Reduced from 30
                rgb[1].saturating_sub(20),
                rgb[2].saturating_sub(20),
                rgb[3]
            );
        }
        
        // Background
        ui.painter().rect_filled(rect, rounding, base_color);
        
        // Liquid Glass Highlight (Top 35% even more subtle - 30% reduction)
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
        
        // Text - Use Black for Light Mode readability if button is light, but here we assume button is colored.
        // The user request was "text color on light mode to be black". 
        // We need to check if the background is light or if we are in light mode.
        // Since we don't have easy access to theme state here without passing it, 
        // let's stick to White text for colored buttons as it's standard, 
        // BUT if the fill_color is very light (like gray(220)), we should use black.
        // However, the user specifically mentioned "gray text in light mode" is hard to read.
        // This likely refers to labels, not buttons. But let's ensure button text is legible.
        // For now, White text on colored buttons is usually safe.
        // If the user meant global text color, that's handled in the style setup.
        
        ui.painter().galley(rect.center() - galley.size() / 2.0, galley, Color32::WHITE);
    }
    
    response
}

async fn run_passive_scan(
    subnet: &str,
    state: Arc<Mutex<AppState>>,
    ctx: egui::Context,
) {
    let parsed_subnet: Result<IpNetwork, _> = subnet.parse();

    if let Ok(network) = parsed_subnet {
        let mut ips: Vec<IpAddr> = Vec::new();
        for host in network.iter() {
            if ips.len() >= 256 { break; }
            ips.push(host);
        }

        let total_ips = ips.len();
        {
            let mut guard = state.lock().unwrap();
            guard.total_ips = total_ips;
            guard.scan_status = format!("Querying Shodan for {} hosts...", total_ips);
        }
        ctx.request_repaint();

        let mut completed = 0;
        for ip in ips {
            // Check cancellation (reusing is_scanning flag logic)
            if !state.lock().unwrap().is_scanning { break; }

            let ports = crate::scanner::external::get_open_ports_from_shodan(&ip).await;
            
            if !ports.is_empty() {
                let mut guard = state.lock().unwrap();
                let host = HostInfo {
                    ip,
                    hostname: None,
                    mac: None,
                    vendor: Some("Shodan InternetDB".to_string()),
                    ports: ports.clone(),
                    os: None,
                    service_names: HashMap::new(),
                    user_alias: None,
                };
                guard.results.push(host);
                guard.scanned_ports += ports.len();
            }

            completed += 1;
            {
                let mut guard = state.lock().unwrap();
                guard.scan_progress = completed as f32 / total_ips as f32;
                guard.scan_status = format!("Passive scan: {}/{} IPs checked", completed, total_ips);
            }
            ctx.request_repaint();
            
            // Rate limit to be polite (5 req/s)
            tokio::time::sleep(Duration::from_millis(200)).await; 
        }

        let mut guard = state.lock().unwrap();
        guard.is_scanning = false;
        guard.scan_status = format!("Passive scan complete. Found {} hosts.", guard.results.len());
        guard.scan_progress = 1.0;
    } else {
        let mut guard = state.lock().unwrap();
        guard.is_scanning = false;
        guard.scan_status = "Invalid subnet.".to_string();
    }
    ctx.request_repaint();
}

async fn run_scan(
    subnet: &str,
    state: Arc<Mutex<AppState>>,
    ctx: egui::Context,
    scan_mode: ScanMode,
) {
    let parsed_subnet: Result<IpNetwork, _> = subnet.parse();

    if let Ok(network) = parsed_subnet {
        let mut ips: Vec<IpAddr> = Vec::new();

        // Use ipnetwork's iter() method to iterate through usable host addresses
        for host in network.iter() {
            if ips.len() >= 256 {
                break;
            }
            ips.push(host);
        }

        if ips.is_empty() {
            let mut state_guard = state.lock().unwrap();
            state_guard.is_scanning = false;
            state_guard.scan_status = "No valid hosts in network range.".to_string();
            state_guard.scan_progress = 0.0;
            return;
        }

        if ips.len() > 256 {
            let mut state_guard = state.lock().unwrap();
            state_guard.is_scanning = false;
            state_guard.scan_status =
                format!("Network too large: {} hosts. Maximum is 256.", ips.len());
            state_guard.scan_progress = 0.0;
            return;
        }

        let scan_ports_opt = scan_mode.ports();
        let ports: Vec<u16> = match scan_ports_opt {
            Some(ports) => ports,
            None => (1..=65535).collect(),
        };

        let total_ips = ips.len();
        let total_ports = ports.len();

        if total_ips == 0 || total_ports == 0 {
            let mut state_guard = state.lock().unwrap();
            state_guard.is_scanning = false;
            state_guard.scan_status = "Invalid scan range: no IPs or ports to scan.".to_string();
            state_guard.scan_progress = 0.0;
            return;
        }

        log::info!("Starting scan: {} IPs x {} ports", total_ips, total_ports);

        {
            let mut state_guard = state.lock().unwrap();
            state_guard.total_ips = total_ips;
        }

        let (batch_size, timeout, udp_scan) = {
            let state_guard = state.lock().unwrap();
            (
                state_guard.batch_size,
                state_guard.timeout_ms,
                state_guard.udp_scan,
            )
        };

        let timeout = Duration::from_millis(timeout.into());

        // Start MDNS discovery in background to enrich hostnames
        let state_mdns_clone = state.clone();
        let ctx_mdns_clone = ctx.clone();
        let mdns_timeout = timeout.max(Duration::from_secs(5)); // Give MDNS some time to breathe
        tokio::spawn(async move {
            let mdns_results = tokio::task::spawn_blocking(move || {
                crate::topology::mdns::resolve_mdns_hostnames(mdns_timeout)
            }).await.unwrap_or_default();
            
            if !mdns_results.is_empty() {
                let mut guard = state_mdns_clone.lock().unwrap();
                for (ip, hostname) in mdns_results {
                    if let Some(host) = guard.results.iter_mut().find(|h| h.ip == ip) {
                        if host.hostname.is_none() {
                            host.hostname = Some(hostname);
                        }
                    }
                }
                drop(guard);
                ctx_mdns_clone.request_repaint();
            }
        });

        // Phase 0: ARP & Vendor Discovery (Immediate results for already cached neighbors)
        {
            let mut guard = state.lock().unwrap();
            guard.scan_status = "Reading ARP cache...".to_string();
            guard.scan_progress = 0.01;
        }
        ctx.request_repaint();

        let ip_set: HashSet<IpAddr> = ips.iter().copied().collect();
        let arp_entries = crate::topology::discovery::get_arp_entries().await;
        {
            let mut guard = state.lock().unwrap();
            for arp in arp_entries {
                let ip = IpAddr::V4(arp.ip);
                if ip_set.contains(&ip) {
                    let vendor = crate::oui::lookup_vendor(&arp.mac);
                    let host = HostInfo {
                        ip,
                        hostname: None,
                        mac: Some(arp.mac),
                        vendor,
                        ports: Vec::new(),
                        os: Some("Active".to_string()),
                        service_names: HashMap::new(),
                        user_alias: None,
                    };
                    guard.results.push(host);
                }
            }
        }
        ctx.request_repaint();

        // Create callbacks for all scan phases
        let state_clone = state.clone();
        let ctx_clone = ctx.clone();

        // Progress should be split between discovery, phase 1 and phase 2
        let on_progress = std::sync::Arc::new(move |progress: f32| {
            if let Ok(mut guard) = state_clone.lock() {
                let status = guard.scan_status.clone();
                if status.contains("Discovering active hosts") {
                    guard.scan_progress = progress * 0.1;
                } else if status.contains("Quick probing") {
                    guard.scan_progress = 0.1 + (progress * 0.2);
                } else if status.contains("Scanning remaining") {
                    guard.scan_progress = 0.3 + (progress * 0.7);
                }
            }
            ctx_clone.request_repaint();
        });

        let state_result_clone = state.clone();
        let ctx_result_clone = ctx.clone();
        let on_result = std::sync::Arc::new(move |socket: std::net::SocketAddr| {
            if let Ok(mut guard) = state_result_clone.lock() {
                let ip = socket.ip();
                let port = socket.port();

                // Find existing host or create new one
                let mut is_new_port = false;
                if let Some(host) = guard.results.iter_mut().find(|h| h.ip == ip) {
                    if !host.ports.contains(&port) {
                        host.ports.push(port);
                        host.ports.sort_unstable();
                        host.os = Some(detect_device_type(&host.ports));
                        is_new_port = true;
                    }
                } else {
                    let mut host = HostInfo {
                        ip,
                        hostname: None,
                        mac: None,
                        vendor: None,
                        ports: vec![port],
                        os: None,
                        service_names: HashMap::new(),
                        user_alias: None,
                    };
                    host.os = Some(detect_device_type(&host.ports));
                    guard.results.push(host);
                    is_new_port = true;
                }

                if is_new_port {
                    guard.scanned_ports += 1;
                }
            }
            ctx_result_clone.request_repaint();
        });

        // Phase 0.5: Active Host Discovery (TCP Ping Sweep)
        // We probe a common port on EVERY IP to ensure the OS populates the ARP table
        // and we find hosts not in the cache.
        {
            let mut guard = state.lock().unwrap();
            guard.scan_status = format!("Discovering active hosts in {}...", subnet);
        }
        ctx.request_repaint();

        // Use a tiny timeout for discovery
        let discovery_timeout = Duration::from_millis(50);
        let discovery_strategy =
            PortStrategy::pick(&None, Some(vec![80, 443, 445, 22]), ScanOrder::Serial);
        let discovery_scanner = Scanner::new(
            &ips,
            batch_size.max(5000), // High concurrency for discovery
            discovery_timeout,
            1,
            true,
            discovery_strategy,
            false,
            vec![],
            false,
        );

        // Result callback for discovery: just to trigger ARP lookups later or find hosts
        let state_discovery_clone = state.clone();
        let on_discovery_result = std::sync::Arc::new(move |socket: std::net::SocketAddr| {
            if let Ok(mut guard) = state_discovery_clone.lock() {
                let ip = socket.ip();
                if !guard.results.iter().any(|h| h.ip == ip) {
                    let host = HostInfo {
                        ip,
                        hostname: None,
                        mac: None,
                        vendor: None,
                        ports: vec![socket.port()],
                        os: Some("Active".to_string()),
                        service_names: HashMap::new(),
                        user_alias: None,
                    };
                    guard.results.push(host);
                }
            }
        });

        discovery_scanner
            .run(Some(on_progress.clone()), Some(on_discovery_result))
            .await;

        // Refresh ARP after discovery probe
        let arp_entries = crate::topology::discovery::get_arp_entries().await;
        {
            let mut guard = state.lock().unwrap();
            for arp in arp_entries {
                let ip = IpAddr::V4(arp.ip);
                if let Some(host) = guard.results.iter_mut().find(|h| h.ip == ip) {
                    if host.mac.is_none() {
                        host.mac = Some(arp.mac.clone());
                        host.vendor = crate::oui::lookup_vendor(&arp.mac);
                    }
                }
            }
        }
        ctx.request_repaint();
        // Phase 1: Quick Scan (Common ports)
        let quick_ports = QUICK_PORTS.to_vec();
        let quick_strategy =
            PortStrategy::pick(&None, Some(quick_ports.clone()), ScanOrder::Serial);
        let quick_scanner = Scanner::new(
            &ips,
            batch_size,
            timeout,
            1,
            true,
            quick_strategy,
            false,
            vec![],
            udp_scan,
        );

        {
            let mut guard = state.lock().unwrap();
            guard.scan_status = format!("Quick probing {} hosts for common services...", total_ips);
        }
        ctx.request_repaint();

        quick_scanner
            .run(Some(on_progress.clone()), Some(on_result.clone()))
            .await;

        // Phase 2: Main Scan (Remaining ports)
        let remaining_ports: Vec<u16> = ports
            .iter()
            .filter(|p| !quick_ports.contains(p))
            .copied()
            .collect();

        if !remaining_ports.is_empty() {
            {
                let mut guard = state.lock().unwrap();
                guard.scan_status = format!("Scanning remaining ports on {} hosts...", total_ips);
            }
            ctx.request_repaint();

            let main_strategy = PortStrategy::pick(&None, Some(remaining_ports), ScanOrder::Serial);
            let main_scanner = Scanner::new(
                &ips,
                batch_size,
                timeout,
                1,
                true,
                main_strategy,
                false,
                vec![],
                udp_scan,
            );
            main_scanner.run(Some(on_progress), Some(on_result)).await;
        }

        // Final update
        let mut state_guard = state.lock().unwrap();
        state_guard.is_scanning = false;
        state_guard.scan_status = format!(
            "Scan complete! Found {} hosts with {} total open ports",
            state_guard.results.len(),
            state_guard.scanned_ports
        );
        state_guard.scan_progress = 1.0;
        drop(state_guard);

        ctx.request_repaint();
    } else {
        let mut state = state.lock().unwrap();
        state.is_scanning = false;
        state.scan_status =
            "Invalid subnet format. Use CIDR notation like 192.168.1.0/24".to_string();
        drop(state);

        ctx.request_repaint();
    }
}

fn spawn_ssh_terminal(
    ip: IpAddr,
    port: u16,
    username: &str,
    password: &str,
) {
    #[cfg(target_os = "windows")]
    {
        // Try plink first (for password automation), fallback to standard ssh (manual password)
        // launching a new cmd window that stays open (/k)
        let _ = std::process::Command::new("cmd")
            .args([
                "/c", 
                "start", 
                "cmd", 
                "/k",
                &format!(
                    "echo Connecting to {}... && (plink -ssh -l {} -pw {} -P {} {} || ssh -p {} {}@{})", 
                    ip, username, password, port, ip, port, username, ip
                )
            ])
            .creation_flags(CREATE_NO_WINDOW) 
            .spawn();
    }

    #[cfg(target_os = "macos")]
    {
        // Use AppleScript to launch Terminal.app and run ssh
        // Try sshpass if available, else just ssh
        let script = format!(
            "tell application \"Terminal\" to do script \"sshpass -p '{}' ssh -p {} {}@{} || ssh -p {} {}@{}\"",
            password, port, username, ip, port, username, ip
        );
        let _ = std::process::Command::new("osascript")
            .args(["-e", &script])
            .spawn();
    }

    #[cfg(target_os = "linux")]
    {
        // Try common terminal emulators
        let terminals = ["x-terminal-emulator", "gnome-terminal", "konsole", "xterm"];
        let cmd = format!(
            "sshpass -p '{}' ssh -p {} {}@{} || ssh -p {} {}@{}",
            password, port, username, ip, port, username, ip
        );

        for term in terminals {
            if std::process::Command::new(term)
                .args(["-e", &format!("bash -c '{} ; exec bash'", cmd)])
                .spawn()
                .is_ok() 
            {
                break;
            }
        }
    }
}

fn draw_list_view(ui: &mut egui::Ui, state: &mut AppState) {
    ui.vertical(|ui| {
        ui.heading(RichText::new("Scan Results").size(18.0).strong());
        ui.add_space(10.0);

        if state.results.is_empty() {
            ui.centered_and_justified(|ui| {
                ui.label(RichText::new("No scan results yet. Click 'Scan Network' to start.").size(14.0));
            });
            return;
        }

        // Controls row
        ui.horizontal(|ui| {
            ui.label(RichText::new("Sort by:").size(14.0));
            egui::ComboBox::from_id_source("sort_combo")
                .selected_text(RichText::new(state.sort_by.label()).size(14.0))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut state.sort_by, SortOption::IpAddress, RichText::new("IP Address").size(14.0));
                    ui.selectable_value(&mut state.sort_by, SortOption::PortCount, RichText::new("Port Count").size(14.0));
                    ui.selectable_value(&mut state.sort_by, SortOption::Vendor, RichText::new("Vendor").size(14.0));
                    ui.selectable_value(&mut state.sort_by, SortOption::Hostname, RichText::new("Hostname").size(14.0));
                });

            if draw_polished_button(ui, RichText::new(if state.sort_ascending { "🔼" } else { "🔽" }).size(14.0), Color32::from_gray(60)).clicked() {
                state.sort_ascending = !state.sort_ascending;
            }

            ui.add_space(20.0);

            if draw_polished_button(ui, RichText::new("📋 Copy CSV").size(14.0), Color32::from_rgb(60, 140, 100)).clicked() {
                copy_results_csv_to_clipboard(state);
            }

            if draw_polished_button(ui, RichText::new("📋 Copy JSON").size(14.0), Color32::from_rgb(60, 100, 140)).clicked() {
                copy_results_json_to_clipboard(state);
            }

            if draw_polished_button(ui, RichText::new("📋 Copy to Clipboard").size(14.0), Color32::from_rgb(100, 100, 100)).clicked() {
                copy_results_to_clipboard(state);
            }
        });

        ui.add_space(10.0);

        ui.horizontal(|ui| {
            ui.label(RichText::new("Filter:").size(14.0));
            ui.add(egui::TextEdit::singleline(&mut state.filter_text).desired_width(200.0).font(egui::FontId::proportional(14.0)));
            if !state.filter_text.is_empty() && draw_polished_button(ui, RichText::new("Clear").size(14.0), Color32::from_gray(60)).clicked() {
                state.filter_text.clear();
            }
        });

        ui.add_space(10.0);

        // Filter and sort results
        let filter_lower = state.filter_text.to_lowercase();
        let mut filtered_results: Vec<_> = state
            .results
            .iter()
            .enumerate() // Keep original index
            .filter(|(_, host)| {
                if state.filter_text.is_empty() {
                    return true;
                }
                let ip_match = host.ip.to_string().contains(&filter_lower);
                let hostname_match = host
                    .hostname
                    .as_ref()
                    .map(|h| h.to_lowercase().contains(&filter_lower))
                    .unwrap_or(false);
                let os_match = host
                    .os
                    .as_ref()
                    .map(|o| o.to_lowercase().contains(&filter_lower))
                    .unwrap_or(false);
                let port_match = state
                    .filter_by_port
                    .is_some_and(|p| host.ports.contains(&p));

                ip_match || hostname_match || os_match || port_match
            })
            .collect();

        match state.sort_by {
            SortOption::IpAddress => filtered_results.sort_by_key(|(_, h)| h.ip),
            SortOption::PortCount => {
                filtered_results.sort_by_key(|(_, h)| std::cmp::Reverse(h.port_count()))
            }
            SortOption::Vendor => {
                filtered_results.sort_by_key(|(_, h)| h.vendor.clone().unwrap_or_default())
            }
            SortOption::Hostname => {
                filtered_results.sort_by_key(|(_, h)| h.hostname.clone().unwrap_or_default())
            }
        }

        if !state.sort_ascending {
            filtered_results.reverse();
        }

        ui.label(RichText::new(format!(
            "Showing {} of {} hosts",
            filtered_results.len(),
            state.results.len()
        )).size(14.0));
        ui.add_space(5.0);
        
        // Actions to apply after the loop
        enum ListAction {
            StartEdit(IpAddr, String),
            UpdateEdit(String),
            CommitEdit,
            CancelEdit,
            SelectResult(usize),
        }
        let mut action = None;
        let editing_state = state.editing_alias.clone();

        // Results list
        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                for (original_idx, host) in filtered_results.iter() {
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            let (display_name, secondary_info) = if let Some(alias) = &host.user_alias {
                                (alias.clone(), format!("{} ({})", host.hostname.as_deref().unwrap_or(""), host.ip))
                            } else {
                                match &host.hostname {
                                    Some(hostname) => (hostname.clone(), host.ip.to_string()),
                                    None => (host.ip.to_string(), String::new()),
                                }
                            };

                            // Device Icon
                            let device_type = host.os.as_deref().unwrap_or("Unknown Device");
                            let icon_source = get_device_icon(device_type);
                            ui.add(egui::Image::new(icon_source).max_width(24.0).rounding(4.0));
                            ui.add_space(4.0);

                            let is_selected = state.selected_result == Some(*original_idx);
                            let text_color = if is_selected {
                                Color32::from_rgb(100, 200, 255)
                            } else {
                                ui.visuals().text_color()
                            };

                            ui.vertical(|ui| {
                                // Check if we are editing this host
                                let is_editing = editing_state.as_ref().map_or(false, |(ip, _)| *ip == host.ip);
                                
                                if is_editing {
                                    let mut text = editing_state.as_ref().unwrap().1.clone();
                                    let resp = ui.add(egui::TextEdit::singleline(&mut text).desired_width(150.0));
                                    
                                    if resp.changed() {
                                        action = Some(ListAction::UpdateEdit(text.clone()));
                                    }
                                    if resp.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                                        action = Some(ListAction::CommitEdit);
                                    }
                                    if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                                        action = Some(ListAction::CancelEdit);
                                    }
                                    
                                    // Auto-focus
                                    resp.request_focus();
                                    
                                    ui.horizontal(|ui| {
                                        if draw_polished_button(ui, RichText::new("💾").size(12.0), Color32::from_rgb(0, 120, 215)).clicked() {
                                            action = Some(ListAction::CommitEdit);
                                        }
                                        if draw_polished_button(ui, RichText::new("❌").size(12.0), Color32::from_gray(60)).clicked() {
                                            action = Some(ListAction::CancelEdit);
                                        }
                                    });
                                } else {
                                    if ui
                                        .selectable_label(
                                            is_selected,
                                            RichText::new(&display_name)
                                                .size(16.0)
                                                .strong()
                                                .color(text_color),
                                        )
                                        .clicked()
                                    {
                                        action = Some(ListAction::SelectResult(*original_idx));
                                    }
                                    if !secondary_info.is_empty() {
                                        ui.label(RichText::new(secondary_info).size(12.0).weak());
                                    }
                                }
                            });

                            ui.add_space(8.0);

                            // Actions Menu next to IP
                            let popup_id = ui.make_persistent_id(format!("actions_menu_{}", original_idx));
                            let actions_resp = draw_polished_button(ui, RichText::new("Actions").size(14.0), Color32::from_gray(60));
                            if actions_resp.clicked() {
                                ui.memory_mut(|m| m.toggle_popup(popup_id));
                            }

                            egui::popup_below_widget(ui, popup_id, &actions_resp, |ui| {
                                ui.set_min_width(200.0);
                                
                                if draw_polished_button(ui, RichText::new("✏ Edit Alias").size(14.0), Color32::from_gray(60)).clicked() {
                                    ui.close_menu();
                                    let current = host.user_alias.clone().unwrap_or(display_name);
                                    action = Some(ListAction::StartEdit(host.ip, current));
                                }
                                
                                if draw_polished_button(ui, RichText::new("🌐 Open in Browser").size(14.0), Color32::from_gray(60)).clicked() {
                                    ui.close_menu();
                                    open_in_browser(&format!("http://{}", host.ip));
                                }

                                if host.ports.contains(&443) && draw_polished_button(ui, RichText::new("🛡 Open HTTPS").size(14.0), Color32::from_gray(60)).clicked() {
                                    ui.close_menu();
                                    open_in_browser(&format!("https://{}", host.ip));
                                }

                                if host.ports.contains(&22) && draw_polished_button(ui, RichText::new("🔑 SSH Connect").size(14.0), Color32::from_gray(60)).clicked() {
                                    ui.close_menu();
                                    state.ssh_dialog = SshDialogState {
                                        show: true,
                                        ip: Some(host.ip),
                                        port: 22,
                                        ..Default::default()
                                    };
                                }

                                if draw_polished_button(ui, RichText::new("📋 Copy IP").size(14.0), Color32::from_gray(60)).clicked() {
                                    ui.close_menu();
                                    ui.output_mut(|o| o.copied_text = host.ip.to_string());
                                }

                                if draw_polished_button(ui, RichText::new("📋 Copy All Ports").size(14.0), Color32::from_gray(60)).clicked() {
                                    ui.close_menu();
                                    let ports_str = host
                                        .ports
                                        .iter()
                                        .map(|p| p.to_string())
                                        .collect::<Vec<_>>()
                                        .join(",");
                                    ui.output_mut(|o| o.copied_text = ports_str);
                                }
                            });

                            ui.add_space(4.0);

                            // Quick actions next to Actions
                            if host.ports.contains(&80) || host.ports.contains(&443) || host.ports.contains(&8080) {
                                let port = if host.ports.contains(&443) {
                                    443
                                } else if host.ports.contains(&8080) {
                                    8080
                                } else {
                                    80
                                };
                                let protocol = if port == 443 { "https" } else { "http" };
                                if draw_polished_button(ui, RichText::new("🌐 Web").size(14.0), Color32::from_rgb(50, 150, 80)).clicked() {
                                    open_in_browser(&format!("{}://{}:{}", protocol, host.ip, port));
                                }
                                ui.add_space(4.0);
                            }

                            if host.ports.contains(&22) {
                                if draw_polished_button(ui, RichText::new("🔑 SSH").size(14.0), Color32::from_rgb(0, 120, 215)).clicked() {
                                    state.ssh_dialog = SshDialogState {
                                        show: true,
                                        ip: Some(host.ip),
                                        port: 22,
                                        ..Default::default()
                                    };
                                }
                                ui.add_space(4.0);
                            }

                            // Details pushed to the right
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.vertical(|ui| {
                                    if let Some(ref os) = host.os {
                                        ui.label(RichText::new(os).size(14.0).color(Color32::from_rgb(100, 200, 100)).strong());
                                    }
                                    if let Some(ref vendor) = host.vendor {
                                        ui.label(RichText::new(vendor).size(12.0).weak());
                                    }
                                    if let Some(ref mac) = host.mac {
                                        ui.label(RichText::new(mac).size(10.0).monospace().weak());
                                    }
                                });
                            });
                        });

                        ui.add_space(5.0);

                        // Port list with wrapping
                        let ports_text = format!(
                            "Ports: {}",
                            host.ports
                                .iter()
                                .map(|p| p.to_string())
                                .collect::<Vec<_>>()
                                .join(", ")
                        );
                        ui.label(RichText::new(ports_text).size(18.0).color(Color32::from_rgb(100, 200, 100)));
                    });
                    ui.add_space(5.0);
                }
            });

        if let Some(action) = action {
            match action {
                ListAction::StartEdit(ip, name) => state.editing_alias = Some((ip, name)),
                ListAction::UpdateEdit(name) => {
                    if let Some((_, ref mut current_name)) = state.editing_alias {
                        *current_name = name;
                    }
                }
                ListAction::CommitEdit => {
                    if let Some((ip, name)) = state.editing_alias.take() {
                        if let Some(host) = state.results.iter_mut().find(|h| h.ip == ip) {
                            if name.trim().is_empty() {
                                host.user_alias = None;
                            } else {
                                host.user_alias = Some(name);
                            }
                        }
                    }
                }
                ListAction::CancelEdit => {
                    state.editing_alias = None;
                }
                ListAction::SelectResult(idx) => {
                    state.selected_result = Some(idx);
                }
            }
        }
    });
}

/// Cross-platform function to open a URL in the default browser
fn open_in_browser(url: &str) {
    #[cfg(target_os = "windows")]
    {
        let _ = std::process::Command::new("cmd")
            .args(["/C", "start", "", url])
            .creation_flags(CREATE_NO_WINDOW)
            .spawn();
    }
    #[cfg(target_os = "macos")]
    {
        let _ = std::process::Command::new("open").arg(url).spawn();
    }
    #[cfg(target_os = "linux")]
    {
        let _ = std::process::Command::new("xdg-open").arg(url).spawn();
    }
}

fn copy_results_csv_to_clipboard(state: &mut AppState) {
    let mut csv = String::new();
    csv.push_str("IP,Hostname,Vendor,MAC,OS,Ports\n");

    for host in &state.results {
        let hostname = host.hostname.as_deref().unwrap_or("");
        let vendor = host.vendor.as_deref().unwrap_or("");
        let mac = host.mac.as_deref().unwrap_or("");
        let os = host.os.as_deref().unwrap_or("");
        let ports_str = host
            .ports
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<_>>()
            .join(";");

        csv.push_str(&format!(
            "{},{},{},{},{},{}\n",
            host.ip, hostname, vendor, mac, os, ports_str
        ));
    }

    let success = copy_to_system_clipboard(&csv);
    state.export_feedback = ExportFeedback {
        show: true,
        message: if success {
            format!("Copied {} hosts as CSV to clipboard", state.results.len())
        } else {
            "Failed to copy CSV to clipboard".to_string()
        },
        is_error: !success,
        timestamp: Some(std::time::Instant::now()),
    };
}

fn copy_results_json_to_clipboard(state: &mut AppState) {
    let json = serde_json::to_string_pretty(&state.results).unwrap_or_default();
    let success = copy_to_system_clipboard(&json);
    state.export_feedback = ExportFeedback {
        show: true,
        message: if success {
            format!("Copied {} hosts as JSON to clipboard", state.results.len())
        } else {
            "Failed to copy JSON to clipboard".to_string()
        },
        is_error: !success,
        timestamp: Some(std::time::Instant::now()),
    };
}

fn copy_results_to_clipboard(state: &mut AppState) {
    let mut text = String::new();

    for host in &state.results {
        text.push_str(&format!("Host: {}\n", host.ip));
        if let Some(hostname) = &host.hostname {
            text.push_str(&format!("  Hostname: {}\n", hostname));
        }
        if let Some(vendor) = &host.vendor {
            text.push_str(&format!("  Vendor: {}\n", vendor));
        }
        if let Some(os) = &host.os {
            text.push_str(&format!("  OS: {}\n", os));
        }
        text.push_str(&format!(
            "  Ports: {}\n\n",
            host.ports
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }

    let success = copy_to_system_clipboard(&text);

    state.export_feedback = ExportFeedback {
        show: true,
        message: if success {
            format!("Copied {} hosts to clipboard", state.results.len())
        } else {
            "Failed to copy to clipboard".to_string()
        },
        is_error: !success,
        timestamp: Some(std::time::Instant::now()),
    };
}

/// Cross-platform clipboard copy
fn copy_to_system_clipboard(text: &str) -> bool {
    #[cfg(target_os = "windows")]
    {
        use std::io::Write;
        use std::process::{Command, Stdio};

        if let Ok(mut child) = Command::new("clip")
            .stdin(Stdio::piped())
            .creation_flags(CREATE_NO_WINDOW)
            .spawn()
        {
            if let Some(ref mut stdin) = child.stdin {
                let _ = stdin.write_all(text.as_bytes());
            }
            return child.wait().map(|s| s.success()).unwrap_or(false);
        }
        false
    }

    #[cfg(target_os = "macos")]
    {
        use std::io::Write;
        use std::process::{Command, Stdio};

        if let Ok(mut child) = Command::new("pbcopy").stdin(Stdio::piped()).spawn() {
            if let Some(ref mut stdin) = child.stdin {
                let _ = stdin.write_all(text.as_bytes());
            }
            return child.wait().map(|s| s.success()).unwrap_or(false);
        }
        false
    }

    #[cfg(target_os = "linux")]
    {
        use std::io::Write;
        use std::process::{Command, Stdio};

        // Try xclip first
        if let Ok(mut child) = Command::new("xclip")
            .args(["-selection", "clipboard"])
            .stdin(Stdio::piped())
            .spawn()
        {
            if let Some(ref mut stdin) = child.stdin {
                let _ = stdin.write_all(text.as_bytes());
            }
            if child.wait().map(|s| s.success()).unwrap_or(false) {
                return true;
            }
        }

        // Fall back to xsel
        if let Ok(mut child) = Command::new("xsel")
            .args(["--clipboard", "--input"])
            .stdin(Stdio::piped())
            .spawn()
        {
            if let Some(ref mut stdin) = child.stdin {
                let _ = stdin.write_all(text.as_bytes());
            }
            return child.wait().map(|s| s.success()).unwrap_or(false);
        }

        false
    }
}

fn draw_topology_view(
    ui: &mut egui::Ui,
    state: &mut AppState,
    state_arc: &Arc<Mutex<AppState>>,
    runtime: &tokio::runtime::Runtime,
) {
    ui.vertical(|ui| {
        ui.heading(RichText::new("Network Topology").size(18.0).strong());
        ui.add_space(10.0);

        // Controls row
        ui.horizontal(|ui| {
            ui.label(RichText::new("Layout:").size(18.0));
            egui::ComboBox::from_id_source("layout_combo")
                .selected_text(RichText::new(match state.layout_type {
                    LayoutType::ForceDirected => "Force-Directed",
                    LayoutType::Circular => "Circular",
                    LayoutType::Hierarchical => "Hierarchical",
                }).size(18.0))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut state.layout_type, LayoutType::ForceDirected, RichText::new("Force-Directed").size(18.0));
                    ui.selectable_value(&mut state.layout_type, LayoutType::Circular, RichText::new("Circular").size(18.0));
                    ui.selectable_value(&mut state.layout_type, LayoutType::Hierarchical, RichText::new("Hierarchical").size(18.0));
                });

            ui.add_space(10.0);

            let discover_enabled = !state.subnet_input.is_empty() && !state.is_discovering_topology;
            let disc_resp = ui.add_enabled_ui(discover_enabled, |ui| {
                draw_polished_button(ui, RichText::new("Discover Network").size(18.0), Color32::from_rgb(0, 120, 215))
            }).inner;
            
            if disc_resp.clicked() {
                state.start_topology_discovery = true;
                state.topology_widget = None;
            }

            if let Some(ref mut widget) = state.topology_widget {
                if draw_polished_button(ui, RichText::new("Re-Layout").size(18.0), Color32::from_gray(60)).clicked() {
                    widget.compute_layout(state.layout_type.clone());
                }

                if draw_polished_button(ui, RichText::new("Zoom Fit").size(18.0), Color32::from_gray(60)).clicked() {
                    widget.zoom_to_fit();
                }

                if draw_polished_button(ui, RichText::new("Reset View").size(18.0), Color32::from_gray(60)).clicked() {
                    widget.reset_view();
                }
            }

            ui.add_space(20.0);

            ui.label(RichText::new("Search:").size(18.0));
            let search_response = ui.add(egui::TextEdit::singleline(&mut state.filter_text).desired_width(150.0).font(egui::FontId::proportional(18.0)));

            if search_response.changed() {
                if let Some(ref mut widget) = state.topology_widget {
                    widget.search(&state.filter_text);
                }
            }
        });

        ui.add_space(10.0);
        ui.separator();
        ui.add_space(10.0);

        // Main topology area
        let available_rect = ui.available_rect_before_wrap();

        if let Some(ref mut widget) = state.topology_widget {
            // Trigger fetch immediately if needed (don't wait for click)
            if state.public_ip.is_none() {
                let state_clone = state_arc.clone();
                let ctx_clone = ui.ctx().clone();
                state.public_ip = Some("Fetching...".to_string());
                
                runtime.spawn(async move {
                    if let Some(ip) = crate::scanner::external::get_public_ip().await {
                        if let Ok(mut guard) = state_clone.lock() {
                            guard.public_ip = Some(ip.to_string());
                        }
                        
                                                // Fetch open ports from external source (Shodan InternetDB)
                                                {
                                                    let ports = crate::scanner::external::get_open_ports_from_shodan(&ip).await;
                                                    
                                                    if let Ok(mut guard) = state_clone.lock() {
                                                        guard.public_ports = Some(ports);
                                                    }
                                                }
                                            } else {
                                                if let Ok(mut guard) = state_clone.lock() {
                                                    guard.public_ip = Some("Failed to fetch".to_string());
                                                    guard.public_ports = Some(vec![]);
                                                }
                                            }
                                            ctx_clone.request_repaint();
                                        });
                                    }
                        
                                    let action = widget.show(ui, available_rect, state.public_ip.as_deref(), state.public_ports.as_deref());            state.topology_stats = Some(widget.get_stats());
            
            // Handle actions from topology view
            match action {
                crate::topology::TopologyAction::ScanHost(ip) => {
                    state.subnet_input = format!("{}/32", ip);
                    state.is_scanning = true;
                    state.scan_status = format!("Full scan on {}...", ip);
                    state.scan_progress = 0.0;
                    state.results.clear();

                    let subnet = state.subnet_input.clone();
                    let scan_mode = ScanMode::Full; // Force Full scan for targeted host
                    let state_clone = state_arc.clone();
                    let ctx_clone = ui.ctx().clone();

                    runtime.spawn(async move {
                        run_scan(&subnet, state_clone, ctx_clone, scan_mode).await;
                    });
                }
                crate::topology::TopologyAction::OpenWeb(ip, _port) => {
                    open_in_browser(&format!("http://{}", ip));
                }
                _ => {}
            }
        } else if state.is_discovering_topology {
            ui.vertical_centered(|ui| {
                ui.add_space(50.0);
                ui.spinner();
                ui.add_space(10.0);
                ui.label("Discovering network topology...");
                ui.add_space(5.0);
                ui.add(egui::ProgressBar::new(state.scan_progress).show_percentage());
                ui.add_space(10.0);
                ui.label(&state.topology_discovery_status);
            });
        } else {
            ui.vertical_centered(|ui| {
                ui.add_space(50.0);
                ui.label("No topology data available");
                ui.add_space(10.0);
                ui.label("Enter a network address above and click 'Discover Network' to map the topology.");

                if !state.subnet_input.is_empty() {
                    ui.add_space(20.0);
                    if draw_polished_button(ui, RichText::new("Start Discovery").size(18.0), Color32::from_rgb(0, 120, 215)).clicked() {
                        state.start_topology_discovery = true;
                    }
                }
            });
        }

        // Stats footer
        if let Some(ref stats) = state.topology_stats {
            ui.add_space(10.0);
            ui.separator();
            ui.add_space(5.0);

            ui.horizontal(|ui| {
                ui.label(RichText::new(format!("Nodes: {}", stats.node_count)).size(14.0));
                ui.add_space(15.0);
                ui.label(RichText::new(format!("Edges: {}", stats.edge_count)).size(14.0));
                ui.add_space(15.0);
                ui.label(RichText::new(format!("Routers: {}", stats.router_count)).size(14.0));
                ui.add_space(15.0);
                ui.label(RichText::new(format!("Servers: {}", stats.server_count)).size(14.0));
                ui.add_space(15.0);
                ui.label(RichText::new(format!("IoT: {}", stats.iot_count)).size(14.0));

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add_space(8.0);
                    ui.label(RichText::new("Shapes: △ Router, □ Server, ⬢ Firewall, ◯ Device").size(14.0).weak());
                    ui.add_space(15.0);
                    ui.separator();
                    ui.add_space(15.0);
                    ui.colored_label(Color32::from_rgb(100, 220, 220), RichText::new("IoT").size(14.0));
                    ui.add_space(8.0);
                    ui.colored_label(Color32::from_rgb(220, 80, 80), RichText::new("Firewall").size(14.0));
                    ui.add_space(8.0);
                    ui.colored_label(Color32::from_rgb(80, 150, 220), RichText::new("Server").size(14.0));
                    ui.add_space(8.0);
                    ui.colored_label(Color32::from_rgb(100, 200, 100), RichText::new("Router").size(14.0));
                    ui.add_space(8.0);
                    ui.label(RichText::new("Legend:").size(14.0));
                });
            });
        }
    });

    // Mark params as used (needed for future async operations)
    let _ = (state_arc, runtime);
}
