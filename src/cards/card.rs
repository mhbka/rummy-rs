use super::{
    deck::DeckConfig,
    suit_rank::{Rank, Suit},
};
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering, fmt::{Debug, Display}, hash::Hash, rc::Rc
};

/// A card.
///
/// Always tied to a `Deck`.
#[derive(Clone, Serialize, Deserialize)]
pub struct Card {
    pub(crate) rank: Rank,
    pub(crate) suit: Suit,

    #[serde(skip_serializing, skip_deserializing)]
    pub(crate) deck_config: Rc<DeckConfig>, // TODO: make this Option so we can default it to None for serde
                                            // TODO: then figure out how to Rc to the deck upon deserializing
}

impl Card {
    /// Gets the card's rank and suit.
    pub fn data(&self) -> (Rank, Suit) {
        (self.rank, self.suit)
    }

    /// Obtain the "value" of a `Card`.
    /// 
    /// If the deck config has a custom `high_rank`, this function computes the correct value
    /// taking that into account.
    /// 
    /// The value is `4*(relative rank value) + (suit value)`.
    pub fn value(&self) -> u8 {
        if self.suit == Suit::Joker || self.rank == Rank::Joker {
            return 0;
        }

        let max_rank = Rank::King as u8;

        let highest_rank = match self.deck_config.high_rank {
           None => max_rank,
           Some(high_rank) => high_rank as u8
        };

        let rank_offset = max_rank - highest_rank;
        let mut relative_self_rank = (self.rank as u8 + rank_offset) % (max_rank + 1);

        // in any custom high rank, Joker is included in offset, so King -> Ace counts as 2 jumps in rank;
        // here we subtract 1 for ranks after King, up to the custom highest rank.
        // TODO: optimize this into 1 calculation if possible
        if let Some(highest_rank) = self.deck_config.high_rank {
            if self.rank >= Rank::Ace && self.rank <= highest_rank {
                relative_self_rank -= 1;
            }
        }

        4 * relative_self_rank + self.suit as u8
    }

    /// Whether or not `other` has the same suit and the consecutive (relative) rank. For eg:
    /// - `high_rank = None`: (Two, Clubs) -> (Three, Clubs) = `true`
    /// - `high_rank = None`: (Two, Clubs) -> (Three, Spades) = `false`
    /// - `high_rank = Some(Two)`: (Two, Clubs) -> (Three, Clubs) = `false`
    /// 
    /// Mostly useful for validating runs.
    pub(crate) fn same_suit_consecutive_rank(&self, other: &Card) -> bool {
        self.value() + 4 == other.value()
    }

    /// Returns whether the card is a `wildcard`, as determined by `deck_config`.
    pub(crate) fn is_wildcard(&self) -> bool {
        Some(self.rank) == self.deck_config.wildcard_rank
    } 
}

/// Equality impls
impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        return self.rank == other.rank && self.suit == other.suit;
    }
}

impl Eq for Card {}

/// Compares cards by rank, then suit.
///
/// For rank, we offset by the high rank provided in the deck's config (if there is one).
/// Thus, the deck can use any rank as high rank,
/// and ordering will count down from there.
///
/// For example, if high rank is 2,
/// then 2 > Ace > King ... 4 > 3.
impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value().cmp(&other.value())
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Display impls
impl Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("")
            .field("Card", &format!("{}", self))
            .finish()
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} of {:?}", self.rank, self.suit)
    }
}

// Hash impl (for checking that 2 collections hold the same Cards regardless of order)
impl Hash for Card {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (self.rank as u8).hash(state);
        (self.suit as u8).hash(state);
    }
}
