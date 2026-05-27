use app::App;
use eframe::egui::{self, RichText};
use theme::{muted, text, GREEN, RED_IAM};
use ui_helpers::*;

impl App {
    pub fn chercher_dispo_modele(&mut self) {
        self.d_modele_periode.clear();
        let query = self.d_recherche_modele.trim().to_lowercase();
        if query.is_empty() {
            return;
        }
        let (Some(du), Some(au)) = (parse_date(&self.d_du), parse_date(&self.d_au)) else {
            return;
        };

        for v in &self.voitures {
            if v.modele.to_lowercase().contains(&query) || v.plaque.to_lowercase().contains(&query)
            {
                let stat = self.statut_voiture(v.id, du, au);
                let details = stat
                    .lignes
                    .iter()
                    .map(|(l, _)| l.clone())
                    .collect::<Vec<_>>()
                    .join(" ; ");
                self.d_modele_periode
                    .push((v.id, v.modele.clone(), stat.libelle, details));
            }
        }
    }
}
