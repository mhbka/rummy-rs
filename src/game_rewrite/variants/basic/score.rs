use crate::{game_rewrite::score::VariantPlayerScore, player::Player};

/// A single player's score in basic Rummy.
pub struct BasicScore {
    score: u32
}

impl BasicScore {
    /// Score a player.
    pub fn score_player(player: &Player) -> Self {
        let score = player.cards
            .iter()
            .fold(0, |score, card| score + card.score_value());
        Self { score: score.into() }
    }
}

impl VariantPlayerScore for BasicScore {
    fn score_value(&self) -> i32 {
        self.score as i32
    }
}