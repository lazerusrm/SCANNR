//! GUI entry point for SCANNR

// Configure Windows to not show console
// #![cfg_attr(windows, windows_subsystem = "windows")]

use SCANNR::gui::ScannrApp;

fn main() -> eframe::Result<()> {
    env_logger::init();

    // Configure eframe options
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([900.0, 600.0])
            .with_title("SCANNR - Modern Port Scanner")
            .with_decorations(true)
            .with_transparent(false),
        ..Default::default()
    };

    // Note: ScannrApp creates its own tokio runtime internally
    eframe::run_native(
        "SCANNR",
        options,
        Box::new(|cc| Box::new(ScannrApp::new(cc))),
    )
}
