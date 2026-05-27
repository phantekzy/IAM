use app::App;
use eframe::egui::{self, RichText};
use models::Onglet;
use storage::{rentals_file, sauvegarder};
use theme::{card2, muted, AMBER, GREEN, RED_IAM};
use ui_helpers::*;

pub fn page_contrats(ui: &mut egui::Ui, app: &mut App) {
    titre_page(ui, "Fichiers & Contrats de Location");

    ui.horizontal(|ui| {
        ui.label("Rechercher client/numéro/plaque :");
        ui.text_edit_singleline(&mut app.c_recherche);
        ui.add_space(10.0);

        egui::ComboBox::from_id_source("filt_statut")
            .selected_text(match app.c_statut { 1 => "Actifs", 2 => "Terminés", 3 => "Annulés", _ => "Tous les Statuts" })
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut app.c_statut, 0, "Tous les Statuts");
                ui.selectable_value(&mut app.c_statut, 1, "Actifs");
                ui.selectable_value(&mut app.c_statut, 2, "Terminés");
                ui.selectable_value(&mut app.c_statut, 3, "Annulés");
            });
    });
