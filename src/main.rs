use crate::commands::registration;
use crate::data::mongodb;
use crate::commands::commands_fn::run_repl;
use std::sync::Arc;

mod commands {
    pub mod commands_fn;
    pub mod registration;
}
mod data {
    pub mod mongodb;
}
mod models {
    pub mod entities;
    pub mod lists;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let client = Arc::new(mongodb::init_db().await?);
    let game_state = registration::initialize_game(client.clone()).await?;

    run_repl(client, game_state).await
}