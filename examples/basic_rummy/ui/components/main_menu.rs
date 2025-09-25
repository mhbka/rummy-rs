use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use rummy::cards::meld::Meldable;

pub fn render_main_menu(f: &mut Frame, area: Rect) {
    let welcome_text = vec![
        Line::from(""),
        Line::from(Span::styled("Welcome to Rummy!", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))),
        Line::from(""),
        Line::from("A classic card game for 2-4 players."),
        Line::from(""),
        Line::from("Press Enter to start a new game!"),
    ];

    let paragraph = Paragraph::new(welcome_text)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title("Main Menu"));
    f.render_widget(paragraph, area);
}