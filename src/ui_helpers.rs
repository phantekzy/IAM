use chrono::{Duration, Local, NaiveDate};
use eframe::egui::{self, Color32, FontId, Margin, RichText, Rounding, Stroke};
use theme::{border, card, card2, muted, text, RED_IAM, WHITE};

pub const INPUT_W: f32 = 300.0;
pub const LABEL_W: f32 = 170.0;
pub const ITEMS_PER_PAGE: usize = 10;

pub fn parse_date(s: &str) -> Option<NaiveDate> {
    let s = s.trim();
    NaiveDate::parse_from_str(s, "%d/%m/%Y")
        .or_else(|_| NaiveDate::parse_from_str(s, "%Y-%m-%d"))
        .ok()
}

pub fn afficher_date(iso: &str) -> String {
    NaiveDate::parse_from_str(iso, "%Y-%m-%d")
        .map(|d| d.format("%d/%m/%Y").to_string())
        .unwrap_or_else(|_| iso.to_string())
}

pub fn stocker_date(d: NaiveDate) -> String {
    d.format("%Y-%m-%d").to_string()
}

pub fn aujourd_hui() -> String {
    Local::now().date_naive().format("%d/%m/%Y").to_string()
}

pub fn dans_7j() -> String {
    (Local::now().date_naive() + Duration::days(7))
        .format("%d/%m/%Y")
        .to_string()
}

pub fn panneau() -> egui::Frame {
    egui::Frame {
        fill: card(),
        inner_margin: Margin::same(18.0),
        rounding: Rounding::same(10.0),
        stroke: Stroke::new(1.0, border()),
        ..Default::default()
    }
}

pub fn titre_page(ui: &mut egui::Ui, t: &str) {
    ui.label(RichText::new(t).size(24.0).color(RED_IAM).strong());
    ui.add_space(6.0);
    ui.separator();
    ui.add_space(12.0);
}

pub fn etiquette(ui: &mut egui::Ui, txt: &str) {
    ui.add_sized(
        [LABEL_W, 28.0],
        egui::Label::new(RichText::new(txt).size(13.5).color(text())),
    );
}

pub fn bouton_principal(ui: &mut egui::Ui, txt: &str) -> bool {
    ui.add_sized(
        [INPUT_W, 36.0],
        egui::Button::new(RichText::new(txt).size(14.0).color(WHITE))
            .fill(RED_IAM)
            .rounding(6.0),
    )
    .clicked()
}

pub fn bouton_danger(ui: &mut egui::Ui, txt: &str, w: f32) -> bool {
    ui.add_sized(
        [w, 30.0],
        egui::Button::new(RichText::new(txt).size(13.0).color(WHITE))
            .fill(Color32::from_rgb(180, 30, 40))
            .rounding(5.0),
    )
    .clicked()
}

pub fn bouton_neutre(ui: &mut egui::Ui, txt: &str, w: f32) -> bool {
    ui.add_sized(
        [w, 30.0],
        egui::Button::new(RichText::new(txt).size(13.0).color(muted()))
            .fill(card2())
            .rounding(5.0),
    )
    .clicked()
}

pub fn bouton_vert(ui: &mut egui::Ui, txt: &str, w: f32) -> bool {
    ui.add_sized(
        [w, 30.0],
        egui::Button::new(RichText::new(txt).size(13.0).color(WHITE))
            .fill(Color32::from_rgb(22, 130, 80))
            .rounding(5.0),
    )
    .clicked()
}

pub fn bouton_modification(ui: &mut egui::Ui, txt: &str, w: f32) -> bool {
    ui.add_sized(
        [w, 30.0],
        egui::Button::new(RichText::new(txt).size(13.0).color(WHITE))
            .fill(Color32::from_rgb(100, 120, 200))
            .rounding(5.0),
    )
    .clicked()
}

pub fn bouton_bleu(ui: &mut egui::Ui, txt: &str, w: f32) -> bool {
    ui.add_sized(
        [w, 28.0],
        egui::Button::new(RichText::new(txt).size(12.5).color(WHITE))
            .fill(Color32::from_rgb(59, 130, 246))
            .rounding(5.0),
    )
    .clicked()
}

pub fn pagination_controls(ui: &mut egui::Ui, page: &mut usize, total_items: usize) {
    let max_p = (total_items.saturating_sub(1)) / ITEMS_PER_PAGE;
    if max_p > 0 {
        ui.horizontal(|ui| {
            ui.add_space(6.0);
            if ui.button("◀ Précédent").clicked() && *page > 0 {
                *page -= 1;
            }
            ui.label(format!("Page {} / {}", *page + 1, max_p + 1));
            if ui.button("Suivant ▶").clicked() && *page < max_p {
                *page += 1;
            }
        });
    }
}
