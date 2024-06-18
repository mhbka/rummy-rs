#[cfg(test)]

use crate::cards::suit_rank::Rank;
use crate::cards::suit_rank::Suit;

use super::super::deck::DeckConfig;
use super::super::deck::Deck;

#[test]
fn default_deck_config() {
    let cfg = DeckConfig {
        shuffle_seed: None,
        pack_count: 1,
        high_rank: None,
        wildcard_rank: None,
    };

    let default_cfg = DeckConfig::new();
    assert_eq!(cfg, default_cfg);
}


#[test]
fn normal_deck() {
    let cfg = DeckConfig::new();
    let deck = Deck::new(cfg.clone());
    assert_eq!(deck.stock().len(), 52);
    assert_eq!(deck.discard_pile().len(), 0);
}

#[test]
fn joker_deck() {
    let mut cfg = DeckConfig::new();
    cfg.wildcard_rank = Some(Rank::Joker);
    let joker_deck = Deck::new(cfg.clone());
    assert_eq!(joker_deck.stock().len(), 54);
}

#[test]
fn zero_pack_deck() {
    let mut cfg = DeckConfig::new();
    cfg.pack_count = 0; // should change to 1 when passed to a Deck::new()
    let zero_pack_deck = Deck::new(cfg.clone());
    assert_eq!(zero_pack_deck.config().pack_count, 1);
    assert_eq!(zero_pack_deck.stock().len(), 52);
}

#[test]
fn two_pack_deck() {
    let mut cfg = DeckConfig::new();
    cfg.pack_count = 2;
    let deck = Deck::new(cfg.clone());
    assert_eq!(deck.stock().len(), 104);
    assert_eq!(deck.discard_pile().len(), 0);

    cfg.wildcard_rank = Some(Rank::Joker);
    let joker_deck = Deck::new(cfg.clone());
    assert_eq!(joker_deck.stock().len(), 108);
}

#[test]
fn no_shuffle_deck() {
    let cfg = DeckConfig {
        shuffle_seed: Some(0),
        pack_count: 1,
        high_rank: None,
        wildcard_rank: None,
    };

    let deck = Deck::new(cfg.clone());
    assert!(deck.stock() // didn't shuffle, so cards should be in increasing order
        .windows(2)
        .all(|w| w[0] < w[1])
    );
}

#[test]
fn draw_and_discard_deck() {
    let mut deck = Deck::new(DeckConfig::new());

    // Drawing 1 card
    let mut card = deck.draw(1).unwrap();
    deck.add_to_discard_pile(&mut card);
    assert_eq!(deck.stock().len(), 51);
    assert_eq!(deck.discard_pile().len(), 1);

    // Drawing several cards
    let mut cards = deck.draw(51).unwrap();
    deck.add_to_discard_pile(&mut cards);
    assert_eq!(deck.stock().len(), 0);
    assert_eq!(deck.discard_pile().len(), 52);

    // Drawing from an empty stock
    assert!(deck.draw(1).is_err());
}

#[test]
fn draw_specific() {
    let mut deck = Deck::new(DeckConfig::new());
    assert!(deck.draw_specific(Rank::Ace, Suit::Clubs).is_ok());
    assert!(deck.draw_specific(Rank::Ace, Suit::Clubs).is_err());
}

#[test]
fn shuffle_discarded_deck() {
    let mut deck = Deck::new(DeckConfig::new());
    let mut cards = deck.draw(52).unwrap();
    deck.add_to_discard_pile(&mut cards);
    deck.shuffle_discarded();

    assert_eq!(deck.stock().len(), 52);
    assert_eq!(deck.discard_pile().len(), 0);
}

#[test]
fn turnover_discarded_deck() {
    let mut cfg = DeckConfig::new();
    cfg.shuffle_seed = Some(0); // set this seed so that we don't shuffle...
    let mut deck = Deck::new(cfg);
    let mut cards = deck.draw(52).unwrap();
    deck.add_to_discard_pile(&mut cards);
    deck.turnover_discarded();

    assert_eq!(deck.stock().len(), 52);
    assert_eq!(deck.discard_pile().len(), 0);
    assert!(deck.stock() // ... since we didn't shuffle, we can verify turnover by increasing order of cards
        .windows(2)
        .all(|w| w[0] > w[1])
    );
}
