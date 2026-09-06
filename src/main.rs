mod app;
mod backend;
mod pages;
mod ui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([900.0, 600.0])
            .with_title("System Tuner"),
        ..Default::default()
    };
    eframe::run_native(
        "system-tuner",
        options,
        Box::new(|cc| Ok(Box::new(app::App::new(cc)))),
    )
}
