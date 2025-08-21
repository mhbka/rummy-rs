use crate::game_rewrite::state::GamePhase;

/// Errors that may be returned from executing a `GameAction`.
#[derive(Debug, Clone)]
pub enum GameError {
    /// The action was executed in the wrong game state.
    InvalidGamePhase { current_phase: GamePhase },
    DrawFailed(String),
    LayOffFailed(String),
    MeldFailed(String),
    DiscardFailed(String),
    /// 
    InvalidCardIndex,
    InvalidMeldIndex,
    InvalidPlayerIndex,
    InsufficientCards,
}