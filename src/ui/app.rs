use crate::models::types::GameState;
use std::sync::Arc;
use mongodb::Client;

#[derive(Debug, Clone, PartialEq)]
pub enum InputMode {
    Normal,
    Confirmation,
    Editing,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConfirmationType {
    NewGame,
    Quit,
    AcceptTerms,
    Collect,
}

pub struct App {
    pub game_state: GameState,
    pub db_client: Arc<Client>,
    pub input: String,
    pub message_log: Vec<String>,
    pub input_mode: InputMode,
    pub confirmation_type: Option<ConfirmationType>,
    pub should_quit: bool,
    pub cursor_position: usize,
    pub is_processing: bool,
    pub scroll_offset: usize,
    pub auto_scroll: bool,
}

impl App {
    pub fn new(game_state: GameState, db_client: Arc<Client>) -> Self {
        let mut app = Self {
            game_state,
            db_client,
            input: String::new(),
            message_log: Vec::new(),
            input_mode: InputMode::Normal,
            confirmation_type: None,
            should_quit: false,
            cursor_position: 0,
            is_processing: false,
            scroll_offset: 0,
            auto_scroll: true,
        };
        
        // Messaggio di benvenuto
        app.add_message("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        app.add_message("🚀 Terminal Company OS v0.1.0");
        app.add_message("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        app.add_message("");
        app.add_message(&format!("Welcome, Operator {}!", app.game_state.players[0].name));
        app.add_message(&format!("Current Location: {}", app.game_state.ship.location));
        app.add_message(&format!("Credits: {}", app.game_state.players[0].credits));
        app.add_message("");
        app.add_message("Type HELP for available commands.");
        app.add_message("");
        
        app
    }
    
    pub fn add_message(&mut self, message: &str) {
        self.message_log.push(message.to_string());
        
        if self.message_log.len() > 1000 {
            self.message_log.drain(0..100);
        }
        
        if self.auto_scroll {
            self.scroll_offset = 0;
        }
    }
    
    pub fn add_messages(&mut self, messages: &[String]) {
        for msg in messages {
            self.add_message(msg);
        }
    }
    
    pub fn clear_input(&mut self) {
        self.input.clear();
        self.cursor_position = 0;
    }
    
    pub fn enter_char(&mut self, c: char) {
        self.input.insert(self.cursor_position, c);
        self.cursor_position += 1;
    }
    
    pub fn delete_char(&mut self) {
        if self.cursor_position > 0 {
            self.input.remove(self.cursor_position - 1);
            self.cursor_position -= 1;
        }
    }
    
    pub fn move_cursor_left(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
        }
    }
    
    pub fn move_cursor_right(&mut self) {
        if self.cursor_position < self.input.len() {
            self.cursor_position += 1;
        }
    }
    
    pub fn move_cursor_home(&mut self) {
        self.cursor_position = 0;
    }
    
    pub fn move_cursor_end(&mut self) {
        self.cursor_position = self.input.len();
    }
    
    pub fn get_prompt(&self) -> String {
        match self.input_mode {
            InputMode::Normal => format!("[{}] > ", self.game_state.ship.location),
            InputMode::Confirmation => {
                if let Some(ref conf_type) = self.confirmation_type {
                    match conf_type {
                        ConfirmationType::NewGame => "Start new game? (YES/NO) > ".to_string(),
                        ConfirmationType::Quit => "Quit game? (YES/NO) > ".to_string(),
                        ConfirmationType::AcceptTerms => "Accept terms? (ACCEPT/DENY) > ".to_string(),
                        ConfirmationType::Collect => "Attempt collecting? (YES/NO) > ".to_string(),
                    }
                } else {
                    "(YES/NO) > ".to_string()
                }
            }
            InputMode::Editing => "Enter value > ".to_string(),
        }
    }
    
    pub fn request_confirmation(&mut self, conf_type: ConfirmationType, message: &str) {
        self.input_mode = InputMode::Confirmation;
        self.confirmation_type = Some(conf_type);
        self.add_message(message);
        self.clear_input();
    }
    
    pub fn scroll_up(&mut self, lines: usize) {
        let max_scroll = self.message_log.len().saturating_sub(1);
        self.scroll_offset = (self.scroll_offset + lines).min(max_scroll);
        self.auto_scroll = false;
    }
    
    pub fn scroll_down(&mut self, lines: usize) {
        if self.scroll_offset >= lines {
            self.scroll_offset -= lines;
        } else {
            self.scroll_offset = 0;
            self.auto_scroll = true;
        }
    }
    
    pub fn scroll_to_bottom(&mut self) {
        self.scroll_offset = 0;
        self.auto_scroll = true;
    }
    
    pub fn scroll_to_top(&mut self) {
        self.scroll_offset = self.message_log.len().saturating_sub(1);
        self.auto_scroll = false;
    }
    
    /// Ottiene informazioni sullo stato del gioco per la UI
    pub fn get_status_info(&self) -> Vec<(String, String)> {
        let player = &self.game_state.players[0];
        vec![
            ("Operator".to_string(), player.name.clone()),
            ("Credits".to_string(), format!("💰 {}", player.credits)),
            ("Location".to_string(), format!("📍 {}", self.game_state.ship.location)),
            ("Turn".to_string(), format!("#{}", self.game_state.turn_number)),
            ("HP".to_string(), format!("❤️  {}/100", player.hp)),
        ]
    }
}
