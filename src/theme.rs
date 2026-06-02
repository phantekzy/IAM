use eframe::egui::Color32;
use std::cell::Cell;

const RED_IAM: Color32 = Color32::from_rgb(200, 30, 40);
const GREEN: Color32 = Color32::from_rgb(34, 197, 94);
const AMBER: Color32 = Color32::from_rgb(245, 158, 11);
#[allow(dead_code)]
const BLUE: Color32 = Color32::from_rgb(59, 130, 246);
const WHITE: Color32 = Color32::WHITE;

use std::cell::Cell;
thread_local! {
    static T_BG:     Cell<[u8;3]> = Cell::new([12,  12,  15]);
    static T_CARD:   Cell<[u8;3]> = Cell::new([25,  25,  32]);
    static T_CARD2:  Cell<[u8;3]> = Cell::new([35,  35,  45]);
    static T_BORDER: Cell<[u8;3]> = Cell::new([50,  50,  65]);
    static T_TEXT:   Cell<[u8;3]> = Cell::new([240, 240, 245]);
    static T_MUTED:  Cell<[u8;3]> = Cell::new([130, 130, 145]);
}

fn c3(tl: &'static std::thread::LocalKey<Cell<[u8; 3]>>) -> Color32 {
    tl.with(|c| {
        let [r, g, b] = c.get();
        Color32::from_rgb(r, g, b)
    })
}
#[allow(dead_code)]
fn bg() -> Color32 {
    c3(&T_BG)
}
fn card() -> Color32 {
    c3(&T_CARD)
}
fn card2() -> Color32 {
    c3(&T_CARD2)
}
fn border() -> Color32 {
    c3(&T_BORDER)
}
fn text() -> Color32 {
    c3(&T_TEXT)
}
fn muted() -> Color32 {
    c3(&T_MUTED)
}

fn tinted_card(tint: Color32, alpha: u8) -> Color32 {
    let [br, bg, bb, _] = card().to_array();
    let [tr, tg, tb, _] = tint.to_array();
    let a = alpha as u16;
    let blend = |base: u8, t: u8| -> u8 { ((base as u16 * (255 - a) + t as u16 * a) / 255) as u8 };
    Color32::from_rgb(blend(br, tr), blend(bg, tg), blend(bb, tb))
}

fn set_theme(dark: bool) {
    if dark {
        T_BG.with(|c| c.set([12, 12, 15]));
        T_CARD.with(|c| c.set([25, 25, 32]));
        T_CARD2.with(|c| c.set([35, 35, 45]));
        T_BORDER.with(|c| c.set([50, 50, 65]));
        T_TEXT.with(|c| c.set([240, 240, 245]));
        T_MUTED.with(|c| c.set([130, 130, 145]));
    } else {
        T_BG.with(|c| c.set([242, 243, 247]));
        T_CARD.with(|c| c.set([255, 255, 255]));
        T_CARD2.with(|c| c.set([235, 236, 242]));
        T_BORDER.with(|c| c.set([200, 202, 215]));
        T_TEXT.with(|c| c.set([20, 20, 30]));
        T_MUTED.with(|c| c.set([90, 90, 110]));
    }
}

const INPUT_W: f32 = 300.0;
const LABEL_W: f32 = 170.0;
const ITEMS_PER_PAGE: usize = 10;
