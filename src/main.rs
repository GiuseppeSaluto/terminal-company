use std::io::{self, Write};
use rand;

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

        let mut moons = vec![
            "Experimentation",
            "Assurance",
            "Vow",
            "Offense",
            "March",
            "Adamance",
            "Rend",
            "Dine",
            "Titan",
            "Artifice",
            "Embrion",
            "Liquidation"
        ];
        let mut store_items = vec![
            "Shovel",
            "Spray Paint",
            "SStun Grenade",
            "TZP-Inhalant",
            "Walkie-Talkie",
            "Zap Gun",
            "Weed Killer",
            "Cruiser",
            "Belt Bag",
            "Survival Kit"
        ];
        let mut ship_upgrade = vec![
            "Teleporter",
            "Inverse Teleporter",
            "Loud Horn",
            "Signal Translator"
        ];
        let mut ship_decorations = vec![
            "Cozy Lights",
            "Decoy Suit",
            "Brown Suit",
            "Purple Suit"
        ];
        let mut bestiary = vec![
            (
                "Xenomorph",
                "Aggressive alien lifeform, avoid close contact.",
            ),
            ("Space Slug", "Slow, harmless, but can block passages."),
            ("Crystal Spider", "Fast, poisonous, found in caves."),
        ];

        match input {
            "moons" => {
                println!("Visitabili: {}", moons.join(", "));
            }
            cmd if cmd.starts_with("go to ") => {
                let moon = cmd.trim_start_matches("go to ").trim();
                if moons.iter().any(|m| m.eq_ignore_ascii_case(moon)) {
                    println!("Viaggio verso {} in corso...", moon);
                } else {
                    println!("Pianeta '{}' non disponibile.", moon);
                }
            }
            "scan" => {
                println!("Scansione ambiente...");
                println!("Nemici rilevati: {}", rand::random::<u8>() % 5);
                println!(
                    "Valore totale oggetti: {} crediti",
                    rand::random::<u16>() % 1000
                );
            }
            cmd if cmd.starts_with("view ") => {
                let ship = cmd.trim_start_matches("view ").trim();
                if ship_upgrade.iter().any(|s| s.eq_ignore_ascii_case(ship)) {
                    println!("Telecamera spostata su '{}'.", ship);
                } else {
                    println!("Nave '{}' non trovata.", ship);
                }
            }
            "bestiary" => {
                println!("Creature scannerizzabili:");
                for (name, desc) in &bestiary {
                    println!("- {}: {}", name, desc);
                }
            }
            "help" => {
                println!("Comandi disponibili:");
                println!("moons - Elenca i pianeti visitabili");
                println!("go to [moon name] - Viaggia verso un pianeta");
                println!("scan - Scansiona l'ambiente");
                println!("view [ship name] - Cambia telecamera su una nave");
                println!("bestiary - Mostra le creature scannerizzabili");
                println!("buy [item name] - Acquista un oggetto");
                println!("help - Mostra questo aiuto");
            }
            cmd if cmd.starts_with("buy ") => {
                let store_item = cmd.trim_start_matches("buy ").trim();
                if store_items.iter().any(|i| i.eq_ignore_ascii_case(store_item)) {
                    println!("Hai acquistato '{}'.", store_item);
                } else {
                    println!("Oggetto '{}' non disponibile.", store_item);
                    println!("Oggetti disponibili: {}", store_items.join(", "));
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
