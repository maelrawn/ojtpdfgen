use pdfium_render::prelude::*;

pub struct TemplateApp {
    pdfium: Pdfium,
    year_start: i32,
    year_end: i32,
    month_start: u32,
    month_end: u32,
    coordinator: String
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            pdfium: Pdfium::new(
            Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))
                .or_else(|_| Pdfium::bind_to_system_library()).expect("No library")),
            // pdfium: Pdfium::new(Pdfium::bind_to_statically_linked_library().unwrap()),
            year_start: 2024,
            year_end: 2024,
            month_start: 1,
            month_end: 1,
            coordinator: "".to_string(),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        Default::default()
    }
}

impl eframe::App for TemplateApp {

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("OJT Timesheet Generator v0.1");

            ui.horizontal(|ui| {
                ui.label("Input start year: ");
                ui.add(egui::Slider::new(&mut self.year_start, 2020..=2040));
            });
            ui.horizontal(|ui| {
                ui.label("Input start month: ");
                ui.add(egui::Slider::new(&mut self.month_start, 1..=12));
            });
            ui.horizontal(|ui| {
                ui.label("Input end year: ");
                ui.add(egui::Slider::new(&mut self.year_end, self.year_start..=(self.year_start+10)));
            });
            ui.horizontal(|ui| {
                ui.label("Input end month: ");
                ui.add(egui::Slider::new(&mut self.month_end, 1..=12));
            });
            ui.horizontal(|ui|{
                ui.label("Input coordinator name: ");
                ui.text_edit_singleline(&mut self.coordinator);
            });
            ui.label("Leave dates identical for single month output.");
            if ui.button("Generate Sheets").clicked() {
                for year in self.year_start..=self.year_end {
                    for mut month in 1..=12 {
                        if year == self.year_start && month < self.month_start { 
                            month = self.month_start; 
                        } else if year == self.year_end && month > self.month_end {
                            break;
                        }
                        super::generate_pdf(&self.pdfium, year, month, &self.coordinator);
                    }
                }
            }

            ui.separator();

            ui.label("Authored by James DeLaura");
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}