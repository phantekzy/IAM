use app::App;
use eframe::egui::{self, Align2, RichText};
use theme::AMBER;
use ui_helpers::afficher_date;

pub fn render_modals(ctx: &egui::Context, app: &mut App) {
    if app.modal_voiture.visible {
        let mut open = true;
        egui::Window::new(format!("Fiche Technique Véhicule ID: {}", app.modal_voiture.voiture_id))
            .open(&mut open)
            .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
            .resizable(true)
            .default_width(500.0)
            .show(ctx, |ui| {
                if let Some(v) = app.voiture_par_id(app.modal_voiture.voiture_id) {
                    ui.heading(&v.modele);
                    ui.label(format!("Immatriculation : {}", v.plaque));
                    ui.label(format!("Catégorie : {}", v.categorie));
                    ui.label(format!("Couleur : {}", v.couleur));
                    ui.label(format!("Année de construction : {}", v.annee));
                    ui.label(format!("Tarif standard : {:.0} DA/jour", v.tarif_jour));
                    ui.label(format!("État Actuel : {}", v.etat));

                    ui.add_space(10.0);
                    ui.separator();
                    ui.label(RichText::new("📜 Historique locatif lié :").strong());
                    let locs = app.contrats_voiture(v.id);
                    if locs.is_empty() {
                        ui.label("Aucun contrat passé enregistré.");
                    } else {
                        for c in locs {
                            ui.label(format!("• Contrat {} - Client: {} | {} au {}", c.numero, c.client_nom, afficher_date(&c.date_debut), afficher_date(&c.date_fin)));
                        }
                    }
                } else {
                    ui.label("Véhicule introuvable.");
                }
            });
        if !open { app.modal_voiture.visible = false; }
    }
