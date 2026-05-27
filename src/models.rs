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
