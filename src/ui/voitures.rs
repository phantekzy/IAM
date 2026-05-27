use app::App;
use eframe::egui::{self, RichText};
use models::Voiture;
use storage::{cars_file, sauvegarder};
use theme::{muted, AMBER, GREEN, RED_IAM};
use ui_helpers::*;

pub fn page_voitures(ui: &mut egui::Ui, app: &mut App) {
    titre_page(ui, "Gestion du Parc Automobile");

    panneau().show(ui, |ui| {
        ui.label(RichText::new("➕ Ajouter un nouveau véhicule").strong());
        egui::Grid::new("v_add").num_columns(2).spacing([8.0, 8.0]).show(ui, |ui| {
            etiquette(ui, "Modèle / Marque :");
            ui.text_edit_singleline(&mut app.v_modele);
            ui.end_row();

            etiquette(ui, "Numéro d'immatriculation / Plaque :");
            ui.text_edit_singleline(&mut app.v_plaque);
            ui.end_row();

            etiquette(ui, "Tarif de base journalier (DA) :");
            ui.text_edit_singleline(&mut app.v_tarif);
            ui.end_row();

            etiquette(ui, "Catégorie :");
            egui::ComboBox::from_id_source("v_cat_cb").selected_text(&app.v_cat).show_ui(ui, |ui| {
                ui.selectable_value(&mut app.v_cat, "Berline".into(), "Berline");
                ui.selectable_value(&mut app.v_cat, "Citadine".into(), "Citadine");
                ui.selectable_value(&mut app.v_cat, "SUV".into(), "SUV");
                ui.selectable_value(&mut app.v_cat, "Utilitaire".into(), "Utilitaire");
            });
            ui.end_row();

            etiquette(ui, "Couleur :");
            ui.text_edit_singleline(&mut app.v_couleur);
            ui.end_row();
        });
