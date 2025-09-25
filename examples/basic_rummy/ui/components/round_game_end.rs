use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use rummy::{cards::meld::Meldable, game::game::Game};
use crate::app::App;

pub fn render_round_end(f: &mut Frame, area: Rect, app: &App) {
    if let Some(ref game) = app.game {
        let gamestate = game.get_state();
        let latest_score = gamestate.round_scores()
            .get(&gamestate.current_round());

        let content = vec![
            Line::from(Span::styled("Round Ended!", Style::default().add_modifier(Modifier::BOLD).fg(Color::Green))),
            Line::from(""),
            Line::from(format!("Round {} complete", gamestate.current_round())),
            Line::from(format!("Scores: {:?}", latest_score)),
            Line::from(""),
            Line::from("Press Enter to continue to next round..."),
        ];

        let paragraph = Paragraph::new(content)
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).title("Round Complete"));
        f.render_widget(paragraph, area);
    }
}

pub fn render_game_end(f: &mut Frame, area: Rect, app: &App) {
    let content = vec![
        Line::from(Span::styled("Game Over!", Style::default().add_modifier(Modifier::BOLD).fg(Color::Red))),
        Line::from(""),
        Line::from("Thanks for playing!"),
        Line::from(""),
        Line::from("Press Enter to return to main menu..."),
    ];

    let paragraph = Paragraph::new(content)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title("Game Over"));
    f.render_widget(paragraph, area);
}