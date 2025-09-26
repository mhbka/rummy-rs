//! Contains the `GameRules` trait, which encapsulates mostly action-evaluating behaviour of a game.
//!
//! This is used by the variant's `Game` struct.

use super::action::*;
use crate::game::{
    error::{ActionError, GameError},
    score::{RoundScore, VariantPlayerScore},
    state::{GameState, VariantState},
};

/// Represents the "rule engine" of a Rummy game, handling action execution and score calculation.
pub trait GameRules
where
    Self: Sized + PartialEq,
{
    /// The state that this variant requires.
    type VariantState: VariantState<Self::VariantScore, Self>;
    /// The score type of this variant (for each player).
    type VariantScore: VariantPlayerScore;

    /// Executes an action, returning an `()` or `ActionError`.
    fn execute_action(
        &self,
        state: &mut GameState<Self::VariantScore, Self>,
        action: GameAction,
    ) -> Result<(), ActionError> {
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
    fn handle_draw_deck(
        &self,
        state: &mut GameState<Self::VariantScore, Self>,
        action: DrawDeckAction,
    ) -> Result<(), ActionError>;

    /// Handle drawing from the discard pile.
    fn handle_draw_discard_pile(
        &self,
        state: &mut GameState<Self::VariantScore, Self>,
        action: DrawDiscardPileAction,
    ) -> Result<(), ActionError>;

    /// Handle laying off a card from the player's hand.
    fn handle_lay_off(
        &self,
        state: &mut GameState<Self::VariantScore, Self>,
        action: LayOffAction,
    ) -> Result<(), ActionError>;

    /// Handle forming a single meld.
    fn handle_form_meld(
        &self,
        state: &mut GameState<Self::VariantScore, Self>,
        action: FormMeldAction,
    ) -> Result<(), ActionError>;

    /// Handle forming multiple melds at one time. Either all melds successfully form, or none do.
    fn handle_form_melds(
        &self,
        state: &mut GameState<Self::VariantScore, Self>,
        action: FormMeldsAction,
    ) -> Result<(), ActionError>;

    /// Handle discarding a card.
    fn handle_discard(
        &self,
        state: &mut GameState<Self::VariantScore, Self>,
        action: DiscardAction,
    ) -> Result<(), ActionError>;

    /// Calculate the score for a round. Returns an `Err` if the round hasn't ended.
    fn calculate_round_score(
        &self,
        state: &GameState<Self::VariantScore, Self>,
    ) -> Result<RoundScore<Self::VariantScore>, GameError>;
}
