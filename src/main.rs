#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use chrono::{Datelike, Duration, Local, NaiveDate};
use eframe::egui::{self, Color32, RichText, Vec2};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

// Data directory

fn data_dir() -> PathBuf {
    #[cfg(windows)]
    let base = std::env::var("APPDATA")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("."));
    #[cfg(not(windows))]
    let base = std::env::var("HOME")
        .map(|h| PathBuf::from(h).join(".local").join("share"))
        .unwrap_or_else(|_| PathBuf::from("."));

    let dir = base.join("IAMBusiness");
    std::fs::create_dir_all(&dir).ok();
    dir
}

fn cars_path() -> PathBuf {
    data_dir().join("cars.csv")
}
fn rentals_path() -> PathBuf {
    data_dir().join("rentals.csv")
}

//  Colour palette

const BG: Color32 = Color32::from_rgb(10, 14, 23);
const CARD: Color32 = Color32::from_rgb(17, 24, 39);
const CARD2: Color32 = Color32::from_rgb(24, 33, 52);
const BORDER: Color32 = Color32::from_rgb(30, 41, 59);
const BLUE: Color32 = Color32::from_rgb(59, 130, 246);
const GREEN: Color32 = Color32::from_rgb(34, 197, 94);
const AMBER: Color32 = Color32::from_rgb(245, 158, 11);
const RED: Color32 = Color32::from_rgb(239, 68, 68);
const PURPLE: Color32 = Color32::from_rgb(168, 85, 247);
const MUTED: Color32 = Color32::from_rgb(100, 116, 139);
const TEXT: Color32 = Color32::from_rgb(226, 232, 240);
const WHITE: Color32 = Color32::WHITE;

// Data models

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Car {
    id: u64,
    model: String,
    plate: String,
    category: String,
    color: String,
    year: u32,
    daily_rate: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Rental {
    id: u64,
    contract_number: String,
    car_id: u64,
    car_model: String,
    car_plate: String,
    client_name: String,
    client_phone: String,
    commercial_name: String,
    start_date: String,
    end_date: String,
    km_start: f64,
    km_end: f64,
    daily_rate: f64,
    notes: String,
    status: String,
}

impl Rental {
    fn days(&self) -> i64 {
        let s = NaiveDate::parse_from_str(&self.start_date, "%Y-%m-%d").ok();
        let e = NaiveDate::parse_from_str(&self.end_date, "%Y-%m-%d").ok();
        s.zip(e)
            .map(|(a, b)| (b - a).num_days().max(1))
            .unwrap_or(1)
    }
    fn total_price(&self) -> f64 {
        self.days() as f64 * self.daily_rate
    }
    fn total_km(&self) -> f64 {
        if self.km_end > self.km_start {
            self.km_end - self.km_start
        } else {
            0.0
        }
    }
    fn overlaps(&self, from: NaiveDate, to: NaiveDate) -> bool {
        if self.status == "Cancelled" {
            return false;
        }
        let s = NaiveDate::parse_from_str(&self.start_date, "%Y-%m-%d").ok();
        let e = NaiveDate::parse_from_str(&self.end_date, "%Y-%m-%d").ok();
        s.zip(e).map(|(a, b)| a <= to && b >= from).unwrap_or(false)
    }
}

//  CSV helpers

fn load_vec<T: for<'de> Deserialize<'de>>(path: &PathBuf) -> Vec<T> {
    csv::Reader::from_path(path)
        .ok()
        .map(|mut r| r.deserialize().filter_map(|x| x.ok()).collect())
        .unwrap_or_default()
}

fn save_vec<T: Serialize>(path: &PathBuf, v: &[T]) {
    if let Ok(mut w) = csv::Writer::from_path(path) {
        for item in v {
            let _ = w.serialize(item);
        }
    }
}

//  Navigation tabs

#[derive(PartialEq, Clone, Copy)]
enum Tab {
    Dashboard,
    Availability,
    NewRental,
    Contracts,
    Fleet,
}

//  Application state

struct App {
    tab: Tab,
    cars: Vec<Car>,
    rentals: Vec<Rental>,

    //  New rental form
    nr_car: usize,
    nr_client: String,
    nr_phone: String,
    nr_agent: String,
    nr_start: String,
    nr_end: String,
    nr_km_start: String,
    nr_km_end: String,
    nr_daily_rate: String,
    nr_notes: String,
    nr_msg: String,
    nr_ok: bool,

    //  Availability
    av_from: String,
    av_to: String,
    av_ids: Vec<u64>,
    av_done: bool,
    av_err: String,

    //  Contracts list
    ct_q: String,
    ct_status: usize,

    //  Car or fleet form
    fl_model: String,
    fl_plate: String,
    fl_year: String,
    fl_cat: String,
    fl_color: String,
    fl_rate: String,
    fl_msg: String,
    fl_ok: bool,
}

impl App {
    fn new() -> Self {
        let today = Local::now().date_naive();
        let in7 = (today + Duration::days(7)).format("%Y-%m-%d").to_string();
        let ts = today.format("%Y-%m-%d").to_string();

        let mut app = Self {
            tab: Tab::Dashboard,
            cars: load_vec(&cars_path()),
            rentals: load_vec(&rentals_path()),
            nr_car: 0,
            nr_client: String::new(),
            nr_phone: String::new(),
            nr_agent: String::new(),
            nr_start: ts.clone(),
            nr_end: in7.clone(),
            nr_km_start: "0".into(),
            nr_km_end: "0".into(),
            nr_daily_rate: String::new(),
            nr_notes: String::new(),
            nr_msg: String::new(),
            nr_ok: false,
            av_from: ts,
            av_to: in7,
            av_ids: vec![],
            av_done: false,
            av_err: String::new(),
            ct_q: String::new(),
            ct_status: 0,
            fl_model: String::new(),
            fl_plate: String::new(),
            fl_year: Local::now().year().to_string(),
            fl_cat: "Sedan".into(),
            fl_color: String::new(),
            fl_rate: "3000".into(),
            fl_msg: String::new(),
            fl_ok: false,
        };

        if app.cars.is_empty() {
            app.cars = seed_fleet();
            save_vec(&cars_path(), &app.cars);
        }
        app
    }

    //  Helpers

    fn is_available(&self, car_id: u64, from: NaiveDate, to: NaiveDate) -> bool {
        !self
            .rentals
            .iter()
            .any(|r| r.car_id == car_id && r.overlaps(from, to))
    }

    fn next_rental_id(&self) -> u64 {
        self.rentals.iter().map(|r| r.id).max().unwrap_or(0) + 1
    }

    fn next_car_id(&self) -> u64 {
        self.cars.iter().map(|c| c.id).max().unwrap_or(0) + 1
    }

    fn next_contract_number(&self) -> String {
        format!("IAM-{}-{:04}", Local::now().year(), self.next_rental_id())
    }

    fn active_today(&self) -> usize {
        let t = Local::now().date_naive();
        self.rentals
            .iter()
            .filter(|r| r.status == "Active" && r.overlaps(t, t))
            .count()
    }

    fn monthly_revenue(&self) -> f64 {
        let n = Local::now().date_naive();
        self.rentals
            .iter()
            .filter(|r| r.status != "Cancelled")
            .filter(|r| {
                NaiveDate::parse_from_str(&r.start_date, "%Y-%m-%d")
                    .map(|d| d.year() == n.year() && d.month() == n.month())
                    .unwrap_or(false)
            })
            .map(|r| r.total_price())
            .sum()
    }

    fn total_revenue(&self) -> f64 {
        self.rentals
            .iter()
            .filter(|r| r.status != "Cancelled")
            .map(|r| r.total_price())
            .sum()
    }
}

//  Default car or fleet

fn seed_fleet() -> Vec<Car> {
    vec![
        Car {
            id: 1,
            model: "Toyota Corolla 2022".into(),
            plate: "100-IAM-01".into(),
            category: "Sedan".into(),
            color: "White".into(),
            year: 2022,
            daily_rate: 3500.0,
        },
        Car {
            id: 2,
            model: "Renault Clio 2021".into(),
            plate: "100-IAM-02".into(),
            category: "Compact".into(),
            color: "Blue".into(),
            year: 2021,
            daily_rate: 2800.0,
        },
        Car {
            id: 3,
            model: "Dacia Duster 2023".into(),
            plate: "100-IAM-03".into(),
            category: "SUV".into(),
            color: "Silver".into(),
            year: 2023,
            daily_rate: 4500.0,
        },
        Car {
            id: 4,
            model: "Peugeot 208 2022".into(),
            plate: "100-IAM-04".into(),
            category: "Compact".into(),
            color: "Red".into(),
            year: 2022,
            daily_rate: 3000.0,
        },
        Car {
            id: 5,
            model: "Hyundai Tucson 2023".into(),
            plate: "100-IAM-05".into(),
            category: "SUV".into(),
            color: "Black".into(),
            year: 2023,
            daily_rate: 5000.0,
        },
    ]
}

//  UI helpers

fn frm(fill: Color32) -> egui::Frame {
    egui::Frame {
        fill,
        inner_margin: egui::Margin::same(16.0),
        rounding: egui::Rounding::same(10.0),
        stroke: egui::Stroke::new(1.0, BORDER),
        ..Default::default()
    }
}

/// Frame used for table headers.
fn hdr_frm() -> egui::Frame {
    egui::Frame {
        fill: CARD2,
        inner_margin: egui::Margin {
            left: 8.0,
            right: 8.0,
            top: 6.0,
            bottom: 6.0,
        },
        rounding: egui::Rounding {
            nw: 8.0,
            ne: 8.0,
            sw: 0.0,
            se: 0.0,
        },
        stroke: egui::Stroke::new(1.0, BORDER),
        ..Default::default()
    }
}

/// Colour for a rental status string.
fn sc(s: &str) -> Color32 {
    match s {
        "Active" => GREEN,
        "Completed" => MUTED,
        "Cancelled" => RED,
        _ => MUTED,
    }
}

fn page_title(ui: &mut egui::Ui, title: &str, sub: &str) {
    ui.label(RichText::new(title).size(22.0).color(TEXT).strong());
    if !sub.is_empty() {
        ui.label(RichText::new(sub).size(13.0).color(MUTED));
    }
}

//  eframe app

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // ── Apply dark theme ──────────────────────────────────────────────
        let mut vis = egui::Visuals::dark();
        vis.panel_fill = BG;
        vis.window_fill = BG;
        vis.extreme_bg_color = Color32::from_rgb(5, 8, 15);
        vis.widgets.noninteractive.bg_fill = CARD2;
        vis.widgets.inactive.bg_fill = CARD2;
        vis.widgets.hovered.bg_fill = Color32::from_rgb(37, 48, 70);
        vis.widgets.active.bg_fill = BLUE;
        ctx.set_visuals(vis);

        //  Top navigation bar
        egui::TopBottomPanel::top("nav")
            .exact_height(52.0)
            .show(ctx, |ui| {
                ui.horizontal_centered(|ui| {
                    ui.add_space(16.0);
                    ui.label(
                        RichText::new("🚗  IAM Business")
                            .size(16.0)
                            .color(TEXT)
                            .strong(),
                    );
                    ui.add_space(24.0);

                    let nav = [
                        ("📊  Dashboard", Tab::Dashboard),
                        ("🔍  Availability", Tab::Availability),
                        ("➕  New Rental", Tab::NewRental),
                        ("📋  Contracts", Tab::Contracts),
                        ("🚘  Fleet", Tab::Fleet),
                    ];
                    for (lbl, t) in nav {
                        let active = self.tab == t;
                        let btn =
                            egui::Button::new(RichText::new(lbl).size(13.0).color(if active {
                                WHITE
                            } else {
                                MUTED
                            }))
                            .fill(if active { BLUE } else { Color32::TRANSPARENT })
                            .rounding(6.0);
                        if ui.add_sized([118.0, 34.0], btn).clicked() {
                            self.tab = t;
                        }
                    }
                });
            });

        //  Central panel
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    ui.add_space(16.0);
                    match self.tab {
                        Tab::Dashboard => self.tab_dashboard(ui),
                        Tab::Availability => self.tab_avail(ui),
                        Tab::NewRental => self.tab_new_rental(ui),
                        Tab::Contracts => self.tab_contracts(ui),
                        Tab::Fleet => self.tab_fleet(ui),
                    }
                    ui.add_space(28.0);
                });
        });
    }
}

// Dashboard

impl App {
    fn tab_dashboard(&mut self, ui: &mut egui::Ui) {
        page_title(
            ui,
            "📊  Dashboard",
            &Local::now().format("%A, %B %d, %Y").to_string(),
        );
        ui.add_space(16.0);

        //  Stat cards sorry but i had to use emojies because idk any  like lucide in web dev
        let fleet = self.cars.len();
        let active = self.active_today();
        let avail = fleet.saturating_sub(active);
        let monthly = self.monthly_revenue();
        let total = self.total_revenue();

        let cw = (ui.available_width() - 64.0) / 4.0;
        ui.horizontal(|ui| {
            for (icon, lbl, val, col) in [
                ("🚗", "Total Fleet", fleet.to_string(), BLUE),
                ("🔑", "Rented Today", active.to_string(), AMBER),
                ("✅", "Available Now", avail.to_string(), GREEN),
                (
                    "💰",
                    "Monthly Revenue",
                    format!("{:.0} DA", monthly),
                    PURPLE,
                ),
            ] {
                frm(CARD).show(ui, |ui| {
                    ui.set_min_width(cw);
                    ui.label(
                        RichText::new(format!("{}  {}", icon, lbl))
                            .size(11.0)
                            .color(MUTED),
                    );
                    ui.add_space(4.0);
                    ui.label(RichText::new(val).size(26.0).color(col).strong());
                });
                ui.add_space(8.0);
            }
        });

        ui.add_space(16.0);

        //  Two-column lower section
        let tw = ui.available_width();
        let lw = tw * 0.60 - 8.0;
        let rw = tw - lw - 16.0;

        ui.horizontal_top(|ui| {
            // Left : Recent rentals
            ui.allocate_ui_with_layout(
                Vec2::new(lw, 0.0),
                egui::Layout::top_down(egui::Align::LEFT),
                |ui| {
                    frm(CARD).show(ui, |ui| {
                        ui.set_min_width(lw - 4.0);
                        ui.label(
                            RichText::new("Recent Contracts")
                                .size(14.0)
                                .color(TEXT)
                                .strong(),
                        );
                        ui.add_space(6.0);
                        let recent: Vec<Rental> =
                            self.rentals.iter().rev().take(8).cloned().collect();
                        if recent.is_empty() {
                            ui.label(
                                RichText::new("No contracts yet. Use '➕ New Rental' to add one.")
                                    .color(MUTED),
                            );
                        } else {
                            egui::Grid::new("dash_recent")
                                .num_columns(6)
                                .spacing([8.0, 4.0])
                                .striped(true)
                                .show(ui, |ui| {
                                    for h in
                                        ["Contract #", "Client", "Car", "Start", "End", "Status"]
                                    {
                                        ui.label(RichText::new(h).size(11.0).color(MUTED).strong());
                                    }
                                    ui.end_row();
                                    for r in &recent {
                                        ui.label(
                                            RichText::new(&r.contract_number)
                                                .size(11.0)
                                                .color(BLUE),
                                        );
                                        ui.label(
                                            RichText::new(&r.client_name).size(12.0).color(TEXT),
                                        );
                                        ui.label(
                                            RichText::new(&r.car_model).size(11.0).color(TEXT),
                                        );
                                        ui.label(
                                            RichText::new(&r.start_date).size(11.0).color(MUTED),
                                        );
                                        ui.label(
                                            RichText::new(&r.end_date).size(11.0).color(MUTED),
                                        );
                                        ui.label(
                                            RichText::new(&r.status)
                                                .size(11.0)
                                                .color(sc(&r.status)),
                                        );
                                        ui.end_row();
                                    }
                                });
                        }
                    });
                },
            );

            ui.add_space(8.0);

            // Right
            ui.allocate_ui_with_layout(
                Vec2::new(rw, 0.0),
                egui::Layout::top_down(egui::Align::LEFT),
                |ui| {
                    frm(CARD).show(ui, |ui| {
                        ui.set_min_width(rw - 4.0);
                        ui.label(
                            RichText::new("Fleet Status Today")
                                .size(14.0)
                                .color(TEXT)
                                .strong(),
                        );
                        ui.add_space(6.0);
                        let today = Local::now().date_naive();
                        let cars_snap = self.cars.clone();
                        for car in &cars_snap {
                            let rented = self.rentals.iter().any(|r| {
                                r.car_id == car.id
                                    && r.status == "Active"
                                    && r.overlaps(today, today)
                            });
                            let (col, lbl) = if rented {
                                (RED, "Rented")
                            } else {
                                (GREEN, "Free")
                            };
                            ui.horizontal(|ui| {
                                ui.colored_label(col, "●");
                                ui.label(RichText::new(&car.model).size(12.0).color(TEXT));
                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::Center),
                                    |ui| {
                                        ui.label(RichText::new(lbl).size(11.0).color(col));
                                        ui.label(RichText::new(&car.plate).size(11.0).color(MUTED));
                                    },
                                );
                            });
                            ui.add_space(2.0);
                        }
                        ui.separator();
                        ui.add_space(4.0);
                        ui.horizontal(|ui| {
                            ui.label(
                                RichText::new("Total Revenue (all time):")
                                    .size(12.0)
                                    .color(MUTED),
                            );
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    ui.label(
                                        RichText::new(format!("{:.0} DA", total))
                                            .size(13.0)
                                            .color(PURPLE)
                                            .strong(),
                                    );
                                },
                            );
                        });
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("Total Contracts:").size(12.0).color(MUTED));
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    ui.label(
                                        RichText::new(self.rentals.len().to_string())
                                            .size(13.0)
                                            .color(BLUE)
                                            .strong(),
                                    );
                                },
                            );
                        });
                    });
                },
            );
        });
    }
}

//    Availability Search

impl App {
    fn tab_avail(&mut self, ui: &mut egui::Ui) {
        page_title(
            ui,
            "🔍  Availability Search",
            "Enter a date range to see which vehicles are free",
        );
        ui.add_space(12.0);

        //  Search bar
        frm(CARD).show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new("From").color(TEXT));
                ui.add(
                    egui::TextEdit::singleline(&mut self.av_from)
                        .desired_width(130.0)
                        .hint_text("YYYY-MM-DD"),
                );
                ui.add_space(6.0);
                ui.label(RichText::new("To").color(TEXT));
                ui.add(
                    egui::TextEdit::singleline(&mut self.av_to)
                        .desired_width(130.0)
                        .hint_text("YYYY-MM-DD"),
                );
                ui.add_space(12.0);
                if ui
                    .add(
                        egui::Button::new(RichText::new("🔍  Check Availability").color(WHITE))
                            .fill(BLUE)
                            .rounding(6.0),
                    )
                    .clicked()
                {
                    self.do_avail_search();
                }
            });
            if !self.av_err.is_empty() {
                ui.add_space(4.0);
                ui.colored_label(RED, &self.av_err.clone());
            }
        });

        ui.add_space(12.0);

        //  Results
        if !self.av_done {
            frm(CARD).show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(18.0);
                    ui.label(
                        RichText::new("Enter a date range above and press '🔍 Check Availability'")
                            .size(14.0)
                            .color(MUTED),
                    );
                    ui.add_space(18.0);
                });
            });
            return;
        }

        let av_ids = self.av_ids.clone();
        let from_s = self.av_from.clone();
        let to_s = self.av_to.clone();

        if av_ids.is_empty() {
            frm(CARD).show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(14.0);
                    ui.label(
                        RichText::new("😔  No vehicles available for this period.")
                            .size(16.0)
                            .color(MUTED),
                    );
                    ui.add_space(14.0);
                });
            });
            return;
        }

        ui.label(
            RichText::new(format!(
                "✅  {} vehicle(s) available  ·  {} → {}",
                av_ids.len(),
                from_s,
                to_s
            ))
            .size(14.0)
            .color(GREEN),
        );
        ui.add_space(8.0);

        // Card grid
        let avail_cars: Vec<Car> = self
            .cars
            .iter()
            .filter(|c| av_ids.contains(&c.id))
            .cloned()
            .collect();

        let car_w = (ui.available_width() - 24.0) / 3.0;
        let mut rent_id: Option<u64> = None;

        for chunk in avail_cars.chunks(3) {
            ui.horizontal_top(|ui| {
                for car in chunk {
                    let mut clicked = false;
                    ui.allocate_ui_with_layout(
                        Vec2::new(car_w, 0.0),
                        egui::Layout::top_down(egui::Align::LEFT),
                        |ui| {
                            frm(CARD2).show(ui, |ui| {
                                ui.set_min_width(car_w - 8.0);
                                // Header row
                                ui.horizontal(|ui| {
                                    ui.label(RichText::new("🚗").size(22.0));
                                    ui.vertical(|ui| {
                                        ui.label(
                                            RichText::new(&car.model)
                                                .size(13.0)
                                                .color(TEXT)
                                                .strong(),
                                        );
                                        ui.label(
                                            RichText::new(format!("🪪  {}", car.plate))
                                                .size(11.0)
                                                .color(MUTED),
                                        );
                                    });
                                });
                                ui.add_space(4.0);
                                egui::Grid::new(format!("av_car_{}", car.id))
                                    .num_columns(2)
                                    .spacing([6.0, 2.0])
                                    .show(ui, |ui| {
                                        ui.label(RichText::new("Category").size(11.0).color(MUTED));
                                        ui.label(
                                            RichText::new(&car.category).size(11.0).color(TEXT),
                                        );
                                        ui.end_row();
                                        ui.label(RichText::new("Color").size(11.0).color(MUTED));
                                        ui.label(RichText::new(&car.color).size(11.0).color(TEXT));
                                        ui.end_row();
                                        ui.label(RichText::new("Year").size(11.0).color(MUTED));
                                        ui.label(
                                            RichText::new(car.year.to_string())
                                                .size(11.0)
                                                .color(TEXT),
                                        );
                                        ui.end_row();
                                        ui.label(RichText::new("Rate").size(11.0).color(MUTED));
                                        ui.label(
                                            RichText::new(format!("{} DA/day", car.daily_rate))
                                                .size(12.0)
                                                .color(AMBER),
                                        );
                                        ui.end_row();
                                    });
                                ui.add_space(6.0);
                                ui.colored_label(GREEN, "✅  Available for this period");
                                ui.add_space(6.0);
                                if ui
                                    .add(
                                        egui::Button::new(
                                            RichText::new("📝  Rent This Car")
                                                .size(12.0)
                                                .color(WHITE),
                                        )
                                        .fill(BLUE)
                                        .rounding(6.0),
                                    )
                                    .clicked()
                                {
                                    clicked = true;
                                }
                            });
                        },
                    );
                    if clicked {
                        rent_id = Some(car.id);
                    }
                    ui.add_space(8.0);
                }
            });
            ui.add_space(8.0);
        }

        // Navigate to New Rental with pre-filled car + dates
        if let Some(cid) = rent_id {
            if let Some(idx) = self.cars.iter().position(|c| c.id == cid) {
                self.nr_car = idx + 1;
                self.nr_start = self.av_from.clone();
                self.nr_end = self.av_to.clone();
                self.nr_daily_rate = self.cars[idx].daily_rate.to_string();
                self.nr_msg.clear();
                self.tab = Tab::NewRental;
            }
        }
    }

    fn do_avail_search(&mut self) {
        self.av_err.clear();
        let from = NaiveDate::parse_from_str(&self.av_from, "%Y-%m-%d");
        let to = NaiveDate::parse_from_str(&self.av_to, "%Y-%m-%d");
        match (from, to) {
            (Ok(f), Ok(t)) if f <= t => {
                self.av_ids = self
                    .cars
                    .iter()
                    .filter(|c| self.is_available(c.id, f, t))
                    .map(|c| c.id)
                    .collect();
                self.av_done = true;
            }
            (Ok(_), Ok(_)) => self.av_err = "⚠  Start date must be before end date.".into(),
            _ => self.av_err = "⚠  Invalid date. Use YYYY-MM-DD format.".into(),
        }
    }
}

//    New Rental

impl App {
    fn tab_new_rental(&mut self, ui: &mut egui::Ui) {
        page_title(
            ui,
            "➕  New Rental Contract",
            "Fill in all details to create a new rental",
        );
        ui.add_space(12.0);

        let form_w = ui.available_width().min(720.0);

        frm(CARD).show(ui, |ui| {
            ui.set_min_width(form_w);

            // Contract number preview
            let contract_preview = self.next_contract_number();
            ui.horizontal(|ui| {
                ui.label(RichText::new("Contract # will be:").size(12.0).color(MUTED));
                ui.label(
                    RichText::new(&contract_preview)
                        .size(13.0)
                        .color(BLUE)
                        .strong(),
                );
            });
            ui.separator();
            ui.add_space(6.0);

            egui::Grid::new("nr_form")
                .num_columns(4)
                .spacing([12.0, 10.0])
                .show(ui, |ui| {
                    //  Row 1 : Vehicle + Agent
                    ui.label(RichText::new("Vehicle  *").color(TEXT));
                    {
                        let car_label = if self.nr_car == 0 || self.nr_car > self.cars.len() {
                            "— Select a vehicle —".to_string()
                        } else {
                            let c = &self.cars[self.nr_car - 1];
                            format!("{} ({})", c.model, c.plate)
                        };
                        egui::ComboBox::from_id_source("nr_car")
                            .selected_text(&car_label)
                            .width(240.0)
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.nr_car, 0, "— Select a vehicle —");
                                let snap: Vec<_> = self.cars.iter().cloned().collect();
                                for (i, c) in snap.iter().enumerate() {
                                    let lbl = format!(
                                        "{} — {} | {} DA/day",
                                        c.model, c.plate, c.daily_rate
                                    );
                                    ui.selectable_value(&mut self.nr_car, i + 1, lbl);
                                }
                            });
                    }

                    ui.label(RichText::new("Commercial Agent  *").color(TEXT));
                    ui.add(
                        egui::TextEdit::singleline(&mut self.nr_agent)
                            .desired_width(240.0)
                            .hint_text("Agent name at IAM Business"),
                    );
                    ui.end_row();

                    // ── Row 2 : Client name + Phone
                    ui.label(RichText::new("Client Name  *").color(TEXT));
                    ui.add(
                        egui::TextEdit::singleline(&mut self.nr_client)
                            .desired_width(240.0)
                            .hint_text("Full name"),
                    );
                    ui.label(RichText::new("Phone Number").color(TEXT));
                    ui.add(
                        egui::TextEdit::singleline(&mut self.nr_phone)
                            .desired_width(240.0)
                            .hint_text("e.g. 0551 234 567"),
                    );
                    ui.end_row();

                    // ── Row 3 : Start date + End date
                    ui.label(RichText::new("Start Date  *").color(TEXT));
                    ui.add(
                        egui::TextEdit::singleline(&mut self.nr_start)
                            .desired_width(240.0)
                            .hint_text("YYYY-MM-DD"),
                    );
                    ui.label(RichText::new("End Date  *").color(TEXT));
                    ui.add(
                        egui::TextEdit::singleline(&mut self.nr_end)
                            .desired_width(240.0)
                            .hint_text("YYYY-MM-DD"),
                    );
                    ui.end_row();

                    // ── Row 4 : KM start + KM end
                    ui.label(RichText::new("KM at Start  *").color(TEXT));
                    ui.add(
                        egui::TextEdit::singleline(&mut self.nr_km_start)
                            .desired_width(240.0)
                            .hint_text("e.g. 12500"),
                    );
                    ui.label(RichText::new("KM at Return").color(TEXT));
                    ui.add(
                        egui::TextEdit::singleline(&mut self.nr_km_end)
                            .desired_width(240.0)
                            .hint_text("Fill when car is returned"),
                    );
                    ui.end_row();

                    // ── Row 5 : Price/day + Notes
                    ui.label(RichText::new("Price / Day (DA)  *").color(TEXT));
                    ui.add(
                        egui::TextEdit::singleline(&mut self.nr_daily_rate)
                            .desired_width(240.0)
                            .hint_text("Auto-filled from vehicle"),
                    );
                    ui.label(RichText::new("Notes").color(TEXT));
                    ui.add(
                        egui::TextEdit::singleline(&mut self.nr_notes)
                            .desired_width(240.0)
                            .hint_text("Optional notes"),
                    );
                    ui.end_row();
                });

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(8.0);

            //  Live preview
            self.rental_preview(ui);

            ui.add_space(10.0);

            //  Action buttons
            ui.horizontal(|ui| {
                if ui
                    .add_sized(
                        [160.0, 36.0],
                        egui::Button::new(RichText::new("💾  Save Contract").color(WHITE))
                            .fill(BLUE)
                            .rounding(6.0),
                    )
                    .clicked()
                {
                    self.save_rental();
                }
                ui.add_space(8.0);
                if ui
                    .add_sized(
                        [120.0, 34.0],
                        egui::Button::new(RichText::new("🗑  Clear Form").color(MUTED))
                            .fill(CARD2)
                            .rounding(6.0),
                    )
                    .clicked()
                {
                    self.clear_nr();
                }
            });

            if !self.nr_msg.is_empty() {
                ui.add_space(6.0);
                ui.colored_label(if self.nr_ok { GREEN } else { RED }, &self.nr_msg.clone());
            }
        });
    }

    /// Live price / availability preview shown inside the form.
    fn rental_preview(&self, ui: &mut egui::Ui) {
        let car_ok = self.nr_car > 0 && self.nr_car <= self.cars.len();
        let start_d = NaiveDate::parse_from_str(&self.nr_start, "%Y-%m-%d").ok();
        let end_d = NaiveDate::parse_from_str(&self.nr_end, "%Y-%m-%d").ok();
        let rate_ok: Option<f64> = self.nr_daily_rate.trim().parse().ok();

        if !car_ok || start_d.is_none() || end_d.is_none() || rate_ok.is_none() {
            return;
        }

        let car = &self.cars[self.nr_car - 1];
        let start = start_d.unwrap();
        let end = end_d.unwrap();
        let rate = rate_ok.unwrap();

        if start > end {
            return;
        }

        let days = (end - start).num_days().max(1);
        let total = days as f64 * rate;
        let avail = self.is_available(car.id, start, end);

        let km_start: f64 = self.nr_km_start.trim().parse().unwrap_or(0.0);
        let km_end: f64 = self.nr_km_end.trim().parse().unwrap_or(0.0);
        let km_diff = if km_end > km_start {
            km_end - km_start
        } else {
            0.0
        };

        let (bg, border, status_col, status_msg) = if avail {
            (
                Color32::from_rgb(10, 30, 20),
                GREEN,
                GREEN,
                "✅  Vehicle is available for this period",
            )
        } else {
            (
                Color32::from_rgb(35, 10, 10),
                RED,
                RED,
                "❌  Vehicle is already booked in this period!",
            )
        };

        egui::Frame {
            fill: bg,
            inner_margin: egui::Margin::same(12.0),
            rounding: egui::Rounding::same(8.0),
            stroke: egui::Stroke::new(1.0, border),
            ..Default::default()
        }
        .show(ui, |ui| {
            ui.colored_label(status_col, status_msg);
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new(format!("📅  {} day(s)", days))
                        .size(12.0)
                        .color(TEXT),
                );
                ui.add_space(16.0);
                ui.label(
                    RichText::new(format!("💵  {} DA/day", rate))
                        .size(12.0)
                        .color(TEXT),
                );
                ui.add_space(16.0);
                ui.label(
                    RichText::new(format!("💰  Total: {} DA", total))
                        .size(14.0)
                        .color(AMBER)
                        .strong(),
                );
                if km_diff > 0.0 {
                    ui.add_space(16.0);
                    ui.label(
                        RichText::new(format!("📍  {:.0} km driven", km_diff))
                            .size(12.0)
                            .color(TEXT),
                    );
                }
            });
        });
    }

    fn save_rental(&mut self) {
        //  Validate
        macro_rules! fail {
            ($msg:expr) => {{
                self.nr_msg = $msg.into();
                self.nr_ok = false;
                return;
            }};
        }
        if self.nr_car == 0 || self.nr_car > self.cars.len() {
            fail!("⚠  Please select a vehicle.");
        }
        if self.nr_client.trim().is_empty() {
            fail!("⚠  Client name is required.");
        }
        if self.nr_agent.trim().is_empty() {
            fail!("⚠  Commercial agent is required.");
        }

        let start = match NaiveDate::parse_from_str(&self.nr_start, "%Y-%m-%d") {
            Ok(d) => d,
            Err(_) => fail!("⚠  Invalid start date. Use YYYY-MM-DD."),
        };
        let end = match NaiveDate::parse_from_str(&self.nr_end, "%Y-%m-%d") {
            Ok(d) => d,
            Err(_) => fail!("⚠  Invalid end date. Use YYYY-MM-DD."),
        };
        if start > end {
            fail!("⚠  Start date must be before or equal to end date.");
        }

        let rate: f64 = match self.nr_daily_rate.trim().parse() {
            Ok(r) if r > 0.0 => r,
            _ => fail!("⚠  Invalid price per day."),
        };
        let km_start: f64 = self.nr_km_start.trim().parse().unwrap_or(0.0);
        let km_end: f64 = self.nr_km_end.trim().parse().unwrap_or(0.0);

        let car = self.cars[self.nr_car - 1].clone();
        if !self.is_available(car.id, start, end) {
            fail!("⚠  This vehicle is already booked for that period!");
        }

        //  Build rental
        let days = (end - start).num_days().max(1);
        let total = days as f64 * rate;
        let contract = self.next_contract_number();

        let rental = Rental {
            id: self.next_rental_id(),
            contract_number: contract.clone(),
            car_id: car.id,
            car_model: car.model.clone(),
            car_plate: car.plate.clone(),
            client_name: self.nr_client.trim().to_string(),
            client_phone: self.nr_phone.trim().to_string(),
            commercial_name: self.nr_agent.trim().to_string(),
            start_date: self.nr_start.clone(),
            end_date: self.nr_end.clone(),
            km_start,
            km_end,
            daily_rate: rate,
            notes: self.nr_notes.trim().to_string(),
            status: "Active".to_string(),
        };

        self.rentals.push(rental);
        save_vec(&rentals_path(), &self.rentals);

        let agent = self.nr_agent.clone();
        let msg = format!(
            "✅  Contract {} saved!  {} days · {:.0} DA total",
            contract, days, total
        );
        self.clear_nr();
        self.nr_agent = agent;
        self.nr_msg = msg;
        self.nr_ok = true;
    }

    fn clear_nr(&mut self) {
        self.nr_car = 0;
        self.nr_client.clear();
        self.nr_phone.clear();
        self.nr_agent.clear();
        let t = Local::now().date_naive();
        self.nr_start = t.format("%Y-%m-%d").to_string();
        self.nr_end = (t + Duration::days(7)).format("%Y-%m-%d").to_string();
        self.nr_km_start = "0".into();
        self.nr_km_end = "0".into();
        self.nr_daily_rate.clear();
        self.nr_notes.clear();
        self.nr_msg.clear();
        self.nr_ok = false;
    }
}

//   Contracts

impl App {
    fn tab_contracts(&mut self, ui: &mut egui::Ui) {
        page_title(
            ui,
            "📋  Contracts",
            "All rental records — searchable & filterable",
        );
        ui.add_space(12.0);

        //  Filter bar
        frm(CARD).show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new("🔎").size(16.0));
                ui.add(
                    egui::TextEdit::singleline(&mut self.ct_q)
                        .desired_width(300.0)
                        .hint_text("Search by contract #, client, car, plate, agent, phone…"),
                );
                ui.add_space(16.0);
                ui.label(RichText::new("Status:").size(12.0).color(MUTED));
                for (i, lbl) in ["All", "Active", "Completed", "Cancelled"]
                    .iter()
                    .enumerate()
                {
                    let sel = self.ct_status == i;
                    if ui
                        .add(
                            egui::Button::new(RichText::new(*lbl).size(12.0).color(if sel {
                                WHITE
                            } else {
                                MUTED
                            }))
                            .fill(if sel { BLUE } else { Color32::TRANSPARENT })
                            .rounding(4.0),
                        )
                        .clicked()
                    {
                        self.ct_status = i;
                    }
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        RichText::new(format!("{} total", self.rentals.len()))
                            .size(11.0)
                            .color(MUTED),
                    );
                });
            });
        });

        ui.add_space(8.0);

        //  Filter rentals
        let statuses = ["All", "Active", "Completed", "Cancelled"];
        let sf = statuses[self.ct_status];
        let q = self.ct_q.to_lowercase();

        let rows: Vec<(usize, Rental)> = self
            .rentals
            .iter()
            .enumerate()
            .filter(|(_, r)| {
                (sf == "All" || r.status == sf)
                    && (q.is_empty()
                        || r.contract_number.to_lowercase().contains(&q)
                        || r.client_name.to_lowercase().contains(&q)
                        || r.car_model.to_lowercase().contains(&q)
                        || r.car_plate.to_lowercase().contains(&q)
                        || r.commercial_name.to_lowercase().contains(&q)
                        || r.client_phone.contains(&q))
            })
            .map(|(i, r)| (i, r.clone()))
            .rev()
            .collect();

        if rows.is_empty() {
            frm(CARD).show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(14.0);
                    ui.label(RichText::new("No contracts found for this filter.").color(MUTED));
                    ui.add_space(14.0);
                });
            });
            return;
        }

        //  Spreadsheet table
        let mut to_complete: Option<usize> = None;
        let mut to_cancel: Option<usize> = None;

        const HDRS: &[&str] = &[
            "Contract #",
            "Client",
            "Phone",
            "Vehicle",
            "Plate",
            "Agent",
            "Start",
            "End",
            "Days",
            "KM Start",
            "KM End",
            "KM Driven",
            "DA/Day",
            "Total DA",
            "Status",
            "Actions",
        ];

        hdr_frm().show(ui, |ui| {
            egui::Grid::new("ct_hdr")
                .num_columns(HDRS.len())
                .spacing([10.0, 0.0])
                .show(ui, |ui| {
                    for h in HDRS {
                        ui.label(RichText::new(*h).size(11.0).color(MUTED).strong());
                    }
                    ui.end_row();
                });
        });

        egui::Frame {
            fill: CARD,
            inner_margin: egui::Margin::same(8.0),
            rounding: egui::Rounding {
                nw: 0.0,
                ne: 0.0,
                sw: 8.0,
                se: 8.0,
            },
            stroke: egui::Stroke::new(1.0, BORDER),
            ..Default::default()
        }
        .show(ui, |ui| {
            egui::ScrollArea::horizontal().show(ui, |ui| {
                egui::Grid::new("ct_body")
                    .num_columns(HDRS.len())
                    .spacing([10.0, 5.0])
                    .striped(true)
                    .min_col_width(60.0)
                    .show(ui, |ui| {
                        for (orig, r) in &rows {
                            let km_driven = r.total_km();

                            ui.label(RichText::new(&r.contract_number).size(11.0).color(BLUE));
                            ui.label(RichText::new(&r.client_name).size(12.0).color(TEXT));
                            ui.label(
                                RichText::new(if r.client_phone.is_empty() {
                                    "—"
                                } else {
                                    &r.client_phone
                                })
                                .size(11.0)
                                .color(MUTED),
                            );
                            ui.label(RichText::new(&r.car_model).size(12.0).color(TEXT));
                            ui.label(RichText::new(&r.car_plate).size(11.0).color(MUTED));
                            ui.label(RichText::new(&r.commercial_name).size(12.0).color(TEXT));
                            ui.label(RichText::new(&r.start_date).size(11.0).color(MUTED));
                            ui.label(RichText::new(&r.end_date).size(11.0).color(MUTED));
                            ui.label(RichText::new(r.days().to_string()).size(11.0).color(TEXT));
                            ui.label(
                                RichText::new(format!("{:.0}", r.km_start))
                                    .size(11.0)
                                    .color(MUTED),
                            );
                            ui.label(
                                RichText::new(if r.km_end > 0.0 {
                                    format!("{:.0}", r.km_end)
                                } else {
                                    "—".into()
                                })
                                .size(11.0)
                                .color(MUTED),
                            );
                            ui.label(
                                RichText::new(if km_driven > 0.0 {
                                    format!("{:.0}", km_driven)
                                } else {
                                    "—".into()
                                })
                                .size(11.0)
                                .color(TEXT),
                            );
                            ui.label(
                                RichText::new(format!("{:.0}", r.daily_rate))
                                    .size(11.0)
                                    .color(TEXT),
                            );
                            ui.label(
                                RichText::new(format!("{:.0}", r.total_price()))
                                    .size(12.0)
                                    .color(AMBER)
                                    .strong(),
                            );
                            ui.label(RichText::new(&r.status).size(11.0).color(sc(&r.status)));
                            // Action buttons
                            ui.horizontal(|ui| {
                                if r.status == "Active" {
                                    if ui
                                        .add(
                                            egui::Button::new(
                                                RichText::new("✓").size(11.0).color(WHITE),
                                            )
                                            .fill(GREEN)
                                            .rounding(3.0)
                                            .min_size(Vec2::new(22.0, 20.0)),
                                        )
                                        .on_hover_text("Mark as Completed")
                                        .clicked()
                                    {
                                        to_complete = Some(*orig);
                                    }
                                    if ui
                                        .add(
                                            egui::Button::new(
                                                RichText::new("✗").size(11.0).color(WHITE),
                                            )
                                            .fill(RED)
                                            .rounding(3.0)
                                            .min_size(Vec2::new(22.0, 20.0)),
                                        )
                                        .on_hover_text("Cancel Rental")
                                        .clicked()
                                    {
                                        to_cancel = Some(*orig);
                                    }
                                }
                            });
                            ui.end_row();
                        }
                    });
            });
        });

        // Apply mutations after render
        if let Some(i) = to_complete {
            self.rentals[i].status = "Completed".into();
            save_vec(&rentals_path(), &self.rentals);
        }
        if let Some(i) = to_cancel {
            self.rentals[i].status = "Cancelled".into();
            save_vec(&rentals_path(), &self.rentals);
        }
    }
}

//   Fleet or cars

impl App {
    fn tab_fleet(&mut self, ui: &mut egui::Ui) {
        page_title(
            ui,
            "🚘  Fleet Management",
            "Register and manage your vehicle inventory",
        );
        ui.add_space(12.0);

        let mut to_delete: Option<usize> = None;
        let cars_snap = self.cars.clone();
        let tw = ui.available_width();
        let lw = tw * 0.62;
        let rw = tw - lw - 16.0;

        ui.horizontal_top(|ui| {
            //  Left : car list
            ui.allocate_ui_with_layout(
                Vec2::new(lw, 0.0),
                egui::Layout::top_down(egui::Align::LEFT),
                |ui| {
                    frm(CARD).show(ui, |ui| {
                        ui.set_min_width(lw - 4.0);
                        ui.label(
                            RichText::new(format!("Registered Vehicles  ({})", cars_snap.len()))
                                .size(14.0)
                                .color(TEXT)
                                .strong(),
                        );
                        ui.add_space(8.0);

                        if cars_snap.is_empty() {
                            ui.label(RichText::new("No vehicles yet.").color(MUTED));
                        } else {
                            egui::Grid::new("fl_list")
                                .num_columns(8)
                                .spacing([10.0, 5.0])
                                .striped(true)
                                .show(ui, |ui| {
                                    for h in [
                                        "Model", "Plate", "Year", "Type", "Color", "DA/day",
                                        "Status", "",
                                    ] {
                                        ui.label(RichText::new(h).size(11.0).color(MUTED).strong());
                                    }
                                    ui.end_row();
                                    let today = Local::now().date_naive();
                                    for (i, c) in cars_snap.iter().enumerate() {
                                        let rented = self.rentals.iter().any(|r| {
                                            r.car_id == c.id
                                                && r.status == "Active"
                                                && r.overlaps(today, today)
                                        });
                                        ui.label(RichText::new(&c.model).size(12.0).color(TEXT));
                                        ui.label(RichText::new(&c.plate).size(11.0).color(MUTED));
                                        ui.label(
                                            RichText::new(c.year.to_string())
                                                .size(11.0)
                                                .color(MUTED),
                                        );
                                        ui.label(
                                            RichText::new(&c.category).size(11.0).color(MUTED),
                                        );
                                        ui.label(RichText::new(&c.color).size(11.0).color(MUTED));
                                        ui.label(
                                            RichText::new(format!("{}", c.daily_rate))
                                                .size(11.0)
                                                .color(AMBER),
                                        );
                                        let (col, lbl) = if rented {
                                            (RED, "Rented")
                                        } else {
                                            (GREEN, "Free")
                                        };
                                        ui.label(RichText::new(lbl).size(11.0).color(col));
                                        if ui
                                            .add(
                                                egui::Button::new(RichText::new("🗑").size(12.0))
                                                    .fill(Color32::TRANSPARENT),
                                            )
                                            .on_hover_text("Remove vehicle")
                                            .clicked()
                                        {
                                            to_delete = Some(i);
                                        }
                                        ui.end_row();
                                    }
                                });
                        }
                    });
                },
            );

            ui.add_space(8.0);

            //  Right : add form
            ui.allocate_ui_with_layout(
                Vec2::new(rw, 0.0),
                egui::Layout::top_down(egui::Align::LEFT),
                |ui| {
                    frm(CARD).show(ui, |ui| {
                        ui.set_min_width(rw - 4.0);
                        ui.label(
                            RichText::new("Add New Vehicle")
                                .size(14.0)
                                .color(TEXT)
                                .strong(),
                        );
                        ui.add_space(8.0);
                        egui::Grid::new("fl_form")
                            .num_columns(2)
                            .spacing([10.0, 8.0])
                            .show(ui, |ui| {
                                ui.label(RichText::new("Model  *").color(TEXT));
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.fl_model)
                                        .desired_width(200.0)
                                        .hint_text("Toyota Corolla 2022"),
                                );
                                ui.end_row();

                                ui.label(RichText::new("Plate  *").color(TEXT));
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.fl_plate)
                                        .desired_width(200.0)
                                        .hint_text("100-IAM-06"),
                                );
                                ui.end_row();

                                ui.label(RichText::new("Year").color(TEXT));
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.fl_year)
                                        .desired_width(200.0)
                                        .hint_text("2023"),
                                );
                                ui.end_row();

                                ui.label(RichText::new("Category").color(TEXT));
                                egui::ComboBox::from_id_source("fl_cat")
                                    .selected_text(&self.fl_cat)
                                    .width(200.0)
                                    .show_ui(ui, |ui| {
                                        for cat in [
                                            "Sedan",
                                            "Compact",
                                            "SUV",
                                            "Hatchback",
                                            "Minivan",
                                            "Pickup",
                                            "Luxury",
                                        ] {
                                            ui.selectable_value(
                                                &mut self.fl_cat,
                                                cat.to_string(),
                                                cat,
                                            );
                                        }
                                    });
                                ui.end_row();

                                ui.label(RichText::new("Color").color(TEXT));
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.fl_color)
                                        .desired_width(200.0)
                                        .hint_text("White"),
                                );
                                ui.end_row();

                                ui.label(RichText::new("Daily Rate (DA)  *").color(TEXT));
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.fl_rate)
                                        .desired_width(200.0)
                                        .hint_text("3000"),
                                );
                                ui.end_row();
                            });

                        ui.add_space(8.0);
                        if ui
                            .add_sized(
                                [170.0, 34.0],
                                egui::Button::new(RichText::new("➕  Add Vehicle").color(WHITE))
                                    .fill(BLUE)
                                    .rounding(6.0),
                            )
                            .clicked()
                        {
                            self.add_car();
                        }

                        if !self.fl_msg.is_empty() {
                            ui.add_space(6.0);
                            ui.colored_label(
                                if self.fl_ok { GREEN } else { RED },
                                &self.fl_msg.clone(),
                            );
                        }
                    });
                },
            );
        });

        // Delete outside the render loop to avoid borrow issues
        if let Some(idx) = to_delete {
            self.cars.remove(idx);
            save_vec(&cars_path(), &self.cars);
        }
    }

    fn add_car(&mut self) {
        macro_rules! fail {
            ($m:expr) => {{
                self.fl_msg = $m.into();
                self.fl_ok = false;
                return;
            }};
        }
        if self.fl_model.trim().is_empty() {
            fail!("⚠  Model is required.");
        }
        if self.fl_plate.trim().is_empty() {
            fail!("⚠  Plate is required.");
        }
        if self
            .cars
            .iter()
            .any(|c| c.plate.trim().to_lowercase() == self.fl_plate.trim().to_lowercase())
        {
            fail!("⚠  A vehicle with this plate already exists.");
        }

        let rate: f64 = match self.fl_rate.trim().parse::<f64>() {
            Ok(r) if r > 0.0 => r,
            _ => fail!("⚠  Invalid daily rate."),
        };
        let year: u32 = self.fl_year.trim().parse().unwrap_or(2020);

        let car = Car {
            id: self.next_car_id(),
            model: self.fl_model.trim().to_string(),
            plate: self.fl_plate.trim().to_string(),
            category: self.fl_cat.clone(),
            color: self.fl_color.trim().to_string(),
            year,
            daily_rate: rate,
        };
        self.fl_msg = format!("✅  '{}' added successfully.", car.model);
        self.fl_ok = true;
        self.cars.push(car);
        save_vec(&cars_path(), &self.cars);

        self.fl_model.clear();
        self.fl_plate.clear();
        self.fl_color.clear();
        self.fl_rate = "3000".into();
        self.fl_year = Local::now().year().to_string();
    }
}

//  Entry point

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "IAM Business",
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_title("IAM Business — Car Rental Manager")
                .with_inner_size([1280.0, 780.0])
                .with_min_inner_size([950.0, 600.0]),
            ..Default::default()
        },
        Box::new(|_cc| Ok(Box::new(App::new()))),
    )
}
