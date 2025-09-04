pub struct Player {
    pub name: String,
    pub role: String,
    pub hp: u32,
    pub inventory: Vec<Item>,
    pub credits: u32
}

pub struct Ship {
    pub location: String,
    pub number_operators_alive: u32,
    pub upgrades: Vec<String>,
    pub decorations: Vec<String>
}

pub struct Item {
    pub name: &'static str,
    pub price: u32,
    pub weight: f32,
    pub description: &'static str,
}

pub struct GameState {
    pub players: Vec<Player>,
    pub ship: Ship,
    pub turn_number: u32,
    pub is_game_over: bool
}