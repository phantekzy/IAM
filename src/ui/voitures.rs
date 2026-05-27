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

if !app.v_msg.is_empty() {
            ui.label(RichText::new(&app.v_msg).color(if app.v_ok { GREEN } else { RED_IAM }));
        }
    });

    ui.add_space(15.0);
    ui.label(RichText::new("🚗 Liste des Véhicules").size(16.0).strong());
    ui.add_space(5.0);

    let start = app.v_page * ITEMS_PER_PAGE;
    let pool = &app.voitures[start.min(app.voitures.len()).. (start + ITEMS_PER_PAGE).min(app.voitures.len())];

    egui::ScrollArea::vertical().max_height(400.0).show(ui, |ui| {
        for v in pool {
            let active_c = app.contrat_actif_pour_voiture(v.id);
            panneau().show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label(RichText::new(&v.modele).bold());
                        ui.label(format!("Plaque: {} | Catégorie: {} | Couleur: {}", v.plaque, v.categorie, v.couleur));
                        ui.label(RichText::new(format!("Tarif: {:.0} DA/Jour", v.tarif_jour)).color(GREEN));
                        
                        if let Some(c) = active_c {
                            ui.label(RichText::new(format!("Status: Loué (Contrat: {} - Client: {})", c.numero, c.client_nom)).color(RED_IAM));
                        } else {
                            let et_cl = if v.etat == "En maintenance" { AMBER } else { GREEN };
                            ui.label(RichText::new(format!("État: {}", v.etat)).color(et_cl));
                        }
                    });

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let vid = v.id;
                        if bouton_danger(ui, "Retirer", 70.0) {
                            app.v_suppr_confirm = Some(vid);
                        }
                        if v.etat == "Bon état" {
                            if bouton_neutre(ui, "Maintenance", 100.0) {
                                if let Some(found) = app.voitures.iter_mut().find(|x| x.id == vid) {
                                    found.etat = "En maintenance".into();
                                    sauvegarder(&cars_file(), &app.voitures);
                                }
                            }
                        } else if bouton_vert(ui, "Rétablir", 100.0) {
                            if let Some(found) = app.voitures.iter_mut().find(|x| x.id == vid) {
                                found.etat = "Bon état".into();
                                sauvegarder(&cars_file(), &app.voitures);
                            }
                        }
                        if bouton_neutre(ui, "Fiche", 60.0) {
                            app.modal_voiture.voiture_id = v.id;
                            app.modal_voiture.visible = true;
                        }
                    });
                });
            });
            ui.add_space(4.0);
        }
    });
