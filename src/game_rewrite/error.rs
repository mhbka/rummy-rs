use crate::game_rewrite::state::GamePhase;

/// Errors that may be returned from executing a `GameAction`.
#[derive(Debug, Clone)]
pub enum ActionError {
    /// The action was executed in the wrong game state.
    InvalidGamePhase { current_phase: GamePhase },
    /// The index for a card was invalid (likely out of bounds).
    InvalidCardIndex,
    /// The index for a meld was invalid (likely out of bounds).
    InvalidMeldIndex,
    /// The ID for a player was invalid/doesn't exist.
    InvalidPlayerID,
}

/// Errors pertaining to the game itself, such as setup or adding/quitting players.
#[derive(Debug, Clone)]
pub enum GameError {
    /// Attempted to perform an end-of-round function (like score calculation) when it hasn't ended.
    RoundHasntEnded,
}