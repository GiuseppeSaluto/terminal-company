use crate::commands::registration;
use crate::data::mongodb;
use crate::data::mongodb::load_collect_config;
use crate::models::collect_events::CollectCreditsEvent;
use crate::models::entities::{GameState, ScanData};
use crate::models::lists::{BESTIARY, MOONS, STORE_ITEMS};
use crate::utils::shortcut::read_and_normalize_input;
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
                "moons" => handle_moons(),
                "store" => handle_store(),
                "inventory" => handle_inv(&game_state),
                "scan" => handle_scan(&mut game_state),
                "collect" => handle_collect(&client, &mut game_state).await,
                "bestiary" => handle_best(),
                "location" => handle_location(&game_state),
                "help" => handle_help(),
                "save" => handle_save(client.clone(), &game_state).await,
                "load" => handle_load(client.clone(), &mut game_state).await,
                "quit" | "exit" => {
                    println!("Exiting game. Goodbye {}!", game_state.players[0].name);
                    return Ok(());
                }
                "new game" => {
                    println!("Are you sure you want to start a new game? (yes/no)");
                    if let Some(confirm) = read_and_normalize_input() {
                        if confirm == "yes" {
                            delete_game_state(&client).await;
                            game_state = registration::handle_registration(client.clone()).await?;
                        }
                    }
                }
                cmd if cmd.starts_with("go to ") => {
                    let moon = cmd.trim_start_matches("go to ").trim();
                    handle_go_to(&mut game_state, moon);
                }
                cmd if cmd.starts_with("buy ") => {
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
        game_state.ship.location = moon.to_string();
        println!("Journey to {} underway...", moon);
        handle_location(game_state);
    } else {
        println!("'{}' Moon not available.", moon);
    }
}

pub fn handle_location(game_state: &GameState) {
    println_separator();
    println!("Your current location is: {}", game_state.ship.location);
    println_separator();
}

pub fn handle_store() {
    println!("Available Items:");
    for item in STORE_ITEMS.iter() {
        let item = item;
        println_separator();
        println!("- {}:", item.name);
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
    game_state.ship.location.eq_ignore_ascii_case("Company")
}

pub fn handle_scan(game_state: &mut GameState) {
    if is_at_company(game_state) {
        println_separator();
        println!("You can't scan anything while at the Company building.");
        println!("Use 'go to [moon name]' to travel to a moon.");
        println_separator();
        return;
    }

    if let Some(scan_data) = game_state.scan_data.get(&game_state.ship.location) {
        println_separator();
        println!(
            "Scan data for {} is already available:",
            game_state.ship.location
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
        let random_threat_level = rng.random_range(1..101);
        let random_scrap_value = rng.random_range(100..1001);

        let scan_data = ScanData {
            weather: random_weather,
            threat_level: random_threat_level,
            scrap_value: random_scrap_value,
        };

        game_state
            .scan_data
            .insert(game_state.ship.location.clone(), scan_data.clone());

        println_separator();
        println!("Scanning environment on {}...", game_state.ship.location);
        println!("Scan data generated:");
        println!("- Weather: {}", scan_data.weather);
        println!("- Threat Level: {}%", scan_data.threat_level);
        println!("- Estimated Scrap Value: {} credits", scan_data.scrap_value);
        println_separator();
    }
}

async fn handle_collect(client: &Client, game_state: &mut GameState) {
    let location = &game_state.ship.location;

    if let Some(scan_data) = game_state.scan_data.get(location) {
        match load_collect_config(client).await {
            Ok(config) => {
                let event = CollectCreditsEvent {
                    scan_data,
                    player_bonus: 0,
                    config: &config,
                };

                match event.attempt() {
                    Some(credits) => {
                        println!("✅ You found {} credits!", credits);
                        game_state.players[0].credits += credits;
                    }
                    None => println!("❌ No credits found this time."),
                }
            }
            Err(e) => {
                println!("⚠️ Error loading collect config: {}", e);
            }
        }
    } else {
        println!(
            "⚠️ No scan data available for {}. Use 'scan' first.",
            location
        );
    }
}

pub fn handle_best() {
    println!("scannable creatures:");
    for (name, desc) in BESTIARY {
        println!("- {}: {}", name, desc);
    }
}

pub fn handle_help() {
    println!("Commands available:");
    println!("moons - Lists visitable planets");
    println!("go to [moon name] - Travel to a planet");
    println!("location - Show your current location");
    println!("store - Show the Store Items");
    println!("scan - Scan the environment");
    println!("collect - Try to collect scrap credits");
    println!("bestiary - Show scannable creatures");
    println!("buy [item name] - Buy an item");
    println!("inventory - Show your inventory");
    println!("save - Save the game state");
    println!("load - Load the game state");
    println!("new game - Delete the game state");
    println!("help - Show this help");
    println!("quit/exit - Exit the game");
}

pub fn handle_buy(game_state: &mut GameState, cmd: &str) {
    let item_name = cmd.trim_start_matches("buy ").trim();
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
                item.name, item.price
            );
            println!("Your remaining credits: {}", player.credits);
            println_separator();
        } else {
            println_separator();
            println!("Not enough credits to purchase '{}'.", item.name);
            println!(
                "You need {} credits, but you have only {}.",
                item.price, player.credits
            );
            println_separator();
        }
    } else {
        println_separator();
        println!("'{}' item not available.", item_name);
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

pub fn println_separator() {
    println!("-------------------------------------------------------------");
}
