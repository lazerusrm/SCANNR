//! Modern GUI for SCANNR with light/dark/auto theme support
use crate::scanner::Scanner;
use crate::input::{PortRange, ScanOrder, PortStrategy};
use crate::topology::widget::{TopologyWidget, WidgetTopologyStats};
use crate::topology::LayoutType;
use egui::{Color32, RichText, Visuals};
use ipnetwork::IpNetwork;
use serde::Serialize;
use std::collections::{HashMap, HashSet};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
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

fn get_optimal_batch_size() -> u16 {
    #[cfg(target_os = "windows")]
    return 3000;

    #[cfg(target_family = "unix")]
    {
        use std::process::Command;
        if let Ok(output) = Command::new("sh").arg("-c").arg("ulimit -n").output() {
            if let Ok(limit) = String::from_utf8_lossy(&output.stdout)
                .trim()
                .parse::<u16>()
            {
                return std::cmp::min(limit.saturating_sub(100), 10000);
            }
        }
        3000
    }
}

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
            .unwrap();

        let state = Arc::new(Mutex::new(AppState::default()));
        let state_clone = state.clone();
        let _ctx = cc.egui_ctx.clone();

        runtime.spawn(async move {
            let interfaces = enumerate_network_interfaces().await;

            let mut state = state_clone.lock().unwrap();
            state.available_interfaces = interfaces;

            if !state.available_interfaces.is_empty() {
                let primary = state.available_interfaces.first().unwrap();
                state.subnet_input = primary.subnet.clone();
                state.selected_interface_idx = 0;
            }

            drop(state);
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

                if let Ok(net) = ipnetwork::IpNetwork::from(addr) {
                    let ip_str = net.network().to_string();
                    let prefix = net.prefix();
                    let gateway = net.broadcast();

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

impl eframe::App for RustScanApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut state = self.state.lock().unwrap();

        match state.theme_mode {
            ThemeMode::Light => {
                ctx.set_visuals(Visuals::light());
            }
            ThemeMode::Dark => {
                ctx.set_visuals(Visuals::dark());
            }
            ThemeMode::Auto => {
                ctx.set_visuals(Visuals::dark());
            }
        }

        if state.is_scanning {
            ctx.request_repaint();
        }

        if state.is_discovering_topology && !state.is_scanning {
            let subnet = state.subnet_input.clone();
            let cancel_flag = Arc::new(AtomicBool::new(false));
            let state_clone = self.state.clone();
            let ctx_clone = ctx.clone();

            self.runtime.spawn(async move {
                let cancel = cancel_flag.clone();

                let graph =
                    crate::topology::graph::discover_and_build_fast(&subnet, 128, 150, cancel)
                        .await;

                let mut state = state_clone.lock().unwrap();
                let mut widget = TopologyWidget::new(graph);
                widget.compute_layout(LayoutType::ForceDirected);
                state.topology_widget = Some(widget);
                state.is_discovering_topology = false;
                state.topology_discovery_status = "Discovery complete".to_string();

                ctx_clone.request_repaint();
            });

            state.is_discovering_topology = false;
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    ui.heading(RichText::new("SCANNR").size(24.0).strong());
                    ui.add_space(20.0);
                    ui.label(RichText::new("The Modern Port Scanner").size(14.0).weak());
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.selectable_value(&mut state.theme_mode, ThemeMode::Light, "Light");
                    ui.selectable_value(&mut state.theme_mode, ThemeMode::Dark, "Dark");
                    ui.selectable_value(&mut state.theme_mode, ThemeMode::Auto, "Auto");

                    ui.add_space(15.0);
                    ui.separator();
                    ui.add_space(15.0);

                    ui.selectable_value(&mut state.view_mode, ViewMode::Topology, "Topology");
                    ui.selectable_value(&mut state.view_mode, ViewMode::List, "List");
                });
            });
            ui.add_space(8.0);
            ui.separator();
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.add_space(30.0);

                ui.group(|ui| {
                    ui.set_min_width(ui.available_width());
                    ui.vertical(|ui| {
                        ui.heading(RichText::new("Network Configuration").size(18.0).strong());
                        ui.add_space(15.0);

                        ui.horizontal(|ui| {
                            ui.label(RichText::new("Network:").size(14.0).strong());
                            ui.add_space(10.0);

                            let interfaces_data: Vec<(String, String, u8)> = state
                                .available_interfaces
                                .iter()
                                .map(|iface| {
                                    (iface.subnet.clone(), iface.name.clone(), iface.priority)
                                })
                                .collect();

                            let selected_text = if interfaces_data.is_empty() {
                                "Detecting...".to_string()
                            } else {
                                let (subnet, name, _) =
                                    &interfaces_data[state.selected_interface_idx];
                                format!("{} ({})", subnet, name)
                            };

                            let mut new_selection: Option<(usize, String)> = None;

                            egui::ComboBox::from_id_source("interface_selector")
                                .selected_text(&selected_text)
                                .width(350.0)
                                .show_ui(ui, |ui| {
                                    for (idx, (subnet, name, _)) in
                                        interfaces_data.iter().enumerate()
                                    {
                                        let response = ui.selectable_label(
                                            state.selected_interface_idx == idx,
                                            format!("{} ({})", subnet, name),
                                        );

                                        if response.clicked() {
                                            new_selection = Some((idx, subnet.clone()));
                                            ui.close_menu();
                                        }
                                    }
                                });

                            if let Some((new_idx, new_subnet)) = new_selection {
                                state.selected_interface_idx = new_idx;
                                state.subnet_input = new_subnet;
                            }

                            ui.add_space(20.0);

                            if ui.button("Refresh Interfaces").clicked() {
                                let state_clone = self.state.clone();
                                let ctx_clone = ctx.clone();

                                self.runtime.spawn(async move {
                                    let interfaces = enumerate_network_interfaces().await;

                                    let mut state = state_clone.lock().unwrap();
                                    state.available_interfaces = interfaces;

                                    if !state.available_interfaces.is_empty() {
                                        let primary = state.available_interfaces.first().unwrap();
                                        state.subnet_input = primary.subnet.clone();
                                        state.selected_interface_idx = 0;
                                    }

                                    ctx_clone.request_repaint();
                                });
                    }
                });

                    ui.add_space(20.0);

                ui.group(|ui| {
                    ui.set_min_width(ui.available_width());

                    match state.view_mode {
                        ViewMode::List => {
                            draw_list_view(ui, state);
                        }
                        ViewMode::Topology => {
                            draw_topology_view(ui, state);
                        }
                    }
                });

                ui.add_space(20.0);
                });

        if state.export_feedback.show {
            let feedback = state.export_feedback.clone();
            let is_error = feedback.is_error;

            egui::Window::new("Export Result")
                .collapsible(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    ui.label(
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
                    ui.label(feedback.message);

                    ui.add_space(15.0);

                        ui.horizontal(|ui| {
                            ui.add_space(ui.available_width() - 80.0);
                            if ui.button("OK").clicked() {
                                state.export_feedback.show = false;
                            }
                        });
                });
            });
        });
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

        // Use ipnetwork's hosts() method to iterate through usable host addresses
        for host in network.hosts() {
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
        let total_ops = total_ips * total_ports;

        if total_ips == 0 || total_ports == 0 {
            let mut state_guard = state.lock().unwrap();
            state_guard.is_scanning = false;
            state_guard.scan_status = "Invalid scan range: no IPs or ports to scan.".to_string();
            state_guard.scan_progress = 0.0;
            return;
        }

        println!(
            "Starting scan: {} IPs x {} ports = {} total operations",
            total_ips, total_ports, total_ops
        );

        {
            let mut state_guard = state.lock().unwrap();
            state_guard.total_ips = total_ips;
        }

        let ports: Vec<u16> = match scan_ports_opt {
            Some(ports) => ports,
            None => (1..=65535).collect(),
        };

        let total_ips = ips.len();
        let total_ports = ports.len();
        let total_ops = total_ips * total_ports;

        if total_ips == 0 || total_ports == 0 {
            let mut state_guard = state.lock().unwrap();
            state_guard.is_scanning = false;
            state_guard.scan_status = "Invalid scan range: no IPs or ports to scan.".to_string();
            state_guard.scan_progress = 0.0;
            return;
        }

        println!(
            "Starting scan: {} IPs x {} ports = {} total operations",
            total_ips, total_ports, total_ops
        );

        {
            let mut state_guard = state.lock().unwrap();
            state_guard.total_ips = total_ips;
        }

        let ports: Vec<u16> = match scan_ports_opt {
            Some(ports) => ports,
            None => (1..=65535).collect(),
        };

        let (batch_size, timeout, udp_scan) = {
            let state_guard = state.lock().unwrap();
            (state_guard.batch_size, state_guard.timeout_ms, state_guard.udp_scan)
        };

        let timeout = Duration::from_millis(timeout.into());

        let port_range = if ports.len() > 0 {
            let min = *ports.iter().min().unwrap();
            let max = *ports.iter().max().unwrap();
            PortRange { start: min, end: max }
        } else {
            PortRange { start: 1, end: 1 }
        };

        let port_strategy = PortStrategy::pick(&Some(port_range), None, ScanOrder::Serial);

        let mut scanned_ports = 0;

        {
            let mut guard = state.lock().unwrap();
            guard.scan_status = "Scanning network...".to_string();
            guard.scan_progress = 0.5;
        }
        ctx.request_repaint();

        let scanner = Scanner::new(
            &ips,
            batch_size,
            timeout,
            1,
            true,
            port_strategy,
            false,
            vec![],
            udp_scan,
        );

        let open_sockets = scanner.run().await;
        println!("Scanner found {} open sockets", open_sockets.len());

        let mut host_map: HashMap<IpAddr, Vec<u16>> = HashMap::new();

        for socket_addr in open_sockets {
            host_map
                .entry(socket_addr.ip())
                .or_insert_with(Vec::new)
                .push(socket_addr.port());
        }

        // Process results and update UI
        let mut results = Vec::new();
        for (ip, ports) in host_map {
            let ports_len = ports.len();
            scanned_ports += ports_len;
            let detected_os = detect_device_type(&ports);
            let host = HostInfo {
                ip,
                hostname: None,
                mac: None,
                vendor: None,
                ports,
                os: Some(detected_os.clone()),
                service_names: HashMap::new(),
            };
            results.push(host);
            println!("Host found: {} - {} ({} ports)", ip, detected_os, ports_len);
        }

        ctx.request_repaint();

        // Final update
        let mut state_guard = state.lock().unwrap();
        state_guard.results = results;
        state_guard.is_scanning = false;
        state_guard.scan_status = format!(
            "Scan complete! Found {} hosts with {} total open ports",
            state_guard.results.len(),
            scanned_ports
        );
        state_guard.scan_progress = 1.0;
        state_guard.scanned_ports = scanned_ports;
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

fn draw_list_view(ui: &mut egui::Ui, state: &mut AppState) {
    ui.vertical(|ui| {
        ui.heading(RichText::new("Scan Results").size(18.0).strong());
        ui.add_space(10.0);

        if state.results.is_empty() {
            ui.centered_and_justified(|ui| {
                ui.add_space(20.0);
                ui.label("No scan results yet. Click 'Scan Network' to start.");
            });
        } else {
            ui.add_space(10.0);

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
        ui.text_edit_singleline(&mut state.filter_text);
    });

    ui.add_space(10.0);

    let mut filtered_results: Vec<_> = state.results.iter().collect();

    filtered_results.retain(|host| {
        let ip_match =
            host.ip.to_string().contains(&state.filter_text) || state.filter_text.is_empty();
        let port_match = state
            .filter_by_port
            .map_or(true, |p| host.ports.contains(&p));

        let hostname_match = host
            .hostname
            .as_ref()
            .map(|h| h.contains(&state.filter_text))
            .unwrap_or(false);

        ip_match || hostname_match || port_match
    });

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

    egui::ScrollArea::new([true, true]).show(ui, |ui| {
        for (idx, host) in filtered_results.iter().enumerate() {
            let _host_response = ui
                .group(|ui| {
                    ui.horizontal(|ui| {
                        let host_text = if let Some(hostname) = &host.hostname {
                            format!("{} ({})", hostname, host.ip)
                        } else {
                            host.ip.to_string()
                        };

                        ui.label(RichText::new(host_text).size(16.0).strong().color(
                            if state.selected_result == Some(idx) {
                                Color32::from_rgb(100, 200, 255)
                            } else {
                                ui.visuals().text_color()
                            },
                        ));

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if let Some(ref vendor) = host.vendor {
                                ui.label(RichText::new(vendor.clone()).small().weak());
                            }

                            ui.add_space(10.0);

                            if let Some(ref os) = host.os {
                                ui.label(
                                    RichText::new(os.clone())
                                        .small()
                                        .color(Color32::from_rgb(100, 200, 100)),
                                );
                            }

                            ui.add_space(10.0);

                            if let Some(ref mac) = host.mac {
                                ui.label(RichText::new(mac.clone()).small().weak());
                            }
                        });
                    });

                    ui.add_space(8.0);

                    ui.label(
                        RichText::new(format!(
                            "Ports: {}",
                            host.ports
                                .iter()
                                .map(|p| p.to_string())
                                .collect::<Vec<_>>()
                                .join(", ")
                        ))
                        .size(14.0)
                        .color(Color32::from_rgb(100, 200, 100)),
                    );

                    ui.add_space(5.0);

                    ui.horizontal(|ui| {
                        ui.menu_button("Actions", |ui| {
                            if ui.button("Open in Browser").clicked() {
                                ui.close_menu();
                                let _ = std::process::Command::new("cmd")
                                    .args(&["/C", "start", &format!("http://{}", host.ip)])
                                    .creation_flags(CREATE_NO_WINDOW)
                                    .output();
                            }

                            if host.ports.contains(&22) {
                                if ui.button("SSH Connect").clicked() {
                                    ui.close_menu();
                                    state.ssh_dialog = SshDialogState {
                                        show: true,
                                        ip: Some(host.ip),
                                        port: 22,
                                        username: String::new(),
                                        password: String::new(),
                                        connecting: false,
                                        error: None,
                                        output: Vec::new(),
                                    };
                                }
                            }

                            if ui.button("Run Nmap").clicked() {
                                ui.close_menu();
                                let _ = std::process::Command::new("cmd")
                                    .args(&["/C", "start", "nmap", &format!("{}", host.ip)])
                                    .creation_flags(CREATE_NO_WINDOW)
                                    .output();
                            }
                        });
                    });

                    ui.add_space(5.0);
                })
                .response;
        }
        });
    });
}

fn export_results_csv(state: &mut AppState) {
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;

    let mut csv = String::new();
    csv.push_str("IP,Hostname,Vendor,MAC,OS,Ports\n");

    for host in &state.results {
        let hostname = host.hostname.as_deref().unwrap_or("");
        let vendor = host.vendor.as_deref().unwrap_or("");
        let mac = host.mac.as_deref().unwrap_or("");
        let os = host.os.as_deref().unwrap_or("");
        let ports = host
            .ports
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<_>>()
            .join(";");

        csv.push_str(&format!(
            "{},{},{},{},{}\n",
            host.ip, hostname, vendor, mac, os, ports
        ));
    }

    let filename = format!("rustscan_export_{}.csv", 
        chrono::Local::now().format("%Y%m%d_%H%M%S"));
    let file_path = std::path::PathBuf::from(&filename);

    match File::create(&file_path) {
        Ok(mut file) => {
            match file.write_all(csv.as_bytes()) {
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
            }
        }
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
    use std::path::PathBuf;

    let json = serde_json::to_string_pretty(&state.results).unwrap_or_default();

    let filename = format!("rustscan_export_{}.json", 
        chrono::Local::now().format("%Y%m%d_%H%M%S"));
    let file_path = std::path::PathBuf::from(&filename);

    match File::create(&file_path) {
        Ok(mut file) => {
            match file.write_all(json.as_bytes()) {
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
            }
        }
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
            "  Ports: {}\n",
            host.ports
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        ));
        text.push_str("\n");
    }

    state.export_feedback = ExportFeedback {
        show: true,
        message: format!("Copied {} hosts to clipboard", state.results.len()),
        is_error: false,
    };

    println!("{}", text);
}

fn draw_topology_view(ui: &mut egui::Ui, state: &mut AppState) {
    ui.vertical(|ui| {
        ui.heading(RichText::new("Network Topology").size(18.0).strong());
        ui.add_space(10.0);

        if state.results.is_empty() && !state.is_discovering_topology {
            ui.centered_and_justified(|ui| {
                ui.add_space(20.0);
                ui.label("No topology data yet. Run a scan to discover network topology.");
            });
        } else {
            ui.heading("Network Topology View");

        ui.add_space(10.0);

        ui.horizontal(|ui| {
            ui.label("Layout:");
            egui::ComboBox::from_id_source("layout_combo")
                .selected_text(format!("{:?}", state.layout_type))
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut state.layout_type,
                        LayoutType::ForceDirected,
                        "Force-Directed",
                    );
                    ui.selectable_value(&mut state.layout_type, LayoutType::Circular, "Circular");
                    ui.selectable_value(
                        &mut state.layout_type,
                        LayoutType::Hierarchical,
                        "Hierarchical",
                    );
                });

            ui.add_space(20.0);

            if ui.button("Refresh").clicked() {
                if !state.subnet_input.is_empty() {
                    state.is_discovering_topology = true;
                    state.topology_widget = None;
                }
            }

            if ui.button("Layout").clicked() {
                if let Some(ref mut widget) = state.topology_widget {
                    widget.compute_layout(state.layout_type.clone());
                }
            }

            if ui.button("Zoom Fit").clicked() {
                if let Some(ref mut widget) = state.topology_widget {
                    widget.zoom_to_fit();
                }
            }

            if ui.button("Reset").clicked() {
                if let Some(ref mut widget) = state.topology_widget {
                    widget.reset_view();
                }
            }

            ui.add_space(20.0);

            ui.label("Search:");
            let response = ui.text_edit_singleline(&mut state.filter_text);

            if response.changed() {
                if let Some(ref mut widget) = state.topology_widget {
                    widget.search(&state.filter_text);
                }
            }
        });

        ui.add_space(10.0);

        ui.separator();

        ui.add_space(10.0);

        let panel_rect = ui.max_rect();

        if let Some(ref mut widget) = state.topology_widget {
            widget.show(ui, panel_rect);

            let stats = widget.get_stats();
            state.topology_stats = Some(stats);
        } else if state.is_discovering_topology {
            ui.centered_and_justified(|ui| {
                ui.spinner();
                ui.add_space(10.0);
                ui.label("Discovering network topology...");
                ui.label(state.topology_discovery_status.clone());
            });
        } else {
            ui.centered_and_justified(|ui| {
                ui.label("No topology data");
                ui.add_space(10.0);
                ui.label("Enter a network above and click 'Refresh' to discover.");

                if ui.button("Discover Network").clicked() {
                    if !state.subnet_input.is_empty() {
                        state.is_discovering_topology = true;
                    }
                }
            });
        }

        ui.add_space(10.0);
        ui.separator();
        ui.add_space(5.0);

        ui.horizontal(|ui| {
            if let Some(ref stats) = state.topology_stats {
                ui.label(RichText::new(format!("Nodes: {}", stats.node_count)).small());
                ui.add_space(20.0);
                ui.label(RichText::new(format!("Edges: {}", stats.edge_count)).small());
                ui.add_space(20.0);
                ui.label(RichText::new(format!("Routers: {}", stats.router_count)).small());
                ui.add_space(20.0);
                ui.label(RichText::new(format!("Servers: {}", stats.server_count)).small());
                ui.add_space(20.0);
                ui.label(RichText::new(format!("IoT: {}", stats.iot_count)).small());
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label("Legend:");
                ui.add_space(10.0);
                ui.colored_label(Color32::from_rgb(100, 200, 100), "Router");
                ui.add_space(10.0);
                ui.colored_label(Color32::from_rgb(80, 150, 220), "Server");
                ui.add_space(10.0);
                ui.colored_label(Color32::from_rgb(220, 80, 80), "Firewall");
                ui.add_space(10.0);
                ui.colored_label(Color32::from_rgb(100, 220, 220), "IoT");
            });
        });
    });
    }
}
