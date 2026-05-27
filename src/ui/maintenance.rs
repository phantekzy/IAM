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

ui.add_space(10.0);
        if ui.button("Enregistrer l'opération mécanique").clicked() {
            app.rep_msg.clear();
            if app.rep_description.trim().is_empty() {
                app.rep_msg = "Erreur: La description est obligatoire.".into();
                return;
            }
            let Some(v_chosen) = app.voitures.get(app.rep_voiture_idx) else {
                app.rep_msg = "Erreur: Véhicule incorrect.".into();
                return;
            };
            let next_rep_id = app.reparations.iter().map(|x| x.id).max().unwrap_or(0) + 1;
            let rep = Reparation {
                id: next_rep_id,
                voiture_id: v_chosen.id,
                voiture_modele: v_chosen.modele.clone(),
                voiture_plaque: v_chosen.plaque.clone(),
                date: app.rep_date.trim().to_string(),
                description: app.rep_description.trim().to_string(),
                prix: app.rep_prix.trim().to_string(),
                observation: app.rep_observation.trim().to_string()
            };
            app.reparations.push(rep);
            sauvegarder(&reparations_file(), &app.reparations);
            app.rep_description.clear(); app.rep_prix.clear(); app.rep_observation.clear();
            app.rep_msg = "Opération technique mémorisée avec succès.".into();
            app.rep_ok = true;
        }
        if !app.rep_msg.is_empty() {
            ui.label(RichText::new(&app.rep_msg).color(if app.rep_ok { GREEN } else { RED_IAM }));
        }
    });

ui.add_space(15.0);
    ui.label(RichText::new("📋 Historique des Interventions Mécaniques").size(16.0).strong());
    ui.add_space(5.0);

    egui::ScrollArea::vertical().max_height(350.0).show(ui, |ui| {
        let mut suppr_req = None;
        for rep_item in &app.reparations {
            let rid = rep_item.id;
            panneau().show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label(RichText::new(format!("🔧 Voiture: {} ({})", rep_item.voiture_modele, rep_item.voiture_plaque)).bold());
                        ui.label(format!("Date: {} | Intervention: {}", rep_item.date, rep_item.description));
                        if !rep_item.prix.is_empty() {
                            ui.label(RichText::new(format!("Coût: {} DA", rep_item.prix)).color(AMBER));
                        }
                        if !rep_item.observation.is_empty() {
                            ui.label(format!("Observation: {}", rep_item.observation));
                        }
                    });

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                        if bouton_danger(ui, "Supprimer", 90.0) {
                            suppr_req = Some(rid);
                        }
                    });
                });
            });
            ui.add_space(5.0);
        }
        if let Some(id) = suppr_req {
            app.rep_suppr_confirm = Some(id);
        }
    });
