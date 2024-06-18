use std::collections::HashSet;
#[cfg(test)]

use std::rc::Rc;
use super::super::deck::DeckConfig;
use super::super::{
    card::Card, 
    deck::Deck, 
    meld::{Run, Set, Meld, Meldable},
    suit_rank::{Suit, Rank}
};

#[test]
fn invalid_run_less_than_3_cards() {
    let cfg = Rc::new(DeckConfig::new());
    let mut cards = vec![
        Card { rank: Rank::Ace, suit: Suit::Clubs, deck_config: cfg.clone() },
        Card { rank: Rank::Two, suit: Suit::Clubs, deck_config: cfg.clone() }
    ];
    let mut indices = vec![0, 1];
    assert!(Run::new(&mut cards, &mut indices).is_err());
}

#[test]
fn invalid_run_different_suits() {
    let cfg = Rc::new(DeckConfig::new());
    let mut cards = vec![
        Card { rank: Rank::Ace, suit: Suit::Clubs, deck_config: cfg.clone() },
        Card { rank: Rank::Two, suit: Suit::Clubs, deck_config: cfg.clone() },
        Card { rank: Rank::Three, suit: Suit::Spades, deck_config: cfg.clone() }
    ];
    let mut indices = vec![0, 1, 2];
    assert!(Run::new(&mut cards, &mut indices).is_err());
}

#[test]
fn invalid_run_invalid_indices() {
    let cfg = Rc::new(DeckConfig::new());
    let mut cards = vec![
        Card { rank: Rank::Ace, suit: Suit::Clubs, deck_config: cfg.clone() },
        Card { rank: Rank::Two, suit: Suit::Clubs, deck_config: cfg.clone() },
        Card { rank: Rank::Three, suit: Suit::Clubs, deck_config: cfg.clone() }
    ];
    let mut indices = vec![0, 1, 2, 3]; // 3 does not exist as index
    assert!(Run::new(&mut cards, &mut indices).is_err());
}

#[test]
fn invalid_run_high_rank() {
    let mut high_rank_cfg = DeckConfig::new();
    high_rank_cfg.high_rank = Some(Rank::Two);
    let high_rank_cfg = Rc::new(high_rank_cfg);
    let mut cards = vec![
        Card { rank: Rank::King, suit: Suit::Clubs, deck_config: high_rank_cfg.clone() },
        Card { rank: Rank::Ace, suit: Suit::Clubs, deck_config: high_rank_cfg.clone() },
        Card { rank: Rank::Two, suit: Suit::Clubs, deck_config: high_rank_cfg.clone() },
        Card { rank: Rank::Three, suit: Suit::Clubs, deck_config: high_rank_cfg.clone() },
    ];
    let mut indices = vec![0, 1, 2, 3];
    assert!(Run::new(&mut cards, &mut indices).is_err()); // Two now highest, so Three is no longer valid as a consecutive rank
}


#[test]
fn valid_run() {
    let cfg = Rc::new(DeckConfig::new());
    let mut cards = vec![
        Card { rank: Rank::Ace, suit: Suit::Clubs, deck_config: cfg.clone() },
        Card { rank: Rank::Two, suit: Suit::Clubs, deck_config: cfg.clone() },
        Card { rank: Rank::Three, suit: Suit::Clubs, deck_config: cfg.clone() }
    ];
    let backup_cards = cards.clone();
    let mut indices = vec![0, 1, 2];
    let run = Run::new(&mut cards, &mut indices);

    assert!(cards.len() == 0);
    assert!(run.is_ok());
    assert!(run.unwrap().cards() == &backup_cards);
}

#[test]
fn valid_run_wrong_order_indices() {
    let cfg = Rc::new(DeckConfig::new());
    let mut cards = vec![
        Card { rank: Rank::Ace, suit: Suit::Clubs, deck_config: cfg.clone() },
        Card { rank: Rank::Two, suit: Suit::Clubs, deck_config: cfg.clone() },
        Card { rank: Rank::Three, suit: Suit::Clubs, deck_config: cfg.clone() }
    ];
    let backup_cards = cards.clone();
    let mut indices = vec![2, 0, 1]; // in the wrong order
    let run = Run::new(&mut cards, &mut indices);

    assert!(cards.len() == 0);
    assert!(run.is_ok());
    assert!(run.unwrap().cards() == &backup_cards);
}

#[test]
fn valid_run_wildcard() {
    let mut cfg = DeckConfig::new();
    cfg.wildcard_rank = Some(Rank::Jack);
    let cfg = Rc::new(cfg);
    let mut cards = vec![
        Card { rank: Rank::Ace, suit: Suit::Clubs, deck_config: cfg.clone() },
        Card { rank: Rank::Jack, suit: Suit::Clubs, deck_config: cfg.clone() }, // the wildcard
        Card { rank: Rank::Two, suit: Suit::Clubs, deck_config: cfg.clone() },
    ];
    let mut backup_cards = cards.clone();
    backup_cards.sort();
    let mut indices = vec![0, 1, 2];
    let run = Run::new(&mut cards, &mut indices);

    assert!(cards.len() == 0);
    assert!(run.is_ok());
    assert!(run.unwrap().cards() == &backup_cards);
}


#[test]
fn valid_run_high_rank() {
    let mut high_rank_cfg = DeckConfig::new();
    high_rank_cfg.high_rank = Some(Rank::Two); // high rank is Two...
    let high_rank_cfg = Rc::new(high_rank_cfg);
    let mut cards = vec![ // ... so this should be valid with King being the lowest rank.
        Card { rank: Rank::King, suit: Suit::Clubs, deck_config: high_rank_cfg.clone() },
        Card { rank: Rank::Ace, suit: Suit::Clubs, deck_config: high_rank_cfg.clone() },
        Card { rank: Rank::Two, suit: Suit::Clubs, deck_config: high_rank_cfg.clone() },
    ]; 
    let backup_cards = cards.clone();
    let mut indices = vec![0, 1, 2];
    let run = Run::new(&mut cards, &mut indices);

    assert!(cards.len() == 0);
    assert!(run.is_ok());
    assert!(run.unwrap().cards() == &backup_cards);
}

#[test]
fn invalid_layoff_run() {
    let cfg = Rc::new(DeckConfig::new());
    let cards = vec![
        Card { rank: Rank::Ace, suit: Suit::Clubs, deck_config: cfg.clone() },
        Card { rank: Rank::Two, suit: Suit::Clubs, deck_config: cfg.clone() },
        Card { rank: Rank::Three, suit: Suit::Clubs, deck_config: cfg.clone() }
    ];
    let mut indices = vec![0, 1, 2];
    let mut run = Run::new(&mut cards.clone(), &mut indices).unwrap();
    let mut layoff_cards = vec![
        Card { rank: Rank::Five, suit: Suit::Clubs, deck_config: cfg.clone() }, // not consecutive rank
        Card { rank: Rank::Four, suit: Suit::Spades, deck_config: cfg.clone() } // wrong suit
    ];

    assert!(run.layoff_card(&mut layoff_cards, 0).is_err());
    assert!(run.layoff_card(&mut layoff_cards, 1).is_err());
}

#[test]
fn valid_layoff_run() {
    let cfg = Rc::new(DeckConfig::new());
    let cards = vec![
        Card { rank: Rank::Two, suit: Suit::Clubs, deck_config: cfg.clone() },
        Card { rank: Rank::Three, suit: Suit::Clubs, deck_config: cfg.clone() },
        Card { rank: Rank::Four, suit: Suit::Clubs, deck_config: cfg.clone() },
        Card { rank: Rank::Five, suit: Suit::Clubs, deck_config: cfg.clone() },
    ];
    let mut indices = vec![0, 1, 2, 3];
    let mut run = Run::new(&mut cards.clone(), &mut indices).unwrap();
    let mut layoff_cards = vec![
        Card { rank: Rank::Ace, suit: Suit::Clubs, deck_config: cfg.clone() }, // at bottom of meld,
        Card { rank: Rank::Six, suit: Suit::Clubs, deck_config: cfg.clone() } // and top of meld 
    ];

    assert!(run.layoff_card(&mut layoff_cards, 0).is_ok());
    assert!(run.layoff_card(&mut layoff_cards, 0).is_ok());
    assert!(layoff_cards.len() == 0);
    assert!(run.cards() == &vec![
        Card { rank: Rank::Ace, suit: Suit::Clubs, deck_config: cfg.clone() },
        Card { rank: Rank::Two, suit: Suit::Clubs, deck_config: cfg.clone() },
        Card { rank: Rank::Three, suit: Suit::Clubs, deck_config: cfg.clone() },
        Card { rank: Rank::Four, suit: Suit::Clubs, deck_config: cfg.clone() },
        Card { rank: Rank::Five, suit: Suit::Clubs, deck_config: cfg.clone() },
        Card { rank: Rank::Six, suit: Suit::Clubs, deck_config: cfg.clone() }
    ]);
}   

#[test]
fn add_wildcard_layoff_run() {
    let mut cfg = DeckConfig::new();
    cfg.wildcard_rank = Some(Rank::Joker);
    let cfg = Rc::new(cfg);

    let cards = vec![
        Card { rank: Rank::Two, suit: Suit::Clubs, deck_config: cfg.clone() },
        Card { rank: Rank::Three, suit: Suit::Clubs, deck_config: cfg.clone() },
        Card { rank: Rank::Four, suit: Suit::Clubs, deck_config: cfg.clone() },
    ];
    let mut indices = vec![0, 1, 2];
    let mut run = Run::new(&mut cards.clone(), &mut indices).unwrap();
    let mut card = vec![Card { rank: Rank::Joker, suit: Suit::Joker, deck_config: cfg.clone() }];


    assert!(run.layoff_card(&mut card, 0).is_ok());
    assert!(card.len() == 0);
    assert!(run.cards() == &vec![
        Card { rank: Rank::Two, suit: Suit::Clubs, deck_config: cfg.clone() },
        Card { rank: Rank::Three, suit: Suit::Clubs, deck_config: cfg.clone() },
        Card { rank: Rank::Four, suit: Suit::Clubs, deck_config: cfg.clone() },
        Card { rank: Rank::Joker, suit: Suit::Joker, deck_config: cfg.clone() }
    ]);
}

#[test]
fn replace_wildcard_layoff_run() {
    let mut cfg = DeckConfig::new();
    cfg.wildcard_rank = Some(Rank::Joker);
    let cfg = Rc::new(cfg);

    let cards = vec![
        Card { rank: Rank::Two, suit: Suit::Clubs, deck_config: cfg.clone() },
        Card { rank: Rank::Three, suit: Suit::Clubs, deck_config: cfg.clone() },
        Card { rank: Rank::Joker, suit: Suit::Joker, deck_config: cfg.clone() }, // replace this with actual card
        Card { rank: Rank::Five, suit: Suit::Clubs, deck_config: cfg.clone() },
    ];
    let mut indices = vec![0, 1, 2, 3];
    let mut run = Run::new(&mut cards.clone(), &mut indices).unwrap();
    let mut card = vec![Card { rank: Rank::Four, suit: Suit::Clubs, deck_config: cfg.clone() }];

    assert!(run.layoff_card(&mut card, 0).is_ok());
    assert!(card == vec![ Card { rank: Rank::Joker, suit: Suit::Joker, deck_config: cfg.clone() }]);
    assert!(run.cards() == &vec![
        Card { rank: Rank::Two, suit: Suit::Clubs, deck_config: cfg.clone() },
        Card { rank: Rank::Three, suit: Suit::Clubs, deck_config: cfg.clone() },
        Card { rank: Rank::Four, suit: Suit::Clubs, deck_config: cfg.clone() }, // replace this with actual card
        Card { rank: Rank::Five, suit: Suit::Clubs, deck_config: cfg.clone() },
    ]);
}