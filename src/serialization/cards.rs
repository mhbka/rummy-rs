use std::sync::Arc;

use crate::cards::{
    card::{Card, CardData},
    deck::{Deck, DeckConfig},
    meld::{Meld, Run, Set},
    suit_rank::{Rank, Suit},
};
use serde::{Deserialize, Serialize};

/// A serializable version of a `Set`.
#[derive(Serialize, Deserialize)]
pub(super) struct SerializableSet {
    cards: Vec<CardData>,
    set_rank: Rank,
}

/// A serializable version of a `Run`.
#[derive(Serialize, Deserialize)]
pub(super) struct SerializableRun {
    cards: Vec<CardData>,
    set_suit: Suit,
}

/// A serializable version of a `Meld`.
#[derive(Serialize, Deserialize)]
pub(super) enum SerializableMeld {
    Set(SerializableSet),
    Run(SerializableRun),
}

impl SerializableMeld {
    /// Convert this to a `Meld`.
    pub fn to_meld(self, deck_config: Arc<DeckConfig>) -> Meld {
        match self {
            Self::Set(set) => {
                let cards = set
                    .cards
                    .into_iter()
                    .map(|c| Card {
                        rank: c.rank,
                        suit: c.suit,
                        deck_config: deck_config.clone(),
                    })
                    .collect();
                let set = Set {
                    cards,
                    set_rank: set.set_rank,
                };
                Meld::Set(set)
            }
            Self::Run(run) => {
                let cards = run
                    .cards
                    .into_iter()
                    .map(|c| Card {
                        rank: c.rank,
                        suit: c.suit,
                        deck_config: deck_config.clone(),
                    })
                    .collect();
                let run = Run {
                    cards,
                    set_suit: run.set_suit,
                };
                Meld::Run(run)
            }
        }
    }

    /// Convert from a `Meld`.
    pub fn from_meld(meld: &Meld) -> Self {
        match meld {
            Meld::Run(run) => {
                let cards = run.cards.iter().map(|c| c.data()).collect();
                let run = SerializableRun {
                    cards,
                    set_suit: run.set_suit,
                };
                Self::Run(run)
            }
            Meld::Set(set) => {
                let cards = set.cards.iter().map(|c| c.data()).collect();
                let set = SerializableSet {
                    cards,
                    set_rank: set.set_rank,
                };
                Self::Set(set)
            }
        }
    }
}

/// A serializable version of a `Deck`.
#[derive(Serialize, Deserialize)]
pub(super) struct SerializableDeck {
    pub stock: Vec<CardData>,
    pub discard_pile: Vec<CardData>,
}

impl SerializableDeck {
    /// Convert to a `Deck`.
    pub fn to_deck(self, deck_config: Arc<DeckConfig>) -> Deck {
        let stock = self
            .stock
            .into_iter()
            .map(|c| Card::from_card_data(c, deck_config.clone()))
            .collect();
        let discard_pile = self
            .discard_pile
            .into_iter()
            .map(|c| Card::from_card_data(c, deck_config.clone()))
            .collect();
        Deck {
            stock,
            discard_pile,
            config: deck_config,
        }
    }

    /// Convert from a `Deck`.
    pub fn from_deck(deck: &Deck) -> Self {
        let stock = deck.stock.iter().map(|c| c.data()).collect();
        let discard_pile = deck.discard_pile.iter().map(|c| c.data()).collect();
        Self {
            stock,
            discard_pile,
        }
    }
}
