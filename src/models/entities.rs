use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    pub name: String,
    pub role: String,
    pub hp: u32,
    pub inventory: Vec<Item>,
    pub credits: u32,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Ship {
    pub location: String,
    pub number_operators_alive: u32,
    pub upgrades: Vec<String>,
    pub decorations: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Item {
    pub name: String,
    pub price: u32,
    pub weight: f32,
    pub description: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct GameState {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub players: Vec<Player>,
    pub ship: Ship,
    pub turn_number: u32,
    pub is_game_over: bool,
    pub scan_data: Option<ScanData>
}

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
            scan_data: None,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ScanData {
    pub weather: String,
    pub threat_level: u32,
    pub scrap_value: u32,
}
