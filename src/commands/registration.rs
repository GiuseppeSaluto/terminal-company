use crate::data::mongodb;
use crate::models::entities::{GameState, Player, Ship};
use ::mongodb::Client;
use std::io::{self, Write};
use std::sync::Arc;
use std::{thread, time};

pub async fn handle_registration(client: Arc<Client>) -> GameState {
    println!("Booting Terminal Company OS...");
    println!("Welcome to Terminal Company.");

    let suspicious_documents = vec![
        "DOC: INSUFFICIENT FUNDS, LIABILITIES OUTWEIGH ASSETS",
        "DOC: COMPANY POLICY 11B-3, ARTICLE 4: NO REFUNDS ON DECEASED OPERATORS",
        "DOC: CONTRACTOR DEBT ACCUMULATED: 92837 CREDITS",
        "DOC: LETHAL COMPANY AGREEMENT VERIFIED: MINIMUM REVENUE MET",
        "DOC: ALL ASSETS ARE THE SOLE PROPERTY OF THE COMPANY",
        "DOC: PERSONNEL DATA RETENTION: 98.7% CHANCE OF MORTALITY",
        "DOC: DEBT COLLECTION AUTOMATION IN PROGRESS",
    ];

    for doc in suspicious_documents {
        println!("{}", doc);
        thread::sleep(time::Duration::from_millis(400));
    }

    println!("-------------------------------------------------------------");
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
            std::process::exit(0);
        } else {
            println!("Please type 'ACCEPT' or 'DENY'.");
        }
    }

    println!("-------------------------------------------------------------");
    println!("Please enter your Operator data.");

    print!("Name: ");
    io::stdout().flush().unwrap();
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();
    let name = name.trim().to_string();

    print!("Role: ");
    io::stdout().flush().unwrap();
    let mut role = String::new();
    io::stdin().read_line(&mut role).unwrap();
    let role = role.trim().to_string();

    let game_state = GameState {
        players: vec![Player {
            name,
            role,
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

    mongodb::save_game_state(&client, &game_state)
        .await
        .expect("Failed to save initial game state.");

    game_state
}
