use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use rummy::cards::meld::Meldable;

pub fn render_error(f: &mut Frame, area: Rect, error_msg: &str) {
    let content = vec![
        Line::from(Span::styled(
            "Error!",
            Style::default().add_modifier(Modifier::BOLD).fg(Color::Red),
        )),
        Line::from(""),
        Line::from(error_msg),
    ];

    let paragraph = Paragraph::new(content)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title("Error"));
    f.render_widget(paragraph, area);
}
