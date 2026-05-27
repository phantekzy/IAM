use app::App;
use eframe::egui::{self, Align2, RichText};
use theme::AMBER;
use ui_helpers::afficher_date;

pub fn render_modals(ctx: &egui::Context, app: &mut App) {
    if app.modal_voiture.visible {
        let mut open = true;
        egui::Window::new(format!(
            "Fiche Technique Véhicule ID: {}",
            app.modal_voiture.voiture_id
        ))
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
                        ui.label(format!(
                            "• Contrat {} - Client: {} | {} au {}",
                            c.numero,
                            c.client_nom,
                            afficher_date(&c.date_debut),
                            afficher_date(&c.date_fin)
                        ));
                    }
                }
            } else {
                ui.label("Véhicule introuvable.");
            }
        });
        if !open {
            app.modal_voiture.visible = false;
        }
    }

    if app.modal_contrat.visible {
        let mut open = true;
        egui::Window::new(format!(
            "Fiche Détaillée Contrat ID: {}",
            app.modal_contrat.contrat_id
        ))
        .open(&mut open)
        .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
        .resizable(true)
        .default_width(450.0)
        .show(ctx, |ui| {
            if let Some(c) = app.contrat_par_id(app.modal_contrat.contrat_id) {
                ui.heading(format!("Contrat référence : {}", c.numero));
                ui.label(format!("Statut actuel : {}", c.statut));
                ui.separator();
                ui.label(format!(
                    "Client : {} (Téléphone: {})",
                    c.client_nom, c.client_tel
                ));
                ui.label(format!(
                    "Véhicule affecté : {} ({})",
                    c.voiture_modele, c.voiture_plaque
                ));
                ui.label(format!("Agent émetteur : {}", c.agent));
                ui.label(format!("Date Début : {}", afficher_date(&c.date_debut)));
                ui.label(format!("Date Fin : {}", afficher_date(&c.date_fin)));
                ui.label(format!("Kilométrage initial : {:.0} km", c.km_depart));
                ui.label(format!("Kilométrage retour : {:.0} km", c.km_retour));
                ui.label(format!("Tarif appliqué : {:.0} DA/jour", c.tarif_jour));
                ui.label(format!("Coût total de la location : {:.0} DA", c.total()));
                ui.label(format!("Montant encaissé : {:.0} DA", c.montant_paye));
                ui.label(
                    RichText::new(format!("Reste dû / Solde : {:.0} DA", c.reste_a_payer()))
                        .color(AMBER)
                        .strong(),
                );
                if !c.notes.is_empty() {
                    ui.label(format!("Notes confidentielles : {}", c.notes));
                }
            } else {
                ui.label("Contrat introuvable.");
            }
        });
        if !open {
            app.modal_contrat.visible = false;
        }
    }
}
