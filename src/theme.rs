use eframe::egui::Color32;
use std::cell::Cell;

pub const RED_IAM: Color32 = Color32::from_rgb(200, 30, 40);
pub const GREEN: Color32 = Color32::from_rgb(34, 197, 94);
pub const AMBER: Color32 = Color32::from_rgb(245, 158, 11);
#[allow(dead_code)]
pub const BLUE: Color32 = Color32::from_rgb(59, 130, 246);
pub const WHITE: Color32 = Color32::WHITE;

thread_local! {
    static T_BG:     Cell<[u8;3]> = Cell::new([12,  12,  15]);
    static T_CARD:   Cell<[u8;3]> = Cell::new([25,  25,  32]);
    static T_CARD2:  Cell<[u8;3]> = Cell::new([35,  35,  45]);
    static T_BORDER: Cell<[u8;3]> = Cell::new([50,  50,  65]);
    static T_TEXT:   Cell<[u8;3]> = Cell::new([240, 240, 245]);
    static T_MUTED:  Cell<[u8;3]> = Cell::new([130, 130, 145]);
}
