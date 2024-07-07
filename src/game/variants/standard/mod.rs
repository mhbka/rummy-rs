pub mod actions;
pub mod state;

use super::super::phases::*;
use crate::{
    cards::{
        deck::{Deck, DeckConfig},
        suit_rank::Rank,
    },
    player::Player,
};
use state::*;

/// Get the number of cards to deal each player at the start of a round,
/// given number of players and number of decks.
///
/// Follows the ruling [here](https://en.wikipedia.org/wiki/Rummy).
const fn get_cards_to_deal(num_players: usize, num_decks: usize) -> usize {
    match (num_players, num_decks) {
        (2, 1) => 10,
        (3, 1) => 7,
        (3, 2) => 10,
        (4..=5, 1) => 7,
        (4..=7, 2) => 10,
        (6, _) => 6,
        (7, _) => 6,
        _ => panic!("Invalid number of players or decks"),
    }
}

/// Entrypoint for starting a standard Rummy game.
pub struct StandardRummyBuilder();

impl StandardRummyBuilder {
    /// Start a new Rummy game with a list of `player_ids`, a game config, and a deck config.
    ///
    /// If there are >7 players, the excess will be truncated.
    pub fn new(
        mut player_ids: Vec<usize>,
        game_config: StandardRummyConfig,
        deck_config: DeckConfig,
    ) -> StandardRummy<RoundEndPhase> {
        player_ids.truncate(7);

        let players = player_ids
            .iter()
            .map(|&id| Player::new(id, true, 0))
            .collect();

        let state = StandardRummyState {
            config: game_config,
            score: StandardRummyScore::new(),
            deck: Deck::new(deck_config),
            players,
            cur_round: 0,
            cur_player: 0,
        };

        StandardRummy {
            phase: RoundEndPhase {
                has_scored_round: false,
            },
            state,
        }
    }

    /// Starts the game with default settings, only requiring a list of `player_ids`.
    ///
    /// If there are >7 players, the excess will be truncated.
    ///
    /// If you want to configure your game, use `new` instead.
    pub fn quickstart(player_ids: Vec<usize>) -> StandardRummy<RoundEndPhase> {
        let deck_config = DeckConfig {
            shuffle_seed: None,
            pack_count: if player_ids.len() < 5 { 1 } else { 2 },
            high_rank: None,
            wildcard_rank: Some(Rank::Joker),
        };

        StandardRummyBuilder::new(player_ids, StandardRummyConfig::new(), deck_config)
    }
}

/// A basic game of Rummy, following the rules/behaviour described [here](https://en.wikipedia.org/wiki/Rummy).
pub struct StandardRummy<P: GamePhase> {
    phase: P,
    state: StandardRummyState,
}

impl<P: GamePhase> StandardRummy<P> {
    /// Returns a mutable reference to the current player.
    fn cur_player(&mut self) -> &mut Player {
        &mut self.state.players[self.state.cur_player]
    }

    /// Returns a reference to the config.
    fn config(&self) -> &StandardRummyConfig {
        &self.state.config
    }
}