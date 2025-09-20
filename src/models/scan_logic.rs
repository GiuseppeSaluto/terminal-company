use crate::models::types::{Bestiary, Monster, ScanData};

fn monsters_for_moon<'a>(bestiary: &'a Bestiary, moon: &str) -> Vec<&'a Monster> {
    bestiary
        .monsters
        .iter()
        .filter(|m| m.moons.iter().any(|mm| mm.eq_ignore_ascii_case(moon)))
        .collect()
}

fn danger_value(level: &Option<String>) -> f32 {
    match level.as_deref() {
        Some("Low") => 1.0,
        Some("Medium") => 2.0,
        Some("High") => 3.0,
        Some("Extreme") => 5.0,
        _ => 1.0,
    }
}

fn calculate_threat_level(monsters: &[&Monster]) -> u32 {
    monsters
        .iter()
        .map(|m| {
            let base = danger_value(&m.danger_level);
            (base * m.power_level).round() as u32
        })
        .sum()
}

pub fn generate_scan_data(
    moon: &str,
    weather: &str,
    scrap_value: u32,
    bestiary: &Bestiary,
) -> ScanData {
    let monsters = monsters_for_moon(bestiary, moon);
    let threat_level = calculate_threat_level(&monsters);

    ScanData {
        weather: weather.to_string(),
        threat_level,
        scrap_value,
        monsters: monsters.into_iter().cloned().collect(),
    }
}
