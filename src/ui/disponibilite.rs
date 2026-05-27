use app::App;
use eframe::egui::{self, RichText};
use theme::{muted, text, GREEN, RED_IAM};
use ui_helpers::*;

impl App {
    pub fn chercher_dispo_modele(&mut self) {
        self.d_modele_periode.clear();
        let query = self.d_recherche_modele.trim().to_lowercase();
        if query.is_empty() {
            return;
        }
        let (Some(du), Some(au)) = (parse_date(&self.d_du), parse_date(&self.d_au)) else {
            return;
        };

        for v in &self.voitures {
            if v.modele.to_lowercase().contains(&query) || v.plaque.to_lowercase().contains(&query)
            {
                let stat = self.statut_voiture(v.id, du, au);
                let details = stat
                    .lignes
                    .iter()
                    .map(|(l, _)| l.clone())
                    .collect::<Vec<_>>()
                    .join(" ; ");
                self.d_modele_periode
                    .push((v.id, v.modele.clone(), stat.libelle, details));
            }
        }
    }
}

pub fn page_disponibilite(ui: &mut egui::Ui, app: &mut App) {
    titre_page(ui, "Recherche de Disponibilité globale");

    panneau().show(ui, |ui| {
        egui::Grid::new("dispo_gr")
            .num_columns(2)
            .spacing([10.0, 10.0])
            .show(ui, |ui| {
                etiquette(ui, "Date début (JJ/MM/AAAA) :");
                ui.text_edit_singleline(&mut app.d_du);
                ui.end_row();

                etiquette(ui, "Date fin (JJ/MM/AAAA) :");
                ui.text_edit_singleline(&mut app.d_au);
                ui.end_row();
            });

        ui.add_space(10.0);
        if ui
            .button(RichText::new("Vérifier Véhicules Disponibles").strong())
            .clicked()
        {
            app.d_erreur.clear();
            app.d_fait = false;
            match (parse_date(&app.d_du), parse_date(&app.d_au)) {
                (Some(du), Some(au)) if du <= au => {
                    app.d_ids = app
                        .voitures
                        .iter()
                        .filter(|v| app.est_disponible(v.id, du, au, None))
                        .map(|v| v.id)
                        .collect();
                    app.d_fait = true;
                }
                (Some(du), Some(au)) => {
                    app.d_erreur = format!(
                        "Erreur: la date de début ({}) est après la fin ({}).",
                        du.format("%d/%m/%Y"),
                        au.format("%d/%m/%Y")
                    )
                }
                _ => {
                    app.d_erreur =
                        "Format de date incorrect. Utilisez JJ/MM/AAAA ou AAAA-MM-DD.".into()
                }
            }
        }
    });

    if !app.d_erreur.is_empty() {
        ui.add_space(10.0);
        ui.label(RichText::new(&app.d_erreur).color(RED_IAM));
    }

    if app.d_fait {
        ui.add_space(15.0);
        ui.label(
            RichText::new(format!(
                "🏁 {} véhicules totalement libres sur cette période :",
                app.d_ids.len()
            ))
            .size(15.0)
            .strong()
            .color(GREEN),
        );
        ui.add_space(5.0);
        egui::ScrollArea::vertical()
            .max_height(200.0)
            .show(ui, |ui| {
                for vid in &app.d_ids {
                    if let Some(v) = app.voiture_par_id(*vid) {
                        ui.horizontal(|ui| {
                            ui.label(format!(
                                "• {} ({}) - Catégorie: {} | Tarif: {:.0} DA/j",
                                v.modele, v.plaque, v.categorie, v.tarif_jour
                            ));
                            if ui.small_button("Historique & Fiche").clicked() {
                                app.modal_voiture.voiture_id = v.id;
                                app.modal_voiture.visible = true;
                            }
                        });
                    }
                }
            });
    }

    ui.add_space(20.0);
    ui.separator();
    ui.add_space(10.0);
    ui.label(
        RichText::new("🔍 Inspection Chronologique par Modèle")
            .size(15.0)
            .strong(),
    );
    ui.add_space(5.0);

    ui.horizontal(|ui| {
        ui.label("Saisir un modèle / plaque :");
        if ui
            .text_edit_singleline(&mut app.d_recherche_modele)
            .changed()
        {
            app.chercher_dispo_modele();
        }
    });

    if !app.d_modele_periode.is_empty() {
        ui.add_space(10.0);
        egui::ScrollArea::vertical()
            .max_height(250.0)
            .show(ui, |ui| {
                for (id, modl, status_lbl, details) in &app.d_modele_periode {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(format!("[{}] {}", status_lbl, modl)).bold());
                        ui.label(RichText::new(details).color(muted()).size(12.0));
                        if ui.small_button("Fiche").clicked() {
                            app.modal_voiture.voiture_id = *id;
                            app.modal_voiture.visible = true;
                        }
                    });
                }
            });
    }
}
