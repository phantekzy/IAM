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
let q = app.c_recherche.trim().to_lowercase();
    let mut contrats_filtres: Vec<(usize, models::Contrat)> = app.contrats.iter().enumerate()
        .filter(|(_, c)| {
            if !q.is_empty() && !c.client_nom.to_lowercase().contains(&q) && !c.numero.to_lowercase().contains(&q) && !c.voiture_plaque.to_lowercase().contains(&q) {
                return false;
            }
            match app.c_statut {
                1 => c.statut == "Actif",
                2 => c.statut == "Terminé",
                3 => c.statut == "Annulé",
                _ => true
            }
        })
        .map(|(i, c)| (i, c.clone()))
        .collect();

    contrats_filtres.reverse();

    ui.add_space(10.0);
    let start_idx = app.c_page * ITEMS_PER_PAGE;
    let pagines = &contrats_filtres[start_idx.min(contrats_filtres.len()).. (start_idx + ITEMS_PER_PAGE).min(contrats_filtres.len())];

    egui::ScrollArea::vertical().max_height(500.0).show(ui, |ui| {
        for (orig_idx, c) in pagines {
            panneau().show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new(format!("📄 Contrat : {}", c.numero)).bold());
                            let cl = match c.statut.as_str() { "Actif" => GREEN, "Terminé" => muted(), _ => RED_IAM };
                            ui.label(RichText::new(format!("({})", c.statut)).color(cl).small());
                        });
                        ui.label(format!("Client: {} (Tél: {}) | Véhicule: {}", c.client_nom, c.client_tel, c.voiture_modele));
                        ui.label(format!("Période: {} au {} ({:?} jours)", afficher_date(&c.date_debut), afficher_date(&c.date_fin), c.jours()));
                        ui.label(RichText::new(format!("Total: {:.0} DA | Payé: {:.0} DA | Reste à payer: {:.0} DA", c.total(), c.montant_paye, c.reste_a_payer())).color(AMBER));
                    });
