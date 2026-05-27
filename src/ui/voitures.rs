use app::App;
use eframe::egui::{self, RichText};
use models::Voiture;
use storage::{cars_file, sauvegarder};
use theme::{muted, AMBER, GREEN, RED_IAM};
use ui_helpers::*;
