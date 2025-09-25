use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};
use rummy::{cards::meld::Meldable, game::game::Game};
use crate::app::{App, InputMode};

pub fn render_layoff_input(f: &mut Frame, area: Rect, app: &App) {
    let prompt = match app.input_mode {
        InputMode::LayOffCardIndex => "Enter card index to lay off:",
        InputMode::LayOffTargetPlayer => "Enter target player index:",
        InputMode::LayOffTargetMeld => "Enter target meld index:",
        _ => "Lay off input:",
    };
    
    let game = match &app.game {
        Some(game) => game,
        None => return
    };
    let gamestate = game.get_state();

    let mut layoff_progress = vec![];
    if let Some(i) = app.layoff_data.card_index {
        let card = &gamestate
            .get_current_player()
            .unwrap()
            .cards()[i];
        layoff_progress.push(
            Line::from(format!("Card: {card}"))
        );
    }
    if let Some(i) = app.layoff_data.target_player_index {
        let player = &gamestate.players()[i];
        layoff_progress.push(
            Line::from(format!("Player ID: {} (melds: {})", player.id(), player.melds().len()))
        );
    }
    if let Some(i) = app.layoff_data.target_meld_index {
        let meld = &gamestate.players()[app.layoff_data.target_player_index.unwrap()].melds()[i];
        layoff_progress.push(
            Line::from(format!("Meld: {meld:?}"))
        );
    }

    let input = vec![
        Line::from(""),
        Line::from(prompt),
        Line::from(format!("> {}", app.input_buffer)),
    ];

    let content: Vec<_> = layoff_progress
        .into_iter()
        .chain(input.into_iter())
        .collect();

    let paragraph = Paragraph::new(content)
        .block(Block::default().borders(Borders::ALL).title("Lay Off"))
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}