use super::{
    card::Card,
    suit_rank::{Rank, Suit},
};

pub trait Meldable {
    /// Attempt to create a new meld out of `Card`s and indices of the chosen cards.
    ///
    /// If valid, the indexed cards are removed and `Ok` is returned.
    ///
    /// Else, `Err` is returned and `hand_cards` is left untouched.
    fn new(hand_cards: &mut Vec<Card>, indices: &Vec<usize>) -> Result<Self, String>
    where
        Self: Sized;

    /// Attempt to add a card from `cards`, as chosen by `index`, to the meld.
    ///
    /// If valid, the card is moved from `cards` into the meld and `Ok` is returned.
    ///
    /// If a wildcard in the meld can be replaced by the layoff card, they are swapped and `Ok` is returned.
    ///
    /// Else, `Err` is returned and `hand_cards` is left untouched.
    fn layoff_card(&mut self, hand_cards: &mut Vec<Card>, index: usize) -> Result<(), String>;
}

/// A Rummy meld.
/// There are 2 types:
/// - **Set**; >=3 cards of same rank
/// - **Run**; >=3 sequential cards of same suit
#[derive(Debug)]
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
}

impl Meldable for Meld {
    fn new(hand_cards: &mut Vec<Card>, indices: &Vec<usize>) -> Result<Self, String>
    where
        Self: Sized,
    {
        match Set::new(hand_cards, indices) {
            Ok(set) => Ok(Meld::Set(set)),
            Err(set_err) => match Run::new(hand_cards, indices) {
                Ok(run) => Ok(Meld::Run(run)),
                Err(run_err) => Err(format!("Couldn't form set ({set_err}) or run ({run_err})")),
            },
        }
    }

    fn layoff_card(&mut self, hand_cards: &mut Vec<Card>, index: usize) -> Result<(), String> {
        match self {
            Meld::Set(set) => set.layoff_card(hand_cards, index),
            Meld::Run(run) => run.layoff_card(hand_cards, index),
        }
    }
}

/// A Rummy meld set.
#[derive(Debug)]
pub struct Set {
    cards: Vec<Card>,
    set_rank: Rank,
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
    fn new(hand_cards: &mut Vec<Card>, indices: &Vec<usize>) -> Result<Self, String> {
        if indices.len() < 3 {
            return Err(format!("Length of indices ({}) less than 3", indices.len()));
        }

        let cards = indices
            .iter()
            .map(|&i| {
                hand_cards
                    .get(i)
                    .ok_or("index is greater than hand_cards size".to_string())
            })
            .collect::<Result<Vec<_>, _>>()?; // lmfao

        match cards[0].deck_config.wildcard_rank {
            // check if every card has same rank, or the wildcard rank
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
                    // if set_rank is None, there is no none-wildcard, which isn't valid
                    if non_wildcard_rank.is_none() {
                        return Err("A set cannot be formed out of only wildcards".into());
                    }
                } else {
                    return Err("Cards do not form a valid set".into());
                }
            }

            // we check if every card has same rank
            None => {
                if !cards.iter().all(|card| card.rank == cards[0].rank) {
                    return Err("Cards do not form a valid set".into());
                }
            }
        }

        // if we reach here, we have a valid set
        let cards: Vec<_> = cards // clone meld cards into a new vec...
            .into_iter()
            .cloned()
            .collect();

        let mut idx = 0;
        hand_cards.retain(|_| {
            // ... and remove them from the hand cards
            idx += 1;
            !indices.contains(&(idx - 1))
        });

        let set_rank = cards.iter().find(|c| !c.is_wildcard()).unwrap().rank;

        Ok(Set { cards, set_rank })
    }

    fn layoff_card(&mut self, hand_cards: &mut Vec<Card>, index: usize) -> Result<(), String> {
        let card = hand_cards
            .get_mut(index)
            .ok_or("index is greater than hand_cards' size")?;

        // if our card has the set's rank, swap with any wildcard in the meld first;
        // if there aren't any, then just push into the meld
        if card.rank == self.set_rank {
            if let Some(wildcard) = self.cards.iter_mut().find(|c| c.is_wildcard()) {
                std::mem::swap(wildcard, card);
            } else {
                self.cards.push(hand_cards.remove(index));
            }
            return Ok(());
        }
        // else, if the layoff card is a wildcard, add it
        else if let Some(wildcard_rank) = card.deck_config.wildcard_rank {
            if card.rank == wildcard_rank {
                self.cards.push(hand_cards.remove(index));
                return Ok(());
            }
        }

        Err("Card cannot be laid off in this set".into())
    }
}

/// A Rummy meld run.
#[derive(Debug)]
pub struct Run {
    cards: Vec<Card>,
    set_suit: Suit,
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
    fn new(hand_cards: &mut Vec<Card>, indices: &Vec<usize>) -> Result<Self, String> {
        if indices.len() < 3 {
            return Err(format!("Length of indices ({}) less than 3", indices.len()));
        }

        let cards = indices
            .iter()
            .map(|&idx| {
                hand_cards
                    .get(idx)
                    .ok_or("Index in indices is greater than cards' size")
            })
            .collect::<Result<Vec<_>, _>>()?; // lmfao nice syntax

        let deck_config = cards[0].deck_config.clone();

        let (mut cards, mut wildcards) = match deck_config.wildcard_rank {
            Some(wildcard_rank) => cards.iter().partition(|&c| c.rank != wildcard_rank),
            None => (cards.iter().collect(), Vec::new()),
        };

        cards.sort();

        // Verify that cards (and wildcards) can form a run
        match deck_config.wildcard_rank {
            None => {
                // No wildcard, so just check for same suit and consecutive (relative) rank
                if !cards
                    .windows(2)
                    .all(|w| w[0].same_suit_consecutive_rank(w[1]))
                {
                    return Err(format!("Cards don't form a valid run"));
                }
            }
            Some(_) => {
                // Check that each card has same suit and +1 rank from previous card (or previous card is wildcard).
                // If not, try to insert a wildcard and continue.
                // If we have no wildcards left to insert, return Err.
                let mut i = 1;
                let mut cards_len = cards.len();
                while i < cards_len {
                    if !cards[i - 1].same_suit_consecutive_rank(cards[i]) {
                        let wildcard = wildcards.pop().ok_or(
                            "Cards don't form valid run (and not enough wildcards to fill gaps)",
                        )?;
                        cards.insert(i, wildcard);
                        i += 1; // since we just added a card to `cards`...
                        cards_len += 1; // ... these 2 have to be incremented
                    }
                    i += 1;
                }
            }
        };

        cards.append(&mut wildcards);

        // reaching here = valid run, so clone out the meld cards...
        let cards: Vec<_> = cards.iter().map(|&&c| c).cloned().collect();

        let mut idx = 0;
        hand_cards.retain(|_| {
            // ...and remove them from the hand cards
            idx += 1;
            !indices.contains(&(idx - 1))
        });

        let set_suit = cards.iter().find(|c| !c.is_wildcard()).unwrap().suit;

        Ok(Run { cards, set_suit })
    }

    fn layoff_card(&mut self, hand_cards: &mut Vec<Card>, index: usize) -> Result<(), String> {
        let layoff_card = hand_cards
            .get(index)
            .ok_or("index is greater than hand_cards' size")?;

        if let Some(wildcard_rank) = layoff_card.deck_config.wildcard_rank {
            // if our card is a wildcard, its always valid to layoff
            if layoff_card.rank == wildcard_rank {
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
            Err("Card cannot be laid off in this run".into())
        }
    }
}
