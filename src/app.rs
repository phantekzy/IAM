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
