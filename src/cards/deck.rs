use std::sync::Arc;
use crate::cards::card::CardData;

use super::card::Card;
use super::suit_rank::{Rank, Suit};
use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use strum::IntoEnumIterator;

/// Configurable values for a deck's behaviour.
/// 
/// ### `shuffle_seed`
/// Optional seed for shuffling, where `0` results in no shuffle. 
/// The default is a completely randomized shuffle.
/// 
/// ### `pack_count`
/// The number of card packs to include in the deck.
/// 
/// ### `high_rank`
/// Optional rank to override the highest rank.
/// If set, the rank right after it becomes the lowest rank. 
/// 
/// For example, if this is `Five`, the lowest ranks would be `Six` -> `Seven` -> `Eight` ...
/// 
/// The default is King.
/// 
/// ### `wildcard_rank`
/// Optional rank to denote as the wildcard (typically the Joker).
/// The default is to have no wildcards.
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

/// The deck, consisting of the:
/// - **config**, dictating shuffling, pack counts, wildcards etc.
/// - **stock**, face-down cards that can be drawn at the start of each turn
/// - **discard pile**, discarded cards, which can also be drawn
#[derive(Debug, Clone)]
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
    /// - If `shuffle_seed` is `None`, it will not be shuffled.
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

    /// (Re)creates the deck and shuffling it.
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
    pub fn try_draw(&mut self, amount: usize) -> Result<Vec<Card>, String> {
        if amount > self.stock.len() {
            return Err(format!(
                "Draw amount ({amount}) greater than stock size ({})",
                self.stock.len()
            ));
        }
        let cards = self.stock.split_off(self.stock.len() - amount);
        Ok(cards)
    }

    /// Draw `amount` cards from the deck stock;
    /// automatically turns over from the discard pile if there wasn't enough cards.
    /// 
    /// If `amount` is still greater than the stock size, `Err` is returned. 
    /// 
    /// ## Note
    /// If this errors, it is probably a serious issue.
    pub fn draw(&mut self, amount: usize) -> Result<Vec<Card>, String> {
        if amount > self.stock.len() {
            self.turnover_discarded();
        }
        if amount > self.stock.len() {
            return Err(format!(
                "Draw amount ({amount}) greater than stock + discard pile size (technically shouldn't happen)"
            ))
        }
        let cards = self.stock.split_off(self.stock.len() - amount);
        Ok(cards)
    }

    /// See the top card of the discard pile, if there is one.
    pub fn peek_discard_pile(&self) -> Option<CardData> {
        self.discard_pile.last().map(|card| card.data())
    }

    /// Attempt to draw a chosen amount of cards from the discard pile.
    ///
    /// If the amount is greater than discard pile's size, or the discard pile is empty,
    /// return `Err`.
    pub fn draw_discard_pile(&mut self, amount: usize) -> Result<Vec<Card>, String> {
        let discard_size = self.discard_pile.len();
        if discard_size == 0 {
            return Err(format!("Can't draw from empty discard pile"));
        } 
        else {
            if amount > discard_size {
                return Err(format!(
                    "Draw amount ({amount}) greater than discard pile size ({discard_size})"
                ));
            }
            return Ok(self.discard_pile.split_off(discard_size - amount));
        }
    }

    /// Drains `cards` into the discard pile.
    pub fn add_multiple_to_discard_pile(&mut self, cards: &mut Vec<Card>) {
        self.discard_pile.append(cards);
    }

    /// Add a single card onto the discard pile.
    pub fn add_to_discard_pile(&mut self, card: Card) {
        self.discard_pile.push(card);
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

/// Used solely for (de)serialization purposes.
#[derive(Serialize, Deserialize)]
struct SerializableDeck {
    pub config: DeckConfig,
    pub stock: Vec<CardData>,
    pub discard_pile: Vec<CardData>,
}

// We serialize to `SerializableDeck`...
impl Serialize for Deck {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let deck_data = SerializableDeck {
            config: (*self.config).clone(), 
            stock: self.stock.iter().map(|card| card.data()).collect(),
            discard_pile: self.discard_pile.iter().map(|card| card.data()).collect(),
        };
        deck_data.serialize(serializer)
    }
}

// And from it!
impl<'de> Deserialize<'de> for Deck {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let deck_data = SerializableDeck::deserialize(deserializer)?;
        let config = Arc::new(deck_data.config);
        
        let make_card = |card_data: CardData| Card {
            rank: card_data.rank,
            suit: card_data.suit,
            deck_config: Arc::clone(&config),
        };

        let stock = deck_data.stock.into_iter().map(make_card).collect(); 
        let discard_pile = deck_data.discard_pile.into_iter().map(make_card).collect();
        
        Ok(Deck {
            config,
            stock,
            discard_pile,
        })
    }
}