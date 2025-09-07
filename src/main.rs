use crate::commands::registration;
use crate::data::mongodb;
use std::io::{self, Write};
use tokio;

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
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = mongodb::init_db().await?;
    let mut game_state = registration::handle_registration(client.clone()).await;

    // Main terminal loop
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        match input {
            "moons" => commands::commands_fn::handle_moons(),
            cmd if cmd.starts_with("go to ") => {
                let moon = cmd.trim_start_matches("go to ").trim();
                commands::commands_fn::handle_go_to(&mut game_state, moon);
            }
            "store" => commands::commands_fn::handle_store(),
            "inventory" => commands::commands_fn::handle_inv(&game_state),
            "scan" => commands::commands_fn::handle_scan(),
            "bestiary" => commands::commands_fn::handle_best(),
            "help" => commands::commands_fn::handle_help(),
            cmd if cmd.starts_with("buy ") => {
                commands::commands_fn::handle_buy(&mut game_state, cmd);
            }
            "save" => {
                commands::commands_fn::handle_save(client.clone(), &game_state).await;
            }
            "load" => {
                commands::commands_fn::handle_load(client.clone(), &mut game_state).await;
            }
            "" => {}
            _ => {
                println!("Command not recognized. Type 'help' for the list of commands.");
            }
        }
    }
}
