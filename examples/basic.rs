use eframe::egui::{self, ThemePreference};
use egui_neko;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Neko Demo",
        native_options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);

            // Use a light theme for the purposes of the demo. It's hard to see the cat's whispers
            // against a black background.
            cc.egui_ctx.set_theme(ThemePreference::Light);

            Ok(Box::new(NekoApp::new(cc)))
        }),
    )
    .expect("app to run");
}

struct NekoApp {
    neko: egui_neko::Neko,
    name: String,
    age: u32,
}

impl NekoApp {
    fn new(_: &eframe::CreationContext<'_>) -> Self {
        Self {
            neko: egui_neko::Neko::new(),
            name: "Neko".to_string(),
            age: 4,
        }
    }
}

impl eframe::App for NekoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Increment").clicked() {
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));

            self.neko.draw(ui);
        });

        ctx.request_repaint();
    }
}
