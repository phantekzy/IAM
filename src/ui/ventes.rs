use app::App;
use eframe::egui::{self, RichText};
use models::VoitureVente;
use storage::{sauvegarder, ventes_file};
use theme::{muted, GREEN, RED_IAM, WHITE};
use ui_helpers::*;
