use crate::commands::registration;
use crate::data::mongodb::load_collect_config;
use crate::data::mongodb::{self, load_bestiary};
use crate::models::collect_credits::CollectCreditsEvent;
use crate::models::lists::{BESTIARY, MOONS, STORE_ITEMS};
use crate::models::scan_logic::generate_scan_data;
use crate::models::types::GameState;
use crate::utils::shortcut::{format_name, println_separator, read_and_normalize_input};
use ::mongodb::Client;
use rand::{self, Rng};
use std::sync::Arc;

pub async fn run_repl(
    client: Arc<Client>,
    mut game_state: GameState,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        if let Some(input) = read_and_normalize_input() {
            match input.as_str() {
                "" => {}
                "MOONS" => handle_moons(),
                "STORE" => handle_store(),
                "INVENTORY" => handle_inv(&game_state),
                "SCAN" => handle_scan(&mut game_state, &client).await,
                "COLLECT" => handle_collect(&client, &mut game_state).await,
                "BESTIARY" => handle_monsters(),
                "LOCATION" => handle_location(&game_state),
                "HELP" => handle_help(),
                "SAVE" => handle_save(client.clone(), &game_state).await,
                "LOAD" => handle_load(client.clone(), &mut game_state).await,
                "QUIT" | "EXIT" => {
                    println!("Exiting game. Goodbye {}!", game_state.players[0].name);
                    return Ok(());
                }
                "NEW GAME" => {
                    println!("Are you sure you want to start a new game? (YES/NO)");
                    if let Some(confirm) = read_and_normalize_input() {
                        if confirm == "YES" {
                            delete_game_state(&client).await;
                            game_state = registration::handle_registration(client.clone()).await?;
                        }
                    }
                }
                cmd if cmd.starts_with("GO TO ") => {
                    let moon = cmd.trim_start_matches("GO TO ").trim();
                    handle_go_to(&mut game_state, moon);
                }
                cmd if cmd.starts_with("BUY ") => {
                    handle_buy(&mut game_state, &cmd);
                }
                _ => println!("❓ Command not recognized. Type 'help' for the list of commands."),
            }
        }
    }
}

pub fn handle_moons() {
    println!("Visitable: {}", MOONS.join(", "));
}

pub fn handle_go_to(game_state: &mut GameState, moon: &str) {
    if MOONS.iter().any(|m| m.eq_ignore_ascii_case(moon)) {
        game_state.ship.location = moon.to_string().to_uppercase();
        println!(
            "Journey to {} underway...",
            format_name(&game_state.ship.location)
        );
        handle_location(game_state);
    } else {
        println!("'{}' Moon not available.", moon);
    }
}

pub fn handle_location(game_state: &GameState) {
    println_separator();
    println!(
        "Your current location is: {}",
        format_name(&game_state.ship.location)
    );
    println_separator();
}

pub fn handle_store() {
    println!("Available Items:");
    for item in STORE_ITEMS.iter() {
        println_separator();
        println!("- {}:", format_name(&item.name));
        println!("  - Price: {} credits", item.price);
        println!("  - Weight: {}", item.weight);
        println!("  - Description: {}", item.description);
    }
    println_separator();
}

pub fn handle_inv(game_state: &GameState) {
    println_separator();
    println!("Your Inventory Status:");
    println_separator();
    let player = &game_state.players[0];

    if player.inventory.is_empty() {
        println!("Your inventory is currently empty. Buy some items from the 'store'!")
    } else {
        for item in player.inventory.iter() {
            println!("- {}: {} credits", item.name, item.price);
            println!("  - Weight: {}", item.weight);
            println!("  - Description: {}", item.description);
        }
    }
    println_separator();
}

pub fn is_at_company(game_state: &GameState) -> bool {
    game_state.ship.location.eq_ignore_ascii_case("COMPANY")
}

pub async fn handle_scan(game_state: &mut GameState, client: &Client) {
    if is_at_company(game_state) {
        println_separator();
        println!("You can't scan anything while at the Company building.");
        println!("Use 'GO TO [moon name]' to travel to a moon.");
        println_separator();
        return;
    }

    let location_key = game_state.ship.location.clone();

    if let Some(scan_data) = game_state.scan_data.get(&location_key) {
        println_separator();
        println!(
            "Scan data for {} is already available:",
            format_name(&location_key)
        );
        println!("- Weather: {}", scan_data.weather);
        println!("- Threat Level: {}%", scan_data.threat_level);
        println!("- Estimated Scrap Value: {} credits", scan_data.scrap_value);
        println_separator();
    } else {
        let mut rng = rand::rng();

        let weather_conditions = ["Clear", "Rainy", "Foggy", "Stormy", "Eclipsed"];
        let random_weather =
            weather_conditions[rng.random_range(0..weather_conditions.len())].to_string();
        let random_scrap_value = rng.random_range(100..1001);

        let bestiary = match load_bestiary(client).await {
            Ok(b) => b,
            Err(e) => {
                println!("⚠️ Error loading bestiary: {}", e);
                return;
            }
        };

        let scan_data = generate_scan_data(
            &game_state.ship.location,
            &random_weather,
            random_scrap_value,
            &bestiary,
        );

        game_state
            .scan_data
            .insert(game_state.ship.location.clone(), scan_data.clone());

        println_separator();
        println!("Scanning environment on {}...", format_name(&location_key));
        println!("Monsters detected:");
        if scan_data.monsters.is_empty() {
            println!("- None detected");
        } else {
            for monster in &scan_data.monsters {
                println!(
                    "- {} (Danger: {})",
                    monster.name,
                    monster.danger_level.unwrap_or(0)
                );
            }
        }
        println!("Scan data generated:");
        println!("- Weather: {}", scan_data.weather);
        println!("- Threat Level: {}%", scan_data.threat_level);
        println!("- Estimated Scrap Value: {} credits", scan_data.scrap_value);
        println_separator();
    }
}

async fn handle_collect(client: &Client, game_state: &mut GameState) {
    let location_key = game_state.ship.location.clone();

    if let Some(scan_data) = game_state.scan_data.get(&location_key) {
        match load_collect_config(client).await {
            Ok(config) => {
                let event = CollectCreditsEvent {
                    scan_data,
                    player_bonus: 0,
                    config: &config,
                };

                let chance = event.calculate_chance();
                println!("Chance to collect credits: {}%", chance);
                println!("Do you want to attempt collecting? (YES/NO)");

                if let Some(confirm) = read_and_normalize_input() {
                    if confirm == "YES" {
                        match event.attempt() {
                            Some(credits) => {
                                println!("✅ You found {} credits!", credits);
                                game_state.players[0].credits += credits;
                            }
                            None => println!("❌ No credits found this time."),
                        }
                    } else {
                        println!("You decided not to collect this time.");
                    }
                }
            }
            Err(e) => {
                println!("⚠️ Error loading collect config: {}", e);
            }
        }
    } else {
        println!(
            "⚠️ No scan data available for {}. Use 'SCAN' first.",
            format_name(&location_key)
        );
    }
}

pub fn handle_monsters() {
    println!("Scannable creatures:");
    for monster in BESTIARY.iter() {
        println!(
            "- {}: {}",
            format_name(&monster.name),
            monster
                .notes
                .as_deref()
                .unwrap_or("No description available.")
        );
    }
}

pub fn handle_help() {
    println!("Commands available:");
    println!("MOONS - Lists visitable planets");
    println!("GO TO [moon name] - Travel to a planet");
    println!("LOCATION - Show your current location");
    println!("STORE - Show the Store Items");
    println!("SCAN - Scan the environment");
    println!("COLLECT - Try to collect scrap credits");
    println!("BESTIARY - Show scannable creatures");
    println!("BUY [item name] - Buy an item");
    println!("INVENTORY - Show your inventory");
    println!("SAVE - Save the game state");
    println!("LOAD - Load the game state");
    println!("NEW GAME - Delete the game state");
    println!("HELP - Show this help");
    println!("QUIT/EXIT - Exit the game");
}

pub fn handle_buy(game_state: &mut GameState, cmd: &str) {
    let item_name = cmd.trim_start_matches("BUY ").trim();
    let player = &mut game_state.players[0];

    if let Some(item) = STORE_ITEMS
        .iter()
        .find(|i| i.name.eq_ignore_ascii_case(item_name))
    {
        if player.credits >= item.price {
            player.credits -= item.price;
            player.inventory.push(item.clone());
            println_separator();
            println!(
                "You have purchased '{}' for {} credits.",
                format_name(&item.name),
                item.price
            );
            println!("Your remaining credits: {}", player.credits);
            println_separator();
        } else {
            println_separator();
            println!(
                "Not enough credits to purchase '{}'.",
                format_name(&item.name)
            );
            println!(
                "You need {} credits, but you have only {}.",
                item.price, player.credits
            );
            println_separator();
        }
    } else {
        println_separator();
        println!("'{}' item not available.", format_name(&item_name));
        println_separator();
    }
}

pub async fn handle_save(client: Arc<Client>, game_state: &GameState) {
    match mongodb::save_game_state(&client, game_state).await {
        Ok(_) => println!("Game state saved successfully."),
        Err(e) => println!("Failed to save game state: {}", e),
    }
}

pub async fn handle_load(client: Arc<Client>, game_state: &mut GameState) {
    match mongodb::load_game_state(&client).await {
        Ok(Some(state)) => {
            *game_state = state;
            println_separator();
            println!("Game state loaded successfully.");
            println_separator();
        }
        Ok(None) => {
            println_separator();
            println!("No saved game state found.");
            println_separator();
        }
        Err(e) => {
            println_separator();
            println!("Error loading game state: {}", e);
            println_separator();
        }
    }
}

pub async fn delete_game_state(client: &Client) {
    match mongodb::delete_game_state(client).await {
        Ok(_) => println!("Game state deleted successfully."),
        Err(e) => println!("Failed to delete game state: {}", e),
    }
}
