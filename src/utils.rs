use chrono::{Duration, Local, NaiveDate};

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

