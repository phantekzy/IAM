use app::App;
use eframe::egui::{self, RichText};
use models::VoitureVente;
use storage::{sauvegarder, ventes_file};
use theme::{muted, GREEN, RED_IAM, WHITE};
use ui_helpers::*;

pub fn page_ventes(ui: &mut egui::Ui, app: &mut App) {
    titre_page(ui, "Vente de Véhicules d'Occasion");

    panneau().show(ui, |ui| {
        ui.label(RichText::new("➕ Mettre un véhicule en vente").strong());
        egui::Grid::new("vt_add").num_columns(2).spacing([8.0, 8.0]).show(ui, |ui| {
            etiquette(ui, "Nom / Modèle du véhicule :");
            ui.text_edit_singleline(&mut app.vt_nom);
            ui.end_row();

            etiquette(ui, "Plaque d'immatriculation :");
            ui.text_edit_singleline(&mut app.vt_plaque);
            ui.end_row();

            etiquette(ui, "Numéro de Châssis :");
            ui.text_edit_singleline(&mut app.vt_chassis);
            ui.end_row();

            etiquette(ui, "Prix d'Achat Interne (DA) :");
            ui.text_edit_singleline(&mut app.vt_prix_achat);
            ui.end_row();

            etiquette(ui, "Prix de Vente Demandé (DA) :");
            ui.text_edit_singleline(&mut app.vt_prix_demande);
            ui.end_row();

            etiquette(ui, "Notes techniques / Options :");
            ui.text_edit_multiline(&mut app.vt_notes);
            ui.end_row();
        });

ui.add_space(10.0);
        if ui.button("Publier l'Annonce de Vente").clicked() {
            app.vt_msg.clear();
            if app.vt_nom.trim().is_empty() {
                app.vt_msg = "Erreur: Le nom du véhicule est obligatoire.".into();
                return;
            }
            let max_id = app.ventes.iter().map(|x| x.id).max().unwrap_or(0) + 1;
            let car_v = VoitureVente {
                id: max_id,
                nom: app.vt_nom.trim().to_string(),
                plaque: app.vt_plaque.trim().to_string(),
                num_chassis: app.vt_chassis.trim().to_string(),
                num_immat: app.vt_immat.trim().to_string(),
                annee: app.vt_annee.trim().parse().unwrap_or(2020),
                prix_achat: app.vt_prix_achat.trim().parse().unwrap_or(0.0),
                prix_demande: app.vt_prix_demande.trim().parse().unwrap_or(0.0),
                prix_vendu: 0.0,
                vendu: false,
                date_vente: String::new(),
                notes: app.vt_notes.trim().to_string()
            };
            app.ventes.push(car_v);
            sauvegarder(&ventes_file(), &app.ventes);
            app.vt_nom.clear(); app.vt_plaque.clear(); app.vt_chassis.clear();
            app.vt_msg = "Véhicule mis en vente avec succès.".into();
            app.vt_ok = true;
        }
        if !app.vt_msg.is_empty() {
            ui.label(RichText::new(&app.vt_msg).color(if app.vt_ok { GREEN } else { RED_IAM }));
        }
    });
