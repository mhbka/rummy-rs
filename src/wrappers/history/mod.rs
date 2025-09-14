use std::collections::HashMap;
use crate::{cards::{card::{Card, CardData}, deck::DeckConfig}, game::{action::GameAction, error::{ActionError, GameError, GameSetupError}, game::Game, rules::GameRules, state::GameState, variants::basic::{config::BasicConfig, game::BasicRummyGame}}};
use chrono::{DateTime, Utc};

/// The type of history this is.
#[derive(Clone, Debug)]
pub enum HistoryEntryData {
    Action(GameAction),
    PlayerJoin { player_id: usize },
    PlayerQuit { player_id: usize },
    HandRearrangement { player_id: usize, hand: Vec<CardData> },
}

/// An entry in the game's history.
#[derive(Clone, Debug)]
pub struct HistoryEntry {
    pub entry: HistoryEntryData,
    pub time: DateTime<Utc>,
    pub successful: bool
}

/// This wrapper tracks every interaction with the game,
/// as well as the initial game state at the start of each round.
/// 
/// This means one can construct the state of the game at each step.
#[derive(Clone, Debug)]
pub struct History<G: Game + Clone> {
    /// The current game.
    current_game: G,
    /// The map of round numbers to initial round states.
    initial_round_states: HashMap<usize, G>,
    /// The map of round numbers to its histories.
    history: HashMap<usize, Vec<HistoryEntry>>
}

impl<G: Game + Clone> History<G> {
    /// Get the initial round states.
    pub fn initial_round_states(&self) -> &HashMap<usize, G> {
        &self.initial_round_states
    }

    /// Get the histories.
    pub fn get_histories(&self) -> &HashMap<usize, Vec<HistoryEntry>> {
        &self.history
    }

    /// Get a mutable ref to the current round's history.
    /// 
    /// ### Panics
    /// If the current round's history doesn't exist. 
    /// This could only happen if you called this after `current_game.next_round()` but before creating the fresh history for that round.
    /// 
    /// Beware of that!
    fn get_current_round_history(&mut self) -> &mut Vec<HistoryEntry> {
        // UNWRAP: This is fine as long as consistent internal state is kept.
        // This mostly means a `History` should only be created with a new `G`, never one in progress.
        let round = self.current_game.get_state().current_round;
        self.history.get_mut(&round).expect("There should always be a round history")
    }
}

impl History<BasicRummyGame> {
    /// Create a basic Rummy game with history.
    pub fn new(player_ids: Vec<usize>, config: BasicConfig, deck_config: DeckConfig) -> Result<Self, GameSetupError> {
        let game = BasicRummyGame::new(player_ids, config, deck_config)?;
        Ok(
            Self {
                current_game: game,
                initial_round_states: HashMap::new(),
                history: HashMap::new()
            }
        )
    }
}

impl<G: Game + Clone> Game for History<G> {
    type Rules = G::Rules;

    fn execute_action(&mut self, action: GameAction) -> Result<(), ActionError> {
        let result = self.current_game.execute_action(action.clone());
        let entry = HistoryEntry {
            entry: HistoryEntryData::Action(action),
            time: Utc::now(),
            successful: result.is_ok()
        };
        self.get_current_round_history().push(entry);
        result
    }

    fn get_state(&self) -> &GameState<<<Self as Game>::Rules as GameRules>::VariantScore, Self::Rules> {
        self.current_game.get_state()
    }

    fn quit_player(&mut self, player_id: usize) -> Result<(), GameError> {
        let result = self.current_game.quit_player(player_id);
        let entry = HistoryEntry {
            entry: HistoryEntryData::PlayerQuit { player_id },
            time: Utc::now(),
            successful: result.is_ok()
        };
        self.get_current_round_history().push(entry);
        result
    }

    fn add_player(&mut self, player_id: usize) -> Result<(), GameError> {
        let result = self.current_game.add_player(player_id);
        let entry = HistoryEntry {
            entry: HistoryEntryData::PlayerJoin { player_id },
            time: Utc::now(),
            successful: result.is_ok()
        };
        self.get_current_round_history().push(entry);
        result
    }

    fn rearrange_player_hand(&mut self, player_id: usize, new_arrangement: Vec<Card>) -> Result<(), GameError> {
        let hand = new_arrangement.iter().map(|c| c.data()).collect();
        let result = self.current_game.rearrange_player_hand(player_id, new_arrangement.clone());
        let entry = HistoryEntry {
            entry: HistoryEntryData::HandRearrangement { player_id, hand },
            time: Utc::now(),
            successful: result.is_ok()
        };
        self.get_current_round_history().push(entry);
        result
    }

    fn next_round(&mut self) -> Result<(), GameError> {
        let result = self.current_game.next_round();

        // note: we don't store failed `next_round()` calls because it doesn't really matter to anyone
        if result.is_ok() {
            let new_history = Vec::new();
            let round = self.current_game.get_state().current_round;
            self.history.insert(round, new_history);
            self.initial_round_states.insert(round, self.current_game.clone());
        }   

        result
    }
}