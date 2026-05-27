use app::App;
use eframe::egui::{self, RichText};
use models::Onglet;
use theme::{muted, text, AMBER, GREEN, RED_IAM};
use ui_helpers::*;

pub fn page_tableau(ui: &mut egui::Ui, app: &mut App) {
    titre_page(ui, "Tableau de Bord");

    ui.horizontal(|ui| {
        panneau().show(ui, |ui| {
            ui.vertical(|ui| {
                ui.label(RichText::new("Revenu Global").size(12.0).color(muted()));
                ui.label(RichText::new(format!("{:.0} DA", app.ca_total())).size(22.0).bold().color(GREEN));
            });
        });
        panneau().show(ui, |ui| {
            ui.vertical(|ui| {
                ui.label(RichText::new("Chiffre d'Affaires du Mois").size(12.0).color(muted()));
                ui.label(RichText::new(format!("{:.0} DA", app.ca_mois())).size(22.0).bold().color(GREEN));
            });
        });
        panneau().show(ui, |ui| {
            ui.vertical(|ui| {
                ui.label(RichText::new("Véhicules Loués Aujourd'hui").size(12.0).color(muted()));
                ui.label(RichText::new(format!("{}", app.loues_auj())).size(22.0).bold().color(RED_IAM));
            });
        });
    });
