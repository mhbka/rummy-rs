use crate::game_rewrite::{action::{ActionOutcome, GameAction}, error::{ActionError, GameError}, game::{Game, GameRules}, state::GameState, variants::basic::rules::BasicRules};

/// The basic/standard form of Rummy.
pub struct BasicRummyGame {
    state: GameState<BasicRules>,
    rules: BasicRules
}

impl BasicRummyGame {

}

impl Game for BasicRummyGame {
    type Rules = BasicRules;

    fn execute_action(&mut self, action: GameAction) -> Result<ActionOutcome, ActionError> {
        self.rules.execute_action(&mut self.state, action)
    }

    fn get_state(&self) -> &GameState<Self::Rules> {
        &self.state
    }

    fn quit_player(&mut self, player_id: usize) -> Result<(), GameError> {
        todo!()
    }

    fn add_player(&mut self) -> Result<usize, GameError> {
        todo!()
    }

    fn next_round(&mut self) -> Result<(), GameError> {
        todo!()
    }
}

