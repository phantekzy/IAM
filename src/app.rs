use chrono::{Datelike, Duration, Local, NaiveDate};
use eframe::egui::{self, Color32};
use models::*;
use std::collections::HashSet;
use storage::*;
use theme::{bg, muted, set_theme, AMBER, GREEN, RED_IAM};
use ui::{
    contrats::page_contrats, disponibilite::page_disponibilite, maintenance::page_maintenance,
    modals::render_modals, nouveau_contrat::page_nouveau_contrat, sidebar::render_sidebar,
    tableau::page_tableau, ventes::page_ventes, voitures::page_voitures,
};
use ui_helpers::*;

pub struct App {
    pub onglet: Onglet,
    pub onglet_prec: Option<Onglet>,
    pub voitures: Vec<Voiture>,
    pub contrats: Vec<Contrat>,
    pub f_numero: String,
    pub f_voiture: usize,
    pub f_voiture_note: String,
    pub f_voiture_search: String,
    pub f_client: String,
    pub f_tel: String,
    pub f_agent: String,
    pub f_debut: String,
    pub f_fin: String,
    pub f_km_dep: String,
    pub f_km_ret: String,
    pub f_tarif: String,
    pub f_notes: String,
    pub f_montant_paye: String,
    pub f_msg: String,
    pub f_ok: bool,
    pub f_en_edition: Option<usize>,
    pub f_car_error_cleared: bool,
    pub d_du: String,
    pub d_au: String,
    pub d_ids: Vec<u64>,
    pub d_fait: bool,
    pub d_erreur: String,
    pub d_recherche_modele: String,
    pub d_modele_periode: Vec<(u64, String, String, String)>,
    pub c_recherche: String,
    pub c_statut: usize,
    pub c_suppr: Option<usize>,
    pub c_page: usize,
    pub c_filter_mois: usize,
    pub c_filter_annee: String,
    pub v_modele: String,
    pub v_plaque: String,
    pub v_annee: String,
    pub v_cat: String,
    pub v_couleur: String,
    pub v_tarif: String,
    pub v_msg: String,
    pub v_ok: bool,
    pub v_recherche: String,
    pub v_page: usize,
    pub v_edit_id: Option<u64>,
    pub v_edit_modele: String,
    pub v_edit_plaque: String,
    pub v_edit_annee: String,
    pub v_edit_cat: String,
    pub v_edit_couleur: String,
    pub v_edit_tarif: String,
    pub v_edit_etat: String,
    pub v_suppr_confirm: Option<u64>,
    pub ventes: Vec<VoitureVente>,
    pub vt_nom: String,
    pub vt_plaque: String,
    pub vt_chassis: String,
    pub vt_immat: String,
    pub vt_annee: String,
    pub vt_prix_achat: String,
    pub vt_prix_demande: String,
    pub vt_notes: String,
    pub vt_msg: String,
    pub vt_ok: bool,
    pub vt_recherche: String,
    pub vt_page: usize,
    pub vt_suppr_confirm: Option<u64>,
    pub vt_marquer_vendu: Option<u64>,
    pub vt_prix_vendu_input: String,
    pub modal_voiture: ModalVoiture,
    pub modal_contrat: ModalContrat,
    pub tb_filter: usize,
    pub tb_page: usize,
    pub dark_mode: bool,
    pub alerte_retour_ok: HashSet<u64>,
    pub tb_date_recherche: String,
    pub reparations: Vec<Reparation>,
    pub rep_voiture_idx: usize,
    pub rep_date: String,
    pub rep_description: String,
    pub rep_prix: String,
    pub rep_observation: String,
    pub rep_msg: String,
    pub rep_ok: bool,
    pub rep_filtre_voiture: usize,
    pub rep_recherche: String,
    pub rep_suppr_confirm: Option<u64>,
}

impl App {
    pub fn new() -> Self {
        let mut a = Self {
            onglet: Onglet::Tableau,
            onglet_prec: None,
            voitures: charger(&cars_file()),
            contrats: charger(&rentals_file()),
            f_numero: String::new(),
            f_voiture: 0,
            f_voiture_note: String::new(),
            f_voiture_search: String::new(),
            f_client: String::new(),
            f_tel: String::new(),
            f_agent: String::new(),
            f_debut: aujourd_hui(),
            f_fin: dans_7j(),
            f_km_dep: "0".into(),
            f_km_ret: "0".into(),
            f_tarif: String::new(),
            f_notes: String::new(),
            f_montant_paye: "0".into(),
            f_msg: String::new(),
            f_ok: false,
            f_en_edition: None,
            f_car_error_cleared: false,
            d_du: aujourd_hui(),
            d_au: dans_7j(),
            d_ids: vec![],
            d_fait: false,
            d_erreur: String::new(),
            d_recherche_modele: String::new(),
            d_modele_periode: vec![],
            c_recherche: String::new(),
            c_statut: 0,
            c_suppr: None,
            c_page: 0,
            c_filter_mois: 0,
            c_filter_annee: String::new(),
            v_modele: String::new(),
            v_plaque: String::new(),
            v_annee: Local::now().year().to_string(),
            v_cat: "Berline".into(),
            v_couleur: String::new(),
            v_tarif: "3000".into(),
            v_msg: String::new(),
            v_ok: false,
            v_recherche: String::new(),
            v_page: 0,
            v_edit_id: None,
            v_edit_modele: String::new(),
            v_edit_plaque: String::new(),
            v_edit_annee: String::new(),
            v_edit_cat: String::new(),
            v_edit_couleur: String::new(),
            v_edit_tarif: String::new(),
            v_edit_etat: String::new(),
            v_suppr_confirm: None,
            ventes: charger(&ventes_file()),
            vt_nom: String::new(),
            vt_plaque: String::new(),
            vt_chassis: String::new(),
            vt_immat: String::new(),
            vt_annee: Local::now().year().to_string(),
            vt_prix_achat: String::new(),
            vt_prix_demande: String::new(),
            vt_notes: String::new(),
            vt_msg: String::new(),
            vt_ok: false,
            vt_recherche: String::new(),
            vt_page: 0,
            vt_suppr_confirm: None,
            vt_marquer_vendu: None,
            vt_prix_vendu_input: String::new(),
            modal_voiture: ModalVoiture::default(),
            modal_contrat: ModalContrat::default(),
            tb_filter: 0,
            tb_page: 0,
            dark_mode: false,
            alerte_retour_ok: HashSet::new(),
            tb_date_recherche: String::new(),
            reparations: charger(&reparations_file()),
            rep_voiture_idx: 0,
            rep_date: aujourd_hui(),
            rep_description: String::new(),
            rep_prix: String::new(),
            rep_observation: String::new(),
            rep_msg: String::new(),
            rep_ok: false,
            rep_filtre_voiture: 0,
            rep_recherche: String::new(),
            rep_suppr_confirm: None,
        };
        if a.voitures.is_empty() {
            a.voitures = voitures_defaut();
            sauvegarder(&cars_file(), &a.voitures);
        }
        for v in &mut a.voitures {
            if v.etat.is_empty() {
                v.etat = "Bon état".to_string();
            }
        }
        a
    }

    pub fn prochain_id_contrat(&self) -> u64 {
        self.contrats.iter().map(|c| c.id).max().unwrap_or(0) + 1
    }
    pub fn prochain_id_voiture(&self) -> u64 {
        self.voitures.iter().map(|v| v.id).max().unwrap_or(0) + 1
    }
    pub fn numero_suggere(&self) -> String {
        format!(
            "IAM-{}-{:04}",
            Local::now().year(),
            self.prochain_id_contrat()
        )
    }
    pub fn est_disponible(
        &self,
        vid: u64,
        du: NaiveDate,
        au: NaiveDate,
        exclude_contrat_id: Option<u64>,
    ) -> bool {
        !self.contrats.iter().any(|c| {
            if let Some(excl) = exclude_contrat_id {
                if c.id == excl {
                    return false;
                }
            }
            c.voiture_id == vid && c.chevauche(du, au)
        })
    }
    pub fn loues_auj(&self) -> usize {
        let t = Local::now().date_naive();
        self.contrats
            .iter()
            .filter(|c| c.statut == "Actif" && c.chevauche(t, t))
            .count()
    }
    pub fn retours_demain(&self) -> Vec<Contrat> {
        let demain = Local::now().date_naive() + Duration::days(1);
        self.contrats
            .iter()
            .filter(|c| c.statut == "Actif")
            .filter(|c| parse_date(&c.date_fin) == Some(demain))
            .cloned()
            .collect()
    }
    pub fn departs_demain(&self) -> Vec<Contrat> {
        let demain = Local::now().date_naive() + Duration::days(1);
        self.contrats
            .iter()
            .filter(|c| c.statut == "Actif")
            .filter(|c| parse_date(&c.date_debut) == Some(demain))
            .cloned()
            .collect()
    }
    pub fn ca_mois(&self) -> f64 {
        let n = Local::now().date_naive();
        self.contrats
            .iter()
            .filter(|c| c.statut != "Annulé")
            .filter(|c| {
                parse_date(&c.date_debut)
                    .map(|d| d.year() == n.year() && d.month() == n.month())
                    .unwrap_or(false)
            })
            .map(|c| c.total())
            .sum()
    }
    pub fn ca_total(&self) -> f64 {
        self.contrats
            .iter()
            .filter(|c| c.statut != "Annulé")
            .map(|c| c.total())
            .sum()
    }
    pub fn naviguer(&mut self, dest: Onglet) {
        if self.onglet != dest {
            self.onglet_prec = Some(self.onglet);
            self.onglet = dest;
        }
    }
    pub fn nav_menu(&mut self, dest: Onglet) {
        self.onglet = dest;
        self.onglet_prec = None;
    }
    pub fn vider_formulaire(&mut self) {
        self.f_numero.clear();
        self.f_voiture = 0;
        self.f_voiture_note.clear();
        self.f_voiture_search.clear();
        self.f_client.clear();
        self.f_tel.clear();
        self.f_debut = aujourd_hui();
        self.f_fin = dans_7j();
        self.f_km_dep = "0".into();
        self.f_km_ret = "0".into();
        self.f_tarif.clear();
        self.f_notes.clear();
        self.f_montant_paye = "0".into();
        self.f_msg.clear();
        self.f_ok = false;
        self.f_en_edition = None;
        self.f_car_error_cleared = false;
    }
    pub fn contrat_par_id(&self, id: u64) -> Option<&Contrat> {
        self.contrats.iter().find(|c| c.id == id)
    }
    pub fn voiture_par_id(&self, id: u64) -> Option<&Voiture> {
        self.voitures.iter().find(|v| v.id == id)
    }
    pub fn contrat_actif_pour_voiture(&self, vid: u64) -> Option<&Contrat> {
        let auj = Local::now().date_naive();
        self.contrats
            .iter()
            .find(|c| c.voiture_id == vid && c.statut == "Actif" && c.chevauche(auj, auj))
    }
    pub fn contrats_voiture(&self, vid: u64) -> Vec<Contrat> {
        let mut contrats: Vec<Contrat> = self
            .contrats
            .iter()
            .filter(|c| c.voiture_id == vid && c.statut != "Annulé")
            .cloned()
            .collect();
        contrats.sort_by(|a, b| {
            let da = parse_date(&a.date_debut).unwrap_or_else(|| Local::now().date_naive());
            let db = parse_date(&b.date_debut).unwrap_or_else(|| Local::now().date_naive());
            db.cmp(&da)
        });
        contrats
    }
    pub fn contrat_resume(c: &Contrat, prefix: &str) -> String {
        format!(
            "{} {} | {} au {} | {} | reste {:.0} DA",
            prefix,
            c.numero,
            afficher_date(&c.date_debut),
            afficher_date(&c.date_fin),
            c.client_nom,
            c.reste_a_payer()
        )
    }
    pub fn statut_voiture(&self, vid: u64, du: NaiveDate, au: NaiveDate) -> StatutVoiture {
        let Some(voiture) = self.voiture_par_id(vid) else {
            return StatutVoiture {
                couleur: muted(),
                libelle: "Inconnue".into(),
                badges: vec![],
                lignes: vec![],
            };
        };
        let contrats = self.contrats_voiture(vid);
        let current = contrats.iter().find(|c| c.chevauche(du, au)).cloned();
        let future = contrats
            .iter()
            .filter_map(|c| parse_date(&c.date_debut).map(|d| (d, c.clone())))
            .filter(|(d, _)| *d > au)
            .min_by_key(|(d, _)| *d)
            .map(|(_, c)| c);
        let last = contrats
            .iter()
            .filter_map(|c| parse_date(&c.date_fin).map(|d| (d, c.clone())))
            .filter(|(d, _)| *d < du)
            .max_by_key(|(d, _)| *d)
            .map(|(_, c)| c);
        let maintenance = voiture.etat == "En maintenance";
        let unpaid_non_future = contrats
            .iter()
            .filter(|c| c.reste_a_payer() > 0.0)
            .filter(|c| parse_date(&c.date_debut).map(|d| d <= au).unwrap_or(false))
            .max_by_key(|c| parse_date(&c.date_fin).unwrap_or_else(|| Local::now().date_naive()))
            .cloned();

        let (couleur, libelle) = if current.is_some() {
            (RED_IAM, "En période")
        } else if future.is_some() {
            (Color32::from_rgb(99, 102, 241), "Réservé")
        } else if unpaid_non_future.is_some() {
            (AMBER, "Impayé")
        } else if maintenance {
            (AMBER, "En maintenance")
        } else if last.is_some() {
            (muted(), "Terminé")
        } else {
            (GREEN, "Disponible")
        };

        let mut badges = Vec::new();
        if let Some(c) = &current {
            badges.push((format!("Actuel: {}", c.numero), RED_IAM));
        }
        if let Some(c) = &future {
            badges.push((
                format!("Futur: {}", c.numero),
                Color32::from_rgb(99, 102, 241),
            ));
        }
        if let Some(c) = &unpaid_non_future {
            badges.push((format!("Reste: {:.0} DA", c.reste_a_payer()), AMBER));
        }
        if maintenance {
            badges.push(("Maintenance".into(), AMBER));
        }
        if let Some(c) = &last {
            badges.push((format!("Terminé: {}", c.numero), muted()));
        }

        let mut lignes = Vec::new();
        if let Some(c) = &current {
            lignes.push((Self::contrat_resume(c, "Période"), RED_IAM));
        }
        if let Some(c) = &future {
            lignes.push((
                Self::contrat_resume(c, "Futur"),
                Color32::from_rgb(99, 102, 241),
            ));
        }
        if let Some(c) = &unpaid_non_future {
            if !lignes.iter().any(|(t, _)| t.contains(&c.numero)) {
                lignes.push((Self::contrat_resume(c, "Solde"), AMBER));
            }
        }
        if let Some(c) = &last {
            if current.is_none() && future.is_none() {
                lignes.push((Self::contrat_resume(c, "Terminé"), muted()));
            }
        }
        if lignes.is_empty() {
            lignes.push((format!("{} est disponible", voiture.modele), GREEN));
        }

        StatutVoiture {
            couleur,
            libelle: libelle.into(),
            badges,
            lignes,
        }
    }
    pub fn rafraichir_recherche(&mut self) {
        self.voitures = charger(&cars_file());
        if self.voitures.is_empty() {
            self.voitures = voitures_defaut();
            sauvegarder(&cars_file(), &self.voitures);
        }
        for v in &mut self.voitures {
            if v.etat.trim().is_empty() {
                v.etat = "Bon état".to_string();
            }
        }
        self.contrats = charger(&rentals_file());
        self.d_ids.clear();
        self.d_fait = false;
        self.d_modele_periode.clear();
        self.d_erreur.clear();
        match (parse_date(&self.d_du), parse_date(&self.d_au)) {
            (Some(du), Some(au)) if du <= au => {
                self.d_ids = self
                    .voitures
                    .iter()
                    .filter(|v| self.est_disponible(v.id, du, au, None))
                    .map(|v| v.id)
                    .collect();
                self.d_fait = true;
            }
            _ => {}
        }
        if !self.d_recherche_modele.trim().is_empty() {
            self.chercher_dispo_modele();
        }
    }
}

pub fn voitures_defaut() -> Vec<Voiture> {
    vec![
        Voiture {
            id: 1,
            modele: "Toyota Corolla 2022".into(),
            plaque: "100-IAM-01".into(),
            categorie: "Berline".into(),
            couleur: "Blanc".into(),
            annee: 2022,
            tarif_jour: 3500.0,
            etat: "Bon état".into(),
        },
        Voiture {
            id: 2,
            modele: "Renault Clio 2021".into(),
            plaque: "100-IAM-02".into(),
            categorie: "Citadine".into(),
            couleur: "Bleu".into(),
            annee: 2021,
            tarif_jour: 2800.0,
            etat: "Bon état".into(),
        },
        Voiture {
            id: 3,
            modele: "Dacia Duster 2023".into(),
            plaque: "100-IAM-03".into(),
            categorie: "SUV".into(),
            couleur: "Gris".into(),
            annee: 2023,
            tarif_jour: 4500.0,
            etat: "Bon état".into(),
        },
    ]
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        set_theme(self.dark_mode);
        egui::CentralPanel::default().fill(bg()).show(ctx, |ui| {
            ui.horizontal(|ui| {
                render_sidebar(ui, self);
                ui.vertical(|ui| {
                    ui.add_space(10.0);
                    match self.onglet {
                        Onglet::Tableau => page_tableau(ui, self),
                        Onglet::Disponibilite => page_disponibilite(ui, self),
                        Onglet::NouveauContrat => page_nouveau_contrat(ui, self),
                        Onglet::Contrats => page_contrats(ui, self),
                        Onglet::Voitures => page_voitures(ui, self),
                        Onglet::Ventes => page_ventes(ui, self),
                        Onglet::Maintenance => page_maintenance(ui, self),
                    }
                });
            });
            render_modals(ctx, self);
        });
    }
}
