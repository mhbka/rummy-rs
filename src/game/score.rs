use std::collections::HashMap;

/// Score information for a completed round.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RoundScore<P: VariantPlayerScore> {
    /// Map of player IDs to their scores.
    pub player_scores: HashMap<usize, P>,
    /// The ID of the player who won.
    pub winner_id: usize,
}

#[cfg(feature = "serde")]
/// Represents a player's score in a Rummy variant.
pub trait VariantPlayerScore: Sized + Clone + PartialEq + Eq {}

#[cfg(not(feature = "serde"))]
/// Represents a player's score in a Rummy variant.
pub trait VariantPlayerScore: Sized + Clone + PartialEq + Eq {}
