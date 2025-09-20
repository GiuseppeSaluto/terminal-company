use crate::derive_struct;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

derive_struct! {
pub struct GameState {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub players: Vec<Player>,
    pub ship: Ship,
    pub turn_number: u32,
    pub is_game_over: bool,
    pub scan_data: HashMap<String, ScanData>,
}}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            id: Some("game_state".to_string()),
            players: vec![],
            ship: Ship {
                location: "Company".to_string(),
                number_operators_alive: 0,
                upgrades: vec![],
                decorations: vec![],
            },
            turn_number: 1,
            is_game_over: false,
            scan_data: HashMap::new(),
        }
    }
}

derive_struct! {
pub struct Player {
    pub name: String,
    pub role: String,
    pub hp: u32,
    pub inventory: Vec<Item>,
    pub credits: u32,
}}

derive_struct! {
pub struct Ship {
    pub location: String,
    pub number_operators_alive: u32,
    pub upgrades: Vec<String>,
    pub decorations: Vec<String>,
}}

derive_struct! {
pub struct Item {
    pub name: String,
    pub price: u32,
    pub weight: f32,
    pub description: String,
}}

derive_struct! {
pub struct Monster {
    pub name: String,
    pub entity_type: String,
    pub behavior: String,
    pub danger_level: Option<String>,
    pub power_level: f32,
    pub spawn_condition: String,
    pub moons: Vec<String>,
    pub speed: Option<String>,
    pub notes: Option<String>,
}}

derive_struct! {
pub struct Bestiary {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub monsters: Vec<Monster>,
}}

impl Default for Bestiary {
    fn default() -> Self {
        Bestiary {
            id: Some("bestiary".to_string()),
            monsters: vec![],
        }
    }
}

derive_struct! {
    pub struct ScanData {
        pub weather: String,
        pub threat_level: u32,
        pub scrap_value: u32,
        #[serde(default)]
        pub monsters: Vec<Monster>,
    }
}

derive_struct! {
    pub struct CollectConfig {
        #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
    pub base_chance: i32,
    pub weather_mods: HashMap<String, i32>,
}}

impl Default for CollectConfig {
    fn default() -> Self {
        let mut weather_mods = HashMap::new();
        weather_mods.insert("Clear".to_string(), 20);
        weather_mods.insert("Rainy".to_string(), -5);
        weather_mods.insert("Foggy".to_string(), -10);
        weather_mods.insert("Stormy".to_string(), -20);
        weather_mods.insert("Eclipsed".to_string(), -30);

        CollectConfig {
            id: Some("collect_config".to_string()),
            base_chance: 50,
            weather_mods,
        }
    }
}
