use crate::{cards::deck::Deck, game_rewrite::{action::GameAction, game::{GameError, GameRules}}, player::Player};



/// The state of the game. Includes state common across all variants like players,
/// as well as `variant_data` for variant-specific information.
#[derive(Debug, Clone)]
pub struct GameState<R: GameRules> {
    pub phase: GamePhase,
    pub players: Vec<Player>,
    pub deck: Deck,
    pub current_player: usize,
    pub round_number: u32,
    pub variant_data: R::VariantState, 
}

impl<R: GameRules> GameState<R> 
where
    R::VariantState: VariantState<R>
{
    /// Validate if the action is valid in the current gamestate.
    pub fn validate_action(&self, action: &GameAction) -> Result<(), GameError> {
        match (self.phase, action) {
            (GamePhase::Draw, GameAction::DrawDeck(_)) => Ok(()),
            (GamePhase::Draw, GameAction::DrawDiscardPile(_)) => Ok(()),
            (GamePhase::Play, GameAction::FormMeld(_)) => Ok(()),
            (GamePhase::Play, GameAction::FormMelds(_)) => Ok(()),
            (GamePhase::Play, GameAction::LayOff(_)) => Ok(()),
            (GamePhase::Play, GameAction::Discard(_)) => Ok(()),
            _ => Err(GameError::InvalidGamePhase { current_phase: self.phase }),
        }?;
        R::VariantState::validate_action(self, action)
    }
}

/// Represents the unique state held by a Rummy variant.
pub trait VariantState<R: GameRules<VariantState = Self>>: Sized {
    /// Validate if an action is **generally** valid in the current gamestate, for the variant.
    /// 
    /// The default implementation is to just return `Ok(())`. If a variant requires its own validation
    /// for general actions, this function can be overridden.
    /// 
    /// ## Note
    /// This should not be used for validating specific actions (ie, if forming a meld is valid).
    /// That should be done in the `GameRules` action handler instead.
    fn validate_action(state: &GameState<R>, action: &GameAction) -> Result<(), GameError> {
        Ok(())
    }
}

/// The possible phases of a Rummy game.
#[derive(Clone, Copy, Debug)]
pub enum GamePhase {
    Draw,
    Play,
    RoundEnd,
    GameEnd
}