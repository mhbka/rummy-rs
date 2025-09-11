use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};
use rummy::{cards::meld::Meldable, game_rewrite::game::Game};

use crate::app::{App, InputMode};

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

pub fn render_game_state(f: &mut Frame, area: Rect, app: &App) {
    if let Some(ref game) = app.game {
        let gamestate = game.get_state();
        
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        // Left side - Current player info
        let current_player = gamestate.get_current_player().unwrap();
        let hand_text = current_player.cards()
            .iter()
            .enumerate()
            .map(|(i, card)| card.to_string())
            .collect::<Vec<_>>()
            .join(", ");

        let left_content = vec![
            Line::from(Span::styled(format!("Round: {}", gamestate.current_round), Style::default().fg(Color::Cyan))),
            Line::from(Span::styled(format!("Current Player: {}", gamestate.current_player), Style::default().fg(Color::Yellow))),
            Line::from(""),
            Line::from(Span::styled("Your Hand:", Style::default().add_modifier(Modifier::BOLD))),
            Line::from(hand_text),
            Line::from(""),
            Line::from(format!("Deck size: {}", gamestate.deck.stock().len())),
            Line::from(format!("Discard pile size: {}", gamestate.deck.discard_pile().len())),
            Line::from(format!("Top discard: {:?}", gamestate.deck.peek_discard_pile())),
        ];

        let left_paragraph = Paragraph::new(left_content)
            .block(Block::default().borders(Borders::ALL).title("Game State"))
            .wrap(Wrap { trim: true });
        f.render_widget(left_paragraph, chunks[0]);

        // Right side - All players info
        let mut players_text = Vec::new();
        for player in &gamestate.players {
            if player.active() {
                players_text.push(Line::from(format!("Player {}: {} cards", player.id(), player.cards().len())));
                for (i, meld) in player.melds().iter().enumerate() {
                    players_text.push(Line::from(format!("  Meld {}: {:?}", i, meld.cards())));
                }
                players_text.push(Line::from(""));
            }
        }

        let right_paragraph = Paragraph::new(players_text)
            .block(Block::default().borders(Borders::ALL).title("All Players"))
            .wrap(Wrap { trim: true });
        f.render_widget(right_paragraph, chunks[1]);
    }
}

pub fn render_layoff_input(f: &mut Frame, area: Rect, app: &App) {
    let prompt = match app.input_mode {
        InputMode::LayOffCardIndex => "Enter card index to lay off:",
        InputMode::LayOffTargetPlayer => "Enter target player index:",
        InputMode::LayOffTargetMeld => "Enter target meld index:",
        InputMode::LayOffPosition => "Enter position in meld:",
        _ => "Lay off input:",
    };

    let content = vec![
        Line::from(Span::styled("Lay Off Action", Style::default().add_modifier(Modifier::BOLD))),
        Line::from(""),
        Line::from(prompt),
        Line::from(format!("> {}", app.input_buffer)),
    ];

    let paragraph = Paragraph::new(content)
        .block(Block::default().borders(Borders::ALL).title("Lay Off"))
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}

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

pub fn render_round_end(f: &mut Frame, area: Rect, app: &App) {
    if let Some(ref game) = app.game {
        let gamestate = game.get_state();
        let latest_score = gamestate.round_scores
            .get(&gamestate.current_round);

        let content = vec![
            Line::from(Span::styled("Round Ended!", Style::default().add_modifier(Modifier::BOLD).fg(Color::Green))),
            Line::from(""),
            Line::from(format!("Round {} complete", gamestate.current_round)),
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

pub fn render_error(f: &mut Frame, area: Rect, error_msg: &str) {
    let content = vec![
        Line::from(Span::styled("Error!", Style::default().add_modifier(Modifier::BOLD).fg(Color::Red))),
        Line::from(""),
        Line::from(error_msg),
    ];

    let paragraph = Paragraph::new(content)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title("Error"));
    f.render_widget(paragraph, area);
}