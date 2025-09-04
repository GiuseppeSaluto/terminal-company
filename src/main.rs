use crate::data::{BESTIARY, MOONS, STORE_ITEMS};
use crate::state::{GameState, Player, Ship};
use rand;
use std::io::{self, Write};

mod data;
mod state;

fn main() {
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
            name: "testOperator".to_string(),
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
            "moons" => {
                println!("Visitable: {}", MOONS.join(", "));
            }
            cmd if cmd.starts_with("go to ") => {
                let moon = cmd.trim_start_matches("go to ").trim();
                if MOONS.iter().any(|m| m.eq_ignore_ascii_case(moon)) {
                    game_state.ship.location = moon.to_string();
                    println!("Journey to {} underway...", moon);
                    println!("Your current location is: {}", game_state.ship.location);
                } else {
                    println!("'{}' Moon not available.", moon);
                }
            }
            "scan" => {
                println!("environment scan...");
                println!("enemies detected: {}", rand::random::<u8>() % 5);
                println!(
                    "Total value of objects: {} credits",
                    rand::random::<u16>() % 1000
                );
            }
            "bestiary" => {
                println!("scannable creatures:");
                for (name, desc) in BESTIARY {
                    println!("- {}: {}", name, desc);
                }
            }
            "help" => {
                println!("Commands available:");
                println!("moons - Lists visitable planets");
                println!("go to [moon name] - Travel to a planet");
                println!("scan - Scan the environment");
                println!("bestiary - Show scannable creatures");
                println!("buy [item name] - Buy an item");
                println!("help - Show this help");
            }
            cmd if cmd.starts_with("buy ") => {
                let item_name = cmd.trim_start_matches("buy ").trim();
                let player = &mut game_state.players[0];

                // search object
                if let Some(item) = STORE_ITEMS
                    .iter()
                    .find(|&i| i.name.eq_ignore_ascii_case(item_name))
                {
                    if player.credits >= item.price {
                        player.credits -= item.price;
                        player.inventory.push(item.clone());
                        println!(
                            "You have purchased '{}' for {} credits.",
                            item.name, item.price
                        );
                        println!("Your remaining credits: {}", player.credits);
                    } else {
                        println!("Not enough credits to purchase '{}'.", item.name);
                        println!(
                            "You need {} credits, but you have only {}.",
                            item.price, player.credits
                        );
                    }
                } else {
                    println!("'{}' item not available.", item_name);
                }
            }
            "" => {}
            _ => {
                println!("Command not recognized. Type 'help' for the list of commands.");
            }
        }
    }
}
