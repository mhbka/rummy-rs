use crate::{cards::meld::MeldError, game_rewrite::state::GamePhase};

/// Errors that may be returned from executing a `GameAction`.
#[derive(Debug, Clone)]
pub enum ActionError {
    /// The action failed to execute.
    FailedAction(FailedActionError),
    /// Serious; indicates something went wrong internally, such as an unexpected invalid state.
    /// May need to stop the game.
    Internal(InternalError)
}

impl From<FailedActionError> for ActionError {
    fn from(value: FailedActionError) -> Self {
        Self::FailedAction(value)
    }
}

impl From<InternalError> for ActionError {
    fn from(value: InternalError) -> Self {
        Self::Internal(value)
    }
}

/// Reasons that a `GameAction` couldn't be executed.
#[derive(Debug, Clone)]
pub enum FailedActionError {
    /// The action was executed in the wrong game state.
    InvalidGamePhase { current_phase: GamePhase },
    /// Couldn't draw from the discard pile as it had no/not enough cards.
    DiscardPileTooSmall,
    /// The index for a card was invalid (likely out of bounds).
    InvalidCardIndex,
    /// The index for a meld was invalid (likely out of bounds).
    InvalidMeldIndex,
    /// The index for a player was invalid/doesn't exist.
    InvalidPlayerIndex,
    /// A meld layoff/formation failed.
    FailedMeld(MeldError)
}

/// Internal errors encountered during the game.
/// 
/// Possibly indicate invalid internal state; as a result, game may need to be terminated.
#[derive(Debug, Clone)]
pub enum InternalError {
    /// There are no/not enough cards left in deck and discard pile to draw the required amount.
    NoCardsInDeckOrDiscardPile,
    /// The current player index is invalid.
    InvalidCurrentPlayer { current: usize }
}

/// Errors pertaining to the game itself, such as setup or adding/quitting players.
#[derive(Debug, Clone)]
pub enum GameError {
    /// Attempted to perform an end-of-round function (like score calculation) when it hasn't ended.
    RoundHasntEnded,
    /// Tried to quit a player which doesn't exist.
    QuitPlayerDoesntExist,
    /// Tried to add a player with already existing ID.
    AddedPlayerAlreadyExists,
    /// The game already has maximum number of players.
    MaxPlayersReached,
    /// Some other internal error occurred.
    Internal(InternalError)
}

impl From<InternalError> for GameError {
    fn from(value: InternalError) -> Self {
        Self::Internal(value)
    }
}