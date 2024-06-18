use actions::*;

pub mod actions;
pub mod error;
pub mod phases;
pub mod state;
pub mod variants;

pub trait Game {
    type InDrawPhase: DrawActions;
    type InPlayPhase: PlayActions;
    type InDiscardPhase: DiscardActions;
    type InRoundEndPhase: RoundEndActions;
    type InGameEndPhase: GameEndActions;
}
