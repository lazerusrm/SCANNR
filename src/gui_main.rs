//! GUI entry point for SCANNR
#![cfg_attr(windows, windows_subsystem = "windows")]

use rustscan::gui::RustScanApp;

fn main() -> eframe::Result<()> {
    env_logger::init();

    // Initialize tokio runtime for async operations
    let rt = tokio::runtime::Runtime::new().unwrap();
    let _guard = rt.enter();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([900.0, 600.0])
            .with_title("SCANNR - Modern Port Scanner")
            .with_decorations(true)
            .with_transparent(false),
        ..Default::default()
    };

    eframe::run_native(
        "SCANNR",
        options,
        Box::new(|cc| Box::new(RustScanApp::new(cc))),
    )
}
