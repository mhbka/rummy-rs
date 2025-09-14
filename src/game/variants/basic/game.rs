use std::collections::HashMap;
use crate::{cards::{card::Card, deck::DeckConfig, suit_rank::Rank}, game::{action::GameAction, error::{ActionError, GameError, GameSetupError}, game::Game, rules::GameRules, state::{GamePhase, GameState}, variants::basic::{config::{BasicConfig, DrawDiscardPileOverride}, rules::BasicRules, score::BasicScore, state::BasicState}}, player::Player};

/// The basic/standard form of Rummy.
#[derive(Clone, Debug)]
pub struct BasicRummyGame {
    state: GameState<BasicScore, BasicRules>,
    rules: BasicRules,
    config: BasicConfig
}

impl BasicRummyGame {
    /// Initialize the Rummy game.
    /// 
    /// Returns an `Err` if there are too few/too many players.
    pub fn new(
        player_ids: Vec<usize>, 
        config: BasicConfig, 
        deck_config: DeckConfig
    ) -> Result<Self, GameSetupError> {
        let state = GameState::initialize(
            player_ids, 
            deck_config, 
            BasicState {}
        );
        let rules = BasicRules::new(config.clone());
        let game = Self {
            state,
            rules,
            config
        };

        game.validate_setup()?;

        Ok(game)
    }

    /// Validates setup of the game.
    /// We can call this when first initializing the game, and before every new round.
    /// 
    /// At the moment, just checks that we can deal cards to all players and have enough left in the stock for 1 iteration of draws.
    fn validate_setup(&self) -> Result<(), GameSetupError> {
        let deal_amount = self.rules.cards_to_deal(&self.state);
        let draw_amount = self.rules.cards_to_draw_from_deck(&self.state);
        let active_players = self.state.players
            .iter()
            .filter(|p| p.active)
            .count();

        let deck_config = self.state.deck.config();
        let mut deck_size = deck_config.pack_count * 52;
        if let Some(Rank::Joker) = deck_config.wildcard_rank {
            deck_size += deck_config.pack_count * 2;
        }

        let min_draw_size = (active_players * deal_amount) + (active_players * draw_amount);

        match deck_size < min_draw_size {
            true => Err(GameSetupError::NotEnoughCards),
            false => Ok(())
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
        match self.state.players
            .iter_mut()
            .find(|p| p.id == player_id)
        { 
            Some(player) => {
                player.active = false;
                Ok(())
            },
            None => Err(GameError::PlayerDoesntExist)
        }?;

        // End the game if only 1 active player is remaining
        let num_active_players = self.state.players
            .iter()
            .fold(0, |acc, p| acc + p.active as usize);
        if num_active_players < 2 {
            self.state.phase = GamePhase::GameEnd;
        }

        Ok(())
    }

    fn add_player(&mut self, player_id: usize) -> Result<(), GameError> {
        return match self.state.players
            .iter()
            .find(|p| p.id == player_id)
        { 
            Some(_) => Err(GameError::AddedPlayerAlreadyExists),
            None => {
                let new_player = Player {
                    id: player_id,
                    cards: Vec::new(),
                    melds: Vec::new(),
                    active: false,
                    joined_in_round: self.state.current_round
                };
                self.state.players.push(new_player);
                Ok(())
            }
        }
    }

    fn rearrange_player_hand(&mut self, player_id: usize, new_arrangement: Vec<Card>) -> Result<(), GameError> {
        if self.state.phase != GamePhase::Draw && self.state.phase != GamePhase::Play {
            return Err(GameError::WrongGamePhase);
        }
        
        match self.state.players
            .iter_mut()
            .find(|p| p.id == player_id)
        {
            Some(player) => {
                // check that player's hand and `new_arrangement` contain same cards
                let mut count = HashMap::new();
                for card in &player.cards {
                    *count.entry(card).or_insert(0) += 1;
                }
                for card in &new_arrangement {
                    let entry = count.entry(card).or_insert(0);
                    *entry -= 1;
                    if *entry == 0 {
                        count.remove(card);
                    }
                }
                if count.is_empty() {
                    player.cards = new_arrangement;
                    Ok(())
                }
                else {
                    Err(GameError::FailedHandRearrangement)
                }
            },
            None => return Err(GameError::PlayerDoesntExist)
        }
    }

    fn next_round(&mut self) -> Result<(), GameError> {
        if self.state.phase != GamePhase::RoundEnd {
            return Err(GameError::WrongGamePhase);
        }

        if self.state.current_round != 0 {  
            let round_score = self.rules.calculate_round_score(&self.state)?;
            self.state.round_scores.insert(self.state.current_round, round_score);
        }
        
        let cards_to_deal = self.rules.cards_to_deal(&self.state);
        let starting_player_index = self.rules.starting_player_index(&self.state);
        self.state.start_new_round(cards_to_deal, starting_player_index)?;

        Ok(())
    }
}

