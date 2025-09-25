use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};
use rummy::cards::meld::Meldable;
use crate::app::App;

pub fn render_discard_input(f: &mut Frame, area: Rect, app: &App) {
    let content = vec![
        Line::from(Span::styled("Discard Action", Style::default().add_modifier(Modifier::BOLD))),
        Line::from(""),
        Line::from("Enter card index to discard:"),
        Line::from(format!("> {}", app.input_buffer)),
    ];

    let paragraph = Paragraph::new(content)
        .block(Block::default().borders(Borders::ALL).title("Discard"))
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}
