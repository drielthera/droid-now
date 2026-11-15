#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod core;
mod i18n;
mod process;
mod rom_check;
mod safety;
mod usb_modes;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 800.0])
            .with_min_inner_size([960.0, 640.0])
            .with_title("DroidNow"),
        ..Default::default()
    };
    eframe::run_native(
        "DroidNow",
        options,
        Box::new(|creation_context| Ok(Box::new(app::DroidNowApp::new(creation_context)))),
    )
}
