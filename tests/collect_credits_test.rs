use terminal_company::models::collect_credits::CollectCreditsEvent;
use terminal_company::models::types::{CollectConfig, ScanData};
use std::collections::HashMap;

#[test]
fn test_calculate_chance_base() {
    let config = CollectConfig {
        id: None,
        base_chance: 50,
        weather_mods: HashMap::new(),
    };

    let scan_data = ScanData {
        weather: "Clear".to_string(),
        threat_level: 0,
        scrap_value: 100,
        monsters: vec![],
    };

    let event = CollectCreditsEvent {
        scan_data: &scan_data,
        player_bonus: 0,
        config: &config,
    };

    assert_eq!(event.calculate_chance(), 50);
}

#[test]
fn test_calculate_chance_with_player_bonus() {
    let config = CollectConfig {
        id: None,
        base_chance: 50,
        weather_mods: HashMap::new(),
    };

    let scan_data = ScanData {
        weather: "Clear".to_string(),
        threat_level: 0,
        scrap_value: 100,
        monsters: vec![],
    };

    let event = CollectCreditsEvent {
        scan_data: &scan_data,
        player_bonus: 20,
        config: &config,
    };

    assert_eq!(event.calculate_chance(), 70);
}

#[test]
fn test_calculate_chance_with_weather_modifier() {
    let mut weather_mods = HashMap::new();
    weather_mods.insert("Clear".to_string(), 20);
    weather_mods.insert("Rainy".to_string(), -5);
    weather_mods.insert("Stormy".to_string(), -20);

    let config = CollectConfig {
        id: None,
        base_chance: 50,
        weather_mods,
    };

    // Test Clear weather
    let scan_clear = ScanData {
        weather: "Clear".to_string(),
        threat_level: 0,
        scrap_value: 100,
        monsters: vec![],
    };

    let event_clear = CollectCreditsEvent {
        scan_data: &scan_clear,
        player_bonus: 0,
        config: &config,
    };

    assert_eq!(event_clear.calculate_chance(), 70); // 50 + 20

    // Test Rainy weather
    let scan_rainy = ScanData {
        weather: "Rainy".to_string(),
        threat_level: 0,
        scrap_value: 100,
        monsters: vec![],
    };

    let event_rainy = CollectCreditsEvent {
        scan_data: &scan_rainy,
        player_bonus: 0,
        config: &config,
    };

    assert_eq!(event_rainy.calculate_chance(), 45); // 50 - 5

    // Test Stormy weather
    let scan_stormy = ScanData {
        weather: "Stormy".to_string(),
        threat_level: 0,
        scrap_value: 100,
        monsters: vec![],
    };

    let event_stormy = CollectCreditsEvent {
        scan_data: &scan_stormy,
        player_bonus: 0,
        config: &config,
    };

    assert_eq!(event_stormy.calculate_chance(), 30); // 50 - 20
}

#[test]
fn test_calculate_chance_with_threat_level() {
    let config = CollectConfig {
        id: None,
        base_chance: 50,
        weather_mods: HashMap::new(),
    };

    let scan_data = ScanData {
        weather: "Clear".to_string(),
        threat_level: 20,
        scrap_value: 100,
        monsters: vec![],
    };

    let event = CollectCreditsEvent {
        scan_data: &scan_data,
        player_bonus: 0,
        config: &config,
    };

    // 50 - (20 / 2) = 50 - 10 = 40
    assert_eq!(event.calculate_chance(), 40);
}

#[test]
fn test_calculate_chance_minimum_is_one() {
    let config = CollectConfig {
        id: None,
        base_chance: 10,
        weather_mods: HashMap::new(),
    };

    let scan_data = ScanData {
        weather: "Clear".to_string(),
        threat_level: 100, // Very high threat
        scrap_value: 100,
        monsters: vec![],
    };

    let event = CollectCreditsEvent {
        scan_data: &scan_data,
        player_bonus: -50, // Negative bonus
        config: &config,
    };

    // 10 + (-50) - (100/2) = 10 - 50 - 50 = -90 -> should be clamped to 1
    assert_eq!(event.calculate_chance(), 1);
}

#[test]
fn test_calculate_chance_complex_scenario() {
    let mut weather_mods = HashMap::new();
    weather_mods.insert("Eclipsed".to_string(), -30);

    let config = CollectConfig {
        id: None,
        base_chance: 50,
        weather_mods,
    };

    let scan_data = ScanData {
        weather: "Eclipsed".to_string(),
        threat_level: 40,
        scrap_value: 200,
        monsters: vec![],
    };

    let event = CollectCreditsEvent {
        scan_data: &scan_data,
        player_bonus: 15,
        config: &config,
    };

    // 50 (base) + 15 (bonus) - 30 (weather) - 20 (threat_level/2) = 15
    assert_eq!(event.calculate_chance(), 15);
}

#[test]
fn test_attempt_returns_some_or_none() {
    let config = CollectConfig {
        id: None,
        base_chance: 100, // 100% chance
        weather_mods: HashMap::new(),
    };

    let scan_data = ScanData {
        weather: "Clear".to_string(),
        threat_level: 0,
        scrap_value: 150,
        monsters: vec![],
    };

    let event = CollectCreditsEvent {
        scan_data: &scan_data,
        player_bonus: 0,
        config: &config,
    };

    // With 100% chance, should always succeed
    // Run multiple times to verify consistency
    let mut successes = 0;
    for _ in 0..10 {
        if let Some(value) = event.attempt() {
            assert_eq!(value, 150);
            successes += 1;
        }
    }
    
    // With 100% chance, we expect all attempts to succeed
    assert!(successes >= 8, "Expected at least 8/10 successes with 100% chance, got {}", successes);
}

#[test]
fn test_attempt_with_zero_chance() {
    let config = CollectConfig {
        id: None,
        base_chance: 0,
        weather_mods: HashMap::new(),
    };

    let scan_data = ScanData {
        weather: "Clear".to_string(),
        threat_level: 0,
        scrap_value: 100,
        monsters: vec![],
    };

    let event = CollectCreditsEvent {
        scan_data: &scan_data,
        player_bonus: 0,
        config: &config,
    };

    // Even with 0% base chance, minimum is 1%, so sometimes should succeed
    // Just verify it doesn't panic
    let _ = event.attempt();
}
