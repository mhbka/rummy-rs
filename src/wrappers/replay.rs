use crate::{cards::card::CardData, game::{action::{GameAction, GameInteractions}, error::{ActionError, GameError}, game::Game, rules::GameRules}, wrappers::history::{History, HistoryEntry}};

/// The state of the replay.
#[derive(Clone, Debug)]
pub struct ReplayState<G: Game> {
    /// The original game.
    game: History<G>,
    /// The game in its "replaying" state.
    replaying_game: G,
    /// The current round of the replay.
    round: usize,
    /// The index of the current action of the replay.
    action: usize,
    /// Whether to skip actions that didn't successfully execute.
    skip_failed_actions: bool
}

impl<G: Game> ReplayState<G> {
    /// Applies the next action(s) and returns it.
    fn next(&mut self) -> Option<&HistoryEntry> {
        // UNWRAP: history and initial round state should always exist for the current round
        let history = self.game
            .get_histories()
            .get(&self.round)
            .unwrap();

        // loop to progress through empty/finished rounds, and (if enabled) failed actions
        loop {
            match history.get(self.action + 1) {
                Some(action) => {
                    self.action += 1;
                    if !action.successful && self.skip_failed_actions {
                        continue;
                    } else {
                        Self::apply_action(&mut self.replaying_game, action);
                        return Some(action);
                    }
                },  
                None => {
                    if self.game
                        .get_histories()
                        .get(&(self.round + 1))
                        .is_none()
                    {
                        return None;
                    } else {
                        self.round += 1;
                        self.action = 0;
                    }
                }
            }
        }
    }

    /// Reverses the previous action(s) and returns it.
    fn previous(&mut self) {
        if self.round == 0 {
            return;
        }

        let round_history;
        if self.action == 0 {
            self.round -= 1;
            round_history = self.game
                .get_histories()
                .get(&self.round)
                .expect("The previous round in a History should always exist");
            self.action = round_history.len() - 1;
            self.replaying_game = self.game
                .get_initial_round_states()
                .get(&self.round)
                .expect("The previous round in a History should always exist")
                .clone();
        }
        else {
            self.action -= 1;
            round_history = self.game
                .get_histories()
                .get(&self.round)
                .expect("The current round in a History should always exist");
            self.replaying_game = self.game
                .get_initial_round_states()
                .get(&self.round)
                .expect("The current round in a History should always exist")
                .clone();
        }

        for i in 0..=self.action {
            Self::apply_action(&mut self.replaying_game, &round_history[i]);
        }
    }

    // Convenience function for applying an action to a game.
    fn apply_action(game: &mut G, action: &HistoryEntry) {
        if action.successful {
            match action.entry.clone() {
                GameInteractions::Action(game_action) => {game.execute_action(game_action);},
                GameInteractions::PlayerJoin { player_id } => {game.add_player(player_id);},
                GameInteractions::PlayerQuit { player_id } => {game.quit_player(player_id);},
                GameInteractions::HandRearrangement { player_id, new_arrangement } => {game.rearrange_player_hand(player_id, new_arrangement);},
            };
        }
    }
}

/// Allows one to replay a game, including undo/redo.
/// 
/// This uses a `History` to reconstruct the game.
/// 
/// ## Note on performance
/// The current replaying state of the game is stored, so applying the next action in the history
/// is simple. 
/// 
/// However, for undoing an action/going back in history, we must reconstruct the game
/// from that round's initial state up to the requested point in history, which is more expensive.
#[derive(Clone, Debug)]
pub struct Replay<G: Game> {
    /// The state of the replay.
    replay_state: ReplayState<G>
}

impl<G: Game> Replay<G> {
    /// Create a replay from a game with history.
    /// 
    /// If you want to skip unsuccessful/failed actions during replay, set `skip_failed_actions` to true.
    pub fn new(game: History<G>, skip_failed_actions: bool) -> Self {
        let replaying_game = game
            .get_initial_round_states()
            .get(&0)
            .expect("`History` should always contain an (empty) initial round state + history for round 0")
            .clone();
        let replay_state = ReplayState {
            game,
            replaying_game,
            round: 0,
            action: 0,
            skip_failed_actions
        };
        Self {
            replay_state
        }
    }

    /// Get a reference to the current game.
    pub fn get_game(&self) -> &G {
        self.replay_state.game.get_game()
    }

    /// Get a reference to the replaying game.
    pub fn get_replaying_game(&self) -> &G {
        &self.replay_state.replaying_game
    }

    /// Applies the next action of the history, and returns that action.
    /// 
    /// If there aren't any actions left for the current round, goes to the next round 
    /// and applies the first action there.
    /// 
    /// If `skip_failed_actions`, skips any failed actions till a successful action is found.
    /// 
    /// If there are no rounds left, returns `None`.
    fn next(&mut self) -> Option<&HistoryEntry> {
        self.replay_state.next()
    }

    /// The exact opposite effect of `next`. Reverses the previous action and returns that action.
    /// 
    /// If `skip_failed_actions`, reverses all the way till the previous successful action.
    fn previous(&mut self) {
        self.replay_state.previous()
    }
}

impl<G: Game> Game for Replay<G> {
    type Rules = G::Rules;

    fn execute_action(&mut self, action: GameAction) -> Result<(), ActionError> {
        self.replay_state.game.execute_action(action)
    }

    fn get_state(&self) -> &crate::game::state::GameState<<<Self as Game>::Rules as GameRules>::VariantScore, Self::Rules> {
        self.replay_state.game.get_state()
    }

    fn quit_player(&mut self, player_id: usize) -> Result<(), GameError> {
        self.replay_state.game.quit_player(player_id)
    }

    fn add_player(&mut self, player_id: usize) -> Result<(), GameError> {
        self.replay_state.game.add_player(player_id)
    }

    fn rearrange_player_hand(&mut self, player_id: usize, new_arrangement: Vec<CardData>) -> Result<(), GameError> {
        self.replay_state.game.rearrange_player_hand(player_id, new_arrangement)
    }

    fn next_round(&mut self) -> Result<(), GameError> {
        self.replay_state.game.next_round()
    }
}