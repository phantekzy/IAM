use app::App;
use eframe::egui::{self, RichText};
use models::Reparation;
use storage::{reparations_file, sauvegarder};
use theme::{AMBER, GREEN, RED_IAM};
use ui_helpers::*;

pub fn page_maintenance(ui: &mut egui::Ui, app: &mut App) {
    titre_page(ui, "Maintenance, Suivi Technique & Réparations");

    panneau().show(ui, |ui| {
        ui.label(RichText::new("🔧 Déclarer un incident / entretien technique").strong());
        egui::Grid::new("rep_add").num_columns(2).spacing([8.0, 8.0]).show(ui, |ui| {
            etiquette(ui, "Sélectionner la voiture :");
            egui::ComboBox::from_id_source("rep_v_cb")
                .selected_text(app.voitures.get(app.rep_voiture_idx).map(|v| v.modele.as_str()).unwrap_or("Choisir"))
                .show_ui(ui, |ui| {
                    for (idx, v) in app.voitures.iter().enumerate() {
                        ui.selectable_value(&mut app.rep_voiture_idx, idx, format!("{} ({})", v.modele, v.plaque));
                    }
                });
            ui.end_row();

            etiquette(ui, "Date opération (JJ/MM/AAAA) :");
            ui.text_edit_singleline(&mut app.rep_date);
            ui.end_row();

            etiquette(ui, "Description de l'intervention :");
            ui.text_edit_singleline(&mut app.rep_description);
            ui.end_row();

            etiquette(ui, "Coût total des réparations (DA) :");
            ui.text_edit_singleline(&mut app.rep_prix);
            ui.end_row();

            etiquette(ui, "Observations mécaniques :");
            ui.text_edit_multiline(&mut app.rep_observation);
            ui.end_row();
        });
