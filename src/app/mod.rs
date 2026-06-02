use chrono::{Datelike, Duration, Local, NaiveDate};
use eframe::egui::Color32;
use crate::models::*;
use crate::storage::*;
use crate::theme::*;
use crate::utils::*;

pub mod widgets;
pub mod update;
pub mod pages;

enum Onglet {
    Tableau,
    Disponibilite,
    NouveauContrat,
    Contrats,
    Voitures,
    Ventes,
    Maintenance,
    Impayes,
}

#[derive(Default)]
struct ModalVoiture {
    visible: bool,
    voiture_id: u64,
    histo_recherche: String,
}

#[derive(Default)]
struct ModalContrat {
    visible: bool,
    contrat_id: u64,
}

#[derive(Clone, Debug)]
struct StatutVoiture {
    couleur: Color32,
    libelle: String,
    badges: Vec<(String, Color32)>,
    lignes: Vec<(String, Color32)>,
}

struct App {
    onglet: Onglet,
    onglet_prec: Option<Onglet>,
    voitures: Vec<Voiture>,
    contrats: Vec<Contrat>,
    f_numero: String,
    f_voiture: usize,
    f_voiture_note: String,
    f_voiture_search: String, 
    f_client: String,
    f_tel: String,
    f_agent: String,
    f_debut: String,
    f_fin: String,
    f_heure_debut: String,
    f_heure_fin: String,
    f_km_dep: String,
    f_km_ret: String,
    f_tarif: String,
    f_notes: String,
    f_montant_paye: String,
    f_msg: String,
    f_ok: bool,
    f_en_edition: Option<usize>,
    f_car_error_cleared: bool, 
    d_du: String,
    d_au: String,
    d_ids: Vec<u64>,
    d_fait: bool,
    d_erreur: String,
    d_recherche_modele: String,
    d_modele_periode: Vec<(u64, String, String, String)>, 
    c_recherche: String,
    c_statut: usize,
    c_suppr: Option<usize>,
    c_page: usize,
    c_filter_mois: usize, 
    c_filter_annee: String,
    v_modele: String,
    v_plaque: String,
    v_annee: String,
    v_cat: String,
    v_couleur: String,
    v_tarif: String,
    v_msg: String,
    v_ok: bool,
    v_recherche: String,
    v_page: usize,
    v_edit_id: Option<u64>, 
    v_edit_modele: String,
    v_edit_plaque: String,
    v_edit_annee: String,
    v_edit_cat: String,
    v_edit_couleur: String,
    v_edit_tarif: String,
    v_edit_etat: String,
    v_suppr_confirm: Option<u64>, 
    ventes: Vec<VoitureVente>,
    vt_nom: String,
    vt_plaque: String,
    vt_chassis: String,
    vt_immat: String,
    vt_annee: String,
    vt_prix_achat: String,
    vt_prix_demande: String,
    vt_notes: String,
    vt_msg: String,
    vt_ok: bool,
    vt_recherche: String,
    vt_page: usize,
    vt_suppr_confirm: Option<u64>,
    vt_marquer_vendu: Option<u64>, 
    vt_prix_vendu_input: String,   
    modal_voiture: ModalVoiture,
    modal_contrat: ModalContrat,
    tb_filter: usize, 
    tb_page: usize,
    dark_mode: bool,
    alerte_retour_ok: std::collections::HashSet<u64>,
    tb_date_recherche: String,
    impaye_recherche: String,
    reparations: Vec<Reparation>,
    rep_voiture_idx: usize,
    rep_date: String,
    rep_description: String,
    rep_prix: String,
    rep_observation: String,
    rep_msg: String,
    rep_ok: bool,
    rep_filtre_voiture: usize,
    rep_recherche: String,
    rep_suppr_confirm: Option<u64>,
}

impl App {
    fn new() -> Self {
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
            f_heure_debut: String::new(),
            f_heure_fin: String::new(),
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
            alerte_retour_ok: std::collections::HashSet::new(),
            tb_date_recherche: String::new(),
            impaye_recherche: String::new(),
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
        // Ensure all cars have etat field
        for v in &mut a.voitures {
            if v.etat.is_empty() {
                v.etat = "Bon état".to_string();
            }
        }
        a
    }

    fn prochain_id_contrat(&self) -> u64 {
        self.contrats.iter().map(|c| c.id).max().unwrap_or(0) + 1
    }
    fn prochain_id_voiture(&self) -> u64 {
        self.voitures.iter().map(|v| v.id).max().unwrap_or(0) + 1
    }
    fn numero_suggere(&self) -> String {
        format!(
            "IAM-{}-{:04}",
            Local::now().year(),
            self.prochain_id_contrat()
        )
    }
    fn est_disponible(
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
    fn loues_auj(&self) -> usize {
        let t = Local::now().date_naive();
        self.contrats
            .iter()
            .filter(|c| c.statut == "Actif" && c.chevauche(t, t))
            .count()
    }
    fn retours_demain(&self) -> Vec<Contrat> {
        let demain = Local::now().date_naive() + Duration::days(1);
        self.contrats
            .iter()
            .filter(|c| c.statut == "Actif")
            .filter(|c| NaiveDate::parse_from_str(&c.date_fin, "%Y-%m-%d").ok() == Some(demain))
            .cloned()
            .collect()
    }
    fn departs_demain(&self) -> Vec<Contrat> {
        let demain = Local::now().date_naive() + Duration::days(1);
        self.contrats
            .iter()
            .filter(|c| c.statut == "Actif")
            .filter(|c| NaiveDate::parse_from_str(&c.date_debut, "%Y-%m-%d").ok() == Some(demain))
            .cloned()
            .collect()
    }
    fn ca_mois(&self) -> f64 {
        let n = Local::now().date_naive();
        self.contrats
            .iter()
            .filter(|c| c.statut != "Annulé")
            .filter(|c| {
                NaiveDate::parse_from_str(&c.date_debut, "%Y-%m-%d")
                    .map(|d| d.year() == n.year() && d.month() == n.month())
                    .unwrap_or(false)
            })
            .map(|c| c.total())
            .sum()
    }
    fn ca_total(&self) -> f64 {
        self.contrats
            .iter()
            .filter(|c| c.statut != "Annulé")
            .map(|c| c.total())
            .sum()
    }
    fn naviguer(&mut self, dest: Onglet) {
        if self.onglet != dest {
            self.onglet_prec = Some(self.onglet);
            self.onglet = dest;
        }
    }
    fn nav_menu(&mut self, dest: Onglet) {
        self.onglet = dest;
        self.onglet_prec = None;
    }
    fn vider_formulaire(&mut self) {
        self.f_numero.clear();
        self.f_voiture = 0;
        self.f_voiture_note.clear();
        self.f_voiture_search.clear();
        self.f_client.clear();
        self.f_tel.clear();
        self.f_debut = aujourd_hui();
        self.f_fin = dans_7j();
        self.f_heure_debut.clear();
        self.f_heure_fin.clear();
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
    #[allow(dead_code)]
    fn contrat_par_id(&self, id: u64) -> Option<&Contrat> {
        self.contrats.iter().find(|c| c.id == id)
    }
    #[allow(dead_code)]
    fn voiture_par_id(&self, id: u64) -> Option<&Voiture> {
        self.voitures.iter().find(|v| v.id == id)
    }
    #[allow(dead_code)]
    fn contrat_actif_pour_voiture(&self, vid: u64) -> Option<&Contrat> {
        let auj = Local::now().date_naive();
        self.contrats
            .iter()
            .find(|c| c.voiture_id == vid && c.statut == "Actif" && c.chevauche(auj, auj))
    }

    fn contrats_voiture(&self, vid: u64) -> Vec<Contrat> {
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

    fn contrat_resume(c: &Contrat, prefix: &str) -> String {
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

    fn statut_voiture(&self, vid: u64, du: NaiveDate, au: NaiveDate) -> StatutVoiture {
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
        let _unpaid = contrats
            .iter()
            .filter(|c| c.reste_a_payer() > 0.0)
            .max_by_key(|c| parse_date(&c.date_fin).unwrap_or_else(|| Local::now().date_naive()))
            .cloned();
        let maintenance = voiture.etat == "En maintenance";

        let today_sv = Local::now().date_naive();
        let unpaid_non_future = contrats
            .iter()
            .filter(|c| c.statut == "Actif")
            .filter(|c| c.reste_a_payer() > 0.0)
            .filter(|c| {
                parse_date(&c.date_debut)
                    .map(|d| d <= today_sv)
                    .unwrap_or(false)
            })
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

    fn rafraichir_recherche(&mut self) {
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

impl App {
    fn new() -> Self {
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
            f_heure_debut: String::new(),
            f_heure_fin: String::new(),
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
            alerte_retour_ok: std::collections::HashSet::new(),
            tb_date_recherche: String::new(),
            impaye_recherche: String::new(),
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
        // Ensure all cars have etat field
        for v in &mut a.voitures {
            if v.etat.is_empty() {
                v.etat = "Bon état".to_string();
            }
        }
        a
    }

    fn prochain_id_contrat(&self) -> u64 {
        self.contrats.iter().map(|c| c.id).max().unwrap_or(0) + 1
    }
    fn prochain_id_voiture(&self) -> u64 {
        self.voitures.iter().map(|v| v.id).max().unwrap_or(0) + 1
    }
    fn numero_suggere(&self) -> String {
        format!(
            "IAM-{}-{:04}",
            Local::now().year(),
            self.prochain_id_contrat()
        )
    }
    fn est_disponible(
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
    fn loues_auj(&self) -> usize {
        let t = Local::now().date_naive();
        self.contrats
            .iter()
            .filter(|c| c.statut == "Actif" && c.chevauche(t, t))
            .count()
    }
    fn retours_demain(&self) -> Vec<Contrat> {
        let demain = Local::now().date_naive() + Duration::days(1);
        self.contrats
            .iter()
            .filter(|c| c.statut == "Actif")
            .filter(|c| NaiveDate::parse_from_str(&c.date_fin, "%Y-%m-%d").ok() == Some(demain))
            .cloned()
            .collect()
    }
    fn departs_demain(&self) -> Vec<Contrat> {
        let demain = Local::now().date_naive() + Duration::days(1);
        self.contrats
            .iter()
            .filter(|c| c.statut == "Actif")
            .filter(|c| NaiveDate::parse_from_str(&c.date_debut, "%Y-%m-%d").ok() == Some(demain))
            .cloned()
            .collect()
    }
    fn ca_mois(&self) -> f64 {
        let n = Local::now().date_naive();
        self.contrats
            .iter()
            .filter(|c| c.statut != "Annulé")
            .filter(|c| {
                NaiveDate::parse_from_str(&c.date_debut, "%Y-%m-%d")
                    .map(|d| d.year() == n.year() && d.month() == n.month())
                    .unwrap_or(false)
            })
            .map(|c| c.total())
            .sum()
    }
    fn ca_total(&self) -> f64 {
        self.contrats
            .iter()
            .filter(|c| c.statut != "Annulé")
            .map(|c| c.total())
            .sum()
    }
    fn naviguer(&mut self, dest: Onglet) {
        if self.onglet != dest {
            self.onglet_prec = Some(self.onglet);
            self.onglet = dest;
        }
    }
    fn nav_menu(&mut self, dest: Onglet) {
        self.onglet = dest;
        self.onglet_prec = None;
    }
    fn vider_formulaire(&mut self) {
        self.f_numero.clear();
        self.f_voiture = 0;
        self.f_voiture_note.clear();
        self.f_voiture_search.clear();
        self.f_client.clear();
        self.f_tel.clear();
        self.f_debut = aujourd_hui();
        self.f_fin = dans_7j();
        self.f_heure_debut.clear();
        self.f_heure_fin.clear();
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
    #[allow(dead_code)]
    fn contrat_par_id(&self, id: u64) -> Option<&Contrat> {
        self.contrats.iter().find(|c| c.id == id)
    }
    #[allow(dead_code)]
    fn voiture_par_id(&self, id: u64) -> Option<&Voiture> {
        self.voitures.iter().find(|v| v.id == id)
    }
    #[allow(dead_code)]
    fn contrat_actif_pour_voiture(&self, vid: u64) -> Option<&Contrat> {
        let auj = Local::now().date_naive();
        self.contrats
            .iter()
            .find(|c| c.voiture_id == vid && c.statut == "Actif" && c.chevauche(auj, auj))
    }

    fn contrats_voiture(&self, vid: u64) -> Vec<Contrat> {
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

    fn contrat_resume(c: &Contrat, prefix: &str) -> String {
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

    fn statut_voiture(&self, vid: u64, du: NaiveDate, au: NaiveDate) -> StatutVoiture {
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
        let _unpaid = contrats
            .iter()
            .filter(|c| c.reste_a_payer() > 0.0)
            .max_by_key(|c| parse_date(&c.date_fin).unwrap_or_else(|| Local::now().date_naive()))
            .cloned();
        let maintenance = voiture.etat == "En maintenance";

        let today_sv = Local::now().date_naive();
        let unpaid_non_future = contrats
            .iter()
            .filter(|c| c.statut == "Actif")
            .filter(|c| c.reste_a_payer() > 0.0)
            .filter(|c| {
                parse_date(&c.date_debut)
                    .map(|d| d <= today_sv)
                    .unwrap_or(false)
            })
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

    fn rafraichir_recherche(&mut self) {
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


