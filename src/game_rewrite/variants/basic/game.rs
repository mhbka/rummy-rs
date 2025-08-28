use crate::{cards::deck::DeckConfig, game_rewrite::{action::{ActionOutcome, GameAction}, error::{ActionError, GameError}, game::{Game, GameRules}, state::{GamePhase, GameState}, variants::basic::{rules::BasicRules, score::BasicScore, state::BasicState}}, player::Player};

/// The basic/standard form of Rummy.
pub struct BasicRummyGame {
    state: GameState<BasicScore, BasicRules>,
    rules: BasicRules
}

impl BasicRummyGame {
    /// Initialize the Rummy game.
    pub fn new(player_ids: Vec<usize>, deck_config: DeckConfig) -> Self {
        let state = GameState::initialize(
            player_ids, 
            deck_config, 
            BasicState {}
        );
        let rules = BasicRules {};
        Self {
            state,
            rules
        }
    }
}

impl Game for BasicRummyGame {
    type Rules = BasicRules;

    fn execute_action(&mut self, action: GameAction) -> Result<ActionOutcome, ActionError> {
        self.rules.execute_action(&mut self.state, action)
    }

    fn get_state(&self) -> &GameState<BasicScore, BasicRules> {
        &self.state
    }

    fn quit_player(&mut self, player_id: usize) -> Result<(), GameError> {
        return match self.state.players
            .iter_mut()
            .find(|p| p.id == player_id)
        { 
            Some(player) => {
                player.active = false;
                Ok(())
            },
            None => Err(GameError::QuitPlayerDoesntExist)
        }
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

    fn next_round(&mut self) -> Result<(), GameError> {
        if self.state.phase != GamePhase::RoundEnd {
            return Err(GameError::RoundHasntEnded);
        }

        let round_score = self.rules.calculate_round_score(&self.state)?;
        self.state.round_scores.insert(self.state.current_round, round_score);
        self.state.current_round += 1;
        self.state.current_player = self.rules.starting_player_index(&self.state);

        Ok(())
    }
}

