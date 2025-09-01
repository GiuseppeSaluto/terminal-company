struct Player {
    name: String,
    role: String,
    hp: u32,
    inventory: Vec<String>,
    credits: u32
}

struct Ship {
    location: String,
    number_operators_alive: u32,
    upgrades: Vec<String>,
    decorations: Vec<String>
}


struct GameState {
    players: Vec<Player>,
    ship: Ship,
    turn_number: u32,
    is_game_over: bool
}