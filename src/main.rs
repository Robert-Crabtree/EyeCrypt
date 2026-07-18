mod app;
mod model;

use eframe::NativeOptions;

fn main() -> Result<(), eframe::Error> {
    let options = NativeOptions::default();

    eframe::run_native(
        "EyeCrypt",
        options,
        Box::new(|cc| Ok(Box::new(app::EyeCryptApp::new(cc)))),
    )
}