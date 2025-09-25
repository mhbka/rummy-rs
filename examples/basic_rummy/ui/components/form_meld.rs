use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};
use rummy::{cards::meld::Meldable, game::game::Game};
use crate::app::{App, InputMode};

pub fn render_meld_input(f: &mut Frame, area: Rect, app: &App) {
    let content = vec![
        Line::from(Span::styled("Form Meld Action", Style::default().add_modifier(Modifier::BOLD))),
        Line::from(""),
        Line::from("Enter card indices to add to meld (press D when done):"),
        Line::from(format!("> {}", app.input_buffer)),
        Line::from(""),
        Line::from(format!("Selected cards: {:?}", app.selected_cards)),
    ];

    let paragraph = Paragraph::new(content)
        .block(Block::default().borders(Borders::ALL).title("Form Meld"))
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}
