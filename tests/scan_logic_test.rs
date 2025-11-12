use terminal_company::models::scan_logic::{
    calculate_threat_level, danger_value, generate_scan_data, moon_tier,
};
use terminal_company::models::types::{Bestiary, Monster};

#[test]
fn test_moon_tier_level_1() {
    assert_eq!(moon_tier("EXPERIMENTATION"), 1);
    assert_eq!(moon_tier("ASSURANCE"), 1);
    assert_eq!(moon_tier("VOW"), 1);
    
    // Test case insensitivity
    assert_eq!(moon_tier("experimentation"), 1);
    assert_eq!(moon_tier("Assurance"), 1);
}

#[test]
fn test_moon_tier_level_2() {
    assert_eq!(moon_tier("OFFENSE"), 2);
    assert_eq!(moon_tier("MARCH"), 2);
    assert_eq!(moon_tier("ADAMANCE"), 2);
    
    assert_eq!(moon_tier("offense"), 2);
}

#[test]
fn test_moon_tier_level_3() {
    assert_eq!(moon_tier("REND"), 3);
    assert_eq!(moon_tier("DINE"), 3);
    assert_eq!(moon_tier("TITAN"), 3);
}

#[test]
fn test_moon_tier_level_4() {
    assert_eq!(moon_tier("ARTIFICE"), 4);
    assert_eq!(moon_tier("EMBRION"), 4);
    assert_eq!(moon_tier("LIQUIDATION"), 4);
}

#[test]
fn test_moon_tier_unknown_defaults_to_1() {
    assert_eq!(moon_tier("UNKNOWN_MOON"), 1);
    assert_eq!(moon_tier(""), 1);
    assert_eq!(moon_tier("random"), 1);
}

#[test]
fn test_danger_value_with_some() {
    assert_eq!(danger_value(&Some(0)), 0.0);
    assert_eq!(danger_value(&Some(20)), 1.0);
    assert_eq!(danger_value(&Some(10)), 0.5);
    assert_eq!(danger_value(&Some(40)), 2.0);
    assert_eq!(danger_value(&Some(5)), 0.25);
}

#[test]
fn test_danger_value_with_none() {
    assert_eq!(danger_value(&None), 1.0);
}

#[test]
fn test_calculate_threat_level_empty() {
    let monsters: Vec<&Monster> = vec![];
    assert_eq!(calculate_threat_level(&monsters), 0);
}

#[test]
fn test_calculate_threat_level_single_monster() {
    let monster = Monster {
        name: "Test Monster".to_string(),
        entity_type: "Entity".to_string(),
        behavior: "Aggressive".to_string(),
        danger_level: Some(20),
        power_level: 2.0,
        spawn_condition: "Always".to_string(),
        moons: vec!["VOW".to_string()],
        speed: Some("Fast".to_string()),
        notes: None,
    };

    let monsters = vec![&monster];
    // danger_value(20) = 20/20 = 1.0
    // 1.0 * 2.0 = 2.0 -> ceil = 2
    assert_eq!(calculate_threat_level(&monsters), 2);
}

#[test]
fn test_calculate_threat_level_multiple_monsters() {
    let monster1 = Monster {
        name: "Monster 1".to_string(),
        entity_type: "Entity".to_string(),
        behavior: "Aggressive".to_string(),
        danger_level: Some(20),
        power_level: 2.0,
        spawn_condition: "Always".to_string(),
        moons: vec!["VOW".to_string()],
        speed: Some("Fast".to_string()),
        notes: None,
    };

    let monster2 = Monster {
        name: "Monster 2".to_string(),
        entity_type: "Entity".to_string(),
        behavior: "Passive".to_string(),
        danger_level: Some(10),
        power_level: 3.0,
        spawn_condition: "Night".to_string(),
        moons: vec!["VOW".to_string()],
        speed: Some("Slow".to_string()),
        notes: None,
    };

    let monsters = vec![&monster1, &monster2];
    // monster1: (20/20) * 2.0 = 2.0 -> 2
    // monster2: (10/20) * 3.0 = 1.5 -> 2
    // Total: 2 + 2 = 4
    assert_eq!(calculate_threat_level(&monsters), 4);
}

#[test]
fn test_calculate_threat_level_with_none_danger() {
    let monster = Monster {
        name: "Unknown Monster".to_string(),
        entity_type: "Entity".to_string(),
        behavior: "Unknown".to_string(),
        danger_level: None,
        power_level: 5.0,
        spawn_condition: "Random".to_string(),
        moons: vec!["TITAN".to_string()],
        speed: None,
        notes: None,
    };

    let monsters = vec![&monster];
    // danger_value(None) = 1.0
    // 1.0 * 5.0 = 5.0 -> 5
    assert_eq!(calculate_threat_level(&monsters), 5);
}

#[test]
fn test_calculate_threat_level_minimum_is_one() {
    let monster = Monster {
        name: "Weak Monster".to_string(),
        entity_type: "Entity".to_string(),
        behavior: "Passive".to_string(),
        danger_level: Some(1),
        power_level: 0.01,
        spawn_condition: "Rare".to_string(),
        moons: vec!["EXPERIMENTATION".to_string()],
        speed: Some("Very Slow".to_string()),
        notes: None,
    };

    let monsters = vec![&monster];
    // danger_value(1) = 1/20 = 0.05
    // 0.05 * 0.01 = 0.0005 -> ceil = 1 -> but then clamped to 1
    let threat = calculate_threat_level(&monsters);
    assert!(threat >= 1, "Threat level should be at least 1, got {}", threat);
}

#[test]
fn test_generate_scan_data_basic() {
    let bestiary = Bestiary {
        id: Some("test_bestiary".to_string()),
        monsters: vec![
            Monster {
                name: "Eyeless Dog".to_string(),
                entity_type: "Outside".to_string(),
                behavior: "Aggressive".to_string(),
                danger_level: Some(20),
                power_level: 3.0,
                spawn_condition: "Day/Night".to_string(),
                moons: vec!["VOW".to_string(), "EXPERIMENTATION".to_string()],
                speed: Some("Fast".to_string()),
                notes: None,
            },
        ],
    };

    let scan = generate_scan_data("VOW", "Clear", 100, &bestiary);

    assert_eq!(scan.weather, "Clear");
    assert_eq!(scan.scrap_value, 100);
    assert!(scan.threat_level > 0, "Should have some threat level");
    assert!(!scan.monsters.is_empty(), "Should select at least one monster");
}

#[test]
fn test_generate_scan_data_no_monsters_for_moon() {
    let bestiary = Bestiary {
        id: Some("test_bestiary".to_string()),
        monsters: vec![
            Monster {
                name: "Test Monster".to_string(),
                entity_type: "Outside".to_string(),
                behavior: "Aggressive".to_string(),
                danger_level: Some(20),
                power_level: 2.0,
                spawn_condition: "Always".to_string(),
                moons: vec!["TITAN".to_string()], // Only on TITAN
                speed: Some("Fast".to_string()),
                notes: None,
            },
        ],
    };

    let scan = generate_scan_data("VOW", "Rainy", 150, &bestiary);

    assert_eq!(scan.weather, "Rainy");
    assert_eq!(scan.scrap_value, 150);
    assert_eq!(scan.threat_level, 0, "Should have zero threat when no monsters");
    assert!(scan.monsters.is_empty(), "Should have no monsters");
}

#[test]
fn test_generate_scan_data_tier_affects_monster_count() {
    let mut monsters = vec![];
    for i in 0..10 {
        monsters.push(Monster {
            name: format!("Monster {}", i),
            entity_type: "Outside".to_string(),
            behavior: "Aggressive".to_string(),
            danger_level: Some(10),
            power_level: 1.0,
            spawn_condition: "Always".to_string(),
            moons: vec!["EXPERIMENTATION".to_string(), "LIQUIDATION".to_string()],
            speed: Some("Medium".to_string()),
            notes: None,
        });
    }

    let bestiary = Bestiary {
        id: Some("test_bestiary".to_string()),
        monsters,
    };

    // Tier 1 moon: should select 1-3 monsters
    let scan_tier1 = generate_scan_data("EXPERIMENTATION", "Clear", 100, &bestiary);
    assert!(
        scan_tier1.monsters.len() >= 1 && scan_tier1.monsters.len() <= 3,
        "Tier 1 should have 1-3 monsters, got {}",
        scan_tier1.monsters.len()
    );

    // Tier 4 moon: should select 4-6 monsters
    let scan_tier4 = generate_scan_data("LIQUIDATION", "Clear", 100, &bestiary);
    assert!(
        scan_tier4.monsters.len() >= 4 && scan_tier4.monsters.len() <= 6,
        "Tier 4 should have 4-6 monsters, got {}",
        scan_tier4.monsters.len()
    );
}
