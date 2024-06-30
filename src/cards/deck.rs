use std::sync::Arc;
use super::card::Card;
use super::suit_rank::{Rank, Suit};
use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

/// Configurable parameters for a deck:
/// - `shuffle_seed`: Optional seed for shuffling; `0` results in no shuffle
/// - `pack_count`: Number of card packs to include in the deck
/// - `use_joker`: Whether to add Jokers and use them as wildcard (2 per pack)
/// - `high_rank`: Whether to override the highest rank (default being King)
/// - `wildcard_rank`: Whether to have a wildcard rank (can also be Joker)
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeckConfig {
    pub shuffle_seed: Option<u64>,
    pub pack_count: usize,
    pub high_rank: Option<Rank>,
    pub wildcard_rank: Option<Rank>,
}

impl DeckConfig {
    /// Creates a new `DeckConfig` with standard settings.
    ///
    /// To customize, create the struct manually with the intended values.
    pub fn new() -> Self {
        DeckConfig {
            shuffle_seed: None,
            pack_count: 1,
            high_rank: None,
            wildcard_rank: None,
        }
    }
}

// TODO: verify cards belong to the deck before adding to discard pile

/// The deck, consisting of the:
/// - **stock**, face-down cards that can be drawn at the start of each turn
/// - **discard pile**, discarded cards, which can also be drawn
#[derive(Debug)]
pub struct Deck {
    config: Arc<DeckConfig>,
    stock: Vec<Card>,
    discard_pile: Vec<Card>,
}

impl Deck {
    /// Creates a new deck following the settings in `config` and shuffles it.
    ///
    /// **Note**:
    /// - If `pack_count` < 1, it will be set to 1.
    /// - If `shuffle_seed` is `Some`, it will always be shuffled according to the seed.
    /// - If `shuffle_seed` is `None`, it will never be shuffled.
    /// - If `wildcard_rank` is `Joker`, 2 jokers will be added per pack.
    pub fn new(mut config: DeckConfig) -> Self {
        config.pack_count = config.pack_count.max(1);

        let config = Arc::new(config);

        let mut deck = Deck {
            config: config.clone(),
            stock: Vec::new(),
            discard_pile: Vec::new(),
        };

        Deck::generate_cards(&mut deck.stock, &config);
        Deck::shuffle_cards(&mut deck.stock, &config);

        deck
    }

    /// Reset the cards by creating a new deck and shuffling it.
    ///
    /// **NOTE**: This refers to the current `DeckConfig`; if it has changed,
    /// the cards generated will be different from what was initially generated.
    pub fn reset(&mut self) {
        self.stock.clear();
        self.discard_pile.clear();
        Deck::generate_cards(&mut self.stock, &self.config);
        Deck::shuffle_cards(&mut self.stock, &self.config);
    }

    /// Draw `amount` cards from the deck stock.
    ///
    /// If `amount` is greater than the stock size, `Err` is returned.
    ///
    /// To replenish the stock, one can call `shuffle_discarded` or `turnover_discarded`.
    pub fn draw(&mut self, amount: usize) -> Result<Vec<Card>, String> {
        if amount > self.stock.len() {
            return Err(format!(
                "Draw amount ({amount}) greater than stock size ({})",
                self.stock.len()
            ));
        }

        let cards = self.stock.split_off(self.stock.len() - amount);
        Ok(cards)
    }

    /// Draw a specific card from the deck stock.
    ///
    /// If the card doesn't exist in the stock, return `Err`.
    ///
    /// If the deck is empty after drawing, shuffle the discarded cards back into it.
    pub fn draw_specific(&mut self, rank: Rank, suit: Suit) -> Result<Card, String> {
        for i in 0..self.stock.len() {
            let card = &self.stock[i];
            if card.rank == rank && card.suit == suit {
                return Ok(self.stock.remove(i));
            }
        }

        Err(format!("No card ({suit:?}, {rank:?}) in the stock"))
    }

    /// See the top card of the discard pile, if there is one.
    pub fn peek_discard_pile(&self) -> Option<(Rank, Suit)> {
        self.discard_pile.last().map(|card| card.data())
    }

    /// Attempt to draw a chosen amount of cards from the discard pile.
    ///
    /// If the amount is greater than discard pile's size, or the discard pile is empty,
    /// return `Err`.
    ///
    /// If `None` amount is specified, attempt to draw the entire discard pile.
    pub fn draw_discard_pile(&mut self, amount: Option<usize>) -> Result<Vec<Card>, String> {
        let discard_size = self.discard_pile.len();
        if discard_size == 0 {
            return Err(format!("Can't draw from empty discard pile"));
        } else if let Some(a) = amount {
            if a > discard_size {
                return Err(format!(
                    "Draw amount ({a}) greater than discard pile size ({discard_size})"
                ));
            }
            return Ok(self.discard_pile.split_off(discard_size - a));
        }
        return Ok(self.discard_pile.split_off(0));
    }

    /// Drains `cards` into the discard pile.
    pub fn add_to_discard_pile(&mut self, cards: &mut Vec<Card>) {
        self.discard_pile.append(cards);
    }

    /// Reset the stock by moving the discard pile into it and shuffling.
    pub fn shuffle_discarded(&mut self) {
        self.stock.append(&mut self.discard_pile);
        self.stock.shuffle(&mut rand::thread_rng());
    }

    /// Reset the stock by moving the discard pile into it and turning it over.
    pub fn turnover_discarded(&mut self) {
        self.stock.append(&mut self.discard_pile);
        self.stock.reverse();
    }

    /// Get a reference to the deck configuration.
    pub fn config(&self) -> &DeckConfig {
        &self.config
    }

    /// Get a reference to the deck stock.
    pub fn stock(&self) -> &Vec<Card> {
        &self.stock
    }

    /// Get a reference to the deck discard pile.
    pub fn discard_pile(&self) -> &Vec<Card> {
        &self.discard_pile
    }

    /// Generating cards into a `stock` based on `config`.
    fn generate_cards(stock: &mut Vec<Card>, config: &Arc<DeckConfig>) {
        for _ in 0..config.pack_count {
            for rank in Rank::iter() {
                if rank == Rank::Joker {
                    continue;
                }
                for suit in Suit::iter() {
                    if suit == Suit::Joker {
                        continue;
                    }
                    stock.push(Card {
                        rank,
                        suit,
                        deck_config: config.clone(),
                    });
                }
            }

            if config.wildcard_rank == Some(Rank::Joker) {
                for _ in 0..2 {
                    // 2 jokers per deck
                    stock.push(Card {
                        rank: Rank::Joker,
                        suit: Suit::Joker,
                        deck_config: config.clone(),
                    });
                }
            }
        }
    }

    /// Shuffles cards in a `stock` based on `config`.
    fn shuffle_cards(stock: &mut Vec<Card>, config: &Arc<DeckConfig>) {
        match config.shuffle_seed {
            Some(seed) => {
                if seed != 0 {
                    stock.shuffle(&mut StdRng::seed_from_u64(seed));
                }
            }
            None => stock.shuffle(&mut rand::thread_rng()),
        }
    }
}
