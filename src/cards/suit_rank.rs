use serde::{Deserialize, Serialize};
use strum::EnumIter;

/// Poker suits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, EnumIter)]
pub enum Suit {
    Joker,
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl Suit {
    /// Returns the Unicode character symbol for the suit
    pub fn as_str(&self) -> &'static str {
        match self {
            Suit::Joker => "🃏",
            Suit::Clubs => "♣",
            Suit::Diamonds => "♦",
            Suit::Hearts => "♥",
            Suit::Spades => "♠",
        }
    }
}

/// Poker ranks.    
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, EnumIter)]
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

impl Rank {
    /// Returns the short form string representation of the rank
    pub fn as_str(&self) -> &'static str {
        match self {
            Rank::Joker => "🃏",
            Rank::Ace => "A",
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "10",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
        }
    }
}