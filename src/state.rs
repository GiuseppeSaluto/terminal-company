use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub role: String,
    pub hp: u32,
    pub inventory: Vec<Item>,
    pub credits: u32
}
#[derive(Serialize, Deserialize)]
pub struct Ship {
    pub location: String,
    pub number_operators_alive: u32,
    pub upgrades: Vec<String>,
    pub decorations: Vec<String>
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Item {
    pub name: String,
    pub price: u32,
    pub weight: f32,
    pub description: String,
}
#[derive(Serialize, Deserialize)]
pub struct GameState {
    pub players: Vec<Player>,
    pub ship: Ship,
    pub turn_number: u32,
    pub is_game_over: bool
}