use crate::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};
use rummy::{cards::meld::Meldable, game::r#trait::Game};

pub fn render_game_state(f: &mut Frame, area: Rect, app: &App) {
    if let Some(ref game) = app.game {
        let gamestate = game.get_state();

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        // Left side - Current player info
        let current_player = gamestate.get_current_player().unwrap();
        let hand_text = current_player
            .cards()
            .iter()
            .enumerate()
            .map(|(i, card)| card.to_string())
            .collect::<Vec<_>>()
            .join(", ");

        let left_content = vec![
            Line::from(Span::styled(
                format!("Round: {}", gamestate.current_round()),
                Style::default().fg(Color::Cyan),
            )),
            Line::from(Span::styled(
                format!(
                    "Current player ID: {}",
                    gamestate.get_current_player().unwrap().id()
                ),
                Style::default().fg(Color::Yellow),
            )),
            Line::from(""),
            Line::from(Span::styled(
                format!(
                    "Your hand ({} cards):",
                    gamestate.get_current_player().unwrap().cards().len()
                ),
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Line::from(hand_text),
            Line::from(""),
            Line::from(format!("Deck size: {}", gamestate.deck().stock().len())),
            Line::from(format!(
                "Discard pile size: {}",
                gamestate.deck().discard_pile().len()
            )),
            Line::from(format!(
                "Top discard: {:?}",
                gamestate.deck().peek_discard_pile()
            )),
        ];

        let left_paragraph = Paragraph::new(left_content)
            .block(Block::default().borders(Borders::ALL).title("Game State"))
            .wrap(Wrap { trim: true });
        f.render_widget(left_paragraph, chunks[0]);

        // Right side - All players info
        let mut players_text = Vec::new();
        for player in gamestate.players() {
            if player.active() {
                players_text.push(Line::from(format!(
                    "Player {}: {} cards",
                    player.id(),
                    player.cards().len()
                )));
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
