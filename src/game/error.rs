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
/// This possibly indicate invalid internal state; as a result, the game may need to be terminated.
#[derive(Debug, Clone)]
pub enum InternalError {
    /// There are no/not enough cards left in deck and discard pile to draw the required amount.
    NoCardsInDeckOrDiscardPile,
    /// The current player index is invalid.
    InvalidCurrentPlayer { current: usize }
}

/// Errors pertaining to the game itself, such as setup or adding/quitting players.
/// 
/// This doesn't indicate any issues with internal state, but rather that some function was called at the "wrong" time.
#[derive(Debug, Clone)]
pub enum GameError {
    /// Attempted to perform some action in the wrong game phase.
    WrongGamePhase,
    /// Tried to reference a player ID which doesn't exist.
    PlayerDoesntExist,
    /// Tried to add a player with already existing ID.
    AddedPlayerAlreadyExists,
    /// The game already has maximum number of players.
    MaxPlayersReached,
    /// The game has too few players.
    TooFewPlayers,
    /// Failed to apply a hand rearrangement, likely due to a mismatch of hand and newly arranged cards.
    FailedHandRearrangement, 
    /// No winner was found for the round.
    RoundHasNoWinner,
    /// Some other internal error occurred.
    Internal(InternalError)
}

impl From<InternalError> for GameError {
    fn from(value: InternalError) -> Self {
        Self::Internal(value)
    }
}