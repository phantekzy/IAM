use app::App;
use eframe::egui::{self, RichText};
use models::Contrat;
use storage::{rentals_file, sauvegarder};
use theme::{card, GREEN, RED_IAM, WHITE};
use ui_helpers::*;

pub fn page_nouveau_contrat(ui: &mut egui::Ui, app: &mut App) {
    if app.f_numero.is_empty() && app.f_en_edition.is_none() {
        app.f_numero = app.numero_suggere();
    }

    titre_page(ui, if app.f_en_edition.is_some() { "Modifier le Contrat" } else { "Créer un Nouveau Contrat de Location" });

    panneau().show(ui, |ui| {
        egui::ScrollArea::vertical().max_height(550.0).show(ui, |ui| {
            egui::Grid::new("f_grid").num_columns(2).spacing([12.0, 12.0]).show(ui, |ui| {
                etiquette(ui, "Numéro de contrat :");
                ui.text_edit_singleline(&mut app.f_numero);
                ui.end_row();

                etiquette(ui, "Sélection Véhicule :");
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Filtrer la liste :");
                        ui.text_edit_singleline(&mut app.f_voiture_search);
                    });
                    let search_q = app.f_voiture_search.trim().to_lowercase();
                    egui::ComboBox::from_id_source("cb_voitures")
                        .selected_text(app.voitures.get(app.f_voiture).map(|v| v.modele.as_str()).unwrap_or("Choisir"))
                        .show_ui(ui, |ui| {
                            for (idx, v) in app.voitures.iter().enumerate() {
                                if search_q.is_empty() || v.modele.to_lowercase().contains(&search_q) || v.plaque.to_lowercase().contains(&search_q) {
                                    if ui.selectable_value(&mut app.f_voiture, idx, format!("{} ({})", v.modele, v.plaque)).clicked() {
                                        app.f_tarif = format!("{:.0}", v.tarif_jour);
                                        app.f_car_error_cleared = true;
                                    }
                                }
                            }
                        });
                });
                ui.end_row();

                etiquette(ui, "Nom Complet Client :");
                ui.text_edit_singleline(&mut app.f_client);
                ui.end_row();

                etiquette(ui, "Téléphone Client :");
                ui.text_edit_singleline(&mut app.f_tel);
                ui.end_row();

                etiquette(ui, "Agent / Gestionnaire :");
                ui.text_edit_singleline(&mut app.f_agent);
                ui.end_row();

                etiquette(ui, "Date début (JJ/MM/AAAA) :");
                ui.text_edit_singleline(&mut app.f_debut);
                ui.end_row();

                etiquette(ui, "Date fin (JJ/MM/AAAA) :");
                ui.text_edit_singleline(&mut app.f_fin);
                ui.end_row();

                etiquette(ui, "Km départ :");
                ui.text_edit_singleline(&mut app.f_km_dep);
                ui.end_row();

                etiquette(ui, "Km Retour (si fin) :");
                ui.text_edit_singleline(&mut app.f_km_ret);
                ui.end_row();

                etiquette(ui, "Tarif journalier (DA) :");
                ui.text_edit_singleline(&mut app.f_tarif);
                ui.end_row();

                etiquette(ui, "Acompte payé (DA) :");
                ui.text_edit_singleline(&mut app.f_montant_paye);
                ui.end_row();

                etiquette(ui, "Notes additionnelles :");
                ui.text_edit_multiline(&mut app.f_notes);
                ui.end_row();
            });

ui.add_space(15.0);
            if bouton_principal(ui, if app.f_en_edition.is_some() { "Mettre à jour le contrat" } else { "Enregistrer le contrat" }) {
                app.f_msg.clear();
                app.f_ok = false;

                let (Some(du), Some(au)) = (parse_date(&app.f_debut), parse_date(&app.f_fin)) else {
                    app.f_msg = "Erreur: Les dates saisies sont invalides.".into();
                    return;
                };
                if du > au {
                    app.f_msg = "Erreur: La date de début ne peut pas être postérieure à la date de fin.".into();
                    return;
                }
                let Some(v_sel) = app.voitures.get(app.f_voiture) else {
                    app.f_msg = "Erreur: Aucun véhicule sélectionné.".into();
                    return;
                };

                let excl_id = app.f_en_edition.map(|idx| app.contrats[idx].id);
                if !app.est_disponible(v_sel.id, du, au, excl_id) && !app.f_car_error_cleared {
                    app.f_msg = "Attention: Ce véhicule est déjà réservé ou occupé sur cette plage.".into();
                    return;
                }

                let t_jour: f64 = app.f_tarif.trim().parse().unwrap_or(v_sel.tarif_jour);
                let mt_p: f64 = app.f_montant_paye.trim().parse().unwrap_or(0.0);
                let k_dep: f64 = app.f_km_dep.trim().parse().unwrap_or(0.0);
                let k_ret: f64 = app.f_km_ret.trim().parse().unwrap_or(0.0);

                if let Some(idx) = app.f_en_edition {
                    let c = &mut app.contrats[idx];
                    c.numero = app.f_numero.trim().to_string();
                    c.voiture_id = v_sel.id;
                    c.voiture_modele = v_sel.modele.clone();
                    c.voiture_plaque = v_sel.plaque.clone();
                    c.client_nom = app.f_client.trim().to_string();
                    c.client_tel = app.f_tel.trim().to_string();
                    c.agent = app.f_agent.trim().to_string();
                    c.date_debut = stocker_date(du);
                    c.date_fin = stocker_date(au);
                    c.km_depart = k_dep;
                    c.km_retour = k_ret;
                    c.tarif_jour = t_jour;
                    c.montant_paye = mt_p;
                    c.notes = app.f_notes.trim().to_string();
                    app.f_msg = "Contrat modifié avec succès.".into();
                } else {
                    let c = Contrat {
                        id: app.prochain_id_contrat(),
                        numero: app.f_numero.trim().to_string(),
                        voiture_id: v_sel.id,
                        voiture_modele: v_sel.modele.clone(),
                        voiture_plaque: v_sel.plaque.clone(),
                        client_nom: app.f_client.trim().to_string(),
                        client_tel: app.f_tel.trim().to_string(),
                        agent: app.f_agent.trim().to_string(),
                        date_debut: stocker_date(du),
                        date_fin: stocker_date(au),
                        km_depart: k_dep,
                        km_retour: k_ret,
                        tarif_jour: t_jour,
                        notes: app.f_notes.trim().to_string(),
                        statut: "Actif".to_string(),
                        montant_paye: mt_p,
                        modifiable_note: String::new(),
                    };
                    app.contrats.push(c);
                    app.f_msg = "Nouveau contrat enregistré avec succès !".into();
                }
