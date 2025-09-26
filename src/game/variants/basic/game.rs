//! Contains the [`BasicRummyGame`], an implementation of basic Rummy.

use crate::{
    cards::{
        card::{Card, CardData},
        deck::DeckConfig,
        suit_rank::Rank,
    },
    game::{
        action::GameAction,
        error::{ActionError, GameError, GameSetupError},
        r#trait::Game,
        rules::GameRules,
        state::{GamePhase, GameState},
        variants::basic::{
            config::BasicConfig, rules::BasicRules, score::BasicScore, state::BasicState,
        },
    },
    player::Player,
};
use std::collections::HashMap;

/// The basic/standard form of Rummy.
#[derive(Clone, Debug, PartialEq)]
pub struct BasicRummyGame {
    pub(crate) state: GameState<BasicScore, BasicRules>,
    pub(crate) rules: BasicRules,
}

impl BasicRummyGame {
    /// Initialize the Rummy game.
    ///
    /// Returns an `Err` if there is only 1 player,
    /// or there aren't enough cards for all players to be dealt + draw from the deck at least once.
    pub fn new(
        player_ids: Vec<usize>,
        config: BasicConfig,
        deck_config: DeckConfig,
    ) -> Result<Self, GameSetupError> {
        let state = GameState::initialize(player_ids, deck_config, BasicState {});
        let rules = BasicRules::new(config.clone());
        let game = Self { state, rules };

        game.validate_setup()?;

        Ok(game)
    }

    /// Validates setup of the game.
    /// We call this when first initializing the game, and before starting every new round.
    ///
    /// At the moment, this just checks that we can deal cards to all (active) players and have enough left in the stock for 1 iteration of draws.
    fn validate_setup(&self) -> Result<(), GameSetupError> {
        // active players are those who are active, or just joined the game
        let active_players = self
            .state
            .players
            .iter()
            .filter(|p| p.active || p.joined_in_round == self.state.current_round)
            .count();
        if active_players < 2 {
            return Err(GameSetupError::TooFewPlayers);
        }

        let deal_amount = self.rules.cards_to_deal(&self.state);
        let draw_amount = self.rules.cards_to_draw_from_deck(&self.state);

        let deck_config = self.state.deck.config();
        let mut deck_size = deck_config.pack_count * 52;
        if let Some(Rank::Joker) = deck_config.wildcard_rank {
            deck_size += deck_config.pack_count * 2;
        }

        let min_draw_size = (active_players * deal_amount) + (active_players * draw_amount);

        match deck_size < min_draw_size {
            true => Err(GameSetupError::NotEnoughCards),
            false => Ok(()),
        }
    }
}

impl Game for BasicRummyGame {
    type Rules = BasicRules;

    fn execute_action(&mut self, action: GameAction) -> Result<(), ActionError> {
        self.rules.execute_action(&mut self.state, action)
    }

    fn get_state(&self) -> &GameState<BasicScore, BasicRules> {
        &self.state
    }

    fn quit_player(&mut self, player_id: usize) -> Result<(), GameError> {
        match self.state.players.iter_mut().find(|p| p.id == player_id) {
            Some(player) => {
                player.active = false;
                Ok(())
            }
            None => Err(GameError::PlayerDoesntExist),
        }?;

        // End the game if only 1 active player is remaining
        let num_active_players = self
            .state
            .players
            .iter()
            .fold(0, |acc, p| acc + p.active as usize);
        if num_active_players < 2 {
            self.state.phase = GamePhase::GameEnd;
        }

        Ok(())
    }

    fn add_player(&mut self, player_id: usize) -> Result<(), GameError> {
        match self.state.players.iter().find(|p| p.id == player_id) {
            Some(_) => Err(GameError::AddedPlayerAlreadyExists),
            None => {
                let new_player = Player {
                    id: player_id,
                    cards: Vec::new(),
                    melds: Vec::new(),
                    active: false,
                    joined_in_round: self.state.current_round,
                };
                self.state.players.push(new_player);
                Ok(())
            }
        }
    }

    fn rearrange_player_hand(
        &mut self,
        player_id: usize,
        new_arrangement: Vec<CardData>,
    ) -> Result<(), GameError> {
        if self.state.phase != GamePhase::Draw && self.state.phase != GamePhase::Play {
            return Err(GameError::WrongGamePhase);
        }

        match self.state.players.iter_mut().find(|p| p.id == player_id) {
            Some(player) => {
                // check that player has cards in hand
                if player.cards.is_empty() {
                    return Err(GameError::FailedHandRearrangement);
                }
                let deck_config = player.cards[0].deck_config();

                // check that player's hand and `new_arrangement` contain same cards
                let mut count = HashMap::new();
                for card in &player.cards {
                    *count.entry(card.data()).or_insert(0) += 1;
                }
                for card in &new_arrangement {
                    let entry = count.entry(*card).or_insert(0);
                    *entry -= 1;
                    if *entry == 0 {
                        count.remove(card);
                    }
                }
                if count.is_empty() {
                    player.cards = new_arrangement
                        .into_iter()
                        .map(|c| Card {
                            rank: c.rank,
                            suit: c.suit,
                            deck_config: deck_config.clone(),
                        })
                        .collect();
                    Ok(())
                } else {
                    Err(GameError::FailedHandRearrangement)
                }
            }
            None => Err(GameError::PlayerDoesntExist),
        }
    }

    fn next_round(&mut self) -> Result<(), GameError> {
        if self.state.phase != GamePhase::RoundEnd {
            return Err(GameError::WrongGamePhase);
        }

        self.validate_setup()?;

        if self.state.current_round != 0 {
            let round_score = self.rules.calculate_round_score(&self.state)?;
            self.state
                .round_scores
                .insert(self.state.current_round, round_score);
        }

        let cards_to_deal = self.rules.cards_to_deal(&self.state);
        let starting_player_index = self.rules.starting_player_index(&self.state);
        self.state
            .start_new_round(cards_to_deal, starting_player_index)?;

        Ok(())
    }
}
