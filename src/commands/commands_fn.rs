use std::sync::Arc;

use crate::data::mongodb;
use crate::models::entities::GameState;
use crate::models::lists::{BESTIARY, MOONS, STORE_ITEMS};
use ::mongodb::Client;
use rand;

pub fn handle_moons() {
    println!("Visitable: {}", MOONS.join(", "));
}

pub fn handle_go_to(game_state: &mut GameState, moon: &str) {
    if MOONS.iter().any(|m| m.eq_ignore_ascii_case(moon)) {
        game_state.ship.location = moon.to_string();
        println!("Journey to {} underway...", moon);
        println!("Your current location is: {}", game_state.ship.location);
    } else {
        println!("'{}' Moon not available.", moon);
    }
}

pub fn handle_store() {
    println!("Available Items:");
    for item in STORE_ITEMS.iter() {
        let item = item;
        println!("-------------------------------------------------------------");
        println!("- {}:", item.name);
        println!("  - Price: {} credits", item.price);
        println!("  - Weight: {}", item.weight);
        println!("  - Description: {}", item.description);
    }
    println!("-------------------------------------------------------------");
}

pub fn handle_inv(game_state: &GameState) {
    println!("-------------------------------------------------------------");
    println!("Your Inventory Status:");
    println!("-------------------------------------------------------------");
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
    println!("-------------------------------------------------------------");
}

pub fn handle_scan() {
    println!("environment scan...");
    println!("enemies detected: {}", rand::random::<u8>() % 5);
    println!(
        "Total value of objects: {} credits",
        rand::random::<u16>() % 1000
    );
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
    println!("store - Show the Store Items");
    println!("scan - Scan the environment");
    println!("bestiary - Show scannable creatures");
    println!("buy [item name] - Buy an item");
    println!("inventory - Show your inventory");
    println!("save - Save the game state");
    println!("load - Load the game state");
    println!("help - Show this help");
}

pub fn handle_buy(game_state: &mut GameState, cmd: &str) {
    let item_name = cmd.trim_start_matches("buy ").trim();
    let player = &mut game_state.players[0];

    if let Some(item) = STORE_ITEMS.iter().find(|i| i.name.eq_ignore_ascii_case(item_name)) {
        if player.credits >= item.price {
            player.credits -= item.price;
            player.inventory.push(item.clone());
            println!("-------------------------------------------------------------");
            println!(
                "You have purchased '{}' for {} credits.",
                item.name, item.price
            );
            println!("Your remaining credits: {}", player.credits);
            println!("-------------------------------------------------------------");
        } else {
            println!("-------------------------------------------------------------");
            println!("Not enough credits to purchase '{}'.", item.name);
            println!(
                "You need {} credits, but you have only {}.",
                item.price, player.credits
            );
            println!("-------------------------------------------------------------");
        }
    } else {
        println!("-------------------------------------------------------------");
        println!("'{}' item not available.", item_name);
        println!("-------------------------------------------------------------");
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
    }
}
