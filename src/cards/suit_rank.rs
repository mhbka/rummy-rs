use serde::{Deserialize, Serialize};
use strum::EnumIter;

/// Poker suits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, EnumIter)]
pub enum Suit {
    Joker,
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

/// Poker ranks.    
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, EnumIter)]
pub enum Rank {
    Joker,
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}
