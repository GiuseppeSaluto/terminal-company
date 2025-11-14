use crate::ui::app::{App, ConfirmationType, InputMode};
use crate::commands::commands_fn;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseEvent, MouseEventKind};

pub fn handle_mouse_event(app: &mut App, mouse: MouseEvent) {
    match mouse.kind {
        MouseEventKind::ScrollUp => {
            app.scroll_up(3);
        }
        MouseEventKind::ScrollDown => {
            app.scroll_down(3);
        }
        _ => {}
    }
}

pub async fn handle_key_event(app: &mut App, key: KeyEvent) {
    match app.input_mode {
        InputMode::Normal => handle_normal_mode(app, key).await,
        InputMode::Confirmation => handle_confirmation_mode(app, key).await,
        InputMode::Editing => handle_editing_mode(app, key).await,
    }
}

async fn handle_normal_mode(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.should_quit = true;
        }
        KeyCode::Esc => {
            app.request_confirmation(
                ConfirmationType::Quit,
                "Are you sure you want to quit?"
            );
        }
        KeyCode::Enter => {
            if !app.input.is_empty() {
                let command = app.input.trim().to_uppercase();
                app.add_message(&format!("> {}", command));
                app.clear_input();
                
                app.scroll_to_bottom();
                
                execute_command(app, &command).await;
            }
        }
        KeyCode::Char(c) => {
            app.enter_char(c);
        }
        KeyCode::Backspace => {
            app.delete_char();
        }
        KeyCode::Left => {
            app.move_cursor_left();
        }
        KeyCode::Right => {
            app.move_cursor_right();
        }
        KeyCode::Home => {
            app.move_cursor_home();
        }
        KeyCode::End => {
            app.move_cursor_end();
        }
        KeyCode::PageUp => {
            app.scroll_up(10);
        }
        KeyCode::PageDown => {
            app.scroll_down(10);
        }
        _ => {}
    }
}

async fn handle_confirmation_mode(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Enter => {
            let response = app.input.trim().to_uppercase();
            app.add_message(&format!("> {}", response));
            
            if let Some(conf_type) = app.confirmation_type.clone() {
                match conf_type {
                    ConfirmationType::Quit => {
                        if response == "YES" || response == "Y" {
                            app.add_message(&format!("Goodbye, {}! 👋", app.game_state.players[0].name));
                            app.should_quit = true;
                        } else {
                            app.add_message("Cancelled.");
                        }
                    }
                    ConfirmationType::NewGame => {
                        if response == "YES" || response == "Y" {
                            app.is_processing = true;
                            app.add_message("Starting new game...");
                            
                            // Elimina il vecchio stato e crea uno nuovo
                            commands_fn::delete_game_state(&app.db_client).await;
                            match crate::commands::registration::handle_registration(app.db_client.clone()).await {
                                Ok(new_state) => {
                                    app.game_state = new_state;
                                    app.add_message("✨ New game started!");
                                    app.add_message(&format!("Welcome, {}!", app.game_state.players[0].name));
                                }
                                Err(e) => {
                                    app.add_message(&format!("⚠️ Error starting new game: {}", e));
                                }
                            }
                            
                            app.is_processing = false;
                        } else {
                            app.add_message("Cancelled starting a new game.");
                        }
                    }
                    ConfirmationType::AcceptTerms => {
                        if response == "ACCEPT" {
                            app.add_message("Thank you. Access granted.");
                        } else if response == "DENY" {
                            app.add_message("Access denied. Shutting down...");
                            app.should_quit = true;
                        } else {
                            app.add_message("Please type 'ACCEPT' or 'DENY'.");
                            app.clear_input();
                            return;
                        }
                    }
                    ConfirmationType::Collect => {
                        if response == "YES" || response == "Y" {
                            app.is_processing = true;
                            
                            let location_key = app.game_state.ship.location.clone();
                            if let Some(scan_data) = app.game_state.scan_data.get(&location_key) {
                                match crate::data::mongodb::load_collect_config(&app.db_client).await {
                                    Ok(config) => {
                                        use crate::models::collect_credits::CollectCreditsEvent;
                                        let event = CollectCreditsEvent {
                                            scan_data,
                                            player_bonus: 0,
                                            config: &config,
                                        };
                                        
                                        match event.attempt() {
                                            Some(credits) => {
                                                app.add_message(&format!("✅ You found {} credits!", credits));
                                                app.game_state.players[0].credits += credits;
                                            }
                                            None => {
                                                app.add_message("❌ No credits found this time.");
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        app.add_message(&format!("⚠️ Error loading collect config: {}", e));
                                    }
                                }
                            }
                            
                            app.is_processing = false;
                        } else {
                            app.add_message("You decided not to collect this time.");
                        }
                    }
                }
            }
            
            app.input_mode = InputMode::Normal;
            app.confirmation_type = None;
            app.clear_input();
        }
        KeyCode::Char(c) => {
            app.enter_char(c);
        }
        KeyCode::Backspace => {
            app.delete_char();
        }
        KeyCode::Esc => {
            app.add_message("Cancelled.");
            app.input_mode = InputMode::Normal;
            app.confirmation_type = None;
            app.clear_input();
        }
        _ => {}
    }
}

async fn handle_editing_mode(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Enter => {
            let value = app.input.clone();
            app.add_message(&format!("> {}", value));
            
            app.input_mode = InputMode::Normal;
            app.clear_input();
        }
        KeyCode::Char(c) => {
            app.enter_char(c);
        }
        KeyCode::Backspace => {
            app.delete_char();
        }
        KeyCode::Esc => {
            app.add_message("Cancelled.");
            app.input_mode = InputMode::Normal;
            app.clear_input();
        }
        _ => {}
    }
}

async fn execute_command(app: &mut App, command: &str) {
    app.is_processing = true;
    
    let result = match command {
        "" => return,
        "MOONS" => {
            handle_moons_ui(app);
            Ok::<(), Box<dyn std::error::Error>>(())
        }
        "STORE" => {
            handle_store_ui(app);
            Ok(())
        }
        "INVENTORY" | "INV" => {
            handle_inv_ui(app);
            Ok(())
        }
        "SCAN" => {
            handle_scan_ui(app).await;
            Ok(())
        }
        "COLLECT" => {
            handle_collect_ui(app).await;
            Ok(())
        }
        "BESTIARY" => {
            handle_monsters_ui(app);
            Ok(())
        }
        "LOCATION" | "LOC" => {
            handle_location_ui(app);
            Ok(())
        }
        "HELP" => {
            handle_help_ui(app);
            Ok(())
        }
        "SAVE" => {
            handle_save_ui(app).await;
            Ok(())
        }
        "LOAD" => {
            handle_load_ui(app).await;
            Ok(())
        }
        "QUIT" | "EXIT" => {
            app.request_confirmation(
                ConfirmationType::Quit,
                &format!("Are you sure you want to quit, {}?", app.game_state.players[0].name)
            );
            Ok(())
        }
        "NEW GAME" => {
            app.request_confirmation(
                ConfirmationType::NewGame,
                "Are you sure you want to start a new game? All progress will be lost."
            );
            Ok(())
        }
        cmd if cmd.starts_with("GO TO ") => {
            let moon = &cmd[6..].trim();
            handle_go_to_ui(app, moon);
            Ok(())
        }
        cmd if cmd.starts_with("BUY ") => {
            handle_buy_ui(app, cmd);
            Ok(())
        }
        _ => {
            app.add_message(&format!("⚠️  Unknown command: '{}'. Type HELP for available commands.", command));
            Ok(())
        }
    };
    
    if let Err(e) = result {
        app.add_message(&format!("⚠️ Command failed: {:?}", e));
    }
    
    app.is_processing = false;
}

// Versioni UI-friendly dei comandi che aggiungono messaggi all'app invece di stampare

fn handle_moons_ui(app: &mut App) {
    use crate::models::lists::MOONS;
    app.add_message(&format!("Visitable: {}", MOONS.join(", ")));
}

fn handle_store_ui(app: &mut App) {
    use crate::models::lists::STORE_ITEMS;
    use crate::utils::shortcut::format_name;
    
    app.add_message("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    app.add_message("Available Items:");
    for item in STORE_ITEMS.iter() {
        app.add_message(&format!("- {}:", format_name(&item.name)));
        app.add_message(&format!("  💰 Price: {} credits", item.price));
        app.add_message(&format!("  ⚖️  Weight: {}", item.weight));
        app.add_message(&format!("  📝 Description: {}", item.description));
        app.add_message("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    }
}

fn handle_inv_ui(app: &mut App) {
    app.add_message("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    app.add_message("Your Inventory Status:");
    app.add_message("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    let is_empty = app.game_state.players[0].inventory.is_empty();
    
    if is_empty {
        app.add_message("Your inventory is currently empty. Buy some items from the 'store'!");
    } else {
        let inventory = app.game_state.players[0].inventory.clone();
        for item in inventory.iter() {
            app.add_message(&format!("- {}: {} credits", item.name, item.price));
            app.add_message(&format!("  ⚖️  Weight: {}", item.weight));
            app.add_message(&format!("  📝 Description: {}", item.description));
        }
    }
    app.add_message("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
}

async fn handle_scan_ui(app: &mut App) {
    use crate::utils::shortcut::format_name;
    use crate::data::mongodb::load_bestiary;
    use crate::models::scan_logic::generate_scan_data;
    use rand::Rng;
    
    if commands_fn::is_at_company(&app.game_state) {
        app.add_message("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        app.add_message("You can't scan anything while at the Company building.");
        app.add_message("Use 'GO TO [moon name]' to travel to a moon.");
        app.add_message("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        return;
    }
    
    let location_key = app.game_state.ship.location.clone();
    
    if let Some(scan_data) = app.game_state.scan_data.get(&location_key).cloned() {
        app.add_message("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        app.add_message(&format!("Scan data for {} is already available:", format_name(&location_key)));
        app.add_message(&format!("☁️  Weather: {}", scan_data.weather));
        app.add_message(&format!("⚠️  Threat Level: {}%", scan_data.threat_level));
        app.add_message(&format!("💰 Estimated Scrap Value: {} credits", scan_data.scrap_value));
        app.add_message("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    } else {
        let mut rng = rand::rng();
        let weather_conditions = ["Clear", "Rainy", "Foggy", "Stormy", "Eclipsed"];
        let random_weather = weather_conditions[rng.random_range(0..weather_conditions.len())].to_string();
        let random_scrap_value = rng.random_range(100..1001);
        
        let bestiary = match load_bestiary(&app.db_client).await {
            Ok(b) => b,
            Err(e) => {
                app.add_message(&format!("⚠️ Error loading bestiary: {}", e));
                return;
            }
        };
        
        let scan_data = generate_scan_data(
            &app.game_state.ship.location,
            &random_weather,
            random_scrap_value,
            &bestiary,
        );
        
        app.add_message("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        app.add_message(&format!("Scanning environment on {}...", format_name(&location_key)));
        app.add_message("Monsters detected:");
        if scan_data.monsters.is_empty() {
            app.add_message("- None detected");
        } else {
            for monster in &scan_data.monsters {
                app.add_message(&format!("- {} (Danger: {})", monster.name, monster.danger_level.unwrap_or(0)));
            }
        }
        app.add_message("Scan data generated:");
        app.add_message(&format!("☁️  Weather: {}", scan_data.weather));
        app.add_message(&format!("⚠️  Threat Level: {}%", scan_data.threat_level));
        app.add_message(&format!("💰 Estimated Scrap Value: {} credits", scan_data.scrap_value));
        app.add_message("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        
        app.game_state.scan_data.insert(location_key, scan_data);
    }
}

async fn handle_collect_ui(app: &mut App) {
    use crate::data::mongodb::load_collect_config;
    use crate::models::collect_credits::CollectCreditsEvent;
    use crate::utils::shortcut::format_name;
    
    let location_key = app.game_state.ship.location.clone();
    
    if let Some(scan_data) = app.game_state.scan_data.get(&location_key) {
        match load_collect_config(&app.db_client).await {
            Ok(config) => {
                let event = CollectCreditsEvent {
                    scan_data,
                    player_bonus: 0,
                    config: &config,
                };
                
                let chance = event.calculate_chance();
                app.add_message(&format!("Chance to collect credits: {}%", chance));
                app.add_message("Do you want to attempt collecting? (YES/NO)");
                
                app.input_mode = InputMode::Confirmation;
                app.confirmation_type = Some(ConfirmationType::Collect);
            }
            Err(e) => {
                app.add_message(&format!("⚠️ Error loading collect config: {}", e));
            }
        }
    } else {
        app.add_message(&format!("⚠️ No scan data available for {}. Use 'SCAN' first.", format_name(&location_key)));
    }
}

fn handle_monsters_ui(app: &mut App) {
    use crate::models::lists::BESTIARY;
    use crate::utils::shortcut::format_name;
    
    app.add_message("Scannable creatures:");
    for monster in BESTIARY.iter() {
        app.add_message(&format!(
            "- {}: {}",
            format_name(&monster.name),
            monster.notes.as_deref().unwrap_or("No description available.")
        ));
    }
}

fn handle_location_ui(app: &mut App) {
    use crate::utils::shortcut::format_name;
    app.add_message("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    app.add_message(&format!("Your current location is: {}", format_name(&app.game_state.ship.location)));
    app.add_message("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
}

fn handle_help_ui(app: &mut App) {
    app.add_message("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    app.add_message("Commands available:");
    app.add_message("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    app.add_message("MOONS            - Lists visitable planets");
    app.add_message("GO TO [moon]     - Travel to a planet");
    app.add_message("LOCATION         - Show your current location");
    app.add_message("STORE            - Show the Store Items");
    app.add_message("SCAN             - Scan the environment");
    app.add_message("COLLECT          - Try to collect scrap credits");
    app.add_message("BESTIARY         - Show scannable creatures");
    app.add_message("BUY [item]       - Buy an item");
    app.add_message("INVENTORY        - Show your inventory");
    app.add_message("SAVE             - Save the game state");
    app.add_message("LOAD             - Load the game state");
    app.add_message("NEW GAME         - Delete the game state");
    app.add_message("HELP             - Show this help");
    app.add_message("QUIT/EXIT        - Exit the game");
    app.add_message("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
}

fn handle_go_to_ui(app: &mut App, moon: &str) {
    use crate::models::lists::MOONS;
    use crate::utils::shortcut::format_name;
    
    if MOONS.iter().any(|m| m.eq_ignore_ascii_case(moon)) {
        app.game_state.ship.location = moon.to_string().to_uppercase();
        app.add_message(&format!("Journey to {} underway...", format_name(&app.game_state.ship.location)));
        handle_location_ui(app);
    } else {
        app.add_message(&format!("'{}' Moon not available.", moon));
    }
}

fn handle_buy_ui(app: &mut App, cmd: &str) {
    use crate::models::lists::STORE_ITEMS;
    use crate::utils::shortcut::format_name;
    
    let item_name = cmd.trim_start_matches("BUY ").trim();
    
    if let Some(item) = STORE_ITEMS.iter().find(|i| i.name.eq_ignore_ascii_case(item_name)) {
        let player_credits = app.game_state.players[0].credits;
        let item_price = item.price;
        let item_name_formatted = format_name(&item.name);
        
        if player_credits >= item_price {
            app.game_state.players[0].credits -= item_price;
            app.game_state.players[0].inventory.push(item.clone());
            app.add_message("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            app.add_message(&format!("✨ You have purchased '{}' for {} credits.", item_name_formatted, item_price));
            app.add_message(&format!("Your remaining credits: {}", app.game_state.players[0].credits));
            app.add_message("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        } else {
            app.add_message("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            app.add_message(&format!("⚠️ Not enough credits to purchase '{}'.", item_name_formatted));
            app.add_message(&format!("You need {} credits, but you have only {}.", item_price, player_credits));
            app.add_message("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        }
    } else {
        app.add_message("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        app.add_message(&format!("'{}' item not available.", format_name(&item_name)));
        app.add_message("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    }
}

async fn handle_save_ui(app: &mut App) {
    use crate::data::mongodb;
    
    match mongodb::save_game_state(&app.db_client, &app.game_state).await {
        Ok(_) => app.add_message("✅ Game state saved successfully."),
        Err(e) => app.add_message(&format!("⚠️ Failed to save game state: {}", e)),
    }
}

async fn handle_load_ui(app: &mut App) {
    use crate::data::mongodb;
    
    match mongodb::load_game_state(&app.db_client).await {
        Ok(Some(state)) => {
            app.game_state = state;
            app.add_message("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            app.add_message("✅ Game state loaded successfully.");
            app.add_message("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        }
        Ok(None) => {
            app.add_message("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            app.add_message("⚠️ No saved game state found.");
            app.add_message("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        }
        Err(e) => {
            app.add_message("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            app.add_message(&format!("⚠️ Error loading game state: {}", e));
            app.add_message("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        }
    }
}
