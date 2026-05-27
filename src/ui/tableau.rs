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

ui.add_space(20.0);
    ui.label(RichText::new("Alertes Mouvements du Lendemain (Départs / Retours)").size(16.0).strong().color(text()));
    ui.add_space(8.0);

    let retours = app.retours_demain();
    let departs = app.departs_demain();

    if retours.is_empty() && departs.is_empty() {
        ui.label(RichText::new("Aucun mouvement prévu pour demain.").italic().color(muted()));
    } else {
        if !retours.is_empty() {
            ui.label(RichText::new("📥 Retours Attendus Demain :").color(AMBER).strong());
            for c in &retours {
                ui.horizontal(|ui| {
                    ui.label(format!("• Contrat {} - {} (Client: {})", c.numero, c.voiture_modele, c.client_nom));
                    if ui.small_button("Détails").clicked() {
                        app.modal_contrat.contrat_id = c.id;
                        app.modal_contrat.visible = true;
                    }
                });
            }
        }
