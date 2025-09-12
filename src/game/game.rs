use super::action::*;
use crate::{cards::card::Card, game_rewrite::{error::{ActionError, GameError}, score::{RoundScore, VariantPlayerScore}, state::{GamePhase, GameState, VariantState}}};

/// Represents a Rummy game.
pub trait Game {
    /// The `GameRules` that this game follows.
    type Rules: GameRules;

    /// Attempt to execute the `GameAction` for the current player.
    /// 
    /// Returns an `Err` if the action couldn't be executed for some reason.
    fn execute_action(&mut self, action: GameAction) -> Result<(), ActionError>;

    /// Inspect the game's current state.
    fn get_state(&self) -> &GameState<<<Self as Game>::Rules as GameRules>::VariantScore, Self::Rules>;

    /// Quit a player whose ID is `player_id`.
    /// 
    /// Returns an `Err` if such player doesn't exist, or the game already ended.
    fn quit_player(&mut self, player_id: usize) -> Result<(), GameError>;

    /// Add a player using the given `player_id`.
    /// 
    /// Returns an `Err` if the maximum number of players has been reached.
    fn add_player(&mut self, player_id: usize) -> Result<(), GameError>;

    /// Rearranges the player's cards according to what is present in `new_arrangement`.
    /// 
    /// Returns an `Err` if such player doesn't exist, 
    /// `new_arrangement` doesn't contain the exact cards currently in that player's hand,
    /// or the game phase isn't `Draw` or `Play`.
    fn rearrange_player_hand(&mut self, player_id: usize, new_arrangement: Vec<Card>) -> Result<(), GameError>;

    /// Calculate and store round scores and start the next round.
    /// 
    /// Returns an `Err` if the game phase is not `RoundEnd`.
    fn next_round(&mut self) -> Result<(), GameError>;
}

/// Represents the "rule engine" of a Rummy game, handling action execution and score calculation.
pub trait GameRules where Self: Sized {
    /// The state that this variant requires.
    type VariantState: VariantState<Self::VariantScore, Self>;
    /// The score type of this variant (for each player).
    type VariantScore: VariantPlayerScore;

    /// Executes an action, returning an `()` or `ActionError`.
    fn execute_action(&mut self, state: &mut GameState<Self::VariantScore, Self>, action: GameAction) -> Result<(), ActionError> {
        state.validate_action(&action)?;
        match action {
            GameAction::DrawDeck(action) => self.handle_draw_deck(state, action),
            GameAction::DrawDiscardPile(action) => self.handle_draw_discard_pile(state, action),
            GameAction::LayOff(action) => self.handle_lay_off(state, action),
            GameAction::FormMeld(action) => self.handle_form_meld(state, action),
            GameAction::FormMelds(action) => self.handle_form_melds(state, action),
            GameAction::Discard(action) => self.handle_discard(state, action),
        }
    }

    /// Handle drawing from the deck.
    fn handle_draw_deck(&mut self, state: &mut GameState<Self::VariantScore, Self>, action: DrawDeckAction) -> Result<(), ActionError>;

    /// Handle drawing from the discard pile.
    fn handle_draw_discard_pile(&mut self, state: &mut GameState<Self::VariantScore, Self>, action: DrawDiscardPileAction) -> Result<(), ActionError>;

    /// Handle laying off a card from the player's hand.
    fn handle_lay_off(&mut self, state: &mut GameState<Self::VariantScore, Self>, action: LayOffAction) -> Result<(), ActionError>;

    /// Handle forming a single meld.
    fn handle_form_meld(&mut self, state: &mut GameState<Self::VariantScore, Self>, action: FormMeldAction) -> Result<(), ActionError>;

    /// Handle forming multiple melds at one time. Either all melds successfully form, or none do.
    fn handle_form_melds(&mut self, state: &mut GameState<Self::VariantScore, Self>, action: FormMeldsAction) -> Result<(), ActionError>;

    /// Handle discarding a card.
    fn handle_discard(&mut self, state: &mut GameState<Self::VariantScore, Self>, action: DiscardAction) -> Result<(), ActionError>;

    /// Calculate the score for a round. Returns an `Err` if the round hasn't ended.
    fn calculate_round_score(&mut self, state: &GameState<Self::VariantScore, Self>) -> Result<RoundScore<Self::VariantScore>, GameError>;
}

