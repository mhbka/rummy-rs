use crate::{
    cards::{
        card::{Card, CardData},
        deck::DeckConfig,
    },
    player::Player,
    serialization::cards::SerializableMeld,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// A serializable version of a `Deck`.
#[derive(Serialize, Deserialize)]
pub(super) struct SerializablePlayer {
    pub id: usize,
    pub cards: Vec<CardData>,
    pub melds: Vec<SerializableMeld>,
    pub active: bool,
    pub joined_in_round: usize,
}

impl SerializablePlayer {
    /// Convert to a `Player`.
    pub fn to_player(self, deck_config: Arc<DeckConfig>) -> Player {
        let cards = self
            .cards
            .into_iter()
            .map(|c| Card::from_card_data(c, deck_config.clone()))
            .collect();
        let melds = self
            .melds
            .into_iter()
            .map(|m| m.to_meld(deck_config.clone()))
            .collect();
        Player {
            id: self.id,
            cards,
            melds,
            active: self.active,
            joined_in_round: self.joined_in_round,
        }
    }

    /// Convert from a `Player`.
    pub fn from_player(player: &Player) -> Self {
        let cards = player.cards.iter().map(|c| c.data()).collect();
        let melds = player
            .melds
            .iter()
            .map(|m| SerializableMeld::from_meld(m))
            .collect();
        Self {
            id: player.id,
            cards,
            melds,
            active: player.active,
            joined_in_round: player.joined_in_round,
        }
    }
}
