use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use crate::models::Voiture;

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

#[derive(Serialize, Deserialize, Clone, Debug)]
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

#[derive(PartialEq, Clone, Copy)]

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


