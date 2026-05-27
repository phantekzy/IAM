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

ui.add_space(15.0);
    ui.label(RichText::new("💰 Catalogue Occasion").size(16.0).strong());
    ui.add_space(5.0);

    let start = app.vt_page * ITEMS_PER_PAGE;
    let v_pool = &app.ventes[start.min(app.ventes.len()).. (start + ITEMS_PER_PAGE).min(app.ventes.len())];

    egui::ScrollArea::vertical().max_height(350.0).show(ui, |ui| {
        for vt in v_pool {
            panneau().show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new(&vt.nom).bold());
                            if vt.vendu {
                                ui.label(RichText::new("[VENDU]").color(GREEN).small());
                            } else {
                                ui.label(RichText::new("[EN VENTE]").color(Color32::from_rgb(59, 130, 246)).small());
                            }
                        });
                        ui.label(format!("Châssis: {} | Immat: {}", vt.num_chassis, vt.plaque));
                        ui.label(format!("Prix d'achat: {:.0} DA | Demandé: {:.0} DA", vt.prix_achat, vt.prix_demande));
                        if vt.vendu {
                            ui.label(RichText::new(format!("Vendu à : {:.0} DA (Bénéfice: {:.0} DA) le {}", vt.prix_vendu, vt.benefice(), vt.date_vente)).color(GREEN));
                        }
                    });

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let vtid = vt.id;
                        if bouton_danger(ui, "Supprimer", 75.0) {
                            app.vt_suppr_confirm = Some(vtid);
                        }
                        if !vt.vendu {
                            if bouton_vert(ui, "Marquer Vendu", 110.0) {
                                app.vt_marquer_vendu = Some(vtid);
                                app.vt_prix_vendu_input = format!("{:.0}", vt.prix_demande);
                            }
                        }
                    });
                });
            });
            ui.add_space(4.0);
        }
    });

if let Some(target_id) = app.vt_marquer_vendu {
        egui::Window::new("Confirmer le prix final de vente").collapsible(false).resizable(false).show(ui.ctx(), |ui| {
            ui.horizontal(|ui| {
                ui.label("Prix réel conclu (DA) :");
                ui.text_edit_singleline(&mut app.vt_prix_vendu_input);
            });
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                if ui.button("Valider la vente").clicked() {
                    let prx: f64 = app.vt_prix_vendu_input.trim().parse().unwrap_or(0.0);
                    if let Some(car) = app.ventes.iter_mut().find(|x| x.id == target_id) {
                        car.vendu = true;
                        car.prix_vendu = prx;
                        car.date_vente = aujourd_hui();
                    }
                    sauvegarder(&ventes_file(), &app.ventes);
                    app.vt_marquer_vendu = None;
                }
                if ui.button("Annuler").clicked() {
                    app.vt_marquer_vendu = None;
                }
            });
        });
    }
