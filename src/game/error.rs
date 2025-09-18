use thiserror::Error;

use crate::{cards::meld::MeldError, game::state::GamePhase};

/// Errors that may be returned from executing a `GameAction`.
#[derive(Debug, Clone, Error)]
pub enum ActionError {
    /// The action failed to execute.
    #[error("{0}")]
    FailedAction(#[from] FailedActionError),
    /// Serious; indicates something went wrong internally, such as an unexpected invalid state.
    /// May need to stop the game.
    #[error("{0}")]
    Internal(#[from] InternalError)
}

/// Reasons that a `GameAction` couldn't be executed.
#[derive(Debug, Clone, Error)]
pub enum FailedActionError {
    #[error("The action was executed in the wrong game state: {current_phase:?}")]
    InvalidGamePhase { current_phase: GamePhase },
    #[error("Couldn't draw from the discard pile as it had no/not enough cards")]
    DiscardPileTooSmall,
    #[error("The index for a card was invalid (likely out of bounds)")]
    InvalidCardIndex,
    #[error("The index for a meld was invalid (likely out of bounds)")]
    InvalidMeldIndex,
    #[error("The index for a player was invalid/doesn't exist")]
    InvalidPlayerIndex,
    #[error("A meld layoff/formation failed")]
    FailedMeld(#[from] MeldError),
}

/// Internal errors encountered during the game.
/// 
/// This possibly indicate invalid internal state; as a result, the game may need to be terminated.
#[derive(Debug, Clone, Error)]
pub enum InternalError {
    #[error("There are no/not enough cards left in deck and discard pile to draw the required amount")]
    NoCardsInDeckOrDiscardPile,
    #[error("The current player index is invalid")]
    InvalidCurrentPlayer { current: usize },
    #[error("The round has no winner despite having already ended")]
    RoundHasNoWinner
}

/// Errors pertaining to the game itself.
/// 
/// This doesn't indicate any issues with internal state, but rather that some function was called at the "wrong" time.
#[derive(Debug, Clone, Error)]
pub enum GameError {
    #[error("Attempted to perform some action in the wrong game phase")]
    WrongGamePhase,
    #[error("Tried to reference a player ID which doesn't exist")]
    PlayerDoesntExist,
    #[error("Tried to add a player ID when it already exists")]
    AddedPlayerAlreadyExists,
    #[error("Failed to rearrange the hand")]
    FailedHandRearrangement, 
    #[error("The round setup failed: {0}")]
    FailedRoundSetup(#[from] GameSetupError),
    #[error("An internal error occurred {0}")]
    Internal(#[from] InternalError)
}

/// Errors while creating a game.
#[derive(Clone, Debug, Error)]
pub enum GameSetupError {
    #[error("The game has too many players")]
    TooManyPlayers,
    #[error("The game has too few players")]
    TooFewPlayers,
    #[error("The deck doesn't have enough cards for the number of players (enough meaning all players can be dealt + draw from deck once)")]
    NotEnoughCards
}