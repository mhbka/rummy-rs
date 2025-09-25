use std::collections::HashSet;

use super::{
    card::Card,
    suit_rank::{Rank, Suit},
};
use strum::Display;
use thiserror::Error;

/// Represents behaviour of a meld.
pub trait Meldable: Sized {
    /// Returns `Ok` if the cards in `hand_cards` indexed by `indices` form a valid meld.
    ///
    /// If not, returns an `Err` with the reason.
    fn valid(hand_cards: &Vec<Card>, indices: &[usize]) -> Result<(), MeldError>;

    /// Attempt to create a new meld out of cards in `hand_cards` indexed by `indices`.
    /// If valid, the indexed cards are removed and `Ok` is returned.
    ///
    /// Else, `Err` is returned and `hand_cards` is left untouched.
    fn new(hand_cards: &mut Vec<Card>, indices: &[usize]) -> Result<Self, MeldError>;

    /// Attempt to add a card from `cards`, as chosen by `index`, to the meld.
    ///
    /// If valid, the card is moved from `cards` into the meld and `Ok` is returned.
    ///
    /// If a wildcard in the meld can be replaced by the layoff card, they are swapped and `Ok` is returned.
    ///
    /// Else, `Err` is returned and `hand_cards` is left untouched.
    fn layoff_card(&mut self, hand_cards: &mut Vec<Card>, index: usize) -> Result<(), MeldError>;

    /// Inspect the meld's cards.
    fn cards(&self) -> &Vec<Card>;
}

/// A Rummy meld.
/// There are 2 types:
/// - **Set**; >=3 cards of the same rank
/// - **Run**; >=3 sequential cards of the same suit
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Meld {
    Set(Set),
    Run(Run),
}

impl Meld {
    /// Return if the meld is a set.
    pub fn is_set(&self) -> bool {
        if let Meld::Set(_) = self {
            return true;
        }
        false
    }

    /// Return if the meld is a run.
    pub fn is_run(&self) -> bool {
        !self.is_set()
    }

    /// Attempt to form multiple melds simultaneously in the order provided in `indices`,
    /// returning all the formed melds if successful.
    ///
    /// Returns with an error of `MeldError::MultipleMelds` at the first failure,
    /// in which case `hand_cards` is not mutated.
    pub fn multiple(
        hand_cards: &mut Vec<Card>,
        indices_of_melds: &Vec<Vec<usize>>,
    ) -> Result<Vec<Self>, MeldError> {
        // Validate all card indices are unique
        let mut all_indices: Vec<_> = indices_of_melds.iter().flatten().collect();
        let before_len = all_indices.len();
        all_indices.sort();
        all_indices.dedup();
        let dedup_len = all_indices.len();
        if before_len != dedup_len {
            let err = Box::new(MeldError::InvalidCardIndex);
            return Err(MeldError::FailedMultipleMelds { meld_index: 0, err });
        }

        // Validate that all meld indices form valid melds
        for (i, indices) in indices_of_melds.iter().enumerate() {
            if let Err(err) = Meld::valid(hand_cards, indices) {
                let err = Box::new(err);
                return Err(MeldError::FailedMultipleMelds { meld_index: i, err });
            }
        }

        // Clone each meld's cards and create the melds
        let melds = indices_of_melds
            .iter()
            .map(|indices| {
                let mut meld_cards = indices.iter().map(|&i| hand_cards[i].clone()).collect();
                Meld::new(&mut meld_cards, &(0..indices.len()).collect::<Vec<_>>())
            })
            .collect::<Result<Vec<_>, _>>()?;

        // Delete all meld cards from `hand_cards` in reverse order (so we don't run into indexing issues)
        for &&i in all_indices.iter().rev() {
            hand_cards.remove(i);
        }

        Ok(melds)
    }
}

impl Meldable for Meld {
    fn new(hand_cards: &mut Vec<Card>, indices: &[usize]) -> Result<Self, MeldError>
    where
        Self: Sized,
    {
        let set: HashSet<_> = indices.iter().collect();
        if set.len() != indices.len() {
            return Err(MeldError::InvalidCardIndex);
        }

        match Set::new(hand_cards, indices) {
            Ok(set) => Ok(Meld::Set(set)),
            Err(set_err) => match Run::new(hand_cards, indices) {
                Ok(run) => Ok(Meld::Run(run)),
                Err(_) => Err(set_err),
            },
        }
    }

    fn valid(hand_cards: &Vec<Card>, indices: &[usize]) -> Result<(), MeldError> {
        match Set::valid(hand_cards, indices) {
            Ok(_) => Ok(()),
            Err(set_err) => match Run::valid(hand_cards, indices) {
                Ok(_) => Ok(()),
                Err(_) => Err(set_err),
            },
        }
    }

    fn layoff_card(&mut self, hand_cards: &mut Vec<Card>, index: usize) -> Result<(), MeldError> {
        match self {
            Meld::Set(set) => set.layoff_card(hand_cards, index),
            Meld::Run(run) => run.layoff_card(hand_cards, index),
        }
    }

    fn cards(&self) -> &Vec<Card> {
        match self {
            Meld::Set(set) => set.cards(),
            Meld::Run(run) => run.cards(),
        }
    }
}

/// A Rummy meld set.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Set {
    pub(crate) cards: Vec<Card>,
    pub(crate) set_rank: Rank,
}

impl Set {
    /// Get an immutable reference to the set's cards.
    pub fn cards(&self) -> &Vec<Card> {
        &self.cards
    }

    /// Get the set's rank.
    pub fn rank(&self) -> Rank {
        self.set_rank
    }
}

impl Meldable for Set {
    fn new(hand_cards: &mut Vec<Card>, indices: &[usize]) -> Result<Self, MeldError> {
        Self::valid(&hand_cards, indices)?;

        let cards = indices
            .iter()
            .map(|&i| {
                hand_cards
                    .get(i)
                    .cloned()
                    // InvalidIndex shouldn't happen here since we already validated in `Self::valid`,
                    // but defensive programming is always good
                    .ok_or(MeldError::InvalidCardIndex)
            })
            .collect::<Result<Vec<_>, _>>()?;

        let mut idx = 0;
        hand_cards.retain(|_| {
            idx += 1;
            !indices.contains(&(idx - 1))
        });
        let set_rank = cards.iter().find(|c| !c.is_wildcard()).unwrap().rank;
        Ok(Set { cards, set_rank })
    }

    fn valid(hand_cards: &Vec<Card>, indices: &[usize]) -> Result<(), MeldError> {
        if indices.len() < 3 {
            return Err(MeldError::InsufficientCards {
                provided: indices.len(),
                minimum: 3,
            });
        }
        let cards = indices
            .iter()
            .map(|&i| hand_cards.get(i).ok_or(MeldError::InvalidCardIndex))
            .collect::<Result<Vec<_>, _>>()?;
        match cards[0].deck_config.wildcard_rank {
            // if there's a wildcard rank, check if every card has same rank or the wildcard rank
            Some(wildcard_rank) => {
                let mut non_wildcard_rank = None;
                if cards.iter().all(|card| {
                    if card.rank == wildcard_rank {
                        true
                    } else {
                        match non_wildcard_rank {
                            Some(rank) => card.rank == rank,
                            None => {
                                non_wildcard_rank = Some(card.rank);
                                true
                            }
                        }
                    }
                }) {
                    // if `non_wildcard_rank` is None, there is no non-wildcard, which isn't valid
                    if non_wildcard_rank.is_none() {
                        return Err(MeldError::OnlyWildcards);
                    }
                } else {
                    return Err(MeldError::InvalidSet);
                }
            }
            // if not, we just check if every card has same rank
            None => {
                if !cards.iter().all(|card| card.rank == cards[0].rank) {
                    return Err(MeldError::InvalidSet);
                }
            }
        }

        Ok(())
    }

    fn layoff_card(&mut self, hand_cards: &mut Vec<Card>, index: usize) -> Result<(), MeldError> {
        let card = hand_cards
            .get_mut(index)
            .ok_or(MeldError::InvalidCardIndex)?;

        // if our card has the set's rank, swap with any wildcard in the meld first.
        // if there aren't any wildcards, then just push into the meld
        if card.rank == self.set_rank {
            if let Some(wildcard) = self.cards.iter_mut().find(|c| c.is_wildcard()) {
                std::mem::swap(wildcard, card);
            } else {
                self.cards.push(hand_cards.remove(index));
            }
            return Ok(());
        }
        // else, if the layoff card is a wildcard, simply add it
        else if card.is_wildcard() {
            self.cards.push(hand_cards.remove(index));
            return Ok(());
        }

        Err(MeldError::InvalidLayoff)
    }

    fn cards(&self) -> &Vec<Card> {
        &self.cards
    }
}

/// A Rummy meld run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Run {
    pub(crate) cards: Vec<Card>,
    pub(crate) set_suit: Suit,
}

impl Run {
    /// Get an immutable reference to the run's cards.
    pub fn cards(&self) -> &Vec<Card> {
        &self.cards
    }

    /// Get the run's suit.
    pub fn suit(&self) -> Suit {
        self.set_suit
    }
}

impl Meldable for Run {
    fn new(hand_cards: &mut Vec<Card>, indices: &[usize]) -> Result<Self, MeldError> {
        Self::valid(&hand_cards, indices)?;

        let cards = indices
            .iter()
            .map(|&idx| {
                hand_cards
                    .get(idx)
                    .cloned()
                    .ok_or(MeldError::InvalidCardIndex)
            })
            .collect::<Result<Vec<_>, _>>()?;

        let mut sorted_indices = indices.to_vec();
        sorted_indices.sort();
        sorted_indices.reverse();
        for &idx in &sorted_indices {
            hand_cards.remove(idx);
        }

        let set_suit = cards
            .iter()
            .find(|c| !c.is_wildcard())
            .ok_or(MeldError::OnlyWildcards)?
            .suit;

        Ok(Run { cards, set_suit })
    }

    fn valid(hand_cards: &Vec<Card>, indices: &[usize]) -> Result<(), MeldError> {
        if indices.len() < 3 {
            return Err(MeldError::InsufficientCards {
                provided: indices.len(),
                minimum: 3,
            });
        }
        let chosen_cards = indices
            .iter()
            .map(|&idx| hand_cards.get(idx).ok_or(MeldError::InvalidCardIndex))
            .collect::<Result<Vec<_>, _>>()?;

        let deck_config = hand_cards[0].deck_config.clone();

        // Verify that cards (and wildcards) can form a run
        match deck_config.wildcard_rank {
            None => {
                // No wildcard, so just check for same suit and consecutive (relative) rank
                if !chosen_cards
                    .windows(2)
                    .all(|w| w[0].same_suit_consecutive_rank(w[1]))
                {
                    return Err(MeldError::InvalidRun);
                }
            }
            Some(wildcard_rank) => {
                // First, split normal cards and wildcards
                let (mut normal_cards, mut wildcards): (Vec<&Card>, Vec<&Card>) =
                    chosen_cards.iter().partition(|&c| c.rank != wildcard_rank);

                // Check that each card has same suit and +1 rank from previous card (or previous card is wildcard).
                // If not, try to insert a wildcard and continue.
                // If we have no wildcards left to insert, return Err.
                let mut i = 1;
                let mut cards_len = normal_cards.len();
                while i < cards_len {
                    if !normal_cards[i - 1].same_suit_consecutive_rank(normal_cards[i]) {
                        let wildcard = wildcards.pop().ok_or(MeldError::InsufficientWildcards)?;
                        normal_cards.insert(i, wildcard);
                        i += 1; // since we just added a card to `cards`...
                        cards_len += 1; // ... these 2 have to be incremented
                    }
                    i += 1;
                }
            }
        };

        Ok(())
    }

    fn layoff_card(&mut self, hand_cards: &mut Vec<Card>, index: usize) -> Result<(), MeldError> {
        let layoff_card = hand_cards.get(index).ok_or(MeldError::InvalidCardIndex)?;

        if layoff_card.deck_config.wildcard_rank.is_some() {
            // if our card is a wildcard, its always valid to layoff
            if layoff_card.is_wildcard() {
                self.cards.push(hand_cards.remove(index));
                return Ok(());
            }
            // else, see if there are any wildcards that we could replace in the meld
            else if let Some((wildcard_idx, _)) = self.cards.iter().enumerate().find(|&(i, _)| {
                self.cards[i].is_wildcard() // the current card is a wildcard...
                   && ((i < self.cards.len()-1 && layoff_card.same_suit_consecutive_rank(&self.cards[i+1])) // ... the next card is compatible with layoff card...
                   || self.cards[i-1].same_suit_consecutive_rank(layoff_card) // ... or the wildcard is last card, and the previous card is compatible.
                   )
            }) {
                let layoff_card = &mut hand_cards[index];
                let wildcard = &mut self.cards[wildcard_idx];
                std::mem::swap(wildcard, layoff_card); // swap our layoff card with the replaceable wildcard
                return Ok(());
            }
        }
        // see if card can be added at the bottom of the run...
        if layoff_card.same_suit_consecutive_rank(&self.cards[0]) {
            self.cards.insert(0, hand_cards.remove(index));
            return Ok(());
        }
        // ...or at the top (the only 2 possible places)
        else if self.cards[self.cards.len() - 1].same_suit_consecutive_rank(layoff_card) {
            self.cards.push(hand_cards.remove(index));
            return Ok(());
        } else {
            Err(MeldError::InvalidLayoff)
        }
    }

    fn cards(&self) -> &Vec<Card> {
        &self.cards
    }
}

/// Potential errors from a meld.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum MeldError {
    #[error("Not enough cards provided: got {provided}, need at least {minimum}")]
    InsufficientCards { provided: usize, minimum: usize },
    #[error("A given card index is out of bounds")]
    InvalidCardIndex,
    #[error("There's at least 1 duplicate card index")]
    DuplicateCardIndex,
    #[error("Cards do not form a valid set")]
    InvalidSet,
    #[error("Cards don't form a valid run")]
    InvalidRun,
    #[error("Cannot create a set from only wildcards")]
    OnlyWildcards,
    #[error("Card cannot be laid off")]
    InvalidLayoff,
    #[error("Cards don't form valid run (and not enough wildcards to fill gaps)")]
    InsufficientWildcards,
    #[error("Failed to form multiple melds. Meld {meld_index} failed with error: {err}")]
    FailedMultipleMelds {
        meld_index: usize,
        err: Box<MeldError>,
    },
}

/// The type of index which caused the `MeldError`.
#[derive(Debug, Display, Clone, PartialEq, Eq)]
pub enum MeldErrorIndexType {
    Card,
    Meld,
    Player,
}
