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
