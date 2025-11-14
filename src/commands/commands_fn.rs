use crate::models::types::GameState;
use ::mongodb::Client;

pub fn is_at_company(game_state: &GameState) -> bool {
    game_state.ship.location.eq_ignore_ascii_case("COMPANY")
}

pub async fn delete_game_state(client: &Client) {
    match crate::data::mongodb::delete_game_state(client).await {
        Ok(_) => {},
        Err(_) => {},
    }
}
