mod app;
mod models;
mod storage;
mod theme;
mod ui;
mod ui_helpers;

use app::App;

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "IAM Business",
        eframe::NativeOptions {
            viewport: eframe::egui::ViewportBuilder::default()
                .with_title("IAM Business - Gestion Location de Voitures by phantekzy")
                .with_inner_size([1200.0, 860.0])
                .with_min_inner_size([1100.0, 700.0]),
            ..Default::default()
        },
        Box::new(|_cc| Ok(Box::new(App::new()))),
    )
}
