use crate::{cards::deck::Deck, game_rewrite::{action::GameAction, error::{ActionError, FailedActionError, GameError, InternalError}, game::GameRules}, player::Player};

/// The state of the game. Includes state common across all variants like players,
/// as well as `variant_state` for variant-specific information.
#[derive(Debug, Clone)]
pub struct GameState<R: GameRules> {
    pub phase: GamePhase,
    pub players: Vec<Player>,
    pub deck: Deck,
    pub current_player: usize,
    pub current_round: usize,
    pub variant_state: R::VariantState, 
}

impl<R: GameRules> GameState<R> 
where
    R::VariantState: VariantState<R>
{   
    /// Validate if the action is valid in the current gamestate.
    pub fn validate_action(&self, action: &GameAction) -> Result<(), ActionError> {
        match (self.phase, action) {
            (GamePhase::Draw, GameAction::DrawDeck(_)) => Ok(()),
            (GamePhase::Draw, GameAction::DrawDiscardPile(_)) => Ok(()),
            (GamePhase::Play, GameAction::FormMeld(_)) => Ok(()),
            (GamePhase::Play, GameAction::FormMelds(_)) => Ok(()),
            (GamePhase::Play, GameAction::LayOff(_)) => Ok(()),
            (GamePhase::Play, GameAction::Discard(_)) => Ok(()),
            _ => {
                let err = FailedActionError::InvalidGamePhase { current_phase: self.phase };
                return Err(ActionError::FailedAction(err));
            },
        }?;
        R::VariantState::validate_action(self, action)
    }

    /// Start a round by wiping cards and dealing new ones.
    /// 
    /// Returns an `Err` if the game phase isn't `RoundEnded`.
    pub fn start_new_round(&mut self, cards_to_deal: usize, starting_player_index: usize) -> Result<(), GameError> {
        if self.phase != GamePhase::RoundEnd {
            return Err(GameError::RoundHasntEnded);
        }
        self.deck.reset();
        for player in &mut self.players {
            player.melds = Vec::new();
            player.cards = self.deck
                .draw(cards_to_deal)
                .map_err(|err| GameError::Internal(InternalError::NoCardsInDeckOrDiscardPile))?;
        }
        self.current_player = starting_player_index;
        self.phase = GamePhase::Draw;
        self.current_round += 1;
        Ok(())
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
    /// This should not be used for validating specific actions (ie, whether forming a meld is valid).
    /// That should be done in the `GameRules` action handler instead.
    fn validate_action(state: &GameState<R>, action: &GameAction) -> Result<(), ActionError> {
        Ok(())
    }
}

/// The possible phases of a Rummy game.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GamePhase {
    Draw,
    Play,
    RoundEnd,
    GameEnd
}