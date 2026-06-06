#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use chrono::{Datelike, Duration, Local, NaiveDate};
use eframe::egui::{
    self, Align2, Color32, FontId, Margin, RichText, Rounding, Sense, Stroke, Vec2,
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

fn data_dir() -> PathBuf {
    #[cfg(windows)]
    let base = std::env::var("APPDATA")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("."));
    #[cfg(not(windows))]
    let base = std::env::var("HOME")
        .map(|h| PathBuf::from(h).join(".local").join("share"))
        .unwrap_or_else(|_| PathBuf::from("."));
    let d = base.join("IAMBusiness");
    std::fs::create_dir_all(&d).ok();
    d
}
fn cars_file() -> PathBuf {
    data_dir().join("voitures.csv")
}
fn rentals_file() -> PathBuf {
    data_dir().join("contrats.csv")
}
fn ventes_file() -> PathBuf {
    data_dir().join("ventes_voitures.csv")
}
fn reparations_file() -> PathBuf {
    data_dir().join("reparations.csv")
}
fn periodes_file() -> PathBuf {
    data_dir().join("caisse_periodes.csv")
}
fn encaissements_file() -> PathBuf {
    data_dir().join("caisse_encaissements.csv")
}
fn decaissements_file() -> PathBuf {
    data_dir().join("caisse_decaissements.csv")
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Reparation {
    id: u64,
    voiture_id: u64,
    voiture_modele: String,
    voiture_plaque: String,
    date: String,
    description: String,
    #[serde(default)]
    prix: String,
    #[serde(default)]
    observation: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct VoitureVente {
    id: u64,
    nom: String,
    plaque: String,
    num_chassis: String,
    num_immat: String,
    annee: u32,
    prix_achat: f64,
    prix_demande: f64,
    prix_vendu: f64,
    vendu: bool,
    date_vente: String,
    notes: String,
}

impl VoitureVente {
    fn benefice(&self) -> f64 {
        if self.vendu && self.prix_achat > 0.0 {
            self.prix_vendu - self.prix_achat
        } else {
            0.0
        }
    }
}

const RED_IAM: Color32 = Color32::from_rgb(200, 30, 40);
const GREEN: Color32 = Color32::from_rgb(34, 197, 94);
const AMBER: Color32 = Color32::from_rgb(245, 158, 11);
#[allow(dead_code)]
const BLUE: Color32 = Color32::from_rgb(59, 130, 246);
const WHITE: Color32 = Color32::WHITE;

use std::cell::Cell;
thread_local! {
    static T_BG:     Cell<[u8;3]> = Cell::new([12,  12,  15]);
    static T_CARD:   Cell<[u8;3]> = Cell::new([25,  25,  32]);
    static T_CARD2:  Cell<[u8;3]> = Cell::new([35,  35,  45]);
    static T_BORDER: Cell<[u8;3]> = Cell::new([50,  50,  65]);
    static T_TEXT:   Cell<[u8;3]> = Cell::new([240, 240, 245]);
    static T_MUTED:  Cell<[u8;3]> = Cell::new([130, 130, 145]);
}

fn c3(tl: &'static std::thread::LocalKey<Cell<[u8; 3]>>) -> Color32 {
    tl.with(|c| {
        let [r, g, b] = c.get();
        Color32::from_rgb(r, g, b)
    })
}
#[allow(dead_code)]
fn bg() -> Color32 {
    c3(&T_BG)
}
fn card() -> Color32 {
    c3(&T_CARD)
}
fn card2() -> Color32 {
    c3(&T_CARD2)
}
fn border() -> Color32 {
    c3(&T_BORDER)
}
fn text() -> Color32 {
    c3(&T_TEXT)
}
fn muted() -> Color32 {
    c3(&T_MUTED)
}

fn tinted_card(tint: Color32, alpha: u8) -> Color32 {
    let [br, bg, bb, _] = card().to_array();
    let [tr, tg, tb, _] = tint.to_array();
    let a = alpha as u16;
    let blend = |base: u8, t: u8| -> u8 { ((base as u16 * (255 - a) + t as u16 * a) / 255) as u8 };
    Color32::from_rgb(blend(br, tr), blend(bg, tg), blend(bb, tb))
}

fn set_theme(dark: bool) {
    if dark {
        T_BG.with(|c| c.set([12, 12, 15]));
        T_CARD.with(|c| c.set([25, 25, 32]));
        T_CARD2.with(|c| c.set([35, 35, 45]));
        T_BORDER.with(|c| c.set([50, 50, 65]));
        T_TEXT.with(|c| c.set([240, 240, 245]));
        T_MUTED.with(|c| c.set([130, 130, 145]));
    } else {
        T_BG.with(|c| c.set([242, 243, 247]));
        T_CARD.with(|c| c.set([255, 255, 255]));
        T_CARD2.with(|c| c.set([235, 236, 242]));
        T_BORDER.with(|c| c.set([200, 202, 215]));
        T_TEXT.with(|c| c.set([20, 20, 30]));
        T_MUTED.with(|c| c.set([90, 90, 110]));
    }
}

const INPUT_W: f32 = 300.0;
const LABEL_W: f32 = 170.0;
const ITEMS_PER_PAGE: usize = 10;

fn parse_date(s: &str) -> Option<NaiveDate> {
    let s = s.trim();
    NaiveDate::parse_from_str(s, "%d/%m/%Y")
        .or_else(|_| NaiveDate::parse_from_str(s, "%Y-%m-%d"))
        .ok()
}
fn afficher_date(iso: &str) -> String {
    NaiveDate::parse_from_str(iso, "%Y-%m-%d")
        .map(|d| d.format("%d/%m/%Y").to_string())
        .unwrap_or_else(|_| iso.to_string())
}
fn stocker_date(d: NaiveDate) -> String {
    d.format("%Y-%m-%d").to_string()
}
fn aujourd_hui() -> String {
    Local::now().date_naive().format("%d/%m/%Y").to_string()
}
fn dans_7j() -> String {
    (Local::now().date_naive() + Duration::days(7))
        .format("%d/%m/%Y")
        .to_string()
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Voiture {
    id: u64,
    modele: String,
    plaque: String,
    categorie: String,
    couleur: String,
    annee: u32,
    tarif_jour: f64,
    #[serde(default)]
    etat: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Contrat {
    id: u64,
    numero: String,
    voiture_id: u64,
    voiture_modele: String,
    voiture_plaque: String,
    client_nom: String,
    client_tel: String,
    agent: String,
    date_debut: String,
    date_fin: String,
    #[serde(default)]
    heure_debut: String,
    #[serde(default)]
    heure_fin: String,
    km_depart: f64,
    km_retour: f64,
    tarif_jour: f64,
    notes: String,
    statut: String,
    montant_paye: f64,
    modifiable_note: String,
}

impl Contrat {
    fn jours(&self) -> i64 {
        let d = NaiveDate::parse_from_str(&self.date_debut, "%Y-%m-%d").ok();
        let f = NaiveDate::parse_from_str(&self.date_fin, "%Y-%m-%d").ok();
        d.zip(f)
            .map(|(a, b)| (b - a).num_days().max(1))
            .unwrap_or(1)
    }
    fn total(&self) -> f64 {
        self.jours() as f64 * self.tarif_jour
    }
    fn reste_a_payer(&self) -> f64 {
        let t = self.total();
        if self.montant_paye >= t {
            0.0
        } else {
            t - self.montant_paye
        }
    }
    #[allow(dead_code)]
    fn km_parcourus(&self) -> f64 {
        if self.km_retour > self.km_depart {
            self.km_retour - self.km_depart
        } else {
            0.0
        }
    }
    fn chevauche(&self, du: NaiveDate, au: NaiveDate) -> bool {
        if self.statut == "Annulé" {
            return false;
        }
        let d = NaiveDate::parse_from_str(&self.date_debut, "%Y-%m-%d").ok();
        let f = NaiveDate::parse_from_str(&self.date_fin, "%Y-%m-%d").ok();
        let (a, b) = match d.zip(f) {
            Some(v) => v,
            None => return false,
        };
        a < au && b > du
    }
}

fn charger<T: for<'de> Deserialize<'de>>(p: &PathBuf) -> Vec<T> {
    csv::Reader::from_path(p)
        .ok()
        .map(|mut r| r.deserialize().filter_map(|x| x.ok()).collect())
        .unwrap_or_default()
}
fn sauvegarder<T: Serialize>(p: &PathBuf, v: &[T]) {
    if let Ok(mut w) = csv::Writer::from_path(p) {
        for x in v {
            let _ = w.serialize(x);
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct PeriodeCaisse {
    id: u64,
    nom: String,
    date_debut: String,
    date_fin: String,
    fermee: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Encaissement {
    id: u64,
    periode_id: u64,
    date: String,
    client_nom: String,
    contrat_numero: String,
    voiture: String,
    details: String,
    montant: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Decaissement {
    id: u64,
    periode_id: u64,
    date: String,
    nom: String,
    details: String,
    montant: f64,
}

#[derive(PartialEq, Clone, Copy)]
enum Onglet {
    Tableau,
    Disponibilite,
    NouveauContrat,
    Contrats,
    Voitures,
    Ventes,
    Maintenance,
    Impayes,
    Caisse,
}

// Modal popup to show car details
#[derive(Default)]
struct ModalVoiture {
    visible: bool,
    voiture_id: u64,
    histo_recherche: String,
}

// Modal popup to show contract details
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
    // Nouveau contrat form
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
    // Disponibilité
    d_du: String,
    d_au: String,
    d_ids: Vec<u64>,
    d_fait: bool,
    d_erreur: String,
    d_recherche_modele: String,
    d_modele_periode: Vec<(u64, String, String, String)>,
    // Contrats list
    c_recherche: String,
    c_statut: usize,
    c_suppr: Option<usize>,
    c_page: usize,
    c_filter_mois: usize,
    c_filter_annee: String,
    // Voitures list
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
    // Ventes de voitures
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
    // Modals
    modal_voiture: ModalVoiture,
    modal_contrat: ModalContrat,
    // Dashboard
    tb_filter: usize,
    tb_page: usize,
    dark_mode: bool,
    alerte_retour_ok: std::collections::HashSet<u64>,
    tb_date_recherche: String,
    impaye_recherche: String,
    // Réparations
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
    #[allow(dead_code)]
    rep_filtre_mois: usize,
    #[allow(dead_code)]
    rep_filtre_annee: String,
    //  Caisse
    caisse_periodes: Vec<PeriodeCaisse>,
    caisse_encaissements: Vec<Encaissement>,
    caisse_decaissements: Vec<Decaissement>,
    caisse_periode_active: Option<u64>,
    caisse_suppr_periode: Option<u64>,
    caisse_creer_visible: bool,
    caisse_new_nom: String,
    caisse_new_debut: String,
    caisse_new_fin: String,
    caisse_new_msg: String,
    enc_client: String,
    enc_contrat: String,
    enc_voiture: String,
    enc_details: String,
    enc_montant: String,
    enc_date: String,
    enc_msg: String,
    dec_nom: String,
    dec_details: String,
    dec_montant: String,
    dec_date: String,
    dec_msg: String,
    enc_suppr: Option<u64>,
    dec_suppr: Option<u64>,
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
            rep_filtre_mois: 0,
            rep_filtre_annee: String::new(),
            caisse_periodes: charger(&periodes_file()),
            caisse_encaissements: charger(&encaissements_file()),
            caisse_decaissements: charger(&decaissements_file()),
            caisse_periode_active: None,
            caisse_suppr_periode: None,
            caisse_creer_visible: false,
            caisse_new_nom: String::new(),
            caisse_new_debut: aujourd_hui(),
            caisse_new_fin: dans_7j(),
            caisse_new_msg: String::new(),
            enc_client: String::new(),
            enc_contrat: String::new(),
            enc_voiture: String::new(),
            enc_details: String::new(),
            enc_montant: String::new(),
            enc_date: aujourd_hui(),
            enc_msg: String::new(),
            dec_nom: String::new(),
            dec_details: String::new(),
            dec_montant: String::new(),
            dec_date: aujourd_hui(),
            dec_msg: String::new(),
            enc_suppr: None,
            dec_suppr: None,
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

fn voitures_defaut() -> Vec<Voiture> {
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

fn panneau() -> egui::Frame {
    egui::Frame {
        fill: card(),
        inner_margin: Margin::same(18.0),
        rounding: Rounding::same(10.0),
        stroke: Stroke::new(1.0, border()),
        ..Default::default()
    }
}

fn titre_page(ui: &mut egui::Ui, t: &str) {
    ui.label(RichText::new(t).size(24.0).color(RED_IAM).strong());
    ui.add_space(6.0);
    ui.separator();
    ui.add_space(12.0);
}

fn etiquette(ui: &mut egui::Ui, txt: &str) {
    ui.add_sized(
        [LABEL_W, 28.0],
        egui::Label::new(RichText::new(txt).size(13.5).color(text())),
    );
}

fn bouton_principal(ui: &mut egui::Ui, txt: &str) -> bool {
    ui.add_sized(
        [INPUT_W, 36.0],
        egui::Button::new(RichText::new(txt).size(14.0).color(WHITE))
            .fill(RED_IAM)
            .rounding(6.0),
    )
    .clicked()
}

fn bouton_danger(ui: &mut egui::Ui, txt: &str, w: f32) -> bool {
    ui.add_sized(
        [w, 30.0],
        egui::Button::new(RichText::new(txt).size(13.0).color(WHITE))
            .fill(Color32::from_rgb(180, 30, 40))
            .rounding(5.0),
    )
    .clicked()
}

fn bouton_neutre(ui: &mut egui::Ui, txt: &str, w: f32) -> bool {
    ui.add_sized(
        [w, 30.0],
        egui::Button::new(RichText::new(txt).size(13.0).color(muted()))
            .fill(card2())
            .rounding(5.0),
    )
    .clicked()
}

fn bouton_vert(ui: &mut egui::Ui, txt: &str, w: f32) -> bool {
    ui.add_sized(
        [w, 30.0],
        egui::Button::new(RichText::new(txt).size(13.0).color(WHITE))
            .fill(Color32::from_rgb(22, 130, 80))
            .rounding(5.0),
    )
    .clicked()
}

fn bouton_modification(ui: &mut egui::Ui, txt: &str, w: f32) -> bool {
    ui.add_sized(
        [w, 30.0],
        egui::Button::new(RichText::new(txt).size(13.0).color(WHITE))
            .fill(Color32::from_rgb(100, 120, 200))
            .rounding(5.0),
    )
    .clicked()
}

#[allow(dead_code)]
fn bouton_bleu(ui: &mut egui::Ui, txt: &str, w: f32) -> bool {
    ui.add_sized(
        [w, 28.0],
        egui::Button::new(RichText::new(txt).size(12.0).color(WHITE))
            .fill(BLUE)
            .rounding(5.0),
    )
    .clicked()
}

fn point_couleur(ui: &mut egui::Ui, c: Color32) {
    let (r, _) = ui.allocate_exact_size(Vec2::splat(10.0), Sense::hover());
    ui.painter().circle_filled(r.center(), 5.0, c);
}

fn logo_widget(ui: &mut egui::Ui, grand: bool) {
    let sz = if grand { 44.0 } else { 32.0 };
    let (r, _) = ui.allocate_exact_size(Vec2::splat(sz), Sense::hover());
    ui.painter()
        .rect_filled(r, Rounding::same(if grand { 8.0 } else { 6.0 }), RED_IAM);
    ui.painter().text(
        r.center(),
        Align2::CENTER_CENTER,
        "IAM",
        FontId::proportional(if grand { 15.0 } else { 11.5 }),
        WHITE,
    );
}

#[allow(dead_code)]
fn pagination_controls(ui: &mut egui::Ui, page: &mut usize, total_items: usize, per_page: usize) {
    let total_pages = (total_items + per_page - 1) / per_page;
    if total_pages <= 1 {
        return;
    }
    ui.add_space(10.0);
    ui.horizontal(|ui| {
        ui.label(
            RichText::new(format!("Page {} / {}", *page + 1, total_pages))
                .size(12.0)
                .color(muted()),
        );
        ui.add_space(8.0);
        if ui
            .add_sized(
                [28.0, 26.0],
                egui::Button::new(RichText::new("‹").size(14.0).color(if *page > 0 {
                    text()
                } else {
                    muted()
                }))
                .fill(card2())
                .rounding(4.0),
            )
            .clicked()
            && *page > 0
        {
            *page -= 1;
        }
        for p in 0..total_pages {
            let active = *page == p;
            if ui
                .add_sized(
                    [26.0, 26.0],
                    egui::Button::new(
                        RichText::new(format!("{}", p + 1))
                            .size(12.0)
                            .color(if active { WHITE } else { muted() }),
                    )
                    .fill(if active { RED_IAM } else { card2() })
                    .rounding(4.0),
                )
                .clicked()
            {
                *page = p;
            }
            if total_pages > 8 && p == 2 && *page > 4 {
                ui.label(RichText::new("…").size(13.0).color(muted()));
                // Skip to near current page
                let skip_to = (*page).saturating_sub(1);
                for pp in skip_to..(*page + 2).min(total_pages - 2) {
                    let a2 = *page == pp;
                    if ui
                        .add_sized(
                            [26.0, 26.0],
                            egui::Button::new(
                                RichText::new(format!("{}", pp + 1))
                                    .size(12.0)
                                    .color(if a2 { WHITE } else { muted() }),
                            )
                            .fill(if a2 { RED_IAM } else { card2() })
                            .rounding(4.0),
                        )
                        .clicked()
                    {
                        *page = pp;
                    }
                }
                ui.label(RichText::new("…").size(13.0).color(muted()));
                break;
            }
        }
        if ui
            .add_sized(
                [28.0, 26.0],
                egui::Button::new(RichText::new("›").size(14.0).color(
                    if *page + 1 < total_pages {
                        text()
                    } else {
                        muted()
                    },
                ))
                .fill(card2())
                .rounding(4.0),
            )
            .clicked()
            && *page + 1 < total_pages
        {
            *page += 1;
        }
    });
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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

                                if !contrats.is_empty() {
                                    ui.add_space(10.0);
                                    ui.label(
                                        RichText::new("Historique / contrats liés")
                                            .size(13.0)
                                            .color(RED_IAM)
                                            .strong(),
                                    );
                                    ui.add_space(6.0);

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
                            });

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
                            });

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
                        ("Caisse", Onglet::Caisse),
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
                        Onglet::Caisse => self.page_caisse(ui),
                    }
                    ui.add_space(32.0);
                });
        });
    }
}

impl App {
    fn page_tableau(&mut self, ui: &mut egui::Ui) {
        titre_page(
            ui,
            &format!(
                "Tableau de Bord – {}",
                Local::now().date_naive().format("%d/%m/%Y")
            ),
        );

        let total = self.voitures.len();
        let loues = self.loues_auj();
        let libres = total.saturating_sub(loues);
        let mois = self.ca_mois();
        let w4 = (ui.available_width() - 48.0) / 4.0;

        ui.horizontal(|ui| {
            for (lbl, val, c) in [
                ("Parc Total", total.to_string(), RED_IAM),
                ("Loués Auj.", loues.to_string(), AMBER),
                ("Disponibles", libres.to_string(), GREEN),
                (
                    "CA Mois",
                    format!("{:.0} DA", mois),
                    Color32::from_rgb(168, 85, 247),
                ),
            ] {
                panneau().show(ui, |ui| {
                    ui.set_min_width(w4);
                    ui.label(RichText::new(lbl).size(12.0).color(muted()));
                    ui.add_space(6.0);
                    ui.label(RichText::new(val).size(28.0).color(c).strong());
                });
                ui.add_space(8.0);
            }
        });

        ui.add_space(16.0);

        let retours = self.retours_demain();
        let departs = self.departs_demain();
        if !retours.is_empty() || !departs.is_empty() {
            egui::Frame {
                fill: card(),
                stroke: Stroke::new(2.0, RED_IAM),
                inner_margin: Margin::same(18.0),
                rounding: Rounding::same(10.0),
                ..Default::default()
            }
            .show(ui, |ui| {
                ui.label(
                    RichText::new("IMPORTANT - DEMAIN")
                        .size(15.0)
                        .color(RED_IAM)
                        .strong(),
                );
                ui.separator();
                ui.add_space(6.0);
                if !retours.is_empty() {
                    ui.label(
                        RichText::new(format!("Retours prévus ({}):", retours.len()))
                            .size(13.5)
                            .color(AMBER)
                            .strong(),
                    );
                    for c in &retours {
                        ui.label(
                            RichText::new(format!(
                                "  • {} – Client: {}",
                                c.voiture_modele, c.client_nom
                            ))
                            .size(12.0)
                            .color(text()),
                        );
                    }
                    ui.add_space(6.0);
                }
                if !departs.is_empty() {
                    ui.label(
                        RichText::new(format!("Départs prévus ({}):", departs.len()))
                            .size(13.5)
                            .color(GREEN)
                            .strong(),
                    );
                    for c in &departs {
                        ui.label(
                            RichText::new(format!(
                                "  • {} – Client: {}",
                                c.voiture_modele, c.client_nom
                            ))
                            .size(12.0)
                            .color(text()),
                        );
                    }
                }
            });
            ui.add_space(16.0);
        }

        let auj_check = Local::now().date_naive();
        let ok_ids: std::collections::HashSet<u64> = self.alerte_retour_ok.clone();
        let retours_auj: Vec<Contrat> = self
            .contrats
            .iter()
            .filter(|c| c.statut == "Actif")
            .filter(|c| NaiveDate::parse_from_str(&c.date_fin, "%Y-%m-%d").ok() == Some(auj_check))
            .filter(|c| !ok_ids.contains(&c.id))
            .cloned()
            .collect();

        let mut marquer_rendu: Option<u64> = None;
        let mut renouveler_id: Option<u64> = None;

        if !retours_auj.is_empty() {
            egui::Frame {
                fill: tinted_card(AMBER, 40),
                stroke: Stroke::new(2.0, AMBER),
                inner_margin: Margin::same(16.0),
                rounding: Rounding::same(10.0),
                ..Default::default()
            }
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new("RETOURS AUJOURD'HUI")
                            .size(14.0)
                            .color(AMBER)
                            .strong(),
                    );
                    ui.add_space(6.0);
                    ui.label(
                        RichText::new(format!("({} contrat(s))", retours_auj.len()))
                            .size(12.0)
                            .color(muted()),
                    );
                });
                ui.separator();
                ui.add_space(6.0);
                for c in &retours_auj {
                    ui.horizontal(|ui| {
                        point_couleur(ui, AMBER);
                        ui.label(
                            RichText::new(format!(
                                "{} – {} – {}",
                                c.numero, c.voiture_modele, c.client_nom
                            ))
                            .size(12.5)
                            .color(text()),
                        );
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if bouton_neutre(ui, "Renouveler", 90.0) {
                                renouveler_id = Some(c.id);
                            }
                            ui.add_space(6.0);
                            if bouton_vert(ui, "Voiture Rendue", 115.0) {
                                marquer_rendu = Some(c.id);
                            }
                        });
                    });
                    ui.add_space(3.0);
                }
            });
            ui.add_space(12.0);
        }

        if let Some(cid) = marquer_rendu {
            if let Some(c) = self.contrats.iter_mut().find(|c| c.id == cid) {
                c.statut = "Terminé".to_string();
            }
            sauvegarder(&rentals_file(), &self.contrats);
            self.alerte_retour_ok.insert(cid);
        }
        if let Some(cid) = renouveler_id {
            self.alerte_retour_ok.insert(cid);
            if let Some(idx) = self.contrats.iter().position(|c| c.id == cid) {
                let (voiture_id, client_nom, client_tel, agent, date_fin, tarif_jour, notes) = {
                    let c = &self.contrats[idx];
                    (
                        c.voiture_id,
                        c.client_nom.clone(),
                        c.client_tel.clone(),
                        c.agent.clone(),
                        c.date_fin.clone(),
                        c.tarif_jour,
                        c.notes.clone(),
                    )
                };
                self.f_voiture = self
                    .voitures
                    .iter()
                    .position(|v| v.id == voiture_id)
                    .map(|i| i + 1)
                    .unwrap_or(0);
                self.f_client = client_nom;
                self.f_tel = client_tel;
                self.f_agent = agent;
                self.f_debut = afficher_date(&date_fin);
                self.f_fin = String::new();
                self.f_tarif = tarif_jour.to_string();
                self.f_notes = notes;
                self.f_montant_paye = "0".to_string();
                self.f_msg.clear();
                self.f_en_edition = None;
                self.naviguer(Onglet::NouveauContrat);
            }
        }

        {
            egui::Frame {
                fill: card2(),
                stroke: Stroke::new(1.0, border()),
                inner_margin: Margin::same(12.0),
                rounding: Rounding::same(8.0),
                ..Default::default()
            }
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new("Recherche par date :")
                            .size(13.0)
                            .color(muted()),
                    );
                    ui.add_space(8.0);
                    ui.add(
                        egui::TextEdit::singleline(&mut self.tb_date_recherche)
                            .desired_width(140.0)
                            .hint_text("JJ/MM/AAAA"),
                    );
                    ui.add_space(8.0);
                    if bouton_neutre(ui, "Effacer", 70.0) {
                        self.tb_date_recherche.clear();
                    }
                });
                if let Some(date_r) = parse_date(&self.tb_date_recherche) {
                    ui.add_space(8.0);
                    let retours_date: Vec<&Contrat> = self
                        .contrats
                        .iter()
                        .filter(|c| c.statut == "Actif")
                        .filter(|c| {
                            NaiveDate::parse_from_str(&c.date_fin, "%Y-%m-%d").ok() == Some(date_r)
                        })
                        .collect();
                    let departs_date: Vec<&Contrat> = self
                        .contrats
                        .iter()
                        .filter(|c| c.statut == "Actif")
                        .filter(|c| {
                            NaiveDate::parse_from_str(&c.date_debut, "%Y-%m-%d").ok()
                                == Some(date_r)
                        })
                        .collect();
                    if retours_date.is_empty() && departs_date.is_empty() {
                        ui.label(
                            RichText::new(format!("Aucune activité le {}", self.tb_date_recherche))
                                .size(12.0)
                                .color(muted()),
                        );
                    } else {
                        if !retours_date.is_empty() {
                            ui.label(
                                RichText::new(format!("Retours ({})", retours_date.len()))
                                    .size(12.5)
                                    .color(AMBER)
                                    .strong(),
                            );
                            for c in &retours_date {
                                ui.label(
                                    RichText::new(format!(
                                        "  • {} – {} – {}",
                                        c.numero, c.voiture_modele, c.client_nom
                                    ))
                                    .size(12.0)
                                    .color(text()),
                                );
                            }
                        }
                        if !departs_date.is_empty() {
                            ui.add_space(4.0);
                            ui.label(
                                RichText::new(format!("Départs ({})", departs_date.len()))
                                    .size(12.5)
                                    .color(GREEN)
                                    .strong(),
                            );
                            for c in &departs_date {
                                ui.label(
                                    RichText::new(format!(
                                        "  • {} – {} – {}",
                                        c.numero, c.voiture_modele, c.client_nom
                                    ))
                                    .size(12.0)
                                    .color(text()),
                                );
                            }
                        }
                    }
                }
            });
            ui.add_space(12.0);
        }

        let tw = ui.available_width();
        let lw = tw * 0.62 - 8.0;
        let rw = tw - lw - 16.0;

        let mut open_voiture_modal: Option<u64> = None;
        let mut open_contrat_modal: Option<u64> = None;

        ui.horizontal_top(|ui| {
            ui.allocate_ui_with_layout(
                Vec2::new(lw, 0.0),
                egui::Layout::top_down(egui::Align::LEFT),
                |ui| {
                    panneau().show(ui, |ui| {
                        ui.set_min_width(lw - 4.0);
                        ui.horizontal(|ui| {
                            ui.label(
                                RichText::new("Derniers Contrats")
                                    .size(15.0)
                                    .color(text())
                                    .strong(),
                            );
                            ui.add_space(16.0);
                            for (i, lbl) in ["Tous", "Actifs", "Terminés", "Réservés"]
                                .iter()
                                .enumerate()
                            {
                                let sel = self.tb_filter == i;
                                if ui
                                    .add(
                                        egui::Button::new(
                                            RichText::new(*lbl).size(11.5).color(if sel {
                                                WHITE
                                            } else {
                                                muted()
                                            }),
                                        )
                                        .fill(if sel { RED_IAM } else { card2() })
                                        .rounding(4.0),
                                    )
                                    .clicked()
                                {
                                    self.tb_filter = i;
                                    self.tb_page = 0;
                                }
                            }
                        });
                        ui.separator();
                        ui.add_space(6.0);

                        let tb_today = Local::now().date_naive();
                        let liste: Vec<Contrat> = self
                            .contrats
                            .iter()
                            .rev()
                            .filter(|c| match self.tb_filter {
                                0 => true,
                                1 => {
                                    c.statut == "Actif"
                                        && NaiveDate::parse_from_str(&c.date_debut, "%Y-%m-%d")
                                            .map(|d| d <= tb_today)
                                            .unwrap_or(true)
                                }
                                2 => c.statut == "Terminé",
                                3 => {
                                    c.statut == "Actif"
                                        && NaiveDate::parse_from_str(&c.date_debut, "%Y-%m-%d")
                                            .map(|d| d > tb_today)
                                            .unwrap_or(false)
                                }
                                _ => true,
                            })
                            .cloned()
                            .collect();

                        if liste.is_empty() {
                            ui.label(RichText::new("Aucun contrat.").color(muted()));
                        } else {
                            let per_page = 8usize;
                            let total_pages = (liste.len() + per_page - 1) / per_page;
                            if self.tb_page >= total_pages {
                                self.tb_page = 0;
                            }
                            let start = self.tb_page * per_page;
                            let slice: Vec<&Contrat> =
                                liste.iter().skip(start).take(per_page).collect();

                            egui::Grid::new("t_rec")
                                .num_columns(7)
                                .spacing([8.0, 6.0])
                                .striped(true)
                                .show(ui, |ui| {
                                    for h in [
                                        "Contrat",
                                        "Client",
                                        "Tél",
                                        "Véhicule",
                                        "Début",
                                        "Fin",
                                        "Statut",
                                    ] {
                                        ui.label(
                                            RichText::new(h).size(12.0).color(muted()).strong(),
                                        );
                                    }
                                    ui.end_row();
                                    for c in &slice {
                                        let row_resp = ui.add(
                                            egui::Button::new(
                                                RichText::new(&c.numero).size(12.5).color(RED_IAM),
                                            )
                                            .fill(Color32::TRANSPARENT)
                                            .rounding(3.0),
                                        );
                                        if row_resp.clicked() {
                                            open_contrat_modal = Some(c.id);
                                        }
                                        ui.label(
                                            RichText::new(&c.client_nom).size(12.5).color(text()),
                                        );
                                        ui.label(
                                            RichText::new(&c.client_tel).size(12.0).color(muted()),
                                        );
                                        ui.label(
                                            RichText::new(&c.voiture_modele)
                                                .size(12.5)
                                                .color(text()),
                                        );
                                        ui.label(
                                            RichText::new(afficher_date(&c.date_debut))
                                                .size(12.0)
                                                .color(muted()),
                                        );
                                        ui.label(
                                            RichText::new(afficher_date(&c.date_fin))
                                                .size(12.0)
                                                .color(muted()),
                                        );
                                        let today_tb = Local::now().date_naive();
                                        let is_future_tb =
                                            NaiveDate::parse_from_str(&c.date_debut, "%Y-%m-%d")
                                                .map(|d| d > today_tb)
                                                .unwrap_or(false);
                                        let (sc, ss) = match c.statut.as_str() {
                                            "Actif" if is_future_tb => {
                                                (Color32::from_rgb(99, 102, 241), "Réservé")
                                            }
                                            "Actif" => (GREEN, "Actif"),
                                            "Terminé" => (muted(), "Terminé"),
                                            "Annulé" => (RED_IAM, "Annulé"),
                                            _ => (muted(), c.statut.as_str()),
                                        };
                                        ui.label(RichText::new(ss).size(12.0).color(sc).strong());
                                        ui.end_row();
                                    }
                                });

                            if total_pages > 1 {
                                ui.add_space(6.0);
                                ui.horizontal(|ui| {
                                    ui.label(
                                        RichText::new(format!(
                                            "Page {}/{}",
                                            self.tb_page + 1,
                                            total_pages
                                        ))
                                        .size(11.0)
                                        .color(muted()),
                                    );
                                    if ui.small_button("‹").clicked() && self.tb_page > 0 {
                                        self.tb_page -= 1;
                                    }
                                    if ui.small_button("›").clicked()
                                        && self.tb_page + 1 < total_pages
                                    {
                                        self.tb_page += 1;
                                    }
                                });
                            }
                        }
                    });
                },
            );

            ui.add_space(8.0);

            ui.allocate_ui_with_layout(
                Vec2::new(rw, 0.0),
                egui::Layout::top_down(egui::Align::LEFT),
                |ui| {
                    panneau().show(ui, |ui| {
                        ui.set_min_width(rw - 4.0);
                        ui.label(
                            RichText::new("État du Parc")
                                .size(15.0)
                                .color(text())
                                .strong(),
                        );
                        ui.separator();
                        ui.add_space(6.0);
                        let auj = Local::now().date_naive();
                        let snap = self.voitures.clone();
                        for v in &snap {
                            let statut = self.statut_voiture(v.id, auj, auj);
                            let col = statut.couleur;
                            let statut_str = &statut.libelle;

                            ui.horizontal(|ui| {
                                point_couleur(ui, col);
                                let row_resp = ui.add(
                                    egui::Button::new(
                                        egui::widget_text::RichText::new(format!(
                                            "{}  —  {}",
                                            v.modele, statut_str
                                        ))
                                        .size(12.5)
                                        .color(col),
                                    )
                                    .fill(Color32::TRANSPARENT)
                                    .rounding(3.0),
                                );
                                if row_resp.clicked() {
                                    open_voiture_modal = Some(v.id);
                                }
                                row_resp.on_hover_text("Cliquer pour voir les détails");
                            });
                            ui.add_space(2.0);
                        }
                        ui.separator();
                        ui.add_space(4.0);
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("CA Total :").size(12.0).color(muted()));
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    ui.label(
                                        RichText::new(format!("{:.0} DA", self.ca_total()))
                                            .size(13.0)
                                            .color(Color32::from_rgb(168, 85, 247))
                                            .strong(),
                                    );
                                },
                            );
                        });
                    });
                },
            );
        });

        if let Some(vid) = open_voiture_modal {
            self.modal_voiture = ModalVoiture {
                visible: true,
                voiture_id: vid,
                histo_recherche: String::new(),
            };
        }
        if let Some(cid) = open_contrat_modal {
            self.modal_contrat = ModalContrat {
                visible: true,
                contrat_id: cid,
            };
        }
    }

    fn page_dispo(&mut self, ui: &mut egui::Ui) {
        titre_page(ui, "Recherche de Disponibilité");

        let left_w = 380.0;
        let right_w = (ui.available_width() - left_w - 12.0).max(420.0);
        let mut open_voiture_modal: Option<u64> = None;
        let mut louer_id: Option<u64> = None;

        ui.horizontal_top(|ui| {
            ui.allocate_ui_with_layout(
                Vec2::new(left_w, 0.0),
                egui::Layout::top_down(egui::Align::LEFT),
                |ui| {
                    panneau().show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(
                                RichText::new("Filtres de recherche")
                                    .size(13.5)
                                    .color(RED_IAM)
                                    .strong(),
                            );
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    if bouton_neutre(ui, "Rafraichir", 122.0) {
                                        self.rafraichir_recherche();
                                    }
                                },
                            );
                        });
                        ui.add_space(10.0);

                        ui.label(
                            RichText::new("Recherche par période")
                                .size(13.5)
                                .color(RED_IAM)
                                .strong(),
                        );
                        ui.add_space(8.0);
                        ui.horizontal(|ui| {
                            etiquette(ui, "Du (JJ/MM/AAAA) :");
                            ui.add(
                                egui::TextEdit::singleline(&mut self.d_du)
                                    .desired_width(INPUT_W)
                                    .hint_text("01/06/2026"),
                            );
                        });
                        ui.add_space(8.0);
                        ui.horizontal(|ui| {
                            etiquette(ui, "Au (JJ/MM/AAAA) :");
                            ui.add(
                                egui::TextEdit::singleline(&mut self.d_au)
                                    .desired_width(INPUT_W)
                                    .hint_text("10/06/2026"),
                            );
                        });
                        ui.add_space(10.0);
                        ui.horizontal(|ui| {
                            if bouton_principal(ui, "Vérifier par période") {
                                self.chercher_dispo_periode();
                            }
                            ui.add_space(8.0);
                            if bouton_neutre(ui, "Réinitialiser", 122.0) {
                                self.d_du = aujourd_hui();
                                self.d_au = dans_7j();
                                self.d_ids.clear();
                                self.d_fait = false;
                                self.d_erreur.clear();
                            }
                        });
                        if !self.d_erreur.is_empty() {
                            ui.add_space(6.0);
                            ui.colored_label(
                                Color32::from_rgb(220, 50, 50),
                                &self.d_erreur.clone(),
                            );
                        }

                        ui.add_space(14.0);
                        ui.separator();
                        ui.add_space(12.0);

                        ui.label(
                            RichText::new("Recherche par modèle / type")
                                .size(13.5)
                                .color(RED_IAM)
                                .strong(),
                        );
                        ui.add_space(8.0);
                        ui.horizontal(|ui| {
                            etiquette(ui, "Modèle :");
                            ui.add(
                                egui::TextEdit::singleline(&mut self.d_recherche_modele)
                                    .desired_width(INPUT_W)
                                    .hint_text("Toyota, SUV, Blanc…"),
                            );
                        });
                        ui.add_space(10.0);
                        ui.horizontal(|ui| {
                            if bouton_principal(ui, "Voir disponibilités par type") {
                                self.chercher_dispo_modele();
                            }
                            ui.add_space(8.0);
                            if bouton_neutre(ui, "Effacer", 90.0) {
                                self.d_recherche_modele.clear();
                                self.d_modele_periode.clear();
                                self.d_erreur.clear();
                            }
                        });
                    });
                },
            );

            ui.add_space(12.0);

            ui.allocate_ui_with_layout(
                Vec2::new(right_w, 0.0),
                egui::Layout::top_down(egui::Align::LEFT),
                |ui| {
                    panneau().show(ui, |ui| {
                        ui.label(RichText::new("Résultats").size(15.0).color(text()).strong());
                        ui.separator();
                        ui.add_space(8.0);

                        let today = Local::now().date_naive();
                        let du_period = parse_date(&self.d_du).unwrap_or(today);
                        let au_period = parse_date(&self.d_au).unwrap_or(today);

                        if self.d_fait {
                            let cards: Vec<Voiture> = self
                                .voitures
                                .iter()
                                .filter(|v| self.d_ids.contains(&v.id))
                                .cloned()
                                .collect();
                            if cards.is_empty() {
                                ui.colored_label(
                                    RED_IAM,
                                    "Aucun véhicule disponible sur cette période.",
                                );
                            } else {
                                ui.label(
                                    RichText::new(format!(
                                        "{} véhicule(s) disponible(s) du {} au {}",
                                        cards.len(),
                                        self.d_du,
                                        self.d_au
                                    ))
                                    .size(13.0)
                                    .color(GREEN),
                                );
                                ui.add_space(10.0);

                                let card_w = (ui.available_width() - 8.0) / 2.0;
                                for row in cards.chunks(2) {
                                    ui.horizontal_top(|ui| {
                                        for voit in row {
                                            let statut =
                                                self.statut_voiture(voit.id, du_period, au_period);
                                            let mut clique_detail = false;
                                            let mut clique_louer = false;
                                            let fill = if statut.couleur == RED_IAM {
                                                tinted_card(RED_IAM, 60)
                                            } else if statut.couleur == AMBER {
                                                tinted_card(AMBER, 60)
                                            } else if statut.couleur
                                                == Color32::from_rgb(99, 102, 241)
                                            {
                                                tinted_card(Color32::from_rgb(99, 102, 241), 60)
                                            } else if statut.couleur == muted() {
                                                tinted_card(muted(), 40)
                                            } else {
                                                card()
                                            };

                                            ui.allocate_ui_with_layout(
                                                Vec2::new(card_w, 0.0),
                                                egui::Layout::top_down(egui::Align::LEFT),
                                                |ui| {
                                                    egui::Frame {
                                                        fill,
                                                        stroke: Stroke::new(1.0, statut.couleur),
                                                        inner_margin: Margin::same(12.0),
                                                        rounding: Rounding::same(10.0),
                                                        ..Default::default()
                                                    }
                                                    .show(ui, |ui| {
                                                        ui.set_min_width(card_w - 8.0);
                                                        let title = ui.add(
                                                            egui::Button::new(
                                                                RichText::new(&voit.modele)
                                                                    .size(14.0)
                                                                    .color(text())
                                                                    .strong(),
                                                            )
                                                            .fill(Color32::TRANSPARENT)
                                                            .rounding(3.0),
                                                        );
                                                        if title.clicked() {
                                                            clique_detail = true;
                                                        }
                                                        title.on_hover_text(
                                                            "Voir la fiche détaillée",
                                                        );
                                                        ui.label(
                                                            RichText::new(&voit.plaque)
                                                                .size(12.0)
                                                                .color(muted()),
                                                        );
                                                        ui.add_space(6.0);
                                                        egui::Grid::new(format!("dv{}", voit.id))
                                                            .num_columns(2)
                                                            .spacing([8.0, 4.0])
                                                            .show(ui, |ui| {
                                                                for (k, v) in [
                                                                    (
                                                                        "Catégorie :",
                                                                        voit.categorie.as_str(),
                                                                    ),
                                                                    (
                                                                        "Couleur :",
                                                                        voit.couleur.as_str(),
                                                                    ),
                                                                    (
                                                                        "Année :",
                                                                        &voit.annee.to_string(),
                                                                    ),
                                                                    (
                                                                        "Tarif :",
                                                                        &format!(
                                                                            "{:.0} DA/jour",
                                                                            voit.tarif_jour
                                                                        ),
                                                                    ),
                                                                    (
                                                                        "État :",
                                                                        if voit.etat.is_empty() {
                                                                            "Bon état"
                                                                        } else {
                                                                            voit.etat.as_str()
                                                                        },
                                                                    ),
                                                                ] {
                                                                    ui.label(
                                                                        RichText::new(k)
                                                                            .size(12.0)
                                                                            .color(muted()),
                                                                    );
                                                                    ui.label(
                                                                        RichText::new(v)
                                                                            .size(12.5)
                                                                            .color(text()),
                                                                    );
                                                                    ui.end_row();
                                                                }
                                                            });
                                                        ui.add_space(8.0);
                                                        ui.horizontal(|ui| {
                                                            point_couleur(ui, statut.couleur);
                                                            ui.label(
                                                                RichText::new(&statut.libelle)
                                                                    .size(12.5)
                                                                    .color(statut.couleur)
                                                                    .strong(),
                                                            );
                                                        });
                                                        for (txt, col) in
                                                            statut.badges.iter().take(2)
                                                        {
                                                            ui.label(
                                                                RichText::new(txt)
                                                                    .size(11.0)
                                                                    .color(*col),
                                                            );
                                                        }
                                                        if !statut.lignes.is_empty() {
                                                            ui.add_space(4.0);
                                                            for (txt, col) in
                                                                statut.lignes.iter().take(2)
                                                            {
                                                                ui.label(
                                                                    RichText::new(txt)
                                                                        .size(11.0)
                                                                        .color(*col),
                                                                );
                                                            }
                                                        }
                                                        ui.add_space(8.0);
                                                        if ui
                                                            .add_sized(
                                                                [card_w - 40.0, 34.0],
                                                                egui::Button::new(
                                                                    RichText::new(
                                                                        "Louer ce véhicule",
                                                                    )
                                                                    .size(13.5)
                                                                    .color(WHITE),
                                                                )
                                                                .fill(RED_IAM)
                                                                .rounding(6.0),
                                                            )
                                                            .clicked()
                                                        {
                                                            clique_louer = true;
                                                        }
                                                    });
                                                },
                                            );
                                            if clique_detail {
                                                open_voiture_modal = Some(voit.id);
                                            }
                                            if clique_louer {
                                                louer_id = Some(voit.id);
                                            }
                                            ui.add_space(8.0);
                                        }
                                    });
                                    ui.add_space(10.0);
                                }

                                if let Some(vid) = louer_id {
                                    if let Some(idx) =
                                        self.voitures.iter().position(|v| v.id == vid)
                                    {
                                        self.f_voiture = idx + 1;
                                        self.f_debut = self.d_du.clone();
                                        self.f_fin = self.d_au.clone();
                                        self.f_tarif = self.voitures[idx].tarif_jour.to_string();
                                        self.f_msg.clear();
                                        self.naviguer(Onglet::NouveauContrat);
                                    }
                                }
                            }
                        }

                        if !self.d_modele_periode.is_empty() {
                            ui.add_space(4.0);
                            ui.separator();
                            ui.add_space(8.0);
                            ui.label(
                                RichText::new("Véhicules correspondant à votre recherche")
                                    .size(14.0)
                                    .color(GREEN)
                                    .strong(),
                            );
                            ui.add_space(10.0);
                            let items = self.d_modele_periode.clone();
                            for (vid, modele, statut, detail) in &items {
                                let col = match statut.as_str() {
                                    s if s.contains("LIBRE") => GREEN,
                                    s if s.contains("FUTUR") => Color32::from_rgb(99, 102, 241),
                                    s if s.contains("PÉRIODE") || s.contains("PERIODE") => RED_IAM,
                                    s if s.contains("IMPAYÉ") || s.contains("IMPAYE") => AMBER,
                                    s if s.contains("TERMIN") => muted(),
                                    _ => GREEN,
                                };
                                egui::Frame {
                                    fill: if col == RED_IAM {
                                        tinted_card(RED_IAM, 60)
                                    } else if col == AMBER {
                                        tinted_card(AMBER, 60)
                                    } else if col == Color32::from_rgb(99, 102, 241) {
                                        tinted_card(Color32::from_rgb(99, 102, 241), 60)
                                    } else if col == muted() {
                                        tinted_card(muted(), 40)
                                    } else {
                                        card()
                                    },
                                    stroke: Stroke::new(1.0, col),
                                    inner_margin: Margin::same(10.0),
                                    rounding: Rounding::same(8.0),
                                    ..Default::default()
                                }
                                .show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        let btn = ui.add(
                                            egui::Button::new(
                                                RichText::new(modele)
                                                    .size(13.5)
                                                    .color(text())
                                                    .strong(),
                                            )
                                            .fill(Color32::TRANSPARENT)
                                            .rounding(3.0),
                                        );
                                        if btn.clicked() {
                                            open_voiture_modal = Some(*vid);
                                        }
                                        btn.on_hover_text("Voir la fiche détaillée");
                                        ui.add_space(12.0);
                                        point_couleur(ui, col);
                                        ui.label(
                                            RichText::new(statut).size(11.5).color(col).strong(),
                                        );
                                    });
                                    ui.add_space(4.0);
                                    ui.label(RichText::new(detail).size(11.5).color(muted()));
                                });
                                ui.add_space(8.0);
                            }
                        }

                        if self.d_fait && self.d_ids.is_empty() && self.d_modele_periode.is_empty()
                        {
                            ui.add_space(4.0);
                            ui.colored_label(
                                muted(),
                                "Lancez une recherche pour voir les véhicules ici.",
                            );
                        }
                    });
                },
            );
        });

        if let Some(vid) = open_voiture_modal {
            self.modal_voiture = ModalVoiture {
                visible: true,
                voiture_id: vid,
                histo_recherche: String::new(),
            };
        }
    }

    fn chercher_dispo_periode(&mut self) {
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
            (Some(_), Some(_)) => {
                self.d_erreur = "La date de début doit être avant la date de fin.".into()
            }
            _ => self.d_erreur = "Format invalide. Utilisez JJ/MM/AAAA.".into(),
        }
    }

    fn chercher_dispo_modele(&mut self) {
        self.d_modele_periode.clear();
        self.d_erreur.clear();
        let q = self.d_recherche_modele.trim().to_lowercase();
        if q.is_empty() {
            self.d_erreur = "Entrez un modèle ou un type à rechercher.".into();
            return;
        }

        let today = Local::now().date_naive();
        for v in &self.voitures {
            let desc = format!("{} {} {}", v.modele, v.categorie, v.couleur).to_lowercase();
            if desc.contains(&q) {
                let statut = self.statut_voiture(v.id, today, today);
                let mut detail_parts = Vec::new();
                for (txt, _) in &statut.lignes {
                    detail_parts.push(txt.clone());
                }
                if detail_parts.is_empty() {
                    detail_parts.push(format!("{:.0} DA/jour", v.tarif_jour));
                }
                self.d_modele_periode.push((
                    v.id,
                    v.modele.clone(),
                    statut.libelle.clone(),
                    detail_parts.join(" • "),
                ));
            }
        }
        if self.d_modele_periode.is_empty() {
            self.d_erreur = "Aucune voiture ne correspond à cette recherche.".into();
        }
    }

    fn page_nouveau(&mut self, ui: &mut egui::Ui) {
        let titre = if self.f_en_edition.is_some() {
            "Modification du Contrat"
        } else {
            "Nouveau Contrat de Location"
        };
        titre_page(ui, titre);

        panneau().show(ui, |ui| {
            let sug = self.numero_suggere();
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new("Suggestion de numéro :")
                        .size(12.0)
                        .color(muted()),
                );
                ui.label(RichText::new(&sug).size(12.5).color(RED_IAM));
            });
            ui.add_space(10.0);

            egui::Grid::new("fn_form")
                .num_columns(2)
                .spacing([8.0, 12.0])
                .show(ui, |ui| {
                    etiquette(ui, "Numéro de Contrat *");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.f_numero)
                            .desired_width(INPUT_W)
                            .hint_text("ex: IAM-2026-0001"),
                    );
                    ui.end_row();

                    etiquette(ui, "Véhicule *");
                    {
                        let lbl = if self.f_voiture == 0 || self.f_voiture > self.voitures.len() {
                            if self.f_voiture_note.is_empty() {
                                "-- Choisir un véhicule --".to_string()
                            } else {
                                format!(">> {} <<", self.f_voiture_note)
                            }
                        } else {
                            let v = &self.voitures[self.f_voiture - 1];
                            format!("{} ({})", v.modele, v.plaque)
                        };

                        let mut sorted_voitures: Vec<(usize, &Voiture)> =
                            self.voitures.iter().enumerate().collect();
                        sorted_voitures.sort_by(|a, b| {
                            a.1.modele.to_lowercase().cmp(&b.1.modele.to_lowercase())
                        });

                        let search_q = self.f_voiture_search.to_lowercase();
                        let filtered: Vec<(usize, &Voiture)> = sorted_voitures
                            .iter()
                            .filter(|(_, v)| {
                                search_q.is_empty()
                                    || v.modele.to_lowercase().contains(&search_q)
                                    || v.plaque.to_lowercase().contains(&search_q)
                            })
                            .map(|(i, v)| (*i, *v))
                            .collect();

                        let old_voiture = self.f_voiture;
                        egui::ComboBox::from_id_salt("fn_voit")
                            .selected_text(&lbl)
                            .width(INPUT_W)
                            .show_ui(ui, |ui| {
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.f_voiture_search)
                                        .desired_width(INPUT_W - 20.0)
                                        .hint_text("Chercher un vehicule..."),
                                );
                                ui.separator();
                                ui.selectable_value(
                                    &mut self.f_voiture,
                                    0,
                                    "-- Aucune voiture sélectionnée --",
                                );
                                for (i, v) in &filtered {
                                    ui.selectable_value(
                                        &mut self.f_voiture,
                                        i + 1,
                                        format!(
                                            "{} — {} | {:.0} DA/j",
                                            v.modele, v.plaque, v.tarif_jour
                                        ),
                                    );
                                }
                                ui.separator();
                                ui.label(
                                    RichText::new("-- Ou décrire le type --")
                                        .size(11.0)
                                        .color(muted()),
                                );
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.f_voiture_note)
                                        .desired_width(INPUT_W - 20.0)
                                        .hint_text("ex: auto transmission…"),
                                );
                            });

                        if self.f_voiture != old_voiture {
                            self.f_car_error_cleared = true;
                            if !self.f_ok {
                                self.f_msg.clear();
                            }
                        }

                        if self.f_tarif.trim().is_empty()
                            && self.f_voiture > 0
                            && self.f_voiture <= self.voitures.len()
                        {
                            self.f_tarif = self.voitures[self.f_voiture - 1].tarif_jour.to_string();
                        }
                    }
                    ui.end_row();

                    etiquette(ui, "Nom du Client *");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.f_client)
                            .desired_width(INPUT_W)
                            .hint_text("Nom complet"),
                    );
                    ui.end_row();

                    etiquette(ui, "Téléphone");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.f_tel)
                            .desired_width(INPUT_W)
                            .hint_text("ex: 0551 234 567"),
                    );
                    ui.end_row();

                    etiquette(ui, "Agent Commercial *");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.f_agent)
                            .desired_width(INPUT_W)
                            .hint_text("Nom de l'agent IAM"),
                    );
                    ui.end_row();

                    etiquette(ui, "Date de Début *");
                    ui.horizontal(|ui| {
                        ui.add(
                            egui::TextEdit::singleline(&mut self.f_debut)
                                .desired_width(150.0)
                                .hint_text("JJ/MM/AAAA"),
                        );
                        ui.add_space(6.0);
                        ui.label(RichText::new("Heure :").size(12.0).color(muted()));
                        ui.add(
                            egui::TextEdit::singleline(&mut self.f_heure_debut)
                                .desired_width(65.0)
                                .hint_text("HH:MM"),
                        );
                    });
                    ui.end_row();

                    etiquette(ui, "Date de Fin *");
                    ui.horizontal(|ui| {
                        ui.add(
                            egui::TextEdit::singleline(&mut self.f_fin)
                                .desired_width(150.0)
                                .hint_text("JJ/MM/AAAA"),
                        );
                        ui.add_space(6.0);
                        ui.label(RichText::new("Heure :").size(12.0).color(muted()));
                        ui.add(
                            egui::TextEdit::singleline(&mut self.f_heure_fin)
                                .desired_width(65.0)
                                .hint_text("HH:MM"),
                        );
                    });
                    ui.end_row();

                    etiquette(ui, "Kilométrage Départ *");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.f_km_dep)
                            .desired_width(INPUT_W)
                            .hint_text("ex: 12500"),
                    );
                    ui.end_row();

                    etiquette(ui, "Kilométrage Retour");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.f_km_ret)
                            .desired_width(INPUT_W)
                            .hint_text("À remplir au retour"),
                    );
                    ui.end_row();

                    etiquette(ui, "Prix / Jour (DA) *");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.f_tarif)
                            .desired_width(INPUT_W)
                            .hint_text("3000"),
                    );
                    ui.end_row();

                    etiquette(ui, "Montant Payé Auj. *");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.f_montant_paye)
                            .desired_width(INPUT_W)
                            .hint_text("0 ou montant partiel"),
                    );
                    ui.end_row();

                    etiquette(ui, "Notes");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.f_notes)
                            .desired_width(INPUT_W)
                            .hint_text("Observations"),
                    );
                    ui.end_row();
                });

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(8.0);

            let vok = self.f_voiture >= 1 && self.f_voiture <= self.voitures.len();
            let s = parse_date(&self.f_debut);
            let e = parse_date(&self.f_fin);
            let rate: Option<f64> = self.f_tarif.trim().parse().ok();
            let paye: f64 = self.f_montant_paye.trim().parse().unwrap_or(0.0);

            if (vok || !self.f_voiture_note.is_empty())
                && s.is_some()
                && e.is_some()
                && rate.is_some()
            {
                let debut = s.unwrap();
                let fin = e.unwrap();
                let tarif = rate.unwrap();
                if debut <= fin {
                    let jours = (fin - debut).num_days().max(1);
                    let total = jours as f64 * tarif;
                    let reste = if paye >= total { 0.0 } else { total - paye };
                    egui::Frame {
                        fill: tinted_card(Color32::from_rgb(59, 130, 246), 50),
                        stroke: Stroke::new(1.0, border()),
                        inner_margin: Margin::same(12.0),
                        rounding: Rounding::same(6.0),
                        ..Default::default()
                    }
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                ui.label(
                                    RichText::new(format!("{} jours", jours))
                                        .size(13.0)
                                        .color(text()),
                                );
                                ui.label(
                                    RichText::new(format!("{:.0} DA/jour", tarif))
                                        .size(13.0)
                                        .color(text()),
                                );
                            });
                            ui.add_space(24.0);
                            ui.vertical(|ui| {
                                ui.label(
                                    RichText::new(format!("Total : {:.0} DA", total))
                                        .size(15.0)
                                        .color(AMBER)
                                        .strong(),
                                );
                                ui.label(
                                    RichText::new(format!("Payé : {:.0} DA", paye))
                                        .size(13.0)
                                        .color(GREEN),
                                );
                                ui.label(
                                    RichText::new(format!("À payer : {:.0} DA", reste))
                                        .size(13.0)
                                        .color(if reste > 0.0 { RED_IAM } else { GREEN }),
                                );
                            });
                        });
                    });
                    ui.add_space(10.0);
                }
            }

            ui.horizontal(|ui| {
                if bouton_principal(
                    ui,
                    if self.f_en_edition.is_some() {
                        "Mettre à jour le Contrat"
                    } else {
                        "Enregistrer le Contrat"
                    },
                ) {
                    self.enregistrer_contrat();
                }
                ui.add_space(10.0);
                if bouton_neutre(ui, "Effacer", 200.0) {
                    self.vider_formulaire();
                }
            });

            if !self.f_msg.is_empty() {
                ui.add_space(8.0);
                let col = if self.f_ok {
                    GREEN
                } else {
                    Color32::from_rgb(220, 50, 50)
                };
                ui.label(RichText::new(&self.f_msg.clone()).size(13.5).color(col));
            }
        });
    }

    fn enregistrer_contrat(&mut self) {
        macro_rules! erreur {
            ($m:expr) => {{
                self.f_msg = $m.into();
                self.f_ok = false;
                return;
            }};
        }
        if self.f_voiture == 0 && self.f_voiture_note.trim().is_empty() {
            erreur!("Sélectionnez un véhicule ou décrivez le type.");
        }
        if self.f_client.trim().is_empty() {
            erreur!("Le nom du client est obligatoire.");
        }
        if self.f_agent.trim().is_empty() {
            erreur!("L'agent commercial est obligatoire.");
        }

        let debut = match parse_date(&self.f_debut) {
            Some(d) => d,
            None => erreur!("Date de début invalide (JJ/MM/AAAA)."),
        };
        let fin = match parse_date(&self.f_fin) {
            Some(d) => d,
            None => erreur!("Date de fin invalide (JJ/MM/AAAA)."),
        };
        if debut > fin {
            erreur!("La date de début doit être avant la date de fin.");
        }

        let tarif: f64 = if self.f_tarif.trim().is_empty() {
            if self.f_voiture > 0 && self.f_voiture <= self.voitures.len() {
                self.voitures[self.f_voiture - 1].tarif_jour
            } else {
                erreur!("Prix par jour invalide.");
            }
        } else {
            match self.f_tarif.trim().parse::<f64>() {
                Ok(t) if t > 0.0 => t,
                _ => erreur!("Prix par jour invalide."),
            }
        };

        let paye: f64 = self.f_montant_paye.trim().parse().unwrap_or(0.0);
        let num = if self.f_numero.trim().is_empty() {
            self.numero_suggere()
        } else {
            self.f_numero.trim().to_string()
        };

        if self.f_en_edition.is_none() {
            if self.contrats.iter().any(|c| c.numero == num) {
                erreur!("Ce numéro de contrat existe déjà.");
            }
        }

        let km_s: f64 = self.f_km_dep.trim().parse().unwrap_or(0.0);
        let km_e: f64 = self.f_km_ret.trim().parse().unwrap_or(0.0);
        let jours = (fin - debut).num_days().max(1);

        if self.f_voiture > 0 && self.f_voiture <= self.voitures.len() {
            let voit = &self.voitures[self.f_voiture - 1];
            let exclude_id = self
                .f_en_edition
                .and_then(|i| self.contrats.get(i))
                .map(|c| c.id);
            if !self.est_disponible(voit.id, debut, fin, exclude_id) {
                erreur!("Ce véhicule est déjà loué sur cette période !");
            }
        }

        let voit_modele = if self.f_voiture > 0 && self.f_voiture <= self.voitures.len() {
            self.voitures[self.f_voiture - 1].modele.clone()
        } else {
            format!("À spécifier: {}", self.f_voiture_note)
        };
        let voit_id = if self.f_voiture > 0 && self.f_voiture <= self.voitures.len() {
            self.voitures[self.f_voiture - 1].id
        } else {
            0
        };
        let voit_plaque = if self.f_voiture > 0 && self.f_voiture <= self.voitures.len() {
            self.voitures[self.f_voiture - 1].plaque.clone()
        } else {
            "À définir".to_string()
        };

        let msg = if let Some(edit_idx) = self.f_en_edition {
            self.contrats[edit_idx].numero = num.clone();
            self.contrats[edit_idx].voiture_id = voit_id;
            self.contrats[edit_idx].voiture_modele = voit_modele;
            self.contrats[edit_idx].voiture_plaque = voit_plaque;
            self.contrats[edit_idx].client_nom = self.f_client.trim().to_string();
            self.contrats[edit_idx].client_tel = self.f_tel.trim().to_string();
            self.contrats[edit_idx].agent = self.f_agent.trim().to_string();
            self.contrats[edit_idx].date_debut = stocker_date(debut);
            self.contrats[edit_idx].date_fin = stocker_date(fin);
            self.contrats[edit_idx].km_depart = km_s;
            self.contrats[edit_idx].km_retour = km_e;
            self.contrats[edit_idx].tarif_jour = tarif;
            self.contrats[edit_idx].notes = self.f_notes.trim().to_string();
            self.contrats[edit_idx].montant_paye = paye;
            self.contrats[edit_idx].modifiable_note = self.f_voiture_note.trim().to_string();
            self.contrats[edit_idx].heure_debut = self.f_heure_debut.trim().to_string();
            self.contrats[edit_idx].heure_fin = self.f_heure_fin.trim().to_string();
            format!("Contrat {} modifié avec succès.", num)
        } else {
            self.contrats.push(Contrat {
                id: self.prochain_id_contrat(),
                numero: num.clone(),
                voiture_id: voit_id,
                voiture_modele: voit_modele,
                voiture_plaque: voit_plaque,
                client_nom: self.f_client.trim().to_string(),
                client_tel: self.f_tel.trim().to_string(),
                agent: self.f_agent.trim().to_string(),
                date_debut: stocker_date(debut),
                date_fin: stocker_date(fin),
                km_depart: km_s,
                km_retour: km_e,
                tarif_jour: tarif,
                notes: self.f_notes.trim().to_string(),
                statut: "Actif".into(),
                montant_paye: paye,
                modifiable_note: self.f_voiture_note.trim().to_string(),
                heure_debut: self.f_heure_debut.trim().to_string(),
                heure_fin: self.f_heure_fin.trim().to_string(),
            });
            format!(
                "Contrat {} enregistré. {} jours – {:.0} DA",
                num,
                jours,
                jours as f64 * tarif
            )
        };
        sauvegarder(&rentals_file(), &self.contrats);
        let agent = self.f_agent.clone();
        self.vider_formulaire();
        self.f_agent = agent;
        self.f_msg = msg;
        self.f_ok = true;
    }

    fn page_contrats(&mut self, ui: &mut egui::Ui) {
        titre_page(ui, "Gestion des Contrats");

        panneau().show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.label(RichText::new("Recherche :").size(13.0).color(text()));
                let old_q = self.c_recherche.clone();
                ui.add(
                    egui::TextEdit::singleline(&mut self.c_recherche)
                        .desired_width(260.0)
                        .hint_text("Contrat, client, véhicule, agent…"),
                );
                if self.c_recherche != old_q {
                    self.c_page = 0;
                }

                ui.add_space(12.0);
                ui.label(RichText::new("Statut :").size(13.0).color(text()));
                for (i, lbl) in ["Tous", "Actifs", "Terminés", "Annulés", "Réservés"]
                    .iter()
                    .enumerate()
                {
                    let sel = self.c_statut == i;
                    if ui
                        .add(
                            egui::Button::new(RichText::new(*lbl).size(13.0).color(if sel {
                                WHITE
                            } else {
                                muted()
                            }))
                            .fill(if sel { RED_IAM } else { card2() })
                            .rounding(5.0),
                        )
                        .clicked()
                    {
                        self.c_statut = i;
                        self.c_page = 0;
                    }
                }

                ui.add_space(12.0);
                ui.label(RichText::new("Mois :").size(12.0).color(muted()));
                let old_m = self.c_filter_mois;
                egui::ComboBox::from_id_salt("c_mois")
                    .selected_text(if self.c_filter_mois == 0 {
                        "Tous".to_string()
                    } else {
                        format!("{:02}", self.c_filter_mois)
                    })
                    .width(70.0)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.c_filter_mois, 0, "Tous");
                        for m in 1..=12u32 {
                            let noms = [
                                "Jan", "Fév", "Mar", "Avr", "Mai", "Jun", "Jul", "Aoû", "Sep",
                                "Oct", "Nov", "Déc",
                            ];
                            ui.selectable_value(
                                &mut self.c_filter_mois,
                                m as usize,
                                noms[(m - 1) as usize],
                            );
                        }
                    });
                if self.c_filter_mois != old_m {
                    self.c_page = 0;
                }

                ui.label(RichText::new("Année :").size(12.0).color(muted()));
                let old_y = self.c_filter_annee.clone();
                ui.add(
                    egui::TextEdit::singleline(&mut self.c_filter_annee)
                        .desired_width(70.0)
                        .hint_text("2026"),
                );
                if self.c_filter_annee != old_y {
                    self.c_page = 0;
                }
            });
        });

        ui.add_space(10.0);

        let mut suppr_confirme: Option<usize> = None;
        if let Some(idx) = self.c_suppr {
            egui::Frame {
                fill: tinted_card(RED_IAM, 80),
                inner_margin: Margin::same(14.0),
                rounding: Rounding::same(7.0),
                stroke: Stroke::new(1.5, Color32::from_rgb(220, 40, 40)),
                ..Default::default()
            }
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new("Confirmer la suppression ? Cette action est définitive.")
                            .size(13.5)
                            .color(Color32::from_rgb(220, 40, 40)),
                    );
                    ui.add_space(16.0);
                    if bouton_danger(ui, "Oui, Supprimer", 140.0) {
                        suppr_confirme = Some(idx);
                        self.c_suppr = None;
                    }
                    ui.add_space(8.0);
                    if bouton_neutre(ui, "Annuler", 100.0) {
                        self.c_suppr = None;
                    }
                });
            });
            ui.add_space(8.0);
        }
        if let Some(i) = suppr_confirme {
            self.contrats.remove(i);
            sauvegarder(&rentals_file(), &self.contrats);
        }

        let gc_today = Local::now().date_naive();
        let q = self.c_recherche.to_lowercase();
        let annee_filtre: Option<i32> = self.c_filter_annee.trim().parse().ok();

        let lignes: Vec<(usize, Contrat)> = self
            .contrats
            .iter()
            .enumerate()
            .filter(|(_, c)| {
                let statut_ok = match self.c_statut {
                    0 => true,
                    1 => {
                        c.statut == "Actif"
                            && NaiveDate::parse_from_str(&c.date_debut, "%Y-%m-%d")
                                .map(|d| d <= gc_today)
                                .unwrap_or(true)
                    }
                    2 => c.statut == "Terminé",
                    3 => c.statut == "Annulé",
                    4 => {
                        c.statut == "Actif"
                            && NaiveDate::parse_from_str(&c.date_debut, "%Y-%m-%d")
                                .map(|d| d > gc_today)
                                .unwrap_or(false)
                    }
                    _ => true,
                };
                (statut_ok)
                    && (q.is_empty()
                        || c.numero.to_lowercase().contains(&q)
                        || c.client_nom.to_lowercase().contains(&q)
                        || c.voiture_modele.to_lowercase().contains(&q)
                        || c.agent.to_lowercase().contains(&q))
                    && (self.c_filter_mois == 0
                        || NaiveDate::parse_from_str(&c.date_debut, "%Y-%m-%d")
                            .map(|d| d.month() as usize == self.c_filter_mois)
                            .unwrap_or(false))
                    && (annee_filtre.is_none()
                        || NaiveDate::parse_from_str(&c.date_debut, "%Y-%m-%d")
                            .map(|d| d.year() == annee_filtre.unwrap())
                            .unwrap_or(false))
            })
            .map(|(i, c)| (i, c.clone()))
            .rev()
            .collect();

        if lignes.is_empty() {
            panneau().show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(14.0);
                    ui.label(
                        RichText::new("Aucun contrat trouvé.")
                            .size(14.0)
                            .color(muted()),
                    );
                    ui.add_space(14.0);
                });
            });
            return;
        }

        // Pagination
        let total = lignes.len();
        let total_pages = (total + ITEMS_PER_PAGE - 1) / ITEMS_PER_PAGE;
        if self.c_page >= total_pages {
            self.c_page = 0;
        }
        let start = self.c_page * ITEMS_PER_PAGE;
        let page_lignes: Vec<(usize, Contrat)> = lignes
            .into_iter()
            .skip(start)
            .take(ITEMS_PER_PAGE)
            .collect();

        let mut faire_terminer: Option<usize> = None;
        let mut faire_annuler: Option<usize> = None;
        let mut faire_supprimer: Option<usize> = None;
        let mut faire_modifier: Option<usize> = None;
        let mut open_contrat_modal: Option<u64> = None;
        let auj_contrats = Local::now().date_naive();

        for (orig, c) in &page_lignes {
            let (sc, ss) = match c.statut.as_str() {
                "Actif" => (GREEN, "Actif"),
                "Terminé" => (muted(), "Terminé"),
                "Annulé" => (Color32::from_rgb(180, 30, 40), "Annulé"),
                _ => (muted(), c.statut.as_str()),
            };
            let reste = c.reste_a_payer();
            let pct = if c.total() > 0.0 {
                (c.montant_paye / c.total() * 100.0).min(100.0)
            } else {
                0.0
            };

            // Future reservation detection
            let is_future = parse_date(&c.date_debut)
                .map(|d| d > auj_contrats)
                .unwrap_or(false);

            // Airport delivery detection
            let notes_lower = c.notes.to_lowercase();
            let is_aeroport = notes_lower.contains("aero")
                || notes_lower.contains("aéro")
                || notes_lower.contains("aeroport")
                || notes_lower.contains("aéroport");

            let card_fill = if is_future {
                tinted_card(Color32::from_rgb(99, 102, 241), 55)
            } else {
                card2()
            };
            let card_stroke = if is_future {
                Color32::from_rgb(99, 102, 241)
            } else {
                border()
            };

            egui::Frame {
                fill: card_fill,
                inner_margin: Margin::same(12.0),
                rounding: Rounding::same(8.0),
                stroke: Stroke::new(if is_future { 1.5 } else { 1.0 }, card_stroke),
                ..Default::default()
            }
            .show(ui, |ui| {
                ui.set_min_height(125.0);
                ui.set_max_height(125.0);

                let btn_col_w = 90.0;
                let available = ui.available_width();
                let info_w = (available - btn_col_w - 16.0).max(180.0);

                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.set_min_width(info_w * 0.52);
                        ui.set_max_width(info_w * 0.52);

                        ui.horizontal(|ui| {
                            let btn = ui.add(
                                egui::Button::new(
                                    RichText::new(&c.numero).size(13.5).color(RED_IAM).strong(),
                                )
                                .fill(Color32::TRANSPARENT)
                                .rounding(3.0),
                            );
                            if btn.clicked() {
                                open_contrat_modal = Some(c.id);
                            }
                            let hover = if c.notes.is_empty() {
                                "Voir les détails complets".to_string()
                            } else {
                                format!("📝 {}", c.notes)
                            };
                            btn.on_hover_text(hover);
                            if is_aeroport {
                                ui.label(
                                    RichText::new("✈ AEROPORT")
                                        .size(10.5)
                                        .color(Color32::from_rgb(220, 100, 0))
                                        .strong(),
                                );
                            }
                            if is_future {
                                ui.label(
                                    RichText::new("Reservation future")
                                        .size(10.0)
                                        .color(Color32::from_rgb(140, 130, 255)),
                                );
                            }
                        });

                        ui.label(RichText::new(&c.client_nom).size(12.5).color(text()));
                        ui.label(
                            RichText::new(format!("🚗 {}", c.voiture_modele))
                                .size(11.5)
                                .color(muted()),
                        );
                        ui.label(
                            RichText::new(format!(
                                "📅 {} au {}",
                                afficher_date(&c.date_debut),
                                afficher_date(&c.date_fin)
                            ))
                            .size(11.0)
                            .color(muted()),
                        );
                        ui.label(
                            RichText::new(format!("👤 {}", c.agent))
                                .size(11.0)
                                .color(muted()),
                        );
                    });

                    ui.add_space(8.0);

                    ui.vertical(|ui| {
                        ui.set_min_width(info_w * 0.48);
                        ui.set_max_width(info_w * 0.48);
                        ui.label(RichText::new(ss).size(12.0).color(sc).strong());
                        ui.add_space(4.0);
                        ui.label(
                            RichText::new(format!("Total : {:.0} DA", c.total()))
                                .size(13.0)
                                .color(AMBER)
                                .strong(),
                        );
                        ui.label(
                            RichText::new(format!("Payé : {:.0} DA ({:.0}%)", c.montant_paye, pct))
                                .size(11.5)
                                .color(GREEN),
                        );
                        ui.label(
                            RichText::new(format!("Reste : {:.0} DA", reste))
                                .size(11.5)
                                .color(if reste > 0.0 { RED_IAM } else { GREEN }),
                        );
                    });

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.set_min_width(btn_col_w);
                        ui.set_max_width(btn_col_w);
                        ui.vertical(|ui| {
                            // Modifier always visible for all statuses
                            if bouton_modification(ui, "Modifier", btn_col_w) {
                                faire_modifier = Some(*orig);
                            }
                            ui.add_space(4.0);
                            if c.statut == "Actif" {
                                if bouton_vert(ui, "Terminer", btn_col_w) {
                                    faire_terminer = Some(*orig);
                                }
                                ui.add_space(4.0);
                                if bouton_danger(ui, "Annuler", btn_col_w) {
                                    faire_annuler = Some(*orig);
                                }
                            } else {
                                if bouton_danger(ui, "Supprimer", btn_col_w) {
                                    faire_supprimer = Some(*orig);
                                }
                            }
                        });
                    });
                });
            });
            ui.add_space(6.0);
        }

        if total_pages > 1 {
            ui.add_space(8.0);
            panneau().show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new(format!(
                            "{} contrats  •  Page {} / {}",
                            total,
                            self.c_page + 1,
                            total_pages
                        ))
                        .size(12.0)
                        .color(muted()),
                    );
                    ui.add_space(16.0);
                    if bouton_neutre(ui, "‹ Préc.", 80.0) && self.c_page > 0 {
                        self.c_page -= 1;
                    }
                    for p in 0..total_pages.min(8) {
                        let active = self.c_page == p;
                        if ui
                            .add_sized(
                                [26.0, 26.0],
                                egui::Button::new(
                                    RichText::new(format!("{}", p + 1))
                                        .size(12.0)
                                        .color(if active { WHITE } else { muted() }),
                                )
                                .fill(if active { RED_IAM } else { card2() })
                                .rounding(4.0),
                            )
                            .clicked()
                        {
                            self.c_page = p;
                        }
                    }
                    if total_pages > 8 {
                        ui.label(RichText::new("…").size(13.0).color(muted()));
                        if ui
                            .add_sized(
                                [26.0, 26.0],
                                egui::Button::new(
                                    RichText::new(format!("{}", total_pages)).size(12.0).color(
                                        if self.c_page == total_pages - 1 {
                                            WHITE
                                        } else {
                                            muted()
                                        },
                                    ),
                                )
                                .fill(if self.c_page == total_pages - 1 {
                                    RED_IAM
                                } else {
                                    card2()
                                })
                                .rounding(4.0),
                            )
                            .clicked()
                        {
                            self.c_page = total_pages - 1;
                        }
                    }
                    if bouton_neutre(ui, "Suiv. ›", 80.0) && self.c_page + 1 < total_pages {
                        self.c_page += 1;
                    }
                });
            });
        }

        if let Some(i) = faire_modifier {
            if let Some(c) = self.contrats.get(i) {
                self.f_numero = c.numero.clone();
                self.f_voiture = self
                    .voitures
                    .iter()
                    .position(|v| v.id == c.voiture_id)
                    .map(|x| x + 1)
                    .unwrap_or(0);
                self.f_voiture_note = c.modifiable_note.clone();
                self.f_voiture_search.clear();
                self.f_client = c.client_nom.clone();
                self.f_tel = c.client_tel.clone();
                self.f_agent = c.agent.clone();
                self.f_debut = afficher_date(&c.date_debut);
                self.f_fin = afficher_date(&c.date_fin);
                self.f_km_dep = c.km_depart.to_string();
                self.f_km_ret = c.km_retour.to_string();
                self.f_tarif = c.tarif_jour.to_string();
                self.f_montant_paye = c.montant_paye.to_string();
                self.f_notes = c.notes.clone();
                self.f_msg.clear();
                self.f_ok = false;
                self.f_en_edition = Some(i);
                self.naviguer(Onglet::NouveauContrat);
            }
        }
        if let Some(i) = faire_terminer {
            self.contrats[i].statut = "Terminé".into();
            sauvegarder(&rentals_file(), &self.contrats);
        }
        if let Some(i) = faire_annuler {
            self.contrats[i].statut = "Annulé".into();
            sauvegarder(&rentals_file(), &self.contrats);
        }
        if let Some(i) = faire_supprimer {
            self.c_suppr = Some(i);
        }
        if let Some(cid) = open_contrat_modal {
            self.modal_contrat = ModalContrat {
                visible: true,
                contrat_id: cid,
            };
        }
    }

    fn page_voitures(&mut self, ui: &mut egui::Ui) {
        titre_page(ui, "Gestion du Parc de Voitures");

        // Boite de confirmation suppression voiture
        if let Some(del_id) = self.v_suppr_confirm {
            let voit_nom = self
                .voitures
                .iter()
                .find(|v| v.id == del_id)
                .map(|v| v.modele.clone())
                .unwrap_or_default();
            egui::Window::new("Confirmer la Suppression")
                .collapsible(false)
                .resizable(false)
                .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
                .fixed_size([420.0, 0.0])
                .show(ui.ctx(), |ui| {
                    ui.add_space(8.0);
                    ui.label(
                        RichText::new(format!(
                            "Voulez-vous vraiment supprimer \"{}\" du parc ?",
                            voit_nom
                        ))
                        .size(14.0)
                        .color(text()),
                    );
                    ui.add_space(4.0);
                    ui.colored_label(
                        AMBER,
                        "Cette action est définitive et supprimera la voiture du fichier CSV.",
                    );
                    ui.add_space(12.0);
                    ui.horizontal(|ui| {
                        if bouton_danger(ui, "Oui, Supprimer", 150.0) {
                            self.voitures.retain(|v| v.id != del_id);
                            sauvegarder(&cars_file(), &self.voitures);
                            self.v_suppr_confirm = None;
                        }
                        ui.add_space(10.0);
                        if bouton_neutre(ui, "Annuler", 120.0) {
                            self.v_suppr_confirm = None;
                        }
                    });
                    ui.add_space(8.0);
                });
        }

        let tw = ui.available_width();
        let left_w = 390.0;
        let right_w = (tw - left_w - 16.0).max(400.0);

        // Edit car dialog
        let mut save_edit: Option<u64> = None;
        let mut cancel_edit = false;

        if let Some(edit_id) = self.v_edit_id {
            egui::Window::new("Modifier la voiture")
                .collapsible(false)
                .resizable(false)
                .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
                .fixed_size([460.0, 0.0])
                .show(ui.ctx(), |ui| {
                    egui::Grid::new("v_edit_form")
                        .num_columns(2)
                        .spacing([8.0, 10.0])
                        .show(ui, |ui| {
                            etiquette(ui, "Modèle *");
                            ui.add(
                                egui::TextEdit::singleline(&mut self.v_edit_modele)
                                    .desired_width(INPUT_W),
                            );
                            ui.end_row();
                            etiquette(ui, "Plaque *");
                            ui.add(
                                egui::TextEdit::singleline(&mut self.v_edit_plaque)
                                    .desired_width(INPUT_W),
                            );
                            ui.end_row();
                            etiquette(ui, "Année");
                            ui.add(
                                egui::TextEdit::singleline(&mut self.v_edit_annee)
                                    .desired_width(INPUT_W),
                            );
                            ui.end_row();
                            etiquette(ui, "Catégorie");
                            egui::ComboBox::from_id_salt("v_edit_cat")
                                .selected_text(&self.v_edit_cat)
                                .width(INPUT_W)
                                .show_ui(ui, |ui| {
                                    for c in [
                                        "Berline", "Citadine", "SUV", "Break", "Minivan", "Pickup",
                                        "Luxe",
                                    ] {
                                        ui.selectable_value(&mut self.v_edit_cat, c.to_string(), c);
                                    }
                                });
                            ui.end_row();
                            etiquette(ui, "Couleur");
                            ui.add(
                                egui::TextEdit::singleline(&mut self.v_edit_couleur)
                                    .desired_width(INPUT_W),
                            );
                            ui.end_row();
                            etiquette(ui, "DA/Jour *");
                            ui.add(
                                egui::TextEdit::singleline(&mut self.v_edit_tarif)
                                    .desired_width(INPUT_W),
                            );
                            ui.end_row();
                            etiquette(ui, "État");
                            egui::ComboBox::from_id_salt("v_edit_etat")
                                .selected_text(&self.v_edit_etat)
                                .width(INPUT_W)
                                .show_ui(ui, |ui| {
                                    for e in ["Bon état", "En maintenance"] {
                                        ui.selectable_value(
                                            &mut self.v_edit_etat,
                                            e.to_string(),
                                            e,
                                        );
                                    }
                                });
                            ui.end_row();
                        });
                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        if bouton_vert(ui, "Sauvegarder", 140.0) {
                            save_edit = Some(edit_id);
                        }
                        ui.add_space(8.0);
                        if bouton_neutre(ui, "Annuler", 100.0) {
                            cancel_edit = true;
                        }
                    });
                });

            if let Some(id) = save_edit {
                if let Some(v) = self.voitures.iter_mut().find(|v| v.id == id) {
                    if !self.v_edit_modele.trim().is_empty() {
                        v.modele = self.v_edit_modele.trim().to_string();
                    }
                    if !self.v_edit_plaque.trim().is_empty() {
                        v.plaque = self.v_edit_plaque.trim().to_string();
                    }
                    if let Ok(a) = self.v_edit_annee.trim().parse::<u32>() {
                        v.annee = a;
                    }
                    if !self.v_edit_cat.is_empty() {
                        v.categorie = self.v_edit_cat.clone();
                    }
                    if !self.v_edit_couleur.trim().is_empty() {
                        v.couleur = self.v_edit_couleur.trim().to_string();
                    }
                    if let Ok(t) = self.v_edit_tarif.trim().parse::<f64>() {
                        if t > 0.0 {
                            v.tarif_jour = t;
                        }
                    }
                    if !self.v_edit_etat.is_empty() {
                        v.etat = self.v_edit_etat.clone();
                    }
                }
                sauvegarder(&cars_file(), &self.voitures);
                self.v_edit_id = None;
            }
            if cancel_edit {
                self.v_edit_id = None;
            }
        }

        ui.horizontal_top(|ui| {
            ui.allocate_ui_with_layout(
                Vec2::new(left_w, 0.0),
                egui::Layout::top_down(egui::Align::LEFT),
                |ui| {
                    panneau().show(ui, |ui| {
                        ui.horizontal(|ui| {
                            logo_widget(ui, true);
                            ui.add_space(10.0);
                            ui.vertical(|ui| {
                                ui.label(
                                    RichText::new("Ajouter une Voiture")
                                        .size(18.0)
                                        .color(RED_IAM)
                                        .strong(),
                                );
                                ui.label(
                                    RichText::new("Remplissez le formulaire")
                                        .size(12.0)
                                        .color(muted()),
                                );
                            });
                        });
                        ui.add_space(12.0);
                        egui::Grid::new("v_form")
                            .num_columns(2)
                            .spacing([8.0, 12.0])
                            .show(ui, |ui| {
                                etiquette(ui, "Modèle *");
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.v_modele)
                                        .desired_width(INPUT_W)
                                        .hint_text("Toyota Corolla 2024"),
                                );
                                ui.end_row();
                                etiquette(ui, "Plaque *");
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.v_plaque)
                                        .desired_width(INPUT_W)
                                        .hint_text("100-IAM-06"),
                                );
                                ui.end_row();
                                etiquette(ui, "Année");
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.v_annee)
                                        .desired_width(INPUT_W)
                                        .hint_text("2024"),
                                );
                                ui.end_row();
                                etiquette(ui, "Catégorie");
                                egui::ComboBox::from_id_salt("v_cat")
                                    .selected_text(&self.v_cat)
                                    .width(INPUT_W)
                                    .show_ui(ui, |ui| {
                                        for c in [
                                            "Berline", "Citadine", "SUV", "Break", "Minivan",
                                            "Pickup", "Luxe",
                                        ] {
                                            ui.selectable_value(&mut self.v_cat, c.to_string(), c);
                                        }
                                    });
                                ui.end_row();
                                etiquette(ui, "Couleur");
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.v_couleur)
                                        .desired_width(INPUT_W)
                                        .hint_text("Blanc"),
                                );
                                ui.end_row();
                                etiquette(ui, "DA/Jour *");
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.v_tarif)
                                        .desired_width(INPUT_W)
                                        .hint_text("3000"),
                                );
                                ui.end_row();
                                etiquette(ui, "État");
                                egui::ComboBox::from_id_salt("v_new_etat")
                                    .selected_text(&self.v_edit_etat)
                                    .width(INPUT_W)
                                    .show_ui(ui, |ui| {
                                        for e in ["Bon état", "En maintenance"] {
                                            ui.selectable_value(
                                                &mut self.v_edit_etat,
                                                e.to_string(),
                                                e,
                                            );
                                        }
                                    });
                                ui.end_row();
                            });
                        ui.add_space(12.0);
                        if bouton_principal(ui, "Ajouter la Voiture") {
                            self.ajouter_voiture();
                        }
                        if !self.v_msg.is_empty() {
                            ui.add_space(8.0);
                            ui.colored_label(
                                if self.v_ok {
                                    GREEN
                                } else {
                                    Color32::from_rgb(220, 50, 50)
                                },
                                &self.v_msg.clone(),
                            );
                        }
                    });
                },
            );

            ui.add_space(16.0);

            ui.allocate_ui_with_layout(
                Vec2::new(right_w, 0.0),
                egui::Layout::top_down(egui::Align::LEFT),
                |ui| {
                    panneau().show(ui, |ui| {
                        // Search bar
                        ui.horizontal(|ui| {
                            ui.label(
                                RichText::new(format!("Parc — {} voitures", self.voitures.len()))
                                    .size(14.0)
                                    .color(RED_IAM)
                                    .strong(),
                            );
                            ui.add_space(16.0);
                            let old_q = self.v_recherche.clone();
                            ui.add(
                                egui::TextEdit::singleline(&mut self.v_recherche)
                                    .desired_width(200.0)
                                    .hint_text("Rechercher..."),
                            );
                            if self.v_recherche != old_q {
                                self.v_page = 0;
                            }
                        });
                        ui.add_space(10.0);

                        let q = self.v_recherche.to_lowercase();
                        let snap: Vec<Voiture> = self
                            .voitures
                            .iter()
                            .filter(|v| {
                                q.is_empty()
                                    || v.modele.to_lowercase().contains(&q)
                                    || v.plaque.to_lowercase().contains(&q)
                                    || v.categorie.to_lowercase().contains(&q)
                            })
                            .cloned()
                            .collect();

                        let total = snap.len();
                        let cards_per_page = 6usize;
                        let total_pages_cards = (total + cards_per_page - 1) / cards_per_page;
                        if self.v_page >= total_pages_cards {
                            self.v_page = 0;
                        }
                        let start = self.v_page * cards_per_page;
                        let page_snap: Vec<&Voiture> =
                            snap.iter().skip(start).take(cards_per_page).collect();

                        let card_w = (ui.available_width() - 8.0) / 2.0;
                        let card_h = 210.0;
                        let mut a_supprimer: Option<usize> = None;
                        let mut open_edit: Option<u64> = None;
                        let mut open_modal: Option<u64> = None;

                        for row in page_snap.chunks(2) {
                            ui.horizontal_top(|ui| {
                                for voit in row {
                                    let actual_idx = self
                                        .voitures
                                        .iter()
                                        .position(|x| x.id == voit.id)
                                        .unwrap_or(0);
                                    let mut suppr = false;
                                    let auj = Local::now().date_naive();
                                    let statut = self.statut_voiture(voit.id, auj, auj);

                                    ui.allocate_ui_with_layout(
                                        Vec2::new(card_w, card_h),
                                        egui::Layout::top_down(egui::Align::LEFT),
                                        |ui| {
                                            panneau().show(ui, |ui| {
                                                ui.set_min_width(card_w - 8.0);
                                                ui.set_min_height(card_h - 8.0);

                                                ui.horizontal(|ui| {
                                                    logo_widget(ui, false);
                                                    ui.add_space(8.0);
                                                    ui.vertical(|ui| {
                                                        let name_btn = ui.add(
                                                            egui::Button::new(
                                                                RichText::new(&voit.modele)
                                                                    .size(13.5)
                                                                    .color(text())
                                                                    .strong(),
                                                            )
                                                            .fill(Color32::TRANSPARENT)
                                                            .rounding(3.0),
                                                        );
                                                        if name_btn.clicked() {
                                                            open_modal = Some(voit.id);
                                                        }
                                                        name_btn.on_hover_text(
                                                            "Voir la fiche complète",
                                                        );
                                                        ui.label(
                                                            RichText::new(&voit.plaque)
                                                                .size(12.0)
                                                                .color(RED_IAM),
                                                        );
                                                    });
                                                });

                                                ui.add_space(6.0);
                                                egui::Grid::new(format!("vi{}", voit.id))
                                                    .num_columns(2)
                                                    .spacing([8.0, 3.0])
                                                    .show(ui, |ui| {
                                                        for (k, v) in [
                                                            (
                                                                "Catégorie :",
                                                                voit.categorie.as_str(),
                                                            ),
                                                            ("Couleur :", voit.couleur.as_str()),
                                                            ("Année :", &voit.annee.to_string()),
                                                            (
                                                                "Tarif :",
                                                                &format!(
                                                                    "{:.0} DA/j",
                                                                    voit.tarif_jour
                                                                ),
                                                            ),
                                                        ] {
                                                            ui.label(
                                                                RichText::new(k)
                                                                    .size(11.0)
                                                                    .color(muted()),
                                                            );
                                                            ui.label(
                                                                RichText::new(v)
                                                                    .size(11.5)
                                                                    .color(text()),
                                                            );
                                                            ui.end_row();
                                                        }
                                                        ui.label(
                                                            RichText::new("État :")
                                                                .size(11.0)
                                                                .color(muted()),
                                                        );
                                                        let etat_str = if voit.etat.is_empty() {
                                                            "Bon état"
                                                        } else {
                                                            voit.etat.as_str()
                                                        };
                                                        ui.label(
                                                            RichText::new(etat_str)
                                                                .size(11.5)
                                                                .color(
                                                                    if voit.etat == "En maintenance"
                                                                    {
                                                                        AMBER
                                                                    } else {
                                                                        GREEN
                                                                    },
                                                                ),
                                                        );
                                                        ui.end_row();
                                                    });

                                                ui.add_space(6.0);
                                                ui.horizontal(|ui| {
                                                    point_couleur(ui, statut.couleur);
                                                    ui.label(
                                                        RichText::new(&statut.libelle)
                                                            .size(12.0)
                                                            .color(statut.couleur)
                                                            .strong(),
                                                    );
                                                });
                                                if !statut.badges.is_empty() {
                                                    ui.add_space(3.0);
                                                    ui.horizontal_wrapped(|ui| {
                                                        for (txt, col) in
                                                            statut.badges.iter().take(2)
                                                        {
                                                            ui.label(
                                                                RichText::new(txt)
                                                                    .size(11.0)
                                                                    .color(*col),
                                                            );
                                                        }
                                                    });
                                                }
                                                for (txt, col) in statut.lignes.iter().take(1) {
                                                    ui.label(
                                                        RichText::new(txt).size(11.0).color(*col),
                                                    );
                                                }

                                                ui.add_space(6.0);
                                                ui.horizontal(|ui| {
                                                    let bw = (card_w - 50.0) / 2.0;
                                                    if bouton_modification(ui, "Modifier", bw) {
                                                        open_edit = Some(voit.id);
                                                    }
                                                    if bouton_danger(ui, "Retirer", bw) {
                                                        suppr = true;
                                                    }
                                                });
                                            });
                                        },
                                    );

                                    if suppr {
                                        a_supprimer = Some(actual_idx);
                                    }
                                }
                            });
                            ui.add_space(8.0);
                        }

                        // Pagination
                        if total_pages_cards > 1 {
                            ui.add_space(4.0);
                            ui.horizontal(|ui| {
                                ui.label(
                                    RichText::new(format!(
                                        "{} voitures • Page {}/{}",
                                        total,
                                        self.v_page + 1,
                                        total_pages_cards
                                    ))
                                    .size(11.0)
                                    .color(muted()),
                                );
                                if bouton_neutre(ui, "‹", 30.0) && self.v_page > 0 {
                                    self.v_page -= 1;
                                }
                                for p in 0..total_pages_cards {
                                    let active = self.v_page == p;
                                    if ui
                                        .add_sized(
                                            [26.0, 26.0],
                                            egui::Button::new(
                                                RichText::new(format!("{}", p + 1))
                                                    .size(12.0)
                                                    .color(if active { WHITE } else { muted() }),
                                            )
                                            .fill(if active { RED_IAM } else { card2() })
                                            .rounding(4.0),
                                        )
                                        .clicked()
                                    {
                                        self.v_page = p;
                                    }
                                }
                                if bouton_neutre(ui, "›", 30.0)
                                    && self.v_page + 1 < total_pages_cards
                                {
                                    self.v_page += 1;
                                }
                            });
                        }

                        // Confirmation de suppression
                        if let Some(i) = a_supprimer {
                            if let Some(v) = self.voitures.get(i) {
                                self.v_suppr_confirm = Some(v.id);
                            }
                        }
                        if let Some(id) = open_edit {
                            if let Some(v) = self.voitures.iter().find(|v| v.id == id) {
                                self.v_edit_id = Some(id);
                                self.v_edit_modele = v.modele.clone();
                                self.v_edit_plaque = v.plaque.clone();
                                self.v_edit_annee = v.annee.to_string();
                                self.v_edit_cat = v.categorie.clone();
                                self.v_edit_couleur = v.couleur.clone();
                                self.v_edit_tarif = v.tarif_jour.to_string();
                                self.v_edit_etat = if v.etat.is_empty() {
                                    "Bon état".to_string()
                                } else {
                                    v.etat.clone()
                                };
                            }
                        }
                        if let Some(vid) = open_modal {
                            self.modal_voiture = ModalVoiture {
                                visible: true,
                                voiture_id: vid,
                                histo_recherche: String::new(),
                            };
                        }
                    });
                },
            );
        });
    }

    fn page_ventes(&mut self, ui: &mut egui::Ui) {
        titre_page(ui, "Ventes de Voitures");

        let tw = ui.available_width();
        let lw = (tw - 16.0) * 0.40;
        let rw = tw - lw - 16.0;

        ui.horizontal_top(|ui| {
            // Formulaire ajout
            ui.allocate_ui_with_layout(Vec2::new(lw, 0.0), egui::Layout::top_down(egui::Align::LEFT), |ui| {
                panneau().show(ui, |ui| {
                    ui.label(RichText::new("Ajouter une voiture à vendre").size(16.0).color(RED_IAM).strong());
                    ui.add_space(8.0);
                    ui.separator();
                    ui.add_space(8.0);
                    egui::Grid::new("vt_form").num_columns(2).spacing([8.0, 10.0]).show(ui, |ui| {
                        etiquette(ui, "Nom / Modèle *");
                        ui.add(egui::TextEdit::singleline(&mut self.vt_nom).desired_width(INPUT_W).hint_text("Toyota Corolla 2020"));
                        ui.end_row();
                        etiquette(ui, "Plaque *");
                        ui.add(egui::TextEdit::singleline(&mut self.vt_plaque).desired_width(INPUT_W).hint_text("100-xxx-01"));
                        ui.end_row();
                        etiquette(ui, "N° Châssis *");
                        ui.add(egui::TextEdit::singleline(&mut self.vt_chassis).desired_width(INPUT_W).hint_text("VIN / Numéro de châssis"));
                        ui.end_row();
                        etiquette(ui, "N° Immatriculation");
                        ui.add(egui::TextEdit::singleline(&mut self.vt_immat).desired_width(INPUT_W).hint_text("Numéro d'immatriculation"));
                        ui.end_row();
                        etiquette(ui, "Année");
                        ui.add(egui::TextEdit::singleline(&mut self.vt_annee).desired_width(INPUT_W).hint_text("2020"));
                        ui.end_row();
                        etiquette(ui, "Prix d'achat (DA)");
                        ui.add(egui::TextEdit::singleline(&mut self.vt_prix_achat).desired_width(INPUT_W).hint_text("0 si non renseigné"));
                        ui.end_row();
                        etiquette(ui, "Prix demandé (DA) *");
                        ui.add(egui::TextEdit::singleline(&mut self.vt_prix_demande).desired_width(INPUT_W).hint_text("Prix de vente souhaité"));
                        ui.end_row();
                        etiquette(ui, "Notes");
                        ui.add(egui::TextEdit::singleline(&mut self.vt_notes).desired_width(INPUT_W).hint_text("Observations, état..."));
                        ui.end_row();
                    });
                    ui.add_space(12.0);
                    if bouton_principal(ui, "Ajouter à la liste") {
                        self.ajouter_vente_voiture();
                    }
                    if !self.vt_msg.is_empty() {
                        ui.add_space(6.0);
                        ui.colored_label(if self.vt_ok { GREEN } else { RED_IAM }, &self.vt_msg.clone());
                    }
                });
            });

            ui.add_space(16.0);

            // Liste des voitures a vendre
            ui.allocate_ui_with_layout(Vec2::new(rw, 0.0), egui::Layout::top_down(egui::Align::LEFT), |ui| {
                panneau().show(ui, |ui| {
                    let total_v = self.ventes.len();
                    let vendues = self.ventes.iter().filter(|v| v.vendu).count();
                    let non_vendues = total_v - vendues;
                    let ca_ventes: f64 = self.ventes.iter().filter(|v| v.vendu).map(|v| v.prix_vendu).sum();
                    let benefice_total: f64 = self.ventes.iter().filter(|v| v.vendu && v.prix_achat > 0.0).map(|v| v.benefice()).sum();

                    ui.horizontal(|ui| {
                        let sw = (rw - 50.0) / 4.0;
                        for (lbl, val, c) in [
                            ("Total", total_v.to_string(), muted()),
                            ("Vendues", vendues.to_string(), GREEN),
                            ("En vente", non_vendues.to_string(), AMBER),
                            ("CA Ventes", format!("{:.0} DA", ca_ventes), Color32::from_rgb(168, 85, 247)),
                        ] {
                            egui::Frame { fill: card2(), inner_margin: Margin::same(10.0), rounding: Rounding::same(8.0), stroke: Stroke::new(1.0, border()), ..Default::default() }
                            .show(ui, |ui| {
                                ui.set_min_width(sw);
                                ui.label(RichText::new(lbl).size(11.0).color(muted()));
                                ui.label(RichText::new(val).size(20.0).color(c).strong());
                            });
                            ui.add_space(6.0);
                        }
                    });
                    if benefice_total > 0.0 {
                        ui.add_space(4.0);
                        ui.colored_label(GREEN, format!("Bénéfice total estimé : {:.0} DA", benefice_total));
                    }

                    ui.add_space(10.0);
                    ui.separator();
                    ui.add_space(8.0);

                    ui.horizontal(|ui| {
                        ui.label(RichText::new(format!("{} voitures", total_v)).size(14.0).color(RED_IAM).strong());
                        ui.add_space(16.0);
                        ui.add(egui::TextEdit::singleline(&mut self.vt_recherche).desired_width(180.0).hint_text("Rechercher..."));
                    });
                    ui.add_space(8.0);

                    let q = self.vt_recherche.to_lowercase();
                    let snap: Vec<VoitureVente> = self.ventes.iter().filter(|v| {
                        q.is_empty() || v.nom.to_lowercase().contains(&q) || v.plaque.to_lowercase().contains(&q) || v.num_chassis.to_lowercase().contains(&q)
                    }).cloned().collect();

                    let per_page = 5usize;
                    let total_pages = (snap.len() + per_page - 1) / per_page;
                    if self.vt_page >= total_pages.max(1) { self.vt_page = 0; }
                    let start = self.vt_page * per_page;
                    let page_items: Vec<&VoitureVente> = snap.iter().skip(start).take(per_page).collect();

                    // Confirmation suppression vente
                    if let Some(del_id) = self.vt_suppr_confirm {
                        egui::Frame { fill: tinted_card(RED_IAM, 80), inner_margin: Margin::same(12.0), rounding: Rounding::same(7.0), stroke: Stroke::new(1.5, RED_IAM), ..Default::default() }
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(RichText::new("Supprimer cette voiture de la liste ? Action définitive.").size(12.5).color(RED_IAM));
                                ui.add_space(10.0);
                                if bouton_danger(ui, "Oui, Supprimer", 120.0) {
                                    self.ventes.retain(|v| v.id != del_id);
                                    sauvegarder(&ventes_file(), &self.ventes);
                                    self.vt_suppr_confirm = None;
                                }
                                ui.add_space(6.0);
                                if bouton_neutre(ui, "Annuler", 80.0) { self.vt_suppr_confirm = None; }
                            });
                        });
                        ui.add_space(6.0);
                    }

                    if let Some(sell_id) = self.vt_marquer_vendu {
                        if let Some(v) = self.ventes.iter().find(|v| v.id == sell_id) {
                            let nom = v.nom.clone();
                            egui::Frame { fill: tinted_card(GREEN, 80), inner_margin: Margin::same(12.0), rounding: Rounding::same(7.0), stroke: Stroke::new(1.5, GREEN), ..Default::default() }
                            .show(ui, |ui| {
                                ui.label(RichText::new(format!("Marquer \"{}\" comme vendue", nom)).size(12.5).color(GREEN));
                                ui.horizontal(|ui| {
                                    ui.label(RichText::new("Prix vendu (DA) :").size(12.0).color(text()));
                                    ui.add(egui::TextEdit::singleline(&mut self.vt_prix_vendu_input).desired_width(120.0).hint_text("0"));
                                    ui.add_space(8.0);
                                    if bouton_vert(ui, "Confirmer Vendue", 140.0) {
                                        let prix: f64 = self.vt_prix_vendu_input.trim().parse().unwrap_or(0.0);
                                        let today_iso = stocker_date(Local::now().date_naive());
                                        if let Some(v) = self.ventes.iter_mut().find(|v| v.id == sell_id) {
                                            v.vendu = true;
                                            v.prix_vendu = prix;
                                            v.date_vente = today_iso;
                                        }
                                        sauvegarder(&ventes_file(), &self.ventes);
                                        self.vt_marquer_vendu = None;
                                        self.vt_prix_vendu_input.clear();
                                    }
                                    if bouton_neutre(ui, "Annuler", 80.0) {
                                        self.vt_marquer_vendu = None;
                                        self.vt_prix_vendu_input.clear();
                                    }
                                });
                            });
                            ui.add_space(6.0);
                        }
                    }

                    if page_items.is_empty() {
                        ui.colored_label(muted(), "Aucune voiture dans la liste de ventes.");
                    } else {
                        let mut to_del: Option<u64> = None;
                        let mut to_sell: Option<u64> = None;
                        let mut to_unsell: Option<u64> = None;

                        for voit in &page_items {
                            let (card_bg, card_border) = if voit.vendu {
                                (tinted_card(GREEN, 55), GREEN)
                            } else {
                                (card2(), border())
                            };
                            egui::Frame { fill: card_bg, inner_margin: Margin::same(12.0), rounding: Rounding::same(8.0), stroke: Stroke::new(1.0, card_border), ..Default::default() }
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    ui.vertical(|ui| {
                                        ui.horizontal(|ui| {
                                            ui.label(RichText::new(&voit.nom).size(13.5).color(text()).strong());
                                            ui.add_space(8.0);
                                            if voit.vendu {
                                                ui.label(RichText::new("VENDUE").size(11.0).color(GREEN).strong());
                                            } else {
                                                ui.label(RichText::new("EN VENTE").size(11.0).color(AMBER).strong());
                                            }
                                        });
                                        egui::Grid::new(format!("vt_{}", voit.id)).num_columns(4).spacing([12.0, 3.0]).show(ui, |ui| {
                                            ui.label(RichText::new("Plaque:").size(11.0).color(muted()));
                                            ui.label(RichText::new(&voit.plaque).size(11.5).color(text()));
                                            ui.label(RichText::new("Châssis:").size(11.0).color(muted()));
                                            ui.label(RichText::new(&voit.num_chassis).size(11.5).color(text()));
                                            ui.end_row();
                                            ui.label(RichText::new("Immat:").size(11.0).color(muted()));
                                            ui.label(RichText::new(&voit.num_immat).size(11.5).color(text()));
                                            ui.label(RichText::new("Année:").size(11.0).color(muted()));
                                            ui.label(RichText::new(voit.annee.to_string()).size(11.5).color(text()));
                                            ui.end_row();
                                            if voit.prix_achat > 0.0 {
                                                ui.label(RichText::new("Achat:").size(11.0).color(muted()));
                                                ui.label(RichText::new(format!("{:.0} DA", voit.prix_achat)).size(11.5).color(muted()));
                                            } else {
                                                ui.label(""); ui.label("");
                                            }
                                            ui.label(RichText::new("Demandé:").size(11.0).color(muted()));
                                            ui.label(RichText::new(format!("{:.0} DA", voit.prix_demande)).size(11.5).color(AMBER));
                                            ui.end_row();
                                            if voit.vendu {
                                                ui.label(RichText::new("Vendu:").size(11.0).color(muted()));
                                                ui.label(RichText::new(format!("{:.0} DA", voit.prix_vendu)).size(11.5).color(GREEN).strong());
                                                if !voit.date_vente.is_empty() {
                                                    ui.label(RichText::new("Date:").size(11.0).color(muted()));
                                                    ui.label(RichText::new(afficher_date(&voit.date_vente)).size(11.0).color(muted()));
                                                } else {
                                                    ui.label(""); ui.label("");
                                                }
                                                ui.end_row();
                                                if voit.prix_achat > 0.0 {
                                                    ui.label(RichText::new("Bénéfice:").size(11.0).color(muted()));
                                                    let ben = voit.benefice();
                                                    ui.label(RichText::new(format!("{:.0} DA", ben)).size(11.5).color(if ben >= 0.0 { GREEN } else { RED_IAM }).strong());
                                                    ui.label(""); ui.label("");
                                                    ui.end_row();
                                                }
                                            }
                                            if !voit.notes.is_empty() {
                                                ui.label(RichText::new("Notes:").size(11.0).color(muted()));
                                                ui.label(RichText::new(&voit.notes).size(11.0).color(muted()));
                                                ui.label(""); ui.label("");
                                                ui.end_row();
                                            }
                                        });
                                    });

                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                                        if bouton_danger(ui, "Supprimer", 90.0) { to_del = Some(voit.id); }
                                        ui.add_space(4.0);
                                        if voit.vendu {
                                            if bouton_neutre(ui, "Non vendue", 90.0) { to_unsell = Some(voit.id); }
                                        } else {
                                            if bouton_vert(ui, "Marquer Vendue", 120.0) { to_sell = Some(voit.id); }
                                        }
                                    });
                                });
                            });
                            ui.add_space(6.0);
                        }

                        if let Some(id) = to_del { self.vt_suppr_confirm = Some(id); }
                        if let Some(id) = to_sell { self.vt_marquer_vendu = Some(id); self.vt_prix_vendu_input.clear(); }
                        if let Some(id) = to_unsell {
                            if let Some(v) = self.ventes.iter_mut().find(|v| v.id == id) {
                                v.vendu = false; v.prix_vendu = 0.0; v.date_vente.clear();
                            }
                            sauvegarder(&ventes_file(), &self.ventes);
                        }
                    }

                    // Pagination
                    if total_pages > 1 {
                        ui.add_space(8.0);
                        ui.horizontal(|ui| {
                            ui.label(RichText::new(format!("Page {}/{}", self.vt_page + 1, total_pages)).size(11.0).color(muted()));
                            if bouton_neutre(ui, "<", 30.0) && self.vt_page > 0 { self.vt_page -= 1; }
                            if bouton_neutre(ui, ">", 30.0) && self.vt_page + 1 < total_pages { self.vt_page += 1; }
                        });
                    }
                });
            });
        });
    }

    fn ajouter_vente_voiture(&mut self) {
        macro_rules! erreur {
            ($m:expr) => {{
                self.vt_msg = $m.into();
                self.vt_ok = false;
                return;
            }};
        }
        if self.vt_nom.trim().is_empty() {
            erreur!("Le nom/modèle est obligatoire.");
        }
        if self.vt_plaque.trim().is_empty() {
            erreur!("La plaque est obligatoire.");
        }
        if self.vt_chassis.trim().is_empty() {
            erreur!("Le numéro de châssis est obligatoire.");
        }
        if self
            .ventes
            .iter()
            .any(|v| v.num_chassis.trim().to_lowercase() == self.vt_chassis.trim().to_lowercase())
        {
            erreur!("Une voiture avec ce numéro de châssis existe déjà.");
        }
        let prix_demande: f64 = match self.vt_prix_demande.trim().parse::<f64>() {
            Ok(p) if p >= 0.0 => p,
            _ => erreur!("Prix demandé invalide."),
        };
        let prix_achat: f64 = self.vt_prix_achat.trim().parse().unwrap_or(0.0);
        let annee: u32 = self.vt_annee.trim().parse().unwrap_or(2020);
        let id = self.ventes.iter().map(|v| v.id).max().unwrap_or(0) + 1;
        let v = VoitureVente {
            id,
            nom: self.vt_nom.trim().to_string(),
            plaque: self.vt_plaque.trim().to_string(),
            num_chassis: self.vt_chassis.trim().to_string(),
            num_immat: self.vt_immat.trim().to_string(),
            annee,
            prix_achat,
            prix_demande,
            prix_vendu: 0.0,
            vendu: false,
            date_vente: String::new(),
            notes: self.vt_notes.trim().to_string(),
        };
        self.vt_msg = format!("\"{}\" ajoutée à la liste de ventes.", v.nom);
        self.vt_ok = true;
        self.ventes.push(v);
        sauvegarder(&ventes_file(), &self.ventes);
        self.vt_nom.clear();
        self.vt_plaque.clear();
        self.vt_chassis.clear();
        self.vt_immat.clear();
        self.vt_prix_achat.clear();
        self.vt_prix_demande.clear();
        self.vt_notes.clear();
        self.vt_annee = Local::now().year().to_string();
    }

    fn ajouter_voiture(&mut self) {
        macro_rules! erreur {
            ($m:expr) => {{
                self.v_msg = $m.into();
                self.v_ok = false;
                return;
            }};
        }
        if self.v_modele.trim().is_empty() {
            erreur!("Le modèle est obligatoire.");
        }
        if self.v_plaque.trim().is_empty() {
            erreur!("La plaque est obligatoire.");
        }
        if self
            .voitures
            .iter()
            .any(|v| v.plaque.trim().to_lowercase() == self.v_plaque.trim().to_lowercase())
        {
            erreur!("Une voiture avec cette plaque existe déjà.");
        }
        let tarif: f64 = match self.v_tarif.trim().parse::<f64>() {
            Ok(t) if t > 0.0 => t,
            _ => erreur!("Tarif journalier invalide."),
        };
        let annee: u32 = self.v_annee.trim().parse().unwrap_or(2024);
        let etat_val = if self.v_edit_etat.is_empty() {
            "Bon état".to_string()
        } else {
            self.v_edit_etat.clone()
        };
        let v = Voiture {
            id: self.prochain_id_voiture(),
            modele: self.v_modele.trim().to_string(),
            plaque: self.v_plaque.trim().to_string(),
            categorie: self.v_cat.clone(),
            couleur: self.v_couleur.trim().to_string(),
            annee,
            tarif_jour: tarif,
            etat: etat_val,
        };
        self.v_msg = format!("'{}' ajouté avec succès.", v.modele);
        self.v_ok = true;
        self.voitures.push(v);
        sauvegarder(&cars_file(), &self.voitures);
        self.v_modele.clear();
        self.v_plaque.clear();
        self.v_couleur.clear();
        self.v_tarif = "3000".into();
        self.v_annee = Local::now().year().to_string();
        self.v_edit_etat = "Bon état".to_string();
    }

    fn page_maintenance(&mut self, ui: &mut egui::Ui) {
        titre_page(ui, "Réparations & Historique");

        let mut confirmed_del: Option<u64> = None;
        if let Some(del_id) = self.rep_suppr_confirm {
            egui::Window::new("Confirmer")
                .collapsible(false)
                .resizable(false)
                .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
                .fixed_size([340.0, 0.0])
                .show(ui.ctx(), |ui| {
                    ui.add_space(6.0);
                    ui.label(
                        RichText::new("Supprimer cette réparation ?")
                            .size(13.5)
                            .color(text()),
                    );
                    ui.add_space(12.0);
                    ui.horizontal(|ui| {
                        if bouton_danger(ui, "Oui, Supprimer", 130.0) {
                            confirmed_del = Some(del_id);
                            self.rep_suppr_confirm = None;
                        }
                        ui.add_space(8.0);
                        if bouton_neutre(ui, "Annuler", 90.0) {
                            self.rep_suppr_confirm = None;
                        }
                    });
                    ui.add_space(6.0);
                });
        }
        if let Some(id) = confirmed_del {
            self.reparations.retain(|r| r.id != id);
            sauvegarder(&reparations_file(), &self.reparations);
        }

        ui.add_space(6.0);

        let total_w = ui.available_width();
        let form_w = 340.0_f32.min(total_w * 0.32);
        let list_w = total_w - form_w - 20.0;

        let voitures_snap: Vec<(u64, String, String)> = self
            .voitures
            .iter()
            .map(|v| (v.id, v.modele.clone(), v.plaque.clone()))
            .collect();

        ui.horizontal_top(|ui| {
            ui.allocate_ui_with_layout(
                egui::Vec2::new(form_w, ui.available_height()),
                egui::Layout::top_down(egui::Align::Min),
                |ui| {
                    egui::Frame {
                        fill: card2(),
                        inner_margin: Margin::same(16.0),
                        rounding: Rounding::same(10.0),
                        stroke: Stroke::new(1.5, border()),
                        ..Default::default()
                    }
                    .show(ui, |ui| {
                        ui.set_min_width(form_w - 32.0);
                        ui.label(
                            RichText::new("Nouvelle Réparation")
                                .size(14.0)
                                .color(RED_IAM)
                                .strong(),
                        );
                        ui.add_space(12.0);

                        ui.label(RichText::new("Voiture *").size(12.0).color(muted()));
                        let car_labels: Vec<String> = std::iter::once("— Choisir —".to_string())
                            .chain(
                                voitures_snap
                                    .iter()
                                    .map(|(_, m, p)| format!("{} — {}", m, p)),
                            )
                            .collect();
                        egui::ComboBox::from_id_salt("rep_voit")
                            .selected_text(&car_labels[self.rep_voiture_idx])
                            .width(form_w - 48.0)
                            .show_ui(ui, |ui| {
                                for (i, lbl) in car_labels.iter().enumerate() {
                                    ui.selectable_value(&mut self.rep_voiture_idx, i, lbl);
                                }
                            });
                        ui.add_space(8.0);

                        ui.label(RichText::new("Date *").size(12.0).color(muted()));
                        ui.add(
                            egui::TextEdit::singleline(&mut self.rep_date)
                                .desired_width(form_w - 48.0)
                                .hint_text("JJ/MM/AAAA"),
                        );
                        ui.add_space(8.0);

                        ui.label(RichText::new("Réparation *").size(12.0).color(muted()));
                        ui.add(
                            egui::TextEdit::singleline(&mut self.rep_description)
                                .desired_width(form_w - 48.0)
                                .hint_text("Vidange, Pneus, Freins, Courroie..."),
                        );
                        ui.add_space(8.0);

                        ui.label(RichText::new("Prix (optionnel)").size(12.0).color(muted()));
                        ui.add(
                            egui::TextEdit::singleline(&mut self.rep_prix)
                                .desired_width(form_w - 48.0)
                                .hint_text("Ex : 5 000 DA"),
                        );
                        ui.add_space(8.0);

                        ui.label(RichText::new("Observation").size(12.0).color(muted()));
                        ui.add(
                            egui::TextEdit::multiline(&mut self.rep_observation)
                                .desired_width(form_w - 48.0)
                                .desired_rows(3)
                                .hint_text("Notes, remarques..."),
                        );
                        ui.add_space(14.0);

                        if self.rep_ok {
                            ui.label(
                                RichText::new("✔ Réparation enregistrée.")
                                    .size(12.0)
                                    .color(GREEN),
                            );
                            ui.add_space(4.0);
                        }
                        if !self.rep_msg.is_empty() {
                            let msg = self.rep_msg.clone();
                            ui.label(RichText::new(msg).size(12.0).color(RED_IAM));
                            ui.add_space(4.0);
                        }

                        if bouton_principal(ui, "Enregistrer") {
                            self.rep_ok = false;
                            self.rep_msg.clear();
                            if self.rep_voiture_idx == 0 {
                                self.rep_msg = "Choisissez une voiture.".into();
                            } else if self.rep_description.trim().is_empty() {
                                self.rep_msg = "La description est obligatoire.".into();
                            } else if parse_date(&self.rep_date).is_none() {
                                self.rep_msg = "Date invalide (JJ/MM/AAAA).".into();
                            } else {
                                let (vid, vmod, vpla) =
                                    voitures_snap[self.rep_voiture_idx - 1].clone();
                                let date_iso = parse_date(&self.rep_date)
                                    .map(|d| d.format("%Y-%m-%d").to_string())
                                    .unwrap_or_default();
                                let new_id =
                                    self.reparations.iter().map(|r| r.id).max().unwrap_or(0) + 1;
                                self.reparations.push(Reparation {
                                    id: new_id,
                                    voiture_id: vid,
                                    voiture_modele: vmod,
                                    voiture_plaque: vpla,
                                    date: date_iso,
                                    description: self.rep_description.trim().to_string(),
                                    prix: self.rep_prix.trim().to_string(),
                                    observation: self.rep_observation.trim().to_string(),
                                });
                                self.reparations.sort_by(|a, b| b.date.cmp(&a.date));
                                sauvegarder(&reparations_file(), &self.reparations);
                                self.rep_description.clear();
                                self.rep_prix.clear();
                                self.rep_observation.clear();
                                self.rep_date = aujourd_hui();
                                self.rep_ok = true;
                            }
                        }
                    });
                },
            );

            ui.add_space(20.0);

            ui.allocate_ui_with_layout(
                egui::Vec2::new(list_w, ui.available_height()),
                egui::Layout::top_down(egui::Align::Min),
                |ui| {
                    egui::Frame {
                        fill: card2(),
                        inner_margin: Margin::symmetric(12.0, 10.0),
                        rounding: Rounding::same(8.0),
                        stroke: Stroke::new(1.0, border()),
                        ..Default::default()
                    }
                    .show(ui, |ui| {
                        ui.set_min_width(list_w);
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("🔍").size(14.0));
                            ui.add(
                                egui::TextEdit::singleline(&mut self.rep_recherche)
                                    .desired_width(200.0)
                                    .hint_text("Modèle, plaque, réparation..."),
                            );
                            ui.add_space(10.0);
                            ui.label(RichText::new("Voiture :").size(12.0).color(muted()));
                            let filter_labels: Vec<String> = std::iter::once("Toutes".to_string())
                                .chain(
                                    voitures_snap
                                        .iter()
                                        .map(|(_, m, p)| format!("{} — {}", m, p)),
                                )
                                .collect();
                            egui::ComboBox::from_id_salt("rep_filtre")
                                .selected_text(&filter_labels[self.rep_filtre_voiture])
                                .width(180.0)
                                .show_ui(ui, |ui| {
                                    for (i, lbl) in filter_labels.iter().enumerate() {
                                        ui.selectable_value(&mut self.rep_filtre_voiture, i, lbl);
                                    }
                                });
                            ui.add_space(8.0);
                            if bouton_neutre(ui, "Effacer", 65.0) {
                                self.rep_recherche.clear();
                                self.rep_filtre_voiture = 0;
                            }
                        });
                    });
                    ui.add_space(8.0);

                    let filter_vid: Option<u64> = if self.rep_filtre_voiture == 0 {
                        None
                    } else {
                        Some(voitures_snap[self.rep_filtre_voiture - 1].0)
                    };
                    let q = self.rep_recherche.trim().to_lowercase();

                    let shown: Vec<Reparation> = self
                        .reparations
                        .iter()
                        .filter(|r| filter_vid.map(|id| r.voiture_id == id).unwrap_or(true))
                        .filter(|r| {
                            if q.is_empty() {
                                return true;
                            }
                            r.voiture_modele.to_lowercase().contains(&q)
                                || r.voiture_plaque.to_lowercase().contains(&q)
                                || r.description.to_lowercase().contains(&q)
                                || r.observation.to_lowercase().contains(&q)
                        })
                        .cloned()
                        .collect();

                    // Cost total
                    let total_cout: f64 = shown
                        .iter()
                        .filter_map(|r| {
                            r.prix
                                .replace(" DA", "")
                                .replace(" da", "")
                                .replace('\u{00A0}', "")
                                .replace(' ', "")
                                .replace(',', ".")
                                .trim()
                                .parse::<f64>()
                                .ok()
                        })
                        .sum();

                    // Summary bar
                    egui::Frame {
                        fill: tinted_card(AMBER, 25),
                        inner_margin: Margin::symmetric(14.0, 8.0),
                        rounding: Rounding::same(8.0),
                        stroke: Stroke::new(1.0, border()),
                        ..Default::default()
                    }
                    .show(ui, |ui| {
                        ui.set_min_width(list_w);
                        ui.horizontal(|ui| {
                            ui.label(
                                RichText::new(format!("🔧  {} réparation(s)", shown.len()))
                                    .size(12.5)
                                    .color(text()),
                            );
                            if total_cout > 0.0 {
                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::Center),
                                    |ui| {
                                        ui.label(
                                            RichText::new(format!(
                                                "Coût total  {:.0} DA",
                                                total_cout
                                            ))
                                            .size(13.0)
                                            .color(AMBER)
                                            .strong(),
                                        );
                                    },
                                );
                            }
                        });
                    });
                    ui.add_space(8.0);

                    // Cards
                    if shown.is_empty() {
                        ui.add_space(40.0);
                        ui.vertical_centered(|ui| {
                            ui.label(
                                RichText::new("Aucune réparation trouvée.")
                                    .size(14.0)
                                    .color(muted()),
                            );
                        });
                    } else {
                        let mut suppr_req: Option<u64> = None;
                        egui::ScrollArea::vertical()
                            .id_salt("rep_hist_scroll")
                            .auto_shrink([false, false])
                            .show(ui, |ui| {
                                ui.set_min_width(list_w - 4.0);
                                for r in &shown {
                                    let rid = r.id;
                                    egui::Frame {
                                        fill: card2(),
                                        inner_margin: Margin::same(14.0),
                                        rounding: Rounding::same(9.0),
                                        stroke: Stroke::new(1.0, border()),
                                        ..Default::default()
                                    }
                                    .show(ui, |ui| {
                                        ui.set_min_width(list_w - 20.0);

                                        ui.horizontal(|ui| {
                                            ui.label(
                                                RichText::new(&r.voiture_modele)
                                                    .size(14.0)
                                                    .color(RED_IAM)
                                                    .strong(),
                                            );
                                            ui.add_space(6.0);
                                            egui::Frame {
                                                fill: tinted_card(AMBER, 40),
                                                inner_margin: Margin::symmetric(8.0, 3.0),
                                                rounding: Rounding::same(5.0),
                                                ..Default::default()
                                            }
                                            .show(
                                                ui,
                                                |ui| {
                                                    ui.label(
                                                        RichText::new(&r.voiture_plaque)
                                                            .size(11.5)
                                                            .color(AMBER)
                                                            .strong(),
                                                    );
                                                },
                                            );
                                            ui.with_layout(
                                                egui::Layout::right_to_left(egui::Align::Center),
                                                |ui| {
                                                    ui.label(
                                                        RichText::new(afficher_date(&r.date))
                                                            .size(12.0)
                                                            .color(muted()),
                                                    );
                                                    ui.label(RichText::new("📅").size(12.0));
                                                },
                                            );
                                        });

                                        ui.add_space(5.0);
                                        ui.separator();
                                        ui.add_space(5.0);

                                        // Description
                                        ui.horizontal(|ui| {
                                            ui.label(RichText::new("🔧").size(13.0));
                                            ui.label(
                                                RichText::new(&r.description)
                                                    .size(13.0)
                                                    .color(text()),
                                            );
                                        });

                                        // Prix
                                        if !r.prix.is_empty() {
                                            ui.add_space(3.0);
                                            ui.horizontal(|ui| {
                                                ui.label(RichText::new("💰").size(12.0));
                                                ui.label(
                                                    RichText::new(&r.prix)
                                                        .size(12.5)
                                                        .color(AMBER)
                                                        .strong(),
                                                );
                                            });
                                        }

                                        // Observation
                                        if !r.observation.is_empty() {
                                            ui.add_space(3.0);
                                            ui.horizontal(|ui| {
                                                ui.label(RichText::new("📝").size(12.0));
                                                ui.label(
                                                    RichText::new(&r.observation)
                                                        .size(11.5)
                                                        .color(muted()),
                                                );
                                            });
                                        }

                                        ui.add_space(8.0);
                                        ui.with_layout(
                                            egui::Layout::right_to_left(egui::Align::Min),
                                            |ui| {
                                                if bouton_danger(ui, "Supprimer", 90.0) {
                                                    suppr_req = Some(rid);
                                                }
                                            },
                                        );
                                    });
                                    ui.add_space(8.0);
                                }
                            });
                        if let Some(id) = suppr_req {
                            self.rep_suppr_confirm = Some(id);
                        }
                    }
                },
            );
        });
    }

    fn page_impayes(&mut self, ui: &mut egui::Ui) {
        titre_page(ui, "Clients Impayés");

        let today = Local::now().date_naive();

        let impayes: Vec<Contrat> = self
            .contrats
            .iter()
            .filter(|c: &&Contrat| c.statut == "Actif" || c.statut == "Terminé")
            .filter(|c: &&Contrat| c.reste_a_payer() > 0.0)
            .filter(|c: &&Contrat| {
                NaiveDate::parse_from_str(&c.date_debut, "%Y-%m-%d")
                    .map(|d| d <= today)
                    .unwrap_or(false)
            })
            .cloned()
            .collect();

        // Search bar
        ui.horizontal(|ui| {
            ui.label(RichText::new("🔍").size(14.0));
            ui.add(
                egui::TextEdit::singleline(&mut self.impaye_recherche)
                    .desired_width(280.0)
                    .hint_text("Client, téléphone, n° contrat, voiture..."),
            );
            if !self.impaye_recherche.is_empty() {
                if bouton_neutre(ui, "Effacer", 65.0) {
                    self.impaye_recherche.clear();
                }
            }
        });
        ui.add_space(10.0);

        let q = self.impaye_recherche.trim().to_lowercase();
        let filtered: Vec<&Contrat> = impayes
            .iter()
            .filter(|c: &&Contrat| {
                if q.is_empty() {
                    return true;
                }
                c.client_nom.to_lowercase().contains(&q)
                    || c.client_tel.to_lowercase().contains(&q)
                    || c.numero.to_lowercase().contains(&q)
                    || c.voiture_modele.to_lowercase().contains(&q)
            })
            .collect();

        let total_du: f64 = filtered.iter().map(|c: &&Contrat| c.reste_a_payer()).sum();

        egui::Frame {
            fill: tinted_card(RED_IAM, 35),
            inner_margin: Margin::symmetric(16.0, 10.0),
            rounding: Rounding::same(8.0),
            stroke: Stroke::new(1.5, RED_IAM),
            ..Default::default()
        }
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new(format!("⚠  {} contrat(s) impayé(s)", filtered.len()))
                        .size(13.5)
                        .color(RED_IAM)
                        .strong(),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        RichText::new(format!("Total dû :  {:.0} DA", total_du))
                            .size(14.0)
                            .color(RED_IAM)
                            .strong(),
                    );
                });
            });
        });
        ui.add_space(10.0);

        if filtered.is_empty() {
            ui.add_space(40.0);
            ui.vertical_centered(|ui| {
                ui.label(
                    RichText::new(if q.is_empty() {
                        "  Aucun client impayé. Tout est règlé !"
                    } else {
                        "Aucun résultat pour cette recherche."
                    })
                    .size(15.0)
                    .color(GREEN),
                );
            });
            return;
        }

        let mut open_modal: Option<u64> = None;

        egui::ScrollArea::vertical()
            .id_salt("impaye_scroll")
            .auto_shrink([false, false])
            .show(ui, |ui| {
                for c in &filtered {
                    let reste: f64 = c.reste_a_payer();
                    let overdue_days: i64 = NaiveDate::parse_from_str(&c.date_fin, "%Y-%m-%d")
                        .map(|d| {
                            if d < today && c.statut == "Actif" {
                                (today - d).num_days()
                            } else {
                                0
                            }
                        })
                        .unwrap_or(0);
                    let is_overdue = overdue_days > 0;
                    let penalite: f64 = overdue_days as f64 * c.tarif_jour;
                    let total_du: f64 = reste + penalite;

                    egui::Frame {
                        fill: card2(),
                        inner_margin: Margin::same(14.0),
                        rounding: Rounding::same(9.0),
                        stroke: Stroke::new(
                            if is_overdue { 2.0 } else { 1.0 },
                            if is_overdue { RED_IAM } else { AMBER },
                        ),
                        ..Default::default()
                    }
                    .show(ui, |ui| {
                        ui.set_min_width(ui.available_width() - 4.0);
                        let right_w = 180.0_f32;
                        ui.horizontal(|ui| {
                            // Left: all info
                            ui.vertical(|ui| {
                                ui.set_max_width(ui.available_width() - right_w - 12.0);
                                ui.horizontal(|ui| {
                                    ui.label(
                                        RichText::new(&c.client_nom)
                                            .size(14.0)
                                            .color(text())
                                            .strong(),
                                    );
                                    if !c.client_tel.is_empty() {
                                        ui.add_space(8.0);
                                        ui.label(
                                            RichText::new(format!("📞 {}", c.client_tel))
                                                .size(12.0)
                                                .color(muted()),
                                        );
                                    }
                                    if is_overdue {
                                        ui.add_space(8.0);
                                        egui::Frame {
                                            fill: tinted_card(RED_IAM, 60),
                                            inner_margin: Margin::symmetric(6.0, 2.0),
                                            rounding: Rounding::same(4.0),
                                            ..Default::default()
                                        }
                                        .show(ui, |ui| {
                                            ui.label(
                                                RichText::new(format!(
                                                    "EN RETARD — {} j",
                                                    overdue_days
                                                ))
                                                .size(10.5)
                                                .color(RED_IAM)
                                                .strong(),
                                            );
                                        });
                                    }
                                });
                                ui.add_space(3.0);
                                ui.horizontal(|ui| {
                                    ui.label(
                                        RichText::new(format!("📋 {}", c.numero))
                                            .size(12.0)
                                            .color(muted()),
                                    );
                                    ui.add_space(10.0);
                                    ui.label(
                                        RichText::new(format!(
                                            "🚗 {} ({})",
                                            c.voiture_modele, c.voiture_plaque
                                        ))
                                        .size(12.0)
                                        .color(muted()),
                                    );
                                    ui.add_space(10.0);
                                    ui.label(
                                        RichText::new(format!(
                                            "{} au {}",
                                            afficher_date(&c.date_debut),
                                            afficher_date(&c.date_fin)
                                        ))
                                        .size(12.0)
                                        .color(muted()),
                                    );
                                });
                                ui.add_space(3.0);
                                ui.horizontal(|ui| {
                                    ui.label(
                                        RichText::new(format!(
                                            "Total contrat : {:.0} DA",
                                            c.total()
                                        ))
                                        .size(12.0)
                                        .color(muted()),
                                    );
                                    ui.add_space(10.0);
                                    ui.label(
                                        RichText::new(format!("Payé : {:.0} DA", c.montant_paye))
                                            .size(12.0)
                                            .color(GREEN),
                                    );
                                });
                                if is_overdue {
                                    ui.add_space(3.0);
                                    ui.horizontal(|ui| {
                                        ui.label(
                                            RichText::new(format!(
                                                "Pénalité retard : {} j × {:.0} DA = {:.0} DA",
                                                overdue_days, c.tarif_jour, penalite
                                            ))
                                            .size(12.0)
                                            .color(RED_IAM),
                                        );
                                    });
                                }
                                if !c.notes.is_empty() {
                                    ui.add_space(2.0);
                                    ui.label(
                                        RichText::new(format!("📝 {}", c.notes))
                                            .size(11.0)
                                            .color(muted()),
                                    );
                                }
                            });
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    if bouton_neutre(ui, "Voir contrat", 95.0) {
                                        open_modal = Some(c.id);
                                    }
                                    ui.add_space(8.0);
                                    // Fixed-size amount box
                                    egui::Frame {
                                        fill: if is_overdue {
                                            tinted_card(RED_IAM, 40)
                                        } else {
                                            tinted_card(AMBER, 30)
                                        },
                                        inner_margin: Margin::same(10.0),
                                        rounding: Rounding::same(7.0),
                                        stroke: Stroke::new(
                                            1.5,
                                            if is_overdue { RED_IAM } else { AMBER },
                                        ),
                                        ..Default::default()
                                    }
                                    .show(ui, |ui| {
                                        ui.set_min_width(right_w - 103.0);
                                        ui.set_max_width(right_w - 103.0);
                                        ui.vertical_centered(|ui| {
                                            if is_overdue {
                                                ui.label(
                                                    RichText::new("Total dû")
                                                        .size(10.5)
                                                        .color(muted()),
                                                );
                                                ui.label(
                                                    RichText::new(format!("{:.0} DA", total_du))
                                                        .size(16.0)
                                                        .color(RED_IAM)
                                                        .strong(),
                                                );
                                                ui.add_space(2.0);
                                                ui.label(
                                                    RichText::new(format!(
                                                        "dont {:.0} retard",
                                                        penalite
                                                    ))
                                                    .size(10.0)
                                                    .color(RED_IAM),
                                                );
                                            } else {
                                                ui.label(
                                                    RichText::new("Reste à payer")
                                                        .size(10.5)
                                                        .color(muted()),
                                                );
                                                ui.label(
                                                    RichText::new(format!("{:.0} DA", reste))
                                                        .size(16.0)
                                                        .color(AMBER)
                                                        .strong(),
                                                );
                                            }
                                        });
                                    });
                                },
                            );
                        });
                    });
                    ui.add_space(8.0);
                }
            });

        if let Some(cid) = open_modal {
            self.modal_contrat = ModalContrat {
                visible: true,
                contrat_id: cid,
            };
        }
    }
}

impl App {
    fn page_caisse(&mut self, ui: &mut egui::Ui) {
        titre_page(ui, "Caisse – Encaissements & Décaissements");

        panneau().show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new("Période :").size(13.0).color(text()).strong());

                // Period dropdown
                let periode_label = match self.caisse_periode_active {
                    None => "— Choisir une période —".to_string(),
                    Some(id) => self
                        .caisse_periodes
                        .iter()
                        .find(|p| p.id == id)
                        .map(|p| {
                            format!(
                                "{} ({} au {})",
                                p.nom,
                                afficher_date(&p.date_debut),
                                afficher_date(&p.date_fin)
                            )
                        })
                        .unwrap_or_else(|| "— Choisir une période —".to_string()),
                };
                egui::ComboBox::from_id_salt("caisse_periode_select")
                    .selected_text(&periode_label)
                    .width(380.0)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.caisse_periode_active,
                            None,
                            "— Choisir une période —",
                        );
                        let ps: Vec<PeriodeCaisse> = self.caisse_periodes.clone();
                        for p in ps.iter().rev() {
                            let lbl = format!(
                                "{}  ({} au {}){}",
                                p.nom,
                                afficher_date(&p.date_debut),
                                afficher_date(&p.date_fin),
                                if p.fermee { "  [Fermée]" } else { "" }
                            );
                            ui.selectable_value(&mut self.caisse_periode_active, Some(p.id), lbl);
                        }
                    });

                ui.add_space(10.0);
                // Create new period
                if ui
                    .add(
                        egui::Button::new(
                            RichText::new("+ Nouvelle Période").size(13.0).color(WHITE),
                        )
                        .fill(RED_IAM)
                        .rounding(6.0),
                    )
                    .clicked()
                {
                    self.caisse_creer_visible = !self.caisse_creer_visible;
                    self.caisse_new_msg.clear();
                }

                // Close period button
                if let Some(pid) = self.caisse_periode_active {
                    let fermee = self
                        .caisse_periodes
                        .iter()
                        .find(|p| p.id == pid)
                        .map(|p| p.fermee)
                        .unwrap_or(false);
                    if !fermee {
                        ui.add_space(8.0);
                        if ui
                            .add(
                                egui::Button::new(
                                    RichText::new("Clôturer & Sauvegarder")
                                        .size(12.5)
                                        .color(WHITE),
                                )
                                .fill(Color32::from_rgb(22, 130, 80))
                                .rounding(6.0),
                            )
                            .clicked()
                        {
                            if let Some(p) = self.caisse_periodes.iter_mut().find(|p| p.id == pid) {
                                p.fermee = true;
                            }
                            sauvegarder(&periodes_file(), &self.caisse_periodes);
                        }
                    } else {
                        ui.add_space(8.0);
                        ui.colored_label(Color32::from_rgb(22, 130, 80), "Fermée");
                    }
                    ui.add_space(8.0);
                    if bouton_danger(ui, "Supprimer Période", 130.0) {
                        self.caisse_suppr_periode = Some(pid);
                    }
                }
            });

            // Create period form
            if self.caisse_creer_visible {
                ui.add_space(10.0);
                ui.separator();
                ui.add_space(8.0);
                egui::Grid::new("caisse_new_form")
                    .num_columns(2)
                    .spacing([10.0, 8.0])
                    .show(ui, |ui| {
                        ui.label(
                            RichText::new("Nom de la période *")
                                .size(12.5)
                                .color(text()),
                        );
                        ui.add(
                            egui::TextEdit::singleline(&mut self.caisse_new_nom)
                                .desired_width(280.0)
                                .hint_text("ex: Semaine 1 – Juin 2026"),
                        );
                        ui.end_row();
                        ui.label(RichText::new("Date début *").size(12.5).color(text()));
                        ui.add(
                            egui::TextEdit::singleline(&mut self.caisse_new_debut)
                                .desired_width(130.0)
                                .hint_text("JJ/MM/AAAA"),
                        );
                        ui.end_row();
                        ui.label(RichText::new("Date fin *").size(12.5).color(text()));
                        ui.add(
                            egui::TextEdit::singleline(&mut self.caisse_new_fin)
                                .desired_width(130.0)
                                .hint_text("JJ/MM/AAAA"),
                        );
                        ui.end_row();
                    });
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    if ui
                        .add(
                            egui::Button::new(
                                RichText::new("Créer la période").size(13.0).color(WHITE),
                            )
                            .fill(RED_IAM)
                            .rounding(6.0),
                        )
                        .clicked()
                    {
                        self.creer_periode_caisse();
                    }
                    if !self.caisse_new_msg.is_empty() {
                        ui.add_space(8.0);
                        ui.colored_label(
                            if self.caisse_new_msg.starts_with("OK") {
                                GREEN
                            } else {
                                RED_IAM
                            },
                            &self.caisse_new_msg.clone(),
                        );
                    }
                });
            }
        });

        ui.add_space(12.0);

        if let Some(del_pid) = self.caisse_suppr_periode {
            egui::Window::new("Supprimer la période")
                .collapsible(false)
                .resizable(false)
                .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
                .fixed_size([380.0, 0.0])
                .show(ui.ctx(), |ui| {
                    ui.add_space(8.0);
                    ui.label(
                        RichText::new("Supprimer cette période et toutes ses entrées ?")
                            .size(13.5)
                            .color(text()),
                    );
                    ui.add_space(4.0);
                    ui.label(
                        RichText::new("Cette action est définitive.")
                            .size(12.0)
                            .color(muted()),
                    );
                    ui.add_space(12.0);
                    ui.horizontal(|ui| {
                        if bouton_danger(ui, "Oui, Supprimer", 130.0) {
                            self.caisse_periodes.retain(|p| p.id != del_pid);
                            self.caisse_encaissements
                                .retain(|e| e.periode_id != del_pid);
                            self.caisse_decaissements
                                .retain(|d| d.periode_id != del_pid);
                            sauvegarder(&periodes_file(), &self.caisse_periodes);
                            sauvegarder(&encaissements_file(), &self.caisse_encaissements);
                            sauvegarder(&decaissements_file(), &self.caisse_decaissements);
                            if self.caisse_periode_active == Some(del_pid) {
                                self.caisse_periode_active = None;
                            }
                            self.caisse_suppr_periode = None;
                        }
                        ui.add_space(8.0);
                        if bouton_neutre(ui, "Annuler", 90.0) {
                            self.caisse_suppr_periode = None;
                        }
                    });
                    ui.add_space(8.0);
                });
        }

        let periode_id = match self.caisse_periode_active {
            Some(id) => id,
            None => {
                panneau().show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(20.0);
                        ui.label(
                            RichText::new("Sélectionnez ou créez une période pour commencer.")
                                .size(14.0)
                                .color(muted()),
                        );
                        ui.add_space(20.0);
                    });
                });
                return;
            }
        };

        let total_enc: f64 = self
            .caisse_encaissements
            .iter()
            .filter(|e| e.periode_id == periode_id)
            .map(|e| e.montant)
            .sum();
        let total_dec: f64 = self
            .caisse_decaissements
            .iter()
            .filter(|d| d.periode_id == periode_id)
            .map(|d| d.montant)
            .sum();
        let caisse = total_enc - total_dec;

        let sw = (ui.available_width() - 24.0) / 3.0;
        ui.horizontal(|ui| {
            for (lbl, val, col) in [
                ("Encaissements", format!("{:.0} DA", total_enc), GREEN),
                ("Décaissements", format!("{:.0} DA", total_dec), RED_IAM),
                (
                    "Caisse",
                    format!("{:.0} DA", caisse),
                    if caisse >= 0.0 { GREEN } else { RED_IAM },
                ),
            ] {
                egui::Frame {
                    fill: card(),
                    inner_margin: Margin::same(16.0),
                    rounding: Rounding::same(10.0),
                    stroke: Stroke::new(1.0, border()),
                    ..Default::default()
                }
                .show(ui, |ui| {
                    ui.set_min_width(sw);
                    ui.label(RichText::new(lbl).size(12.0).color(muted()));
                    ui.add_space(4.0);
                    ui.label(RichText::new(val).size(26.0).color(col).strong());
                });
                ui.add_space(8.0);
            }
        });

        ui.add_space(14.0);

        //  Delete confirmation
        if let Some(enc_id) = self.enc_suppr {
            egui::Frame {
                fill: tinted_card(RED_IAM, 40),
                inner_margin: Margin::same(12.0),
                rounding: Rounding::same(7.0),
                stroke: Stroke::new(1.0, RED_IAM),
                ..Default::default()
            }
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new("Supprimer cet encaissement ?")
                            .size(13.0)
                            .color(RED_IAM),
                    );
                    ui.add_space(12.0);
                    if ui
                        .add(
                            egui::Button::new(RichText::new("Oui").size(12.5).color(WHITE))
                                .fill(RED_IAM)
                                .rounding(5.0),
                        )
                        .clicked()
                    {
                        self.caisse_encaissements.retain(|e| e.id != enc_id);
                        sauvegarder(&encaissements_file(), &self.caisse_encaissements);
                        self.enc_suppr = None;
                    }
                    if ui
                        .add(
                            egui::Button::new(RichText::new("Annuler").size(12.5).color(muted()))
                                .fill(card2())
                                .rounding(5.0),
                        )
                        .clicked()
                    {
                        self.enc_suppr = None;
                    }
                });
            });
            ui.add_space(8.0);
        }
        if let Some(dec_id) = self.dec_suppr {
            egui::Frame {
                fill: tinted_card(RED_IAM, 40),
                inner_margin: Margin::same(12.0),
                rounding: Rounding::same(7.0),
                stroke: Stroke::new(1.0, RED_IAM),
                ..Default::default()
            }
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new("Supprimer ce décaissement ?")
                            .size(13.0)
                            .color(RED_IAM),
                    );
                    ui.add_space(12.0);
                    if ui
                        .add(
                            egui::Button::new(RichText::new("Oui").size(12.5).color(WHITE))
                                .fill(RED_IAM)
                                .rounding(5.0),
                        )
                        .clicked()
                    {
                        self.caisse_decaissements.retain(|d| d.id != dec_id);
                        sauvegarder(&decaissements_file(), &self.caisse_decaissements);
                        self.dec_suppr = None;
                    }
                    if ui
                        .add(
                            egui::Button::new(RichText::new("Annuler").size(12.5).color(muted()))
                                .fill(card2())
                                .rounding(5.0),
                        )
                        .clicked()
                    {
                        self.dec_suppr = None;
                    }
                });
            });
            ui.add_space(8.0);
        }

        let tw = ui.available_width();
        let lw = tw * 0.60 - 8.0;
        let rw = tw - lw - 16.0;

        ui.horizontal_top(|ui| {
            // Encaissements
            ui.allocate_ui_with_layout(
                Vec2::new(lw, 0.0),
                egui::Layout::top_down(egui::Align::LEFT),
                |ui| {
                    egui::Frame {
                        fill: card(),
                        inner_margin: Margin::same(14.0),
                        rounding: Rounding::same(10.0),
                        stroke: Stroke::new(1.0, border()),
                        ..Default::default()
                    }
                    .show(ui, |ui| {
                        ui.set_min_width(lw - 4.0);
                        ui.label(
                            RichText::new("Ajouter un encaissement")
                                .size(14.0)
                                .color(GREEN)
                                .strong(),
                        );
                        ui.add_space(8.0);
                        let fw = lw - 40.0;
                        egui::Grid::new("enc_form")
                            .num_columns(2)
                            .spacing([8.0, 8.0])
                            .show(ui, |ui| {
                                ui.label(RichText::new("Date *").size(12.5).color(text()));
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.enc_date)
                                        .desired_width(fw)
                                        .hint_text("JJ/MM/AAAA"),
                                );
                                ui.end_row();
                                ui.label(RichText::new("Nom du Client *").size(12.5).color(text()));
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.enc_client)
                                        .desired_width(fw)
                                        .hint_text("Nom complet"),
                                );
                                ui.end_row();
                                ui.label(RichText::new("N° Contrat").size(12.5).color(text()));
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.enc_contrat)
                                        .desired_width(fw)
                                        .hint_text("IAM-2026-0001"),
                                );
                                ui.end_row();
                                ui.label(RichText::new("Voiture").size(12.5).color(text()));
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.enc_voiture)
                                        .desired_width(fw)
                                        .hint_text("Modèle / plaque"),
                                );
                                ui.end_row();
                                ui.label(RichText::new("Détails").size(12.5).color(text()));
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.enc_details)
                                        .desired_width(fw)
                                        .hint_text("Loyer, acompte..."),
                                );
                                ui.end_row();
                                ui.label(RichText::new("Montant (DA) *").size(12.5).color(text()));
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.enc_montant)
                                        .desired_width(fw)
                                        .hint_text("0"),
                                );
                                ui.end_row();
                            });
                        ui.add_space(8.0);
                        ui.vertical_centered(|ui| {
                            if ui
                                .add_sized(
                                    [fw.min(260.0), 34.0],
                                    egui::Button::new(
                                        RichText::new("✚  Ajouter Encaissement")
                                            .size(13.0)
                                            .color(WHITE),
                                    )
                                    .fill(GREEN)
                                    .rounding(8.0),
                                )
                                .clicked()
                            {
                                self.ajouter_encaissement(periode_id);
                            }
                        });
                        if !self.enc_msg.is_empty() {
                            ui.colored_label(
                                if self.enc_msg.starts_with("OK") {
                                    GREEN
                                } else {
                                    RED_IAM
                                },
                                &self.enc_msg.clone(),
                            );
                        }
                    });

                    ui.add_space(12.0);

                    egui::Frame {
                        fill: card(),
                        inner_margin: Margin::same(14.0),
                        rounding: Rounding::same(10.0),
                        stroke: Stroke::new(1.0, border()),
                        ..Default::default()
                    }
                    .show(ui, |ui| {
                        ui.set_min_width(lw - 4.0);
                        let encs: Vec<Encaissement> = self
                            .caisse_encaissements
                            .iter()
                            .filter(|e| e.periode_id == periode_id)
                            .cloned()
                            .collect();
                        ui.horizontal(|ui| {
                            ui.label(
                                RichText::new("Historique Encaissements")
                                    .size(14.0)
                                    .color(GREEN)
                                    .strong(),
                            );
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    ui.label(
                                        RichText::new(format!("{} entrée(s)", encs.len()))
                                            .size(12.0)
                                            .color(muted()),
                                    );
                                },
                            );
                        });
                        ui.separator();
                        ui.add_space(6.0);
                        if encs.is_empty() {
                            ui.colored_label(muted(), "Aucun encaissement pour cette période.");
                        } else {
                            let mut del_enc: Option<u64> = None;
                            egui::ScrollArea::vertical()
                                .id_salt("enc_scroll")
                                .min_scrolled_height(300.0)
                                .auto_shrink([false, false])
                                .show(ui, |ui| {
                                    let w_date = 75.0_f32;
                                    let w_client = 110.0_f32;
                                    let w_contrat = 100.0_f32;
                                    let w_voiture = 100.0_f32;
                                    let w_montant = 80.0_f32;
                                    let w_btn = 82.0_f32;
                                    let w_fixed = w_date
                                        + w_client
                                        + w_contrat
                                        + w_voiture
                                        + w_montant
                                        + w_btn
                                        + 36.0;
                                    let w_details = (ui.available_width() - w_fixed).max(80.0);

                                    // Header row
                                    ui.horizontal(|ui| {
                                        ui.add_space(2.0);
                                        for (h, w) in [
                                            ("Date", w_date),
                                            ("Client", w_client),
                                            ("Contrat", w_contrat),
                                            ("Voiture", w_voiture),
                                            ("Détails", w_details),
                                            ("Montant", w_montant),
                                        ] {
                                            ui.add_sized(
                                                [w, 16.0],
                                                egui::Label::new(
                                                    RichText::new(h)
                                                        .size(11.0)
                                                        .color(muted())
                                                        .strong(),
                                                ),
                                            );
                                        }
                                    });
                                    ui.separator();
                                    for e in &encs {
                                        let bg = tinted_card(muted(), 10);
                                        egui::Frame {
                                            fill: bg,
                                            inner_margin: Margin::symmetric(2.0, 2.0),
                                            ..Default::default()
                                        }
                                        .show(ui, |ui| {
                                            ui.horizontal(|ui| {
                                                ui.add_sized(
                                                    [w_date, 16.0],
                                                    egui::Label::new(
                                                        RichText::new(afficher_date(&e.date))
                                                            .size(11.0)
                                                            .color(muted()),
                                                    ),
                                                );
                                                ui.add_sized(
                                                    [w_client, 16.0],
                                                    egui::Label::new(
                                                        RichText::new(&e.client_nom)
                                                            .size(11.5)
                                                            .color(text()),
                                                    ),
                                                );
                                                ui.add_sized(
                                                    [w_contrat, 16.0],
                                                    egui::Label::new(
                                                        RichText::new(&e.contrat_numero)
                                                            .size(11.0)
                                                            .color(RED_IAM),
                                                    ),
                                                );
                                                ui.add_sized(
                                                    [w_voiture, 16.0],
                                                    egui::Label::new(
                                                        RichText::new(&e.voiture)
                                                            .size(11.0)
                                                            .color(text()),
                                                    ),
                                                );
                                                ui.add_sized(
                                                    [w_details, 16.0],
                                                    egui::Label::new(
                                                        RichText::new(&e.details)
                                                            .size(11.0)
                                                            .color(muted()),
                                                    )
                                                    .wrap(),
                                                );
                                                ui.add_sized(
                                                    [w_montant, 16.0],
                                                    egui::Label::new(
                                                        RichText::new(format!(
                                                            "{:.0} DA",
                                                            e.montant
                                                        ))
                                                        .size(12.0)
                                                        .color(GREEN)
                                                        .strong(),
                                                    ),
                                                );
                                                if bouton_danger(ui, "Supprimer", 80.0) {
                                                    del_enc = Some(e.id);
                                                }
                                            });
                                        });
                                    }
                                });
                            if let Some(id) = del_enc {
                                self.enc_suppr = Some(id);
                            }
                        }
                    });
                },
            );

            ui.add_space(16.0);

            // decaissement
            ui.allocate_ui_with_layout(
                Vec2::new(rw, 0.0),
                egui::Layout::top_down(egui::Align::LEFT),
                |ui| {
                    egui::Frame {
                        fill: card(),
                        inner_margin: Margin::same(14.0),
                        rounding: Rounding::same(10.0),
                        stroke: Stroke::new(1.0, border()),
                        ..Default::default()
                    }
                    .show(ui, |ui| {
                        ui.set_min_width(rw - 4.0);
                        ui.label(
                            RichText::new("Ajouter un décaissement")
                                .size(14.0)
                                .color(RED_IAM)
                                .strong(),
                        );
                        ui.add_space(8.0);
                        let fw2 = rw - 40.0;
                        egui::Grid::new("dec_form")
                            .num_columns(2)
                            .spacing([8.0, 8.0])
                            .show(ui, |ui| {
                                ui.label(RichText::new("Date *").size(12.5).color(text()));
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.dec_date)
                                        .desired_width(fw2)
                                        .hint_text("JJ/MM/AAAA"),
                                );
                                ui.end_row();
                                ui.label(RichText::new("Nom *").size(12.5).color(text()));
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.dec_nom)
                                        .desired_width(fw2)
                                        .hint_text("Qui a reçu l'argent"),
                                );
                                ui.end_row();
                                ui.label(RichText::new("Détails").size(12.5).color(text()));
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.dec_details)
                                        .desired_width(fw2)
                                        .hint_text("Raison du paiement..."),
                                );
                                ui.end_row();
                                ui.label(RichText::new("Montant (DA) *").size(12.5).color(text()));
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.dec_montant)
                                        .desired_width(fw2)
                                        .hint_text("0"),
                                );
                                ui.end_row();
                            });
                        ui.add_space(8.0);
                        ui.vertical_centered(|ui| {
                            if ui
                                .add_sized(
                                    [fw2.min(260.0), 34.0],
                                    egui::Button::new(
                                        RichText::new("✚  Ajouter Décaissement")
                                            .size(13.0)
                                            .color(WHITE),
                                    )
                                    .fill(RED_IAM)
                                    .rounding(8.0),
                                )
                                .clicked()
                            {
                                self.ajouter_decaissement(periode_id);
                            }
                        });
                        if !self.dec_msg.is_empty() {
                            ui.colored_label(
                                if self.dec_msg.starts_with("OK") {
                                    GREEN
                                } else {
                                    RED_IAM
                                },
                                &self.dec_msg.clone(),
                            );
                        }
                    });

                    ui.add_space(12.0);

                    egui::Frame {
                        fill: card(),
                        inner_margin: Margin::same(14.0),
                        rounding: Rounding::same(10.0),
                        stroke: Stroke::new(1.0, border()),
                        ..Default::default()
                    }
                    .show(ui, |ui| {
                        ui.set_min_width(rw - 4.0);
                        let decs: Vec<Decaissement> = self
                            .caisse_decaissements
                            .iter()
                            .filter(|d| d.periode_id == periode_id)
                            .cloned()
                            .collect();
                        ui.horizontal(|ui| {
                            ui.label(
                                RichText::new("Historique Décaissements")
                                    .size(14.0)
                                    .color(RED_IAM)
                                    .strong(),
                            );
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    ui.label(
                                        RichText::new(format!("{} entrée(s)", decs.len()))
                                            .size(12.0)
                                            .color(muted()),
                                    );
                                },
                            );
                        });
                        ui.separator();
                        ui.add_space(6.0);
                        if decs.is_empty() {
                            ui.colored_label(muted(), "Aucun décaissement pour cette période.");
                        } else {
                            let mut del_dec: Option<u64> = None;
                            egui::ScrollArea::vertical()
                                .id_salt("dec_scroll")
                                .min_scrolled_height(300.0)
                                .auto_shrink([false, false])
                                .show(ui, |ui| {
                                    let w_date2 = 75.0_f32;
                                    let w_nom = 120.0_f32;
                                    let w_montant2 = 85.0_f32;
                                    let w_btn2 = 82.0_f32;
                                    let w_fixed2 = w_date2 + w_nom + w_montant2 + w_btn2 + 24.0;
                                    let w_details2 = (ui.available_width() - w_fixed2).max(80.0);

                                    // Header row
                                    ui.horizontal(|ui| {
                                        ui.add_space(2.0);
                                        for (h, w) in [
                                            ("Date", w_date2),
                                            ("Nom", w_nom),
                                            ("Détails", w_details2),
                                            ("Montant", w_montant2),
                                        ] {
                                            ui.add_sized(
                                                [w, 16.0],
                                                egui::Label::new(
                                                    RichText::new(h)
                                                        .size(11.0)
                                                        .color(muted())
                                                        .strong(),
                                                ),
                                            );
                                        }
                                    });
                                    ui.separator();
                                    for d in &decs {
                                        let bg = tinted_card(muted(), 10);
                                        egui::Frame {
                                            fill: bg,
                                            inner_margin: Margin::symmetric(2.0, 2.0),
                                            ..Default::default()
                                        }
                                        .show(ui, |ui| {
                                            ui.horizontal(|ui| {
                                                ui.add_sized(
                                                    [w_date2, 16.0],
                                                    egui::Label::new(
                                                        RichText::new(afficher_date(&d.date))
                                                            .size(11.0)
                                                            .color(muted()),
                                                    ),
                                                );
                                                ui.add_sized(
                                                    [w_nom, 16.0],
                                                    egui::Label::new(
                                                        RichText::new(&d.nom)
                                                            .size(11.5)
                                                            .color(text()),
                                                    ),
                                                );
                                                ui.add_sized(
                                                    [w_details2, 16.0],
                                                    egui::Label::new(
                                                        RichText::new(&d.details)
                                                            .size(11.0)
                                                            .color(muted()),
                                                    )
                                                    .wrap(),
                                                );
                                                ui.add_sized(
                                                    [w_montant2, 16.0],
                                                    egui::Label::new(
                                                        RichText::new(format!(
                                                            "{:.0} DA",
                                                            d.montant
                                                        ))
                                                        .size(12.0)
                                                        .color(RED_IAM)
                                                        .strong(),
                                                    ),
                                                );
                                                if bouton_danger(ui, "Supprimer", 80.0) {
                                                    del_dec = Some(d.id);
                                                }
                                            });
                                        });
                                    }
                                });
                            if let Some(id) = del_dec {
                                self.dec_suppr = Some(id);
                            }
                        }
                    });
                },
            );
        });
    }

    fn creer_periode_caisse(&mut self) {
        if self.caisse_new_nom.trim().is_empty() {
            self.caisse_new_msg = "Le nom de la période est obligatoire.".into();
            return;
        }
        if parse_date(&self.caisse_new_debut).is_none() {
            self.caisse_new_msg = "Date de début invalide (JJ/MM/AAAA).".into();
            return;
        }
        if parse_date(&self.caisse_new_fin).is_none() {
            self.caisse_new_msg = "Date de fin invalide (JJ/MM/AAAA).".into();
            return;
        }
        let id = self.caisse_periodes.iter().map(|p| p.id).max().unwrap_or(0) + 1;
        let p = PeriodeCaisse {
            id,
            nom: self.caisse_new_nom.trim().to_string(),
            date_debut: stocker_date(parse_date(&self.caisse_new_debut).unwrap()),
            date_fin: stocker_date(parse_date(&self.caisse_new_fin).unwrap()),
            fermee: false,
        };
        self.caisse_new_msg = format!("OK Période \"{}\" créée.", p.nom);
        self.caisse_periode_active = Some(id);
        self.caisse_periodes.push(p);
        sauvegarder(&periodes_file(), &self.caisse_periodes);
        self.caisse_new_nom.clear();
        self.caisse_new_debut = aujourd_hui();
        self.caisse_new_fin = dans_7j();
        self.caisse_creer_visible = false;
    }

    fn ajouter_encaissement(&mut self, periode_id: u64) {
        if self.enc_client.trim().is_empty() {
            self.enc_msg = "Le nom du client est obligatoire.".into();
            return;
        }
        if parse_date(&self.enc_date).is_none() {
            self.enc_msg = "Date invalide (JJ/MM/AAAA).".into();
            return;
        }
        let montant: f64 = match self.enc_montant.trim().parse() {
            Ok(v) if v >= 0.0 => v,
            _ => {
                self.enc_msg = "Montant invalide.".into();
                return;
            }
        };
        let id = self
            .caisse_encaissements
            .iter()
            .map(|e| e.id)
            .max()
            .unwrap_or(0)
            + 1;
        self.caisse_encaissements.push(Encaissement {
            id,
            periode_id,
            date: stocker_date(parse_date(&self.enc_date).unwrap()),
            client_nom: self.enc_client.trim().to_string(),
            contrat_numero: self.enc_contrat.trim().to_string(),
            voiture: self.enc_voiture.trim().to_string(),
            details: self.enc_details.trim().to_string(),
            montant,
        });
        sauvegarder(&encaissements_file(), &self.caisse_encaissements);
        self.enc_msg = format!("OK Encaissement de {:.0} DA ajouté.", montant);
        self.enc_client.clear();
        self.enc_contrat.clear();
        self.enc_voiture.clear();
        self.enc_details.clear();
        self.enc_montant.clear();
        self.enc_date = aujourd_hui();
    }

    fn ajouter_decaissement(&mut self, periode_id: u64) {
        if self.dec_nom.trim().is_empty() {
            self.dec_msg = "Le nom est obligatoire.".into();
            return;
        }
        if parse_date(&self.dec_date).is_none() {
            self.dec_msg = "Date invalide (JJ/MM/AAAA).".into();
            return;
        }
        let montant: f64 = match self.dec_montant.trim().parse() {
            Ok(v) if v >= 0.0 => v,
            _ => {
                self.dec_msg = "Montant invalide.".into();
                return;
            }
        };
        let id = self
            .caisse_decaissements
            .iter()
            .map(|d| d.id)
            .max()
            .unwrap_or(0)
            + 1;
        self.caisse_decaissements.push(Decaissement {
            id,
            periode_id,
            date: stocker_date(parse_date(&self.dec_date).unwrap()),
            nom: self.dec_nom.trim().to_string(),
            details: self.dec_details.trim().to_string(),
            montant,
        });
        sauvegarder(&decaissements_file(), &self.caisse_decaissements);
        self.dec_msg = format!("OK Décaissement de {:.0} DA ajouté.", montant);
        self.dec_nom.clear();
        self.dec_details.clear();
        self.dec_montant.clear();
        self.dec_date = aujourd_hui();
    }
}

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "IAM Business",
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_title("IAM Business  Gestion Location de Voitures by phantekzy")
                .with_inner_size([1200.0, 860.0])
                .with_min_inner_size([1100.0, 700.0]),
            ..Default::default()
        },
        Box::new(|_cc| Ok(Box::new(App::new()))),
    )
}
