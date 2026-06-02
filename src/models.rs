use chrono::{Duration, NaiveDate};
use serde::{Deserialize, Serialize};

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
