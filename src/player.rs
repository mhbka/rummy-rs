use super::cards::{card::Card, meld::Meld};

/// A Rummy player.
#[derive(Debug, Clone)]
pub struct Player {
    pub(crate) id: usize,
    pub(crate) cards: Vec<Card>,
    pub(crate) melds: Vec<Meld>,
    pub(crate) active: bool,
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

    // Reference getters
    pub fn id(&self) -> usize {
        self.id
    }

    pub fn cards(&self) -> &Vec<Card> {
        &self.cards
    }

    pub fn melds(&self) -> &Vec<Meld> {
        &self.melds
    }

    pub fn active(&self) -> bool {
        self.active
    }

    pub fn joined_in_round(&self) -> usize {
        self.joined_in_round
    }
}
