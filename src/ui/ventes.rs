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
