/// Trait indicating a game phase.
pub trait GamePhase {}

/// Trait indicating a phase where the game can still be played.
pub trait PlayablePhase {}

// GamePhase options.
pub struct DrawPhase {
    pub(super) has_drawn: bool,
}
pub struct PlayPhase {
    pub(super) play_count: usize,
}
pub struct DiscardPhase {
    pub(super) has_discarded: bool,
}
pub struct RoundEndPhase {
    pub(super) has_scored_round: bool,
}
pub struct GameEndPhase {
    // no state needed, game has ended
}

// Mark these as GamePhases.
impl GamePhase for DrawPhase {}
impl GamePhase for PlayPhase {}
impl GamePhase for DiscardPhase {}
impl GamePhase for RoundEndPhase {}
impl GamePhase for GameEndPhase {}

// Mark these as PlayablePhases (for PlayableActions).
impl PlayablePhase for DrawPhase {}
impl PlayablePhase for PlayPhase {}
impl PlayablePhase for DiscardPhase {}
impl PlayablePhase for RoundEndPhase {}
