use chrono::{Datelike, Local, NaiveDate};
use eframe::egui::{self, Align2, Color32, Margin, RichText, Rounding, Stroke, Vec2};
use crate::models::*;
use crate::storage::*;
use crate::theme::*;
use crate::utils::*;
use crate::app::widgets::*;
use crate::app::App;

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Set theme colors for this frame (must be before any rendering)
        set_theme(self.dark_mode);

        if self.dark_mode {
            let mut v = egui::Visuals::dark();
            v.panel_fill = Color32::from_rgb(12, 12, 15);
            v.window_fill = Color32::from_rgb(12, 12, 15);
            v.extreme_bg_color = Color32::from_rgb(8, 8, 10);
            v.faint_bg_color = Color32::from_rgb(20, 20, 28);
            v.widgets.noninteractive.bg_fill = Color32::from_rgb(35, 35, 45);
            v.widgets.noninteractive.fg_stroke = Stroke::new(1.0, Color32::from_rgb(240, 240, 245));
            v.widgets.inactive.bg_fill = Color32::from_rgb(35, 35, 45);
            v.widgets.inactive.fg_stroke = Stroke::new(1.0, Color32::from_rgb(240, 240, 245));
            v.widgets.hovered.bg_fill = Color32::from_rgb(50, 50, 65);
            v.widgets.hovered.fg_stroke = Stroke::new(1.0, Color32::from_rgb(240, 240, 245));
            v.widgets.active.bg_fill = RED_IAM;
            v.widgets.active.fg_stroke = Stroke::new(1.0, WHITE);
            v.override_text_color = Some(Color32::from_rgb(240, 240, 245));
            v.selection.bg_fill = Color32::from_rgb(200, 30, 40).linear_multiply(0.4);
            ctx.set_visuals(v);
        } else {
            let mut v = egui::Visuals::light();
            v.panel_fill = Color32::from_rgb(242, 243, 247);
            v.window_fill = Color32::WHITE;
            v.extreme_bg_color = Color32::from_rgb(225, 226, 232);
            v.faint_bg_color = Color32::from_rgb(235, 236, 242);
            v.widgets.noninteractive.bg_fill = Color32::from_rgb(235, 236, 242);
            v.widgets.noninteractive.fg_stroke = Stroke::new(1.0, Color32::from_rgb(20, 20, 30));
            v.widgets.inactive.bg_fill = Color32::from_rgb(220, 222, 230);
            v.widgets.inactive.fg_stroke = Stroke::new(1.0, Color32::from_rgb(20, 20, 30));
            v.widgets.hovered.bg_fill = Color32::from_rgb(205, 207, 218);
            v.widgets.hovered.fg_stroke = Stroke::new(1.0, Color32::from_rgb(20, 20, 30));
            v.widgets.active.bg_fill = RED_IAM;
            v.widgets.active.fg_stroke = Stroke::new(1.0, WHITE);
            v.override_text_color = Some(Color32::from_rgb(20, 20, 30));
            v.selection.bg_fill = Color32::from_rgb(200, 30, 40).linear_multiply(0.3);
            ctx.set_visuals(v);
        }
        let mut style = (*ctx.style()).clone();
        style.spacing.item_spacing = Vec2::new(8.0, 8.0);
        style.spacing.button_padding = Vec2::new(12.0, 6.0);
        ctx.set_style(style);

        // ── MODAL: Fiche Voiture ──
        if self.modal_voiture.visible {
            let vid = self.modal_voiture.voiture_id;
            let voit = self.voitures.iter().find(|v| v.id == vid).cloned();
            let mut close = false;
            if let Some(v) = voit {
                let auj = Local::now().date_naive();
                let statut = self.statut_voiture(v.id, auj, auj);
                let contrats = self.contrats_voiture(v.id);

                egui::Window::new(format!("Voiture – {}", v.modele))
                    .collapsible(false)
                    .resizable(false)
                    .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
                    .fixed_size([560.0, 600.0])
                    .show(ctx, |ui| {
                        // ── Scrollable body ──────────────────────────────
                        let body_height = ui.available_height() - 48.0;
                        egui::ScrollArea::vertical()
                            .max_height(body_height)
                            .auto_shrink([false, false])
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    logo_widget(ui, true);
                                    ui.add_space(10.0);
                                    ui.vertical(|ui| {
                                        ui.label(
                                            RichText::new(&v.modele)
                                                .size(18.0)
                                                .color(text())
                                                .strong(),
                                        );
                                        ui.label(
                                            RichText::new(&v.plaque).size(14.0).color(RED_IAM),
                                        );
                                    });
                                });
                                ui.add_space(10.0);
                                ui.separator();
                                ui.add_space(8.0);

                                egui::Grid::new("mv_info")
                                    .num_columns(2)
                                    .spacing([16.0, 6.0])
                                    .show(ui, |ui| {
                                        for (k, val) in [
                                            ("Catégorie", v.categorie.as_str()),
                                            ("Couleur", v.couleur.as_str()),
                                            ("Année", &v.annee.to_string()),
                                            ("Tarif / jour", &format!("{:.0} DA", v.tarif_jour)),
                                            (
                                                "État",
                                                if v.etat.is_empty() {
                                                    "Bon état"
                                                } else {
                                                    v.etat.as_str()
                                                },
                                            ),
                                        ] {
                                            ui.label(RichText::new(k).size(12.0).color(muted()));
                                            ui.label(RichText::new(val).size(13.0).color(text()));
                                            ui.end_row();
                                        }
                                    });

                                ui.add_space(10.0);
                                ui.separator();
                                ui.add_space(6.0);

                                ui.horizontal(|ui| {
                                    point_couleur(ui, statut.couleur);
                                    ui.label(
                                        RichText::new(&statut.libelle)
                                            .size(14.0)
                                            .color(statut.couleur)
                                            .strong(),
                                    );
                                });

                                if !statut.badges.is_empty() {
                                    ui.add_space(8.0);
                                    ui.horizontal_wrapped(|ui| {
                                        for (txt, col) in &statut.badges {
                                            egui::Frame {
                                                fill: card2(),
                                                inner_margin: Margin::symmetric(8.0, 4.0),
                                                rounding: Rounding::same(6.0),
                                                stroke: Stroke::new(1.0, *col),
                                                ..Default::default()
                                            }
                                            .show(
                                                ui,
                                                |ui| {
                                                    ui.label(
                                                        RichText::new(txt)
                                                            .size(11.5)
                                                            .color(*col)
                                                            .strong(),
                                                    );
                                                },
                                            );
                                        }
                                    });
                                }

                                // ── Historique / contrats liés ───────────
                                if !contrats.is_empty() {
                                    ui.add_space(10.0);
                                    ui.label(
                                        RichText::new("Historique / contrats liés")
                                            .size(13.0)
                                            .color(RED_IAM)
                                            .strong(),
                                    );
                                    ui.add_space(6.0);

                                    // Search bar
                                    ui.horizontal(|ui| {
                                        ui.label(RichText::new("🔍").size(12.0).color(muted()));
                                        ui.add_space(4.0);
                                        ui.add(
                                            egui::TextEdit::singleline(
                                                &mut self.modal_voiture.histo_recherche,
                                            )
                                            .hint_text("Rechercher un contrat…")
                                            .desired_width(f32::INFINITY)
                                            .font(egui::TextStyle::Small),
                                        );
                                    });
                                    ui.add_space(6.0);

                                    let filtre = self.modal_voiture.histo_recherche.to_lowercase();
                                    let contrats_filtres: Vec<_> = contrats
                                        .iter()
                                        .filter(|c| {
                                            filtre.is_empty()
                                                || c.numero.to_lowercase().contains(&filtre)
                                                || c.client_nom.to_lowercase().contains(&filtre)
                                                || c.client_tel.to_lowercase().contains(&filtre)
                                                || c.agent.to_lowercase().contains(&filtre)
                                        })
                                        .collect();

                                    let mut to_delete: Option<u64> = None;
                                    for c in &contrats_filtres {
                                        let (col, titre) = if c.chevauche(auj, auj) {
                                            (RED_IAM, "En période")
                                        } else if parse_date(&c.date_debut)
                                            .map(|d| d > auj)
                                            .unwrap_or(false)
                                        {
                                            (Color32::from_rgb(99, 102, 241), "Futur")
                                        } else if c.reste_a_payer() > 0.0 {
                                            (AMBER, "Impayé")
                                        } else if c.statut == "Terminé" {
                                            (muted(), "Terminé")
                                        } else {
                                            (GREEN, "Contrat")
                                        };

                                        egui::Frame {
                                            fill: if col == RED_IAM {
                                                tinted_card(RED_IAM, 60)
                                            } else if col == AMBER {
                                                tinted_card(AMBER, 60)
                                            } else if col == GREEN {
                                                tinted_card(GREEN, 55)
                                            } else {
                                                tinted_card(Color32::from_rgb(99, 102, 241), 40)
                                            },
                                            inner_margin: Margin::same(10.0),
                                            rounding: Rounding::same(6.0),
                                            stroke: Stroke::new(1.0, col),
                                            ..Default::default()
                                        }
                                        .show(ui, |ui| {
                                            ui.horizontal(|ui| {
                                                point_couleur(ui, col);
                                                ui.label(
                                                    RichText::new(titre)
                                                        .size(12.5)
                                                        .color(col)
                                                        .strong(),
                                                );
                                                ui.add_space(8.0);
                                                ui.label(
                                                    RichText::new(&c.numero)
                                                        .size(12.0)
                                                        .color(text()),
                                                );
                                                ui.with_layout(
                                                    egui::Layout::right_to_left(
                                                        egui::Align::Center,
                                                    ),
                                                    |ui| {
                                                        if ui
                                                            .add(
                                                                egui::Button::new(
                                                                    RichText::new("🗑")
                                                                        .size(12.0)
                                                                        .color(RED_IAM),
                                                                )
                                                                .fill(Color32::TRANSPARENT)
                                                                .rounding(4.0),
                                                            )
                                                            .on_hover_text("Supprimer ce contrat")
                                                            .clicked()
                                                        {
                                                            to_delete = Some(c.id);
                                                        }
                                                    },
                                                );
                                            });
                                            ui.add_space(4.0);
                                            egui::Grid::new(format!("mv_contrat_{}", c.id))
                                                .num_columns(2)
                                                .spacing([12.0, 4.0])
                                                .show(ui, |ui| {
                                                    for (k, val) in [
                                                        ("Client", c.client_nom.as_str()),
                                                        ("Téléphone", c.client_tel.as_str()),
                                                        ("Agent", c.agent.as_str()),
                                                        ("Début", &afficher_date(&c.date_debut)),
                                                        ("Fin", &afficher_date(&c.date_fin)),
                                                        (
                                                            "Tarif",
                                                            &format!("{:.0} DA/j", c.tarif_jour),
                                                        ),
                                                        ("Total", &format!("{:.0} DA", c.total())),
                                                        (
                                                            "Payé",
                                                            &format!("{:.0} DA", c.montant_paye),
                                                        ),
                                                        (
                                                            "Reste",
                                                            &format!("{:.0} DA", c.reste_a_payer()),
                                                        ),
                                                        ("Notes", c.notes.as_str()),
                                                    ] {
                                                        ui.label(
                                                            RichText::new(k)
                                                                .size(11.5)
                                                                .color(muted()),
                                                        );
                                                        ui.label(
                                                            RichText::new(val)
                                                                .size(12.0)
                                                                .color(text()),
                                                        );
                                                        ui.end_row();
                                                    }
                                                });
                                        });
                                        ui.add_space(6.0);
                                    }
                                    if let Some(del_id) = to_delete {
                                        self.contrats.retain(|c| c.id != del_id);
                                        sauvegarder(&rentals_file(), &self.contrats);
                                    }
                                    if contrats_filtres.is_empty() && !filtre.is_empty() {
                                        ui.label(
                                            RichText::new("Aucun contrat trouvé.")
                                                .size(12.0)
                                                .color(muted()),
                                        );
                                    }
                                }
                            }); // end ScrollArea

                        // ── Fixed bottom bar ─────────────────────────────
                        ui.separator();
                        ui.add_space(6.0);
                        ui.horizontal(|ui| {
                            if bouton_neutre(ui, "Fermer", 120.0) {
                                close = true;
                            }
                        });
                        ui.add_space(6.0);
                    });
            } else {
                close = true;
            }
            if close {
                self.modal_voiture.visible = false;
            }
        }

        // ── MODAL: Détail Contrat ──
        if self.modal_contrat.visible {
            let cid = self.modal_contrat.contrat_id;
            let contrat = self.contrats.iter().find(|c| c.id == cid).cloned();
            let mut close = false;
            if let Some(c) = contrat {
                let today_modal = Local::now().date_naive();
                let is_future_c = NaiveDate::parse_from_str(&c.date_debut, "%Y-%m-%d")
                    .map(|d| d > today_modal)
                    .unwrap_or(false);
                let (sc, ss) = match c.statut.as_str() {
                    "Actif" if is_future_c => (Color32::from_rgb(99, 102, 241), "Réservé"),
                    "Actif" => (GREEN, "Actif"),
                    "Terminé" => (muted(), "Terminé"),
                    "Annulé" => (RED_IAM, "Annulé"),
                    _ => (muted(), c.statut.as_str()),
                };
                egui::Window::new(format!("Contrat – {}", c.numero))
                    .collapsible(false)
                    .resizable(false)
                    .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
                    .fixed_size([500.0, 560.0])
                    .show(ctx, |ui| {
                        // ── Scrollable body ──────────────────────────────
                        let body_height = ui.available_height() - 48.0;
                        egui::ScrollArea::vertical()
                            .max_height(body_height)
                            .auto_shrink([false, false])
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    ui.label(
                                        RichText::new(&c.numero).size(18.0).color(RED_IAM).strong(),
                                    );
                                    ui.add_space(12.0);
                                    ui.label(RichText::new(ss).size(14.0).color(sc).strong());
                                });
                                ui.add_space(8.0);
                                ui.separator();
                                ui.add_space(8.0);
                                egui::Grid::new("mc_info")
                                    .num_columns(2)
                                    .spacing([16.0, 6.0])
                                    .show(ui, |ui| {
                                        for (k, val) in [
                                            ("Véhicule", c.voiture_modele.as_str()),
                                            ("Plaque", c.voiture_plaque.as_str()),
                                            ("Client", c.client_nom.as_str()),
                                            ("Téléphone", c.client_tel.as_str()),
                                            ("Agent Commercial", c.agent.as_str()),
                                            (
                                                "Date Début",
                                                &if c.heure_debut.is_empty() {
                                                    afficher_date(&c.date_debut)
                                                } else {
                                                    format!(
                                                        "{} à {}",
                                                        afficher_date(&c.date_debut),
                                                        c.heure_debut
                                                    )
                                                },
                                            ),
                                            (
                                                "Date Fin",
                                                &if c.heure_fin.is_empty() {
                                                    afficher_date(&c.date_fin)
                                                } else {
                                                    format!(
                                                        "{} à {}",
                                                        afficher_date(&c.date_fin),
                                                        c.heure_fin
                                                    )
                                                },
                                            ),
                                            ("Durée", &format!("{} jours", c.jours())),
                                            ("Tarif / jour", &format!("{:.0} DA", c.tarif_jour)),
                                            ("Total", &format!("{:.0} DA", c.total())),
                                            ("Montant Payé", &format!("{:.0} DA", c.montant_paye)),
                                            (
                                                "Reste à payer",
                                                &format!("{:.0} DA", c.reste_a_payer()),
                                            ),
                                            ("KM Départ", &format!("{:.0}", c.km_depart)),
                                            ("KM Retour", &format!("{:.0}", c.km_retour)),
                                            ("Notes", c.notes.as_str()),
                                        ] {
                                            ui.label(RichText::new(k).size(12.0).color(muted()));
                                            ui.label(RichText::new(val).size(13.0).color(text()));
                                            ui.end_row();
                                        }
                                    });
                            }); // end ScrollArea

                        // ── Fixed bottom bar ─────────────────────────────
                        ui.separator();
                        ui.add_space(6.0);
                        ui.horizontal(|ui| {
                            if bouton_neutre(ui, "Fermer", 120.0) {
                                close = true;
                            }
                        });
                        ui.add_space(6.0);
                    });
            } else {
                close = true;
            }
            if close {
                self.modal_contrat.visible = false;
            }
        }

        egui::TopBottomPanel::top("nav")
            .exact_height(56.0)
            .frame(egui::Frame {
                fill: card(),
                stroke: Stroke::new(1.0, border()),
                ..Default::default()
            })
            .show(ctx, |ui| {
                ui.horizontal_centered(|ui| {
                    ui.add_space(16.0);
                    logo_widget(ui, false);
                    ui.add_space(8.0);
                    ui.label(
                        RichText::new("IAM Business")
                            .size(18.0)
                            .color(RED_IAM)
                            .strong(),
                    );
                    ui.add_space(20.0);
                    if self.onglet_prec.is_some() {
                        if ui
                            .add(
                                egui::Button::new(
                                    RichText::new("< Retour").size(13.0).color(RED_IAM),
                                )
                                .fill(Color32::TRANSPARENT)
                                .rounding(6.0),
                            )
                            .clicked()
                        {
                            self.onglet = self.onglet_prec.take().unwrap();
                        }
                        ui.separator();
                        ui.add_space(8.0);
                    }
                    for (lbl, ong) in [
                        ("Tableau", Onglet::Tableau),
                        ("Recherche", Onglet::Disponibilite),
                        ("Nouveau", Onglet::NouveauContrat),
                        ("Contrats", Onglet::Contrats),
                        ("Impayés", Onglet::Impayes),
                        ("Voitures", Onglet::Voitures),
                        ("Ventes", Onglet::Ventes),
                        ("Réparations", Onglet::Maintenance),
                    ] {
                        let actif = self.onglet == ong;
                        if ui
                            .add_sized(
                                [120.0, 36.0],
                                egui::Button::new(RichText::new(lbl).size(13.5).color(if actif {
                                    WHITE
                                } else {
                                    text()
                                }))
                                .fill(if actif { RED_IAM } else { Color32::TRANSPARENT })
                                .rounding(6.0),
                            )
                            .clicked()
                        {
                            self.nav_menu(ong);
                        }
                    }
                    // Theme toggle — right side
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.add_space(16.0);
                        let (theme_lbl, toggle_fill, toggle_text) = if self.dark_mode {
                            ("☀ Clair", Color32::from_rgb(60, 62, 80), WHITE)
                        } else {
                            (
                                "🌙 Sombre",
                                Color32::from_rgb(210, 212, 225),
                                Color32::from_rgb(20, 20, 30),
                            )
                        };
                        if ui
                            .add(
                                egui::Button::new(
                                    RichText::new(theme_lbl).size(12.5).color(toggle_text),
                                )
                                .fill(toggle_fill)
                                .rounding(6.0),
                            )
                            .clicked()
                        {
                            self.dark_mode = !self.dark_mode;
                        }
                    });
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    ui.set_max_width(1180.0);
                    ui.add_space(16.0);
                    match self.onglet {
                        Onglet::Tableau => self.page_tableau(ui),
                        Onglet::Disponibilite => self.page_dispo(ui),
                        Onglet::NouveauContrat => self.page_nouveau(ui),
                        Onglet::Contrats => self.page_contrats(ui),
                        Onglet::Voitures => self.page_voitures(ui),
                        Onglet::Ventes => self.page_ventes(ui),
                        Onglet::Maintenance => self.page_maintenance(ui),
                        Onglet::Impayes => self.page_impayes(ui),
                    }
                    ui.add_space(32.0);
                });
        });
    }
}

impl App {

