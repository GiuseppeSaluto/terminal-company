use crate::ui::app::App;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

pub fn render(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),      // Header
            Constraint::Min(10),         // Messages area
            Constraint::Length(3),       // Input area
            Constraint::Length(1),       // Footer with help
        ])
        .split(f.area());

    render_header(f, chunks[0], app);
    render_messages(f, chunks[1], app);
    render_input(f, chunks[2], app);
    render_footer(f, chunks[3], app);
}

fn render_header(f: &mut Frame, area: Rect, app: &App) {
    let status_info = app.get_status_info();
    
    let mut spans = Vec::new();
    for (i, (label, value)) in status_info.iter().enumerate() {
        if i > 0 {
            spans.push(Span::raw(" │ "));
        }
        spans.push(Span::styled(
            format!("{}: ", label),
            Style::default().fg(Color::DarkGray),
        ));
        spans.push(Span::styled(
            value,
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        ));
    }
    
    let header = Paragraph::new(Line::from(spans))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Terminal Company ")
                .title_alignment(Alignment::Center)
                .border_style(Style::default().fg(Color::Green))
        )
        .alignment(Alignment::Left);
    
    f.render_widget(header, area);
}

fn render_messages(f: &mut Frame, area: Rect, app: &App) {
    let total_messages = app.message_log.len();
    let available_height = area.height.saturating_sub(2) as usize;
    
    let end_idx = total_messages.saturating_sub(app.scroll_offset);
    let start_idx = end_idx.saturating_sub(available_height);
    
    let messages: Vec<ListItem> = app.message_log[start_idx..end_idx]
        .iter()
        .map(|msg| {
            let style = if msg.contains("⚠️") || msg.contains("Error") || msg.contains("failed") {
                Style::default().fg(Color::Red)
            } else if msg.contains("✨") || msg.contains("Success") {
                Style::default().fg(Color::Green)
            } else if msg.contains("━━━") {
                Style::default().fg(Color::DarkGray)
            } else if msg.starts_with("🚀") || msg.starts_with("Welcome") {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            
            ListItem::new(Line::from(msg.clone())).style(style)
        })
        .collect();
    
    let title = if app.scroll_offset > 0 {
        format!(" Output (↑ {} messages above) ", app.scroll_offset)
    } else if !app.auto_scroll {
        " Output (at bottom) ".to_string()
    } else {
        " Output ".to_string()
    };

    let messages_list = List::new(messages)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title)
                .border_style(Style::default().fg(Color::White))
        );

    f.render_widget(messages_list, area);
}

fn render_input(f: &mut Frame, area: Rect, app: &App) {
    let prompt = app.get_prompt();
    
    let input_text = if app.is_processing {
        format!("{}⏳ Processing...", prompt)
    } else {
        format!("{}{}", prompt, app.input)
    };
    
    let input_style = match app.input_mode {
        crate::ui::app::InputMode::Normal => Style::default().fg(Color::White),
        crate::ui::app::InputMode::Confirmation => Style::default().fg(Color::Yellow),
        crate::ui::app::InputMode::Editing => Style::default().fg(Color::Cyan),
    };
    
    let input = Paragraph::new(input_text)
        .style(input_style)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Command ")
                .border_style(
                    if app.is_processing {
                        Style::default().fg(Color::Yellow)
                    } else {
                        Style::default().fg(Color::Green)
                    }
                )
        );
    
    f.render_widget(input, area);
    
    if !app.is_processing {
        f.set_cursor_position((
            area.x + prompt.len() as u16 + app.cursor_position as u16 + 1,
            area.y + 1,
        ));
    }
}

fn render_footer(f: &mut Frame, area: Rect, app: &App) {
    let help_text = match app.input_mode {
        crate::ui::app::InputMode::Normal => {
            "ESC: quit | ENTER: submit | PgUp/PgDn/Mouse: scroll | Ctrl+C: force quit"
        }
        crate::ui::app::InputMode::Confirmation => {
            "Type YES or NO, then press ENTER"
        }
        crate::ui::app::InputMode::Editing => {
            "Type your input, then press ENTER"
        }
    };
    
    let footer = Paragraph::new(help_text)
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center);
    
    f.render_widget(footer, area);
}
