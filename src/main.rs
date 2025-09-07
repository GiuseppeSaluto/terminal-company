use crate::commands::commands_fn;
use crate::commands::registration;
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
async fn main() {
    let mut game_state = registration::handle_registration().await;

    // Main terminal loop
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        match input {
            "moons" => commands_fn::handle_moons(),
            cmd if cmd.starts_with("go to ") => {
                let moon = cmd.trim_start_matches("go to ").trim();
                commands_fn::handle_go_to(&mut game_state, moon);
            }
            "store" => commands_fn::handle_store(),
            "inventory" => {
                commands_fn::handle_inv(&game_state);
            }
            // Scanner
            "scan" => {
                commands_fn::handle_scan();
            }
            // Bestiary
            "bestiary" => {
                commands_fn::handle_best();
            }
            // Help
            "help" => {
                commands_fn::handle_help();
            }
            // Buy in the Store
            cmd if cmd.starts_with("buy ") => {
                commands_fn::handle_buy(&mut game_state, cmd);
            }
            "save" => {
                commands_fn::handle_save(&game_state).await;
            }

            "load" => {
                commands_fn::handle_load(&mut game_state).await;
            }
            "" => {}
            _ => {
                println!("Command not recognized. Type 'help' for the list of commands.");
            }
        }
    }
}
