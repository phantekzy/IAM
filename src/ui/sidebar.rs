use app::App;
use eframe::egui::{self, RichText};
use models::Onglet;
use theme::{card2, muted, text, RED_IAM};

pub fn render_sidebar(ui: &mut egui::Ui, app: &mut App) {
    ui.add_space(5.0);
    ui.vertical(|ui| {
        ui.label(
            RichText::new("IAM BUSINESS")
                .size(20.0)
                .bold()
                .color(RED_IAM),
        );
        ui.label(RichText::new("Location v5.0").size(11.0).color(muted()));
        ui.add_space(15.0);

        let boutons = [
            (Onglet::Tableau, "Tableau de bord"),
            (Onglet::Disponibilite, "Disponibilité"),
            (Onglet::NouveauContrat, "Nouveau Contrat"),
            (Onglet::Contrats, "Liste Contrats"),
            (Onglet::Voitures, "Parc Automobiles"),
            (Onglet::Ventes, "Ventes Véhicules"),
            (Onglet::Maintenance, "Maintenance / Réparations"),
        ];

        for (ong, libelle) in boutons {
            let select = app.onglet == ong;
            let mut btn = egui::Button::new(RichText::new(libelle).size(13.5).color(if select {
                RED_IAM
            } else {
                text()
            }));
            if select {
                btn = btn.fill(card2());
            }
            if ui.add_sized([220.0, 36.0], btn).clicked() {
                app.nav_menu(ong);
            }
            ui.add_space(4.0);
        }

        ui.add_space(30.0);
        ui.checkbox(&mut app.dark_mode, "Mode Sombre");
    });
    ui.add_space(10.0);
    ui.separator();
    ui.add_space(10.0);
}
