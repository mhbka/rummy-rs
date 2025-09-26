//! Contains the representation for a player's score in basic Rummy.

use crate::{game::score::VariantPlayerScore, player::Player};

/// A single player's score in basic Rummy.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BasicScore {
    score: u32,
}

impl BasicScore {
    /// Score a player.
    pub fn score_player(player: &Player) -> Self {
        let score = player
            .cards
            .iter()
            .fold(0, |score, card| score + card.score_value());
        Self {
            score: score.into(),
        }
    }

    /// Get the score.
    pub fn score(&self) -> u32 {
        self.score
    }
}

impl VariantPlayerScore for BasicScore {}
