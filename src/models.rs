use serde::{Deserialize, Serialize};
use Color32;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Reparation {
    pub id: u64,
    pub voiture_id: u64,
    pub voiture_modele: String,
    pub voiture_plaque: String,
    pub date: String,
    pub description: String,
    #[serde(default)]
    pub prix: String,
    #[serde(default)]
    pub observation: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VoitureVente {
    pub id: u64,
    pub nom: String,
    pub plaque: String,
    pub num_chassis: String,
    pub num_immat: String,
    pub annee: u32,
    pub prix_achat: f64,
    pub prix_demande: f64,
    pub prix_vendu: f64,
    pub vendu: bool,
    pub date_vente: String,
    pub notes: String,
}

impl VoitureVente {
    pub fn benefice(&self) -> f64 {
        if self.vendu && self.prix_achat > 0.0 {
            self.prix_vendu - self.prix_achat
        } else {
            0.0
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Voiture {
    pub id: u64,
    pub modele: String,
    pub plaque: String,
    pub categorie: String,
    pub couleur: String,
    pub annee: u32,
    pub tarif_jour: f64,
    #[serde(default)]
    pub etat: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Contrat {
    pub id: u64,
    pub numero: String,
    pub voiture_id: u64,
    pub voiture_modele: String,
    pub voiture_plaque: String,
    pub client_nom: String,
    pub client_tel: String,
    pub agent: String,
    pub date_debut: String,
    pub date_fin: String,
    pub km_depart: f64,
    pub km_retour: f64,
    pub tarif_jour: f64,
    pub notes: String,
    pub statut: String,
    pub montant_paye: f64,
    pub modifiable_note: String,
}

impl Contrat {
    pub fn jours(&self) -> i64 {
        let d = chrono::NaiveDate::parse_from_str(&self.date_debut, "%Y-%m-%d").ok();
        let f = chrono::NaiveDate::parse_from_str(&self.date_fin, "%Y-%m-%d").ok();
        d.zip(f)
            .map(|(a, b)| (b - a).num_days().max(1))
            .unwrap_or(1)
    }

    pub fn total(&self) -> f64 {
        self.jours() as f64 * self.tarif_jour
    }

    pub fn reste_a_payer(&self) -> f64 {
        let t = self.total();
        if self.montant_paye >= t {
            0.0
        } else {
            t - self.montant_paye
        }
    }

    #[allow(dead_code)]
    pub fn km_parcourus(&self) -> f64 {
        if self.km_retour > self.km_depart {
            self.km_retour - self.km_depart
        } else {
            0.0
        }
    }

    pub fn chevauche(&self, du: chrono::NaiveDate, au: chrono::NaiveDate) -> bool {
        if self.statut == "Annulé" {
            return false;
        }
        if self.statut == "Terminé" {
            let f = chrono::NaiveDate::parse_from_str(&self.date_fin, "%Y-%m-%d").ok();
            let d = chrono::NaiveDate::parse_from_str(&self.date_debut, "%Y-%m-%d").ok();
            return d.zip(f).map(|(a, b)| a <= au && b > du).unwrap_or(false);
        }
        let d = chrono::NaiveDate::parse_from_str(&self.date_debut, "%Y-%m-%d").ok();
        let f = chrono::NaiveDate::parse_from_str(&self.date_fin, "%Y-%m-%d").ok();
        d.zip(f).map(|(a, b)| a <= au && b >= du).unwrap_or(false)
    }
}
