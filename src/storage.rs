use std::path::PathBuf;
use serde::{Deserialize, Serialize};

pub fn data_dir() -> PathBuf {
    #[cfg(windows)]
    let base = std::env::var("APPDATA")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("."));
    #[cfg(not(windows))]\n    let base = std::env::var("HOME")
        .map(|h| PathBuf::from(h).join(".local").join("share"))
        .unwrap_or_else(|_| PathBuf::from("."));
    let d = base.join("IAMBusiness");
    std::fs::create_dir_all(&d).ok();
    d
}

pub fn cars_file() -> PathBuf {
    data_dir().join("voitures.csv")
}

pub fn rentals_file() -> PathBuf {
    data_dir().join("contrats.csv")
}

pub fn ventes_file() -> PathBuf {
    data_dir().join("ventes_voitures.csv")
}

pub fn reparations_file() -> PathBuf {
    data_dir().join("reparations.csv")
}

pub fn charger<T: for<'de> Deserialize<'de>>(p: &PathBuf) -> Vec<T> {
    csv::Reader::from_path(p)
        .ok()
        .map(|mut r| r.deserialize().filter_map(|x| x.ok()).collect())
        .unwrap_or_default()
}

pub fn sauvegarder<T: Serialize>(p: &PathBuf, v: &[T]) {
    if let Ok(mut w) = csv::Writer::from_path(p) {
        for x in v {
            let _ = w.serialize(x);
        }
    }
}
