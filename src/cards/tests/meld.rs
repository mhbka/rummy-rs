#[cfg(test)]
use std::rc::Rc;
use super::super::deck::DeckConfig;
use super::super::{
    card::Card, 
    deck::Deck, 
    meld::{Set, Meld, Meldable},
    suit_rank::{Suit, Rank}
};


#[test]
fn invalid_meld_set() {
    let cfg = Rc::new(DeckConfig::new());
    let mut cards = vec![
        Card { rank: Rank::Ace, suit: Suit::Clubs, deck_config: cfg.clone() },
        Card { rank: Rank::Ace, suit: Suit::Spades, deck_config: cfg.clone() },
        Card { rank: Rank::Two, suit: Suit::Clubs, deck_config: cfg.clone() },
    ];
    let mut indices = vec![0, 1, 2];
    assert!(Meld::new(&mut cards, &mut indices).is_err());
}

#[test]
fn valid_meld_set() {
    let cfg = Rc::new(DeckConfig::new());
    let cards = vec![
        Card { rank: Rank::Ace, suit: Suit::Clubs, deck_config: cfg.clone() },
        Card { rank: Rank::Ace, suit: Suit::Diamonds, deck_config: cfg.clone() },
        Card { rank: Rank::Ace, suit: Suit::Hearts, deck_config: cfg.clone() },
    ];
    let mut indices = vec![0, 1, 2];
    let meld = Meld::new(&mut cards.clone(), &mut indices);
    assert!(meld.is_ok());
    assert!(meld.unwrap().is_set());
}

#[test]
fn invalid_layoff_meld_set() {
    let cfg = Rc::new(DeckConfig::new());
    let cards = vec![
        Card { rank: Rank::Ace, suit: Suit::Clubs, deck_config: cfg.clone() },
        Card { rank: Rank::Ace, suit: Suit::Diamonds, deck_config: cfg.clone() },
        Card { rank: Rank::Ace, suit: Suit::Hearts, deck_config: cfg.clone() },
    ];
    let mut indices = vec![0, 1, 2];
    let mut meld = Meld::new(&mut cards.clone(), &mut indices).unwrap();
    let mut layoff_card = vec![Card { rank: Rank::Two, suit: Suit::Clubs, deck_config: cfg.clone() }];

    assert!(meld.layoff_card(&mut layoff_card, 0).is_err());
}

#[test]
fn valid_layoff_meld_set() {
    let cfg = Rc::new(DeckConfig::new());
    let cards = vec![
        Card { rank: Rank::Ace, suit: Suit::Clubs, deck_config: cfg.clone() },
        Card { rank: Rank::Ace, suit: Suit::Diamonds, deck_config: cfg.clone() },
        Card { rank: Rank::Ace, suit: Suit::Hearts, deck_config: cfg.clone() },
    ];
    let mut indices = vec![0, 1, 2];
    let mut meld = Set::new(&mut cards.clone(), &mut indices).unwrap();
    let mut layoff_card = vec![Card { rank: Rank::Ace, suit: Suit::Spades, deck_config: cfg.clone() }];
    assert!(meld.layoff_card(&mut layoff_card, 0).is_ok());
}

#[test]
fn invalid_meld_run() {
    let cfg = Rc::new(DeckConfig::new());
    let mut cards = vec![
        Card { rank: Rank::Ace, suit: Suit::Clubs, deck_config: cfg.clone() },
        Card { rank: Rank::Two, suit: Suit::Clubs, deck_config: cfg.clone() },
        Card { rank: Rank::Three, suit: Suit::Spades, deck_config: cfg.clone() }
    ];
    let mut indices = vec![0, 1, 2];
    assert!(Meld::new(&mut cards, &mut indices).is_err());
}

#[test]
fn valid_meld_run() {
    let cfg = Rc::new(DeckConfig::new());
    let cards = vec![
        Card { rank: Rank::Ace, suit: Suit::Clubs, deck_config: cfg.clone() },
        Card { rank: Rank::Two, suit: Suit::Clubs, deck_config: cfg.clone() },
        Card { rank: Rank::Three, suit: Suit::Clubs, deck_config: cfg.clone() }
    ];
    let mut indices = vec![0, 1, 2];
    let meld = Meld::new(&mut cards.clone(), &mut indices);
    assert!(meld.is_ok());
    assert!(meld.unwrap().is_run());
}

#[test]
fn invalid_layoff_meld_run() {
    let cfg = Rc::new(DeckConfig::new());
    let cards = vec![
        Card { rank: Rank::Ace, suit: Suit::Clubs, deck_config: cfg.clone() },
        Card { rank: Rank::Two, suit: Suit::Clubs, deck_config: cfg.clone() },
        Card { rank: Rank::Three, suit: Suit::Clubs, deck_config: cfg.clone() }
    ];
    let mut indices = vec![0, 1, 2];
    let mut meld = Meld::new(&mut cards.clone(), &mut indices).unwrap();
    let mut layoff_cards = vec![
        Card { rank: Rank::Five, suit: Suit::Clubs, deck_config: cfg.clone() }, // not consecutive rank
        Card { rank: Rank::Four, suit: Suit::Spades, deck_config: cfg.clone() } // wrong suit
    ];

    assert!(meld.layoff_card(&mut layoff_cards, 0).is_err());
    assert!(meld.layoff_card(&mut layoff_cards, 1).is_err());
}

#[test]
fn valid_layoff_meld_run() {
    let cfg = Rc::new(DeckConfig::new());
    let cards = vec![
        Card { rank: Rank::Two, suit: Suit::Clubs, deck_config: cfg.clone() },
        Card { rank: Rank::Three, suit: Suit::Clubs, deck_config: cfg.clone() },
        Card { rank: Rank::Four, suit: Suit::Clubs, deck_config: cfg.clone() },
        Card { rank: Rank::Five, suit: Suit::Clubs, deck_config: cfg.clone() },
    ];
    let mut indices = vec![0, 1, 2, 3];
    let mut meld = Meld::new(&mut cards.clone(), &mut indices).unwrap();

    let mut layoff_cards = vec![
        Card { rank: Rank::Ace, suit: Suit::Clubs, deck_config: cfg.clone() }, // at bottom of meld,
        Card { rank: Rank::Six, suit: Suit::Clubs, deck_config: cfg.clone() } // and top of meld 
    ];
    assert!(meld.layoff_card(&mut layoff_cards, 0).is_ok());
    assert!(meld.layoff_card(&mut layoff_cards, 0).is_ok());
}