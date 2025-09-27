use crate::models::types::{Bestiary, Monster, ScanData};
use rand::{Rng, seq::SliceRandom};

fn moon_tier(moon: &str) -> u8 {
    match moon.to_uppercase().as_str() {
        "EXPERIMENTATION" | "ASSURANCE" | "VOW" => 1,
        "OFFENSE" | "MARCH" | "ADAMANCE" => 2,
        "REND" | "DINE" | "TITAN" => 3,
        "ARTIFICE" | "EMBRION" | "LIQUIDATION" => 4,
        _ => 1,
    }
}

fn monsters_for_moon<'a>(bestiary: &'a Bestiary, moon: &str) -> Vec<&'a Monster> {
    bestiary
        .monsters
        .iter()
        .filter(|m| m.moons.contains(&moon.to_string()))
        .collect()
}

fn danger_value(level: &Option<u32>) -> f32 {
    match level {
        Some(l) => *l as f32 / 20.0,
        None => 1.0,
    }
}

fn calculate_threat_level(monsters: &[&Monster]) -> u32 {
    monsters
        .iter()
        .map(|m| {
            let base = danger_value(&m.danger_level);
            let val = (base * m.power_level).ceil() as u32;
            if val == 0 { 1 } else { val }
        })
        .sum()
}

pub fn generate_scan_data(
    moon: &str,
    weather: &str,
    scrap_value: u32,
    bestiary: &Bestiary,
) -> ScanData {
    let mut rng = rand::rng();

    let candidates = monsters_for_moon(bestiary, moon);

    let tier = moon_tier(moon);
    let max_monsters = match tier {
        1 => rng.random_range(1..=3),
        2 => rng.random_range(2..=4),
        3 => rng.random_range(3..=5),
        4 => rng.random_range(4..=6),
        _ => 1,
    };

    let mut selected = Vec::new();
    if !candidates.is_empty() {
        //let candidates_for_debug = candidates.clone();
        let mut shuffled: Vec<&Monster> = candidates.into_iter().collect();
        shuffled.shuffle(&mut rng);
        selected.extend(shuffled.into_iter().take(max_monsters));

        // eprintln!(
        //     "[DEBUG] Candidates: {:?}",
        //     candidates_for_debug.iter().map(|m| &m.name).collect::<Vec<_>>()
        // );
        // eprintln!(
        //     "[DEBUG] Selected monsters for {}: {:?}",
        //     moon,
        //     selected.iter().map(|m| &m.name).collect::<Vec<_>>()
        // );
    }

    let threat_level = calculate_threat_level(&selected);

    ScanData {
        weather: weather.to_string(),
        threat_level,
        scrap_value,
        monsters: selected.into_iter().cloned().collect(),
    }
}
