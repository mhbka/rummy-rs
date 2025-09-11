mod components;

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::{App, AppState};
use components::*;

pub fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),    // Title
            Constraint::Min(10),      // Main content
            Constraint::Length(3),    // Status/Input
            Constraint::Length(3),    // Instructions
        ])
        .split(f.size());

    // Title
    let title = Paragraph::new("🃏 Rummy Game 🃏")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    match app.state {
        AppState::MainMenu => render_main_menu(f, chunks[1]),
        AppState::DrawPhase | AppState::PlayPhase | AppState::GamePlay => {
            render_game_state(f, chunks[1], app)
        }
        AppState::LayOffInput => render_layoff_input(f, chunks[1], app),
        AppState::FormMeldInput => render_meld_input(f, chunks[1], app),
        AppState::DiscardInput => render_discard_input(f, chunks[1], app),
        AppState::RoundEnd => render_round_end(f, chunks[1], app),
        AppState::GameEnd => render_game_end(f, chunks[1], app),
        AppState::Error(ref msg) => render_error(f, chunks[1], msg),
    }

    // Status bar
    render_status_bar(f, chunks[2], app);

    // Instructions
    render_instructions(f, chunks[3], app);
}

fn render_status_bar(f: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    use crate::app::InputMode;
    
    let status = if let Some(ref msg) = app.error_message {
        msg.clone()
    } else {
        match app.input_mode {
            InputMode::Normal => "Ready".to_string(),
            _ => format!("Input: {}", app.input_buffer),
        }
    };
    
    let status_paragraph = Paragraph::new(status)
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL).title("Status"));
    f.render_widget(status_paragraph, area);
}

fn render_instructions(f: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    let instructions = match app.state {
        AppState::MainMenu => "Press Enter to start new game • Q to quit",
        AppState::DrawPhase => "Press 1 for deck, 2 for discard pile • ESC to cancel • Q to quit",
        AppState::PlayPhase => "Press 1 for layoff, 2 for meld, 3 for discard • Q to quit",
        AppState::LayOffInput => "Enter card index, then target player, meld, position • ESC to cancel",
        AppState::FormMeldInput => "Enter card indices, press D when done • ESC to cancel",
        AppState::DiscardInput => "Enter card index to discard • ESC to cancel",
        AppState::RoundEnd => "Press Enter to continue to next round • Q to quit",
        AppState::GameEnd => "Press Enter to return to main menu • Q to quit",
        _ => "ESC to go back • Q to quit",
    };

    let instructions_paragraph = Paragraph::new(instructions)
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title("Instructions"));
    f.render_widget(instructions_paragraph, area);
}