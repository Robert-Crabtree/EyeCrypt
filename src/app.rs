use eframe::egui;

pub struct EyeCryptApp {
    version: &'static str,
}

impl EyeCryptApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            version: "0.1.0",
        }
    }
}

impl eframe::App for EyeCryptApp {
    fn update(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
    ) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("👁 EyeCrypt");

            ui.separator();

            ui.label(format!("Version {}", self.version));

            ui.separator();

            ui.heading("Dataset");

            ui.label("No dataset loaded.");

            ui.separator();

            ui.heading("Pipeline");

            ui.label("Traversal:");
            ui.label("Community");

            ui.separator();

            ui.heading("Output");

            ui.code("Nothing decoded yet.");
        });
    }
}