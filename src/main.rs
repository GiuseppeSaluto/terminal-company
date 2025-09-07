mod models {
    pub mod entities;
    pub mod lists;
}
mod data {
    pub mod mongodb;
}
mod commands {
    pub mod commands_fn;
}
use crate::commands::commands_fn;
use models::entities::{GameState, Player, Ship};
use std::io::{self, Write};

#[tokio::main]
async fn main() {
    println!("Booting Terminal Company OS...");
    println!("Welcome to Terminal Company.");
    println!("Before proceeding, you must accept the Terms and Conditions.");
    println!("Type 'ACCEPT' to continue or 'DENY' to exit.");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().to_uppercase();

        if input == "ACCEPT" {
            println!("Thank you. Access granted.");
            break;
        } else if input == "DENY" {
            println!("Access denied. Shutting down...");
            return;
        } else {
            println!("Please type 'ACCEPT' or 'DENY'.");
        }
    }

    // GameState
    let mut game_state = GameState {
        players: vec![Player {
            name: "tester01".to_string(),
            role: "Operator".to_string(),
            hp: 100,
            inventory: Vec::new(),
            credits: 30,
        }],
        ship: Ship {
            location: "Company".to_string(),
            number_operators_alive: 1,
            upgrades: Vec::new(),
            decorations: Vec::new(),
        },
        turn_number: 1,
        is_game_over: false,
    };

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
            "save" => match data::mongodb::save_game_state(&game_state).await {
                Ok(_) => println!("Game state saved successfully."),
                Err(e) => println!("Failed to save game state: {}", e),
            },
            "load" => match data::mongodb::load_game_state().await {
                Ok(Some(state)) => {
                    game_state = state;
                    println!("-------------------------------------------------------------");
                    println!("Game state loaded successfully.");
                    println!("-------------------------------------------------------------");
                }
                Ok(None) => {
                    println!("-------------------------------------------------------------");
                    println!("No saved game state found.");
                    println!("-------------------------------------------------------------");
                }
                Err(e) => {
                    println!("-------------------------------------------------------------");
                    println!("Error loading game state: {}", e);
                    println!("-------------------------------------------------------------");
                }
            },
            "" => {}
            _ => {
                println!("Command not recognized. Type 'help' for the list of commands.");
            }
        }
    }
}
