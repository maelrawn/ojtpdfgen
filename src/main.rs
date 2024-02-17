// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
//todo!("Bundle binary with pdfium-render libs for each platform");
use pdfium_render::prelude::*;
use std::error::Error;

// have bleach ready for your eyes if you read this. I just needed to get it out
// in one sitting w/ no experience using any of this or any idea what I'd need.
// will refactor out of embarrassment later.

mod window;
pub use window::TemplateApp;
mod date;
pub use date::*;
mod pdf;
pub use pdf::*;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), Box<dyn Error>> {
    // Initialize eframe & egui
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([300.0, 300.0])
             .with_min_inner_size([300.0, 300.0]),
        ..Default::default()
    };
    eframe::run_native(
        "OJT PDF Generator",
        native_options,
        Box::new(|cc| Box::new(TemplateApp::new(cc))),
    );

    // Initialize pdfium-render

    // todo!("Generate display window");
    // todo!("Take user input");
    // todo!("Validate user input");
    Ok(())
}