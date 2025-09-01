use rand;
use std::io::{self, Write};

use crate::data::{BESTIARY, MOONS, SHIP_UPGRADE, STORE_ITEMS};

mod data;

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
                    println!("Journey to {} underway...", moon);
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
            // cmd if cmd.starts_with("view ") => {
            //     let ship = cmd.trim_start_matches("view ").trim();
            //     if SHIP_UPGRADE.iter().any(|s| s.eq_ignore_ascii_case(ship)) {
            //         println!("Telecamera spostata su '{}'.", ship);
            //     } else {
            //         println!("Nave '{}' non trovata.", ship);
            //     }
            // }
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
                println!("view [ship name] - Switch cameras on an operator");
                println!("bestiary - Show scannable creatures");
                println!("buy [item name] - Buy an item");
                println!("help - Show this help");
            }
            cmd if cmd.starts_with("buy ") => {
                let store_item = cmd.trim_start_matches("buy ").trim();
                if STORE_ITEMS
                    .iter()
                    .any(|i| i.eq_ignore_ascii_case(store_item))
                {
                    println!("Hai acquistato '{}'.", store_item);
                } else {
                    println!("Oggetto '{}' non disponibile.", store_item);
                    println!("Oggetti disponibili: {}", STORE_ITEMS.join(", "));
                }
            }
            "" => {}
            _ => {
                println!("Comando non riconosciuto. Digita 'help' per la lista dei comandi.");
            }
        }
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
    }
}
