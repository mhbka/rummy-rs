mod edge_cases;
mod errors;
mod meld_creation;
mod meld_layoff;
mod meld_properties;

use super::super::deck::DeckConfig;
use super::super::{
    card::Card,
    meld::{Meld, MeldError, Meldable, Run, Set},
    suit_rank::{Rank, Suit},
};
use std::sync::Arc;

/// Helper function to create a card with given rank and suit
pub fn create_card(rank: Rank, suit: Suit, config: Arc<DeckConfig>) -> Card {
    Card {
        rank,
        suit,
        deck_config: config,
    }
}

/// Helper to create a basic deck config
pub fn basic_config() -> Arc<DeckConfig> {
    Arc::new(DeckConfig::new())
}

/// Helper to create a set of cards with the same rank
pub fn create_set_cards(rank: Rank, suits: &[Suit], config: Arc<DeckConfig>) -> Vec<Card> {
    suits
        .iter()
        .map(|&suit| create_card(rank, suit, config.clone()))
        .collect()
}

/// Helper to create a run of cards in the same suit
pub fn create_run_cards(
    start_rank: Rank,
    length: usize,
    suit: Suit,
    config: Arc<DeckConfig>,
) -> Vec<Card> {
    // You'll need to implement rank iteration logic based on your Rank enum
    // This is a simplified version
    (0..length)
        .map(|i| create_card(start_rank, suit, config.clone())) // Placeholder - needs proper rank arithmetic
        .collect()
}
