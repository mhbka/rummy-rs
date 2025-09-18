use std::collections::HashMap;

/// Score information for a completed round.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoundScore<P: VariantPlayerScore> {
    /// Map of player IDs to their scores.
    pub player_scores: HashMap<usize, P>,
    /// The ID of the player who won.
    pub winner_id: usize,
}

/// Represents a single player's score, which depends on the variant's scoring system.
/// 
/// Most importantly, the scoring system should be able to represent the score as a simple `i32`.
/// 
/// Since different scoring systems may require different information to score a player, 
/// this is left out of the trait.
pub trait VariantPlayerScore: Sized + Clone + PartialEq + Eq {
    /// Represent the score as an `i32` value.
    /// 
    /// It is up to the user to interpret this value.
    fn score_value(&self) -> i32;
}