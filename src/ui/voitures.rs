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

ui.add_space(10.0);
        if ui.button("Insérer au Parc").clicked() {
            app.v_msg.clear();
            if app.v_modele.trim().is_empty() || app.v_plaque.trim().is_empty() {
                app.v_msg = "Erreur: Le modèle et la plaque sont requis.".into();
                return;
            }
            let rate: f64 = app.v_tarif.trim().parse().unwrap_or(3000.0);
            let v = Voiture {
                id: app.prochain_id_voiture(),
                modele: app.v_modele.trim().to_string(),
                plaque: app.v_plaque.trim().to_string(),
                categorie: app.v_cat.clone(),
                couleur: app.v_couleur.trim().to_string(),
                annee: app.v_annee.trim().parse().unwrap_or(2024),
                tarif_jour: rate,
                etat: "Bon état".into()
            };
            app.voitures.push(v);
            sauvegarder(&cars_file(), &app.voitures);
            app.v_modele.clear(); app.v_plaque.clear(); app.v_couleur.clear();
            app.v_msg = "Véhicule inséré avec succès !".into();
            app.v_ok = true;
        }
