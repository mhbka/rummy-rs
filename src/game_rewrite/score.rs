use std::collections::HashMap;

/// Score information for a completed round.
#[derive(Debug, Clone, PartialEq)]
pub struct RoundScore<P: VariantPlayerScore> {
    pub round_number: u32,
    pub player_scores: HashMap<usize, P>,
    pub winner: Option<usize>,
}

/// Represents a player score, which depends on the variant's scoring system.
/// 
/// Most importantly, the scoring system should be able to represent the score as a simple `i32`.
pub trait VariantPlayerScore {
    /// Represent the score as an `i32` value.
    fn score(&self) -> i32;
}