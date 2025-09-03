use crate::{cards::deck::DeckConfig, game_rewrite::{action::GameAction, error::{ActionError, GameError}, game::{Game, GameRules}, state::{GamePhase, GameState}, variants::basic::{rules::BasicRules, score::BasicScore, state::BasicState}}, player::Player};

/// The basic/standard form of Rummy.
pub struct BasicRummyGame {
    state: GameState<BasicScore, BasicRules>,
    rules: BasicRules
}

impl BasicRummyGame {
    /// Initialize the Rummy game.
    pub fn new(player_ids: Vec<usize>, deck_config: DeckConfig) -> Result<Self, GameError> {
        if player_ids.len() < 2 {
            return Err(GameError::TooFewPlayers);
        }
        let state = GameState::initialize(
            player_ids, 
            deck_config, 
            BasicState {}
        );
        let rules = BasicRules {};
        let game = Self {
            state,
            rules
        };
        Ok(game)
    }

    /// Returns the number of cards to deal at the start of a round.
    fn cards_to_deal(&self) -> usize {
        let active_players = self.state.players
            .iter()
            .filter(|p| p.active)
            .count();
        match active_players {
            2 => 10,
            3..=4 => 7,
            5..=6 => 6,
            _ => 10 // NOTE: if >6 players, at least 2 decks are required
        }
    }
    
    /// Returns the player index who should start in a round.
    fn starting_player_index(&self) -> usize {
        let active_players = self.state.players
            .iter()
            .filter(|p| p.active)
            .count();
        self.state.current_round % active_players
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
            None => Err(GameError::QuitPlayerDoesntExist)
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

    fn next_round(&mut self) -> Result<(), GameError> {
        if self.state.phase != GamePhase::RoundEnd {
            return Err(GameError::RoundHasntEnded);
        }
        
        let round_score = self.rules.calculate_round_score(&self.state)?;
        self.state.round_scores.insert(self.state.current_round, round_score);
        self.state.start_new_round(self.cards_to_deal(), self.starting_player_index())?;

        Ok(())
    }
}

