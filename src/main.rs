use crate::commands::registration;
use crate::data::mongodb;
use crate::ui::app::App;
use crate::ui::components;
use crate::ui::event::{Event, EventHandler};
use crate::ui::inputs;
use crossterm::{
    execute,
    event::{EnableMouseCapture, DisableMouseCapture},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use dotenv::dotenv;
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use std::sync::Arc;
use std::time::Duration;

mod commands {
    pub mod commands_fn;
    pub mod registration;
}
mod data {
    pub mod mongodb;
}
mod models {
    pub mod types;
    pub mod lists;
    pub mod collect_credits;
    pub mod scan_logic;
}
mod utils {
    pub mod shortcut;
}
mod ui {
    pub mod app;
    pub mod components;
    pub mod event;
    pub mod inputs;
    pub mod output_capture;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    env_logger::init();

    let client = Arc::new(mongodb::init_db().await?);
    let game_state = registration::initialize_game(client.clone()).await?;
    
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(game_state, client);
    let event_handler = EventHandler::new(Duration::from_millis(250));

    let result = run_app(&mut terminal, &mut app, &event_handler).await;

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), DisableMouseCapture, LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = result {
        eprintln!("Error: {:?}", err);
    }

    Ok(())
}

async fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    event_handler: &EventHandler,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| components::render(f, app))?;

        match event_handler.next()? {
            Event::Key(key) => {
                inputs::handle_key_event(app, key).await;
            }
            Event::Resize(_, _) => {}
            Event::Tick => {}
            Event::Mouse(mouse) => {
                inputs::handle_mouse_event(app, mouse);
            }
        }

        if app.should_quit {
            break;
        }
    }

    Ok(())
}