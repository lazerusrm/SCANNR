//! Modern GUI for SCANNR with light/dark/auto theme support
use crate::input::ScanOrder;
use crate::port_strategy::PortStrategy;
use crate::scanner::Scanner;
use crate::topology::widget::{TopologyWidget, WidgetTopologyStats};
use crate::topology::LayoutType;
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
}

impl Default for ExportFeedback {
    fn default() -> Self {
        Self {
            show: false,
            message: String::new(),
            is_error: false,
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
    pub error: Option<String>,
    pub output: Vec<String>,
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
            error: None,
            output: Vec::new(),
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
    pub topology_stats: Option<WidgetTopologyStats>,
    pub scan_mode: ScanMode,
    pub advanced_settings_open: bool,
    pub batch_size: u16,
    pub timeout_ms: u32,
    pub udp_scan: bool,
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
            scan_mode: ScanMode::Standard,
            advanced_settings_open: false,
            batch_size: 3000,
            timeout_ms: 100,
            udp_scan: false,
        }
    }
}

// Quick port list - essential services
const QUICK_PORTS: [u16; 15] = [
    21, 22, 23, 25, 53, 80, 110, 143, 443, 993, 995, 3389, 8080, 8443, 554,
];

// Standard port list - extended for IoT/network devices
const STANDARD_PORTS: [u16; 49] = [
    // Standard services
    21, 22, 23, 25, 53, 80, 110, 143, 443, 993, 995, 3389, // IP Cameras
    81, 88, 554, 8000, 8554, 9000, 10554, // Network equipment
    161, 162, 8291, 8728, 8729, 10001, 20561, // Video/NVR
    37777, 37778, 37810, 8090, // IoT/Industrial
    1883, 502, 5683, 47808, 623, // NAS/Storage
    139, 445, 2049, 5000, // Management
    5900, 62078, 8443, 8883, 1900, // Additional common
    5555, 5080, 5300, 5200, 10000,
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

pub struct ScannrApp {
    state: Arc<Mutex<AppState>>,
    runtime: tokio::runtime::Runtime,
}

impl ScannrApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
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
                    state.subnet_input = format!("{}/24", primary.subnet[..pos].to_string());
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

fn enumerate_network_interfaces() -> impl std::future::Future<Output = Vec<NetworkInterface>> {
    async {
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

        // Apply theme
        match state.theme_mode {
            ThemeMode::Light => ctx.set_visuals(Visuals::light()),
            ThemeMode::Dark => ctx.set_visuals(Visuals::dark()),
            ThemeMode::Auto => ctx.set_visuals(Visuals::dark()),
        }

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
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.heading(RichText::new("SCANNR").size(24.0).strong());
                ui.add_space(20.0);
                ui.label(RichText::new("The Modern Port Scanner").size(14.0).weak());

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // Theme selector (right-to-left so Light appears last/rightmost)
                    ui.selectable_value(&mut state.theme_mode, ThemeMode::Light, "Light");
                    ui.selectable_value(&mut state.theme_mode, ThemeMode::Dark, "Dark");
                    ui.selectable_value(&mut state.theme_mode, ThemeMode::Auto, "Auto");

                    ui.add_space(15.0);
                    ui.separator();
                    ui.add_space(15.0);

                    // View mode selector
                    ui.selectable_value(&mut state.view_mode, ViewMode::Topology, "Topology");
                    ui.selectable_value(&mut state.view_mode, ViewMode::List, "List");
                });
            });
            ui.add_space(8.0);
        });

        // Central panel - main content
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(10.0);

            // Network selection row
            ui.horizontal(|ui| {
                ui.label("Network:");

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

                egui::ComboBox::from_id_source("network_interface")
                    .width(250.0)
                    .selected_text(&current_selection)
                    .show_ui(ui, |ui| {
                        for (idx, name, subnet) in &interfaces {
                            let selected = state.selected_interface_idx == *idx;
                            if ui.selectable_label(selected, name).clicked() {
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

                ui.add_space(10.0);

                // Manual input override
                ui.label("or CIDR:");
                ui.add(egui::TextEdit::singleline(&mut state.subnet_input).desired_width(150.0));

                ui.add_space(10.0);

                egui::ComboBox::from_id_source("scan_mode")
                    .selected_text(state.scan_mode.label())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut state.scan_mode,
                            ScanMode::Quick,
                            ScanMode::Quick.label(),
                        );
                        ui.selectable_value(
                            &mut state.scan_mode,
                            ScanMode::Standard,
                            ScanMode::Standard.label(),
                        );
                        ui.selectable_value(
                            &mut state.scan_mode,
                            ScanMode::Full,
                            ScanMode::Full.label(),
                        );
                    });

                ui.add_space(10.0);

                if ui
                    .small_button(if state.advanced_settings_open {
                        "▼ Advanced"
                    } else {
                        "▶ Advanced"
                    })
                    .clicked()
                {
                    state.advanced_settings_open = !state.advanced_settings_open;
                }
            });

            ui.add_space(10.0);

            // Large scan button row
            ui.horizontal(|ui| {
                let button_text = if state.is_scanning {
                    "Cancel Scan"
                } else {
                    "Scan Network"
                };
                let button_color = if state.is_scanning {
                    Color32::from_rgb(200, 80, 80)
                } else {
                    Color32::from_rgb(50, 120, 200)
                };

                let button = egui::Button::new(
                    RichText::new(button_text)
                        .size(18.0)
                        .strong()
                        .color(Color32::WHITE),
                )
                .fill(button_color)
                .min_size(egui::vec2(200.0, 40.0));

                let enabled = !state.subnet_input.is_empty() || state.is_scanning;

                if ui.add_enabled(enabled, button).clicked() {
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

                ui.add_space(20.0);

                // Show scan info
                if !state.is_scanning && !state.results.is_empty() {
                    ui.label(format!(
                        "Last scan: {} hosts, {} open ports",
                        state.results.len(),
                        state.scanned_ports
                    ));
                }
            });

            // Advanced settings panel
            if state.advanced_settings_open {
                ui.add_space(10.0);
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Batch Size:");
                        ui.add(
                            egui::Slider::new(&mut state.batch_size, 100..=10000).logarithmic(true),
                        );

                        ui.add_space(20.0);

                        ui.label("Timeout (ms):");
                        ui.add(
                            egui::Slider::new(&mut state.timeout_ms, 10..=5000).logarithmic(true),
                        );

                        ui.add_space(20.0);

                        ui.checkbox(&mut state.udp_scan, "UDP Scan");
                    });
                });
            }

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);

            // Scan progress/status
            if state.is_scanning {
                ui.horizontal(|ui| {
                    ui.spinner();
                    ui.label(&state.scan_status);
                });
                ui.add_space(5.0);
                ui.add(egui::ProgressBar::new(state.scan_progress).show_percentage());
                ui.add_space(10.0);
            } else if !state.scan_status.is_empty() {
                ui.label(&state.scan_status);
                ui.add_space(10.0);
            }

            // Results view
            if !state.results.is_empty() || state.view_mode == ViewMode::Topology {
                ui.separator();
                ui.add_space(10.0);

                match state.view_mode {
                    ViewMode::List => draw_list_view(ui, &mut state),
                    ViewMode::Topology => {
                        draw_topology_view(ui, &mut state, &self.state, &self.runtime)
                    }
                }
            }
        });

        // Export feedback modal (outside panels)
        if export_feedback_show {
            let feedback = state.export_feedback.clone();
            egui::Window::new("Export Result")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    let is_error = feedback.is_error;
                    ui.heading(
                        RichText::new(if is_error {
                            "Export Failed"
                        } else {
                            "Export Successful"
                        })
                        .size(18.0)
                        .color(if is_error {
                            Color32::from_rgb(255, 100, 100)
                        } else {
                            Color32::from_rgb(100, 255, 100)
                        }),
                    );

                    ui.add_space(10.0);
                    ui.label(&feedback.message);
                    ui.add_space(15.0);

                    if ui.button("OK").clicked() {
                        state.export_feedback.show = false;
                    }
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
                        ui.label(format!("Connecting to: {}:{}", ip, state.ssh_dialog.port));
                    }
                    ui.add_space(10.0);

                    egui::Grid::new("ssh_form")
                        .num_columns(2)
                        .spacing([10.0, 8.0])
                        .show(ui, |ui| {
                            ui.label("Username:");
                            ui.add(
                                egui::TextEdit::singleline(&mut state.ssh_dialog.username)
                                    .desired_width(200.0),
                            );
                            ui.end_row();

                            ui.label("Password:");
                            ui.add(
                                egui::TextEdit::singleline(&mut state.ssh_dialog.password)
                                    .password(true)
                                    .desired_width(200.0),
                            );
                            ui.end_row();
                        });

                    ui.add_space(10.0);

                    if state.ssh_dialog.connecting {
                        ui.horizontal(|ui| {
                            ui.spinner();
                            ui.label("Connecting...");
                        });
                    } else {
                        ui.horizontal(|ui| {
                            if ui.button("Connect").clicked()
                                && !state.ssh_dialog.username.is_empty()
                                && !state.ssh_dialog.password.is_empty()
                            {
                                let ip = state.ssh_dialog.ip;
                                let port = state.ssh_dialog.port;
                                let username = state.ssh_dialog.username.clone();
                                let password = state.ssh_dialog.password.clone();
                                let state_clone = self.state.clone();
                                let ctx_clone = ctx.clone();

                                state.ssh_dialog.connecting = true;

                                self.runtime.spawn(async move {
                                    let result = attempt_ssh_connection(
                                        ip.unwrap(),
                                        port,
                                        &username,
                                        &password,
                                    )
                                    .await;

                                    let mut state = state_clone.lock().unwrap();
                                    state.ssh_dialog.connecting = false;
                                    match result {
                                        Ok(output) => {
                                            state.ssh_dialog.output = output;
                                            state.ssh_dialog.error = None;
                                        }
                                        Err(err) => {
                                            state.ssh_dialog.error = Some(err);
                                        }
                                    }
                                    ctx_clone.request_repaint();
                                });
                            }

                            if ui.button("Cancel").clicked() {
                                state.ssh_dialog = SshDialogState::default();
                            }
                        });
                    }

                    if let Some(ref error) = state.ssh_dialog.error {
                        ui.add_space(10.0);
                        ui.colored_label(
                            Color32::from_rgb(255, 100, 100),
                            format!("Error: {}", error),
                        );
                    }

                    if !state.ssh_dialog.output.is_empty() {
                        ui.add_space(10.0);
                        ui.separator();
                        ui.label("Command Output:");
                        egui::ScrollArea::vertical()
                            .max_height(200.0)
                            .show(ui, |ui| {
                                for line in &state.ssh_dialog.output {
                                    ui.monospace(line);
                                }
                            });
                    }
                });
        }
    }
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
                        os: Some("Active (Cached)".to_string()),
                        service_names: HashMap::new(),
                    };
                    guard.results.push(host);
                }
            }
        }
        ctx.request_repaint();

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
                    };
                    guard.results.push(host);
                }
            }
        });

        discovery_scanner.run(Some(on_progress.clone()), Some(on_discovery_result)).await;

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

        // Create callbacks for main scan phases
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

        // Create result callback for real-time updates
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

        quick_scanner.run(Some(on_progress.clone()), Some(on_result.clone())).await;

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

#[cfg(target_os = "windows")]
async fn attempt_ssh_connection(
    ip: IpAddr,
    port: u16,
    username: &str,
    password: &str,
) -> Result<Vec<String>, String> {
    use std::process::Command;

    let output = Command::new("cmd")
        .args([
            "/C",
            &format!(
                "echo exit | plink -ssh -P {} -l {} -pw {} {} \"uname -a && uptime\"",
                port, username, password, ip
            ),
        ])
        .creation_flags(CREATE_NO_WINDOW)
        .output()
        .map_err(|e| format!("Failed to execute SSH command: {}", e))?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut lines = Vec::new();
        for line in stdout.lines() {
            if !line.trim().is_empty() {
                lines.push(line.to_string());
            }
        }
        Ok(lines)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("SSH connection failed: {}", stderr))
    }
}

#[cfg(target_family = "unix")]
async fn attempt_ssh_connection(
    ip: IpAddr,
    port: u16,
    username: &str,
    password: &str,
) -> Result<Vec<String>, String> {
    use std::process::Command;

    let output = Command::new("sshpass")
        .args([
            "-p",
            password,
            "ssh",
            "-p",
            &port.to_string(),
            &format!("{}@{}", username, ip),
            "uname -a && uptime",
        ])
        .output()
        .map_err(|e| format!("Failed to execute SSH command: {}", e))?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut lines = Vec::new();
        for line in stdout.lines() {
            if !line.trim().is_empty() {
                lines.push(line.to_string());
            }
        }
        Ok(lines)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("SSH connection failed: {}", stderr))
    }
}

fn draw_list_view(ui: &mut egui::Ui, state: &mut AppState) {
    ui.vertical(|ui| {
        ui.heading(RichText::new("Scan Results").size(18.0).strong());
        ui.add_space(10.0);

        if state.results.is_empty() {
            ui.centered_and_justified(|ui| {
                ui.label("No scan results yet. Click 'Scan Network' to start.");
            });
            return;
        }

        // Controls row
        ui.horizontal(|ui| {
            ui.label("Sort by:");
            egui::ComboBox::from_id_source("sort_combo")
                .selected_text(state.sort_by.label())
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut state.sort_by, SortOption::IpAddress, "IP Address");
                    ui.selectable_value(&mut state.sort_by, SortOption::PortCount, "Port Count");
                    ui.selectable_value(&mut state.sort_by, SortOption::Vendor, "Vendor");
                    ui.selectable_value(&mut state.sort_by, SortOption::Hostname, "Hostname");
                });

            if ui
                .button(if state.sort_ascending { "▲" } else { "▼" })
                .clicked()
            {
                state.sort_ascending = !state.sort_ascending;
            }

            ui.add_space(20.0);

            if ui.button("Export CSV").clicked() {
                export_results_csv(state);
            }

            if ui.button("Export JSON").clicked() {
                export_results_json(state);
            }

            if ui.button("Copy to Clipboard").clicked() {
                copy_results_to_clipboard(state);
            }
        });

        ui.add_space(10.0);

        ui.horizontal(|ui| {
            ui.label("Filter:");
            ui.add(egui::TextEdit::singleline(&mut state.filter_text).desired_width(200.0));
            if !state.filter_text.is_empty() && ui.button("Clear").clicked() {
                state.filter_text.clear();
            }
        });

        ui.add_space(10.0);

        // Filter and sort results
        let filter_lower = state.filter_text.to_lowercase();
        let mut filtered_results: Vec<_> = state
            .results
            .iter()
            .filter(|host| {
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
                    .map_or(false, |p| host.ports.contains(&p));

                ip_match || hostname_match || os_match || port_match
            })
            .collect();

        match state.sort_by {
            SortOption::IpAddress => filtered_results.sort_by_key(|h| h.ip),
            SortOption::PortCount => {
                filtered_results.sort_by_key(|h| std::cmp::Reverse(h.port_count()))
            }
            SortOption::Vendor => {
                filtered_results.sort_by_key(|h| h.vendor.clone().unwrap_or_default())
            }
            SortOption::Hostname => {
                filtered_results.sort_by_key(|h| h.hostname.clone().unwrap_or_default())
            }
        }

        if !state.sort_ascending {
            filtered_results.reverse();
        }

        ui.label(format!(
            "Showing {} of {} hosts",
            filtered_results.len(),
            state.results.len()
        ));
        ui.add_space(5.0);

        // Results list
        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                for (idx, host) in filtered_results.iter().enumerate() {
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            let host_text = match &host.hostname {
                                Some(hostname) => format!("{} ({})", hostname, host.ip),
                                None => host.ip.to_string(),
                            };

                            let is_selected = state.selected_result == Some(idx);
                            let text_color = if is_selected {
                                Color32::from_rgb(100, 200, 255)
                            } else {
                                ui.visuals().text_color()
                            };

                            if ui
                                .selectable_label(
                                    is_selected,
                                    RichText::new(&host_text)
                                        .size(16.0)
                                        .strong()
                                        .color(text_color),
                                )
                                .clicked()
                            {
                                state.selected_result = Some(idx);
                            }

                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    if let Some(ref os) = host.os {
                                        ui.label(
                                            RichText::new(os)
                                                .size(16.0)
                                                .color(Color32::from_rgb(100, 200, 100)),
                                        );
                                    }
                                    if let Some(ref vendor) = host.vendor {
                                        ui.add_space(10.0);
                                        ui.label(RichText::new(vendor).size(14.0).weak());
                                    }
                                    if let Some(ref mac) = host.mac {
                                        ui.add_space(10.0);
                                        ui.label(RichText::new(mac).size(14.0).weak());
                                    }
                                },
                            );
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
                        ui.label(
                            RichText::new(ports_text)
                                .size(16.0)
                                .color(Color32::from_rgb(100, 200, 100)),
                        );

                        ui.add_space(5.0);

                        // Actions
                        ui.horizontal(|ui| {
                            ui.menu_button("Actions", |ui| {
                                if ui.button("Open in Browser").clicked() {
                                    ui.close_menu();
                                    open_in_browser(&format!("http://{}", host.ip));
                                }

                                if host.ports.contains(&443) && ui.button("Open HTTPS").clicked() {
                                    ui.close_menu();
                                    open_in_browser(&format!("https://{}", host.ip));
                                }

                                if host.ports.contains(&22) && ui.button("SSH Connect").clicked() {
                                    ui.close_menu();
                                    state.ssh_dialog = SshDialogState {
                                        show: true,
                                        ip: Some(host.ip),
                                        port: 22,
                                        ..Default::default()
                                    };
                                }

                                if ui.button("Copy IP").clicked() {
                                    ui.close_menu();
                                    ui.output_mut(|o| o.copied_text = host.ip.to_string());
                                }

                                if ui.button("Copy All Ports").clicked() {
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

                            // Quick action buttons
                            if host.ports.contains(&80)
                                || host.ports.contains(&443)
                                || host.ports.contains(&8080)
                            {
                                if ui.small_button("🌐 Web").clicked() {
                                    let port = if host.ports.contains(&443) {
                                        443
                                    } else if host.ports.contains(&8080) {
                                        8080
                                    } else {
                                        80
                                    };
                                    let protocol = if port == 443 { "https" } else { "http" };
                                    open_in_browser(&format!(
                                        "{}://{}:{}",
                                        protocol, host.ip, port
                                    ));
                                }
                            }

                            if host.ports.contains(&22) {
                                if ui.small_button("🔑 SSH").clicked() {
                                    state.ssh_dialog = SshDialogState {
                                        show: true,
                                        ip: Some(host.ip),
                                        port: 22,
                                        ..Default::default()
                                    };
                                }
                            }
                        });
                    });
                    ui.add_space(5.0);
                }
            });
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

fn export_results_csv(state: &mut AppState) {
    use std::fs::File;
    use std::io::Write;

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

    let filename = format!(
        "scannr_export_{}.csv",
        chrono::Local::now().format("%Y%m%d_%H%M%S")
    );
    let file_path = std::path::PathBuf::from(&filename);

    match File::create(&file_path) {
        Ok(mut file) => match file.write_all(csv.as_bytes()) {
            Ok(_) => {
                state.export_feedback = ExportFeedback {
                    show: true,
                    message: format!("Exported {} hosts to {}", state.results.len(), filename),
                    is_error: false,
                };
            }
            Err(e) => {
                state.export_feedback = ExportFeedback {
                    show: true,
                    message: format!("Failed to write file: {}", e),
                    is_error: true,
                };
            }
        },
        Err(e) => {
            state.export_feedback = ExportFeedback {
                show: true,
                message: format!("Failed to create file: {}", e),
                is_error: true,
            };
        }
    }
}

fn export_results_json(state: &mut AppState) {
    use std::fs::File;
    use std::io::Write;

    let json = serde_json::to_string_pretty(&state.results).unwrap_or_default();

    let filename = format!(
        "scannr_export_{}.json",
        chrono::Local::now().format("%Y%m%d_%H%M%S")
    );
    let file_path = std::path::PathBuf::from(&filename);

    match File::create(&file_path) {
        Ok(mut file) => match file.write_all(json.as_bytes()) {
            Ok(_) => {
                state.export_feedback = ExportFeedback {
                    show: true,
                    message: format!("Exported {} hosts to {}", state.results.len(), filename),
                    is_error: false,
                };
            }
            Err(e) => {
                state.export_feedback = ExportFeedback {
                    show: true,
                    message: format!("Failed to write file: {}", e),
                    is_error: true,
                };
            }
        },
        Err(e) => {
            state.export_feedback = ExportFeedback {
                show: true,
                message: format!("Failed to create file: {}", e),
                is_error: true,
            };
        }
    }
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
            ui.label("Layout:");
            egui::ComboBox::from_id_source("layout_combo")
                .selected_text(match state.layout_type {
                    LayoutType::ForceDirected => "Force-Directed",
                    LayoutType::Circular => "Circular",
                    LayoutType::Hierarchical => "Hierarchical",
                })
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut state.layout_type, LayoutType::ForceDirected, "Force-Directed");
                    ui.selectable_value(&mut state.layout_type, LayoutType::Circular, "Circular");
                    ui.selectable_value(&mut state.layout_type, LayoutType::Hierarchical, "Hierarchical");
                });

            ui.add_space(10.0);

            let discover_enabled = !state.subnet_input.is_empty() && !state.is_discovering_topology;
            if ui.add_enabled(discover_enabled, egui::Button::new("Discover Network")).clicked() {
                state.start_topology_discovery = true;
                state.topology_widget = None;
            }

            if let Some(ref mut widget) = state.topology_widget {
                if ui.button("Re-Layout").clicked() {
                    widget.compute_layout(state.layout_type.clone());
                }

                if ui.button("Zoom Fit").clicked() {
                    widget.zoom_to_fit();
                }

                if ui.button("Reset View").clicked() {
                    widget.reset_view();
                }
            }

            ui.add_space(20.0);

            ui.label("Search:");
            let search_response = ui.add(egui::TextEdit::singleline(&mut state.filter_text).desired_width(150.0));

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
            widget.show(ui, available_rect);
            state.topology_stats = Some(widget.get_stats());
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
                    if ui.button("Start Discovery").clicked() {
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
                ui.label(RichText::new(format!("Nodes: {}", stats.node_count)).small());
                ui.add_space(15.0);
                ui.label(RichText::new(format!("Edges: {}", stats.edge_count)).small());
                ui.add_space(15.0);
                ui.label(RichText::new(format!("Routers: {}", stats.router_count)).small());
                ui.add_space(15.0);
                ui.label(RichText::new(format!("Servers: {}", stats.server_count)).small());
                ui.add_space(15.0);
                ui.label(RichText::new(format!("IoT: {}", stats.iot_count)).small());

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.colored_label(Color32::from_rgb(100, 220, 220), "IoT");
                    ui.add_space(8.0);
                    ui.colored_label(Color32::from_rgb(220, 80, 80), "Firewall");
                    ui.add_space(8.0);
                    ui.colored_label(Color32::from_rgb(80, 150, 220), "Server");
                    ui.add_space(8.0);
                    ui.colored_label(Color32::from_rgb(100, 200, 100), "Router");
                    ui.add_space(8.0);
                    ui.label("Legend:");
                });
            });
        }
    });

    // Mark params as used (needed for future async operations)
    let _ = (state_arc, runtime);
}
