use super::cards::{card::Card, meld::Meld};

/// A Rummy player.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    /// How they're identified.
    pub(crate) id: usize,
    /// Their hand.
    pub(crate) cards: Vec<Card>,
    /// Their melds.
    pub(crate) melds: Vec<Meld>,
    /// Whether they're currently playing.
    pub(crate) active: bool,
    /// The round they joined in.
    pub(crate) joined_in_round: usize,
}

impl Player {
    /// Creates a new player.
    pub(crate) fn new(id: usize, active: bool, joined_in_round: usize) -> Self {
        Player {
            id,
            cards: Vec::new(),
            melds: Vec::new(),
            active,
            joined_in_round,
        }
    }

    /// Get the player's ID.
    pub fn id(&self) -> usize {
        self.id
    }

    /// Get the player's cards.
    pub fn cards(&self) -> &Vec<Card> {
        &self.cards
    }

    /// Get the player's melds.
    pub fn melds(&self) -> &Vec<Meld> {
        &self.melds
    }

    /// Whether the player is currently active.
    pub fn active(&self) -> bool {
        self.active
    }

    /// Which round the player joined in.
    pub fn joined_in_round(&self) -> usize {
        self.joined_in_round
    }
}
