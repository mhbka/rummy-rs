use super::super::deck::DeckConfig;
use super::super::{
    card::Card,
    deck::Deck,
    meld::{Meld, Meldable, Set},
    suit_rank::{Rank, Suit},
};
#[cfg(test)]
use std::collections::HashSet;
use std::sync::Arc;

#[test]
fn invalid_set_less_than_3_cards() {
    let cfg = Arc::new(DeckConfig::new());
    let mut cards = vec![
        Card {
            rank: Rank::Ace,
            suit: Suit::Clubs,
            deck_config: cfg.clone(),
        },
        Card {
            rank: Rank::Ace,
            suit: Suit::Spades,
            deck_config: cfg.clone(),
        },
    ];
    let mut indices = vec![0, 1];
    assert!(Set::new(&mut cards, &mut indices).is_err());
}

#[test]
fn invalid_set_different_ranks() {
    let cfg = Arc::new(DeckConfig::new());
    let mut cards = vec![
        Card {
            rank: Rank::Ace,
            suit: Suit::Clubs,
            deck_config: cfg.clone(),
        },
        Card {
            rank: Rank::Ace,
            suit: Suit::Spades,
            deck_config: cfg.clone(),
        },
        Card {
            rank: Rank::Two,
            suit: Suit::Clubs,
            deck_config: cfg.clone(),
        },
    ];
    let mut indices = vec![0, 1, 2];
    assert!(Set::new(&mut cards, &mut indices).is_err());
}

#[test]
fn invalid_set_invalid_indices() {
    let cfg = Arc::new(DeckConfig::new());
    let mut cards = vec![
        Card {
            rank: Rank::Ace,
            suit: Suit::Clubs,
            deck_config: cfg.clone(),
        },
        Card {
            rank: Rank::Ace,
            suit: Suit::Diamonds,
            deck_config: cfg.clone(),
        },
        Card {
            rank: Rank::Ace,
            suit: Suit::Hearts,
            deck_config: cfg.clone(),
        },
    ];
    let mut indices = vec![0, 1, 2, 3]; // 3 does not exist as index
    assert!(Set::new(&mut cards, &mut indices).is_err());
}

#[test]
fn valid_set() {
    let cfg = Arc::new(DeckConfig::new());
    let mut cards = vec![
        Card {
            rank: Rank::Ace,
            suit: Suit::Clubs,
            deck_config: cfg.clone(),
        },
        Card {
            rank: Rank::Ace,
            suit: Suit::Diamonds,
            deck_config: cfg.clone(),
        },
        Card {
            rank: Rank::Ace,
            suit: Suit::Hearts,
            deck_config: cfg.clone(),
        },
    ];
    let backup_cards = cards.clone();
    let mut indices = vec![0, 1, 2];
    let set = Set::new(&mut cards, &mut indices);

    assert!(cards.len() == 0);
    assert!(set.is_ok());
    assert!(set.unwrap().cards() == &backup_cards);
}

#[test]
fn valid_set_wrong_order_indices() {
    let cfg = Arc::new(DeckConfig::new());
    let mut cards = vec![
        Card {
            rank: Rank::Ace,
            suit: Suit::Clubs,
            deck_config: cfg.clone(),
        },
        Card {
            rank: Rank::Ace,
            suit: Suit::Diamonds,
            deck_config: cfg.clone(),
        },
        Card {
            rank: Rank::Ace,
            suit: Suit::Hearts,
            deck_config: cfg.clone(),
        },
        Card {
            rank: Rank::Ace,
            suit: Suit::Spades,
            deck_config: cfg.clone(),
        },
    ];
    let backup_cards = cards.clone();
    let backup_cards_set = backup_cards.iter().collect();
    let mut indices = vec![2, 0, 3, 1]; // in the wrong order
    let set = Set::new(&mut cards, &mut indices);

    assert!(cards.len() == 0);
    assert!(set.is_ok());
    assert!(set.unwrap().cards().iter().collect::<HashSet<_>>() == backup_cards_set);
    // since order is different from in `cards`, just check existence of elements
}

#[test]
fn valid_set_wildcard() {
    let mut cfg = DeckConfig::new();
    cfg.wildcard_rank = Some(Rank::Five);
    let cfg = Arc::new(cfg);
    let mut cards = vec![
        Card {
            rank: Rank::Ace,
            suit: Suit::Clubs,
            deck_config: cfg.clone(),
        },
        Card {
            rank: Rank::Ace,
            suit: Suit::Diamonds,
            deck_config: cfg.clone(),
        },
        Card {
            rank: Rank::Five,
            suit: Suit::Hearts,
            deck_config: cfg.clone(),
        }, // the wildcard
    ];
    let backup_cards = cards.clone();
    let mut indices = vec![0, 1, 2];
    let set = Set::new(&mut cards, &mut indices);

    assert!(set.is_ok());
    assert!(cards.len() == 0);
    assert!(set.unwrap().cards() == &backup_cards);
}

#[test]
fn invalid_layoff_set() {
    let cfg = Arc::new(DeckConfig::new());
    let cards = vec![
        Card {
            rank: Rank::Ace,
            suit: Suit::Clubs,
            deck_config: cfg.clone(),
        },
        Card {
            rank: Rank::Ace,
            suit: Suit::Diamonds,
            deck_config: cfg.clone(),
        },
        Card {
            rank: Rank::Ace,
            suit: Suit::Hearts,
            deck_config: cfg.clone(),
        },
    ];
    let mut indices = vec![0, 1, 2];
    let mut set = Set::new(&mut cards.clone(), &mut indices).unwrap();
    let mut layoff_card = vec![Card {
        rank: Rank::Two,
        suit: Suit::Clubs,
        deck_config: cfg.clone(),
    }];

    assert!(set.layoff_card(&mut layoff_card, 0).is_err());
}

#[test]
fn valid_layoff_set() {
    let cfg = Arc::new(DeckConfig::new());
    let cards = vec![
        Card {
            rank: Rank::Ace,
            suit: Suit::Clubs,
            deck_config: cfg.clone(),
        },
        Card {
            rank: Rank::Ace,
            suit: Suit::Diamonds,
            deck_config: cfg.clone(),
        },
        Card {
            rank: Rank::Ace,
            suit: Suit::Hearts,
            deck_config: cfg.clone(),
        },
    ];
    let mut indices = vec![0, 1, 2];
    let mut set = Set::new(&mut cards.clone(), &mut indices).unwrap();
    let mut layoff_card = vec![Card {
        rank: Rank::Ace,
        suit: Suit::Spades,
        deck_config: cfg.clone(),
    }];

    assert!(set.layoff_card(&mut layoff_card, 0).is_ok());
    assert!(layoff_card.len() == 0);
    assert!(
        set.cards()
            == &vec![
                Card {
                    rank: Rank::Ace,
                    suit: Suit::Clubs,
                    deck_config: cfg.clone()
                },
                Card {
                    rank: Rank::Ace,
                    suit: Suit::Diamonds,
                    deck_config: cfg.clone()
                },
                Card {
                    rank: Rank::Ace,
                    suit: Suit::Hearts,
                    deck_config: cfg.clone()
                },
                Card {
                    rank: Rank::Ace,
                    suit: Suit::Spades,
                    deck_config: cfg.clone()
                }
            ]
    );
}

#[test]
fn valid_layoff_add_wildcard_set() {
    let mut cfg = DeckConfig::new();
    cfg.wildcard_rank = Some(Rank::Joker);
    let cfg = Arc::new(cfg);

    let cards = vec![
        Card {
            rank: Rank::Ace,
            suit: Suit::Clubs,
            deck_config: cfg.clone(),
        },
        Card {
            rank: Rank::Ace,
            suit: Suit::Diamonds,
            deck_config: cfg.clone(),
        },
        Card {
            rank: Rank::Ace,
            suit: Suit::Hearts,
            deck_config: cfg.clone(),
        },
    ];
    let mut indices = vec![0, 1, 2];
    let mut set = Set::new(&mut cards.clone(), &mut indices).unwrap();
    let mut card = vec![Card {
        rank: Rank::Joker,
        suit: Suit::Joker,
        deck_config: cfg.clone(),
    }];

    assert!(set.layoff_card(&mut card, 0).is_ok());
    assert!(card.len() == 0);
    assert!(
        set.cards()
            == &vec![
                Card {
                    rank: Rank::Ace,
                    suit: Suit::Clubs,
                    deck_config: cfg.clone()
                },
                Card {
                    rank: Rank::Ace,
                    suit: Suit::Diamonds,
                    deck_config: cfg.clone()
                },
                Card {
                    rank: Rank::Ace,
                    suit: Suit::Hearts,
                    deck_config: cfg.clone()
                },
                Card {
                    rank: Rank::Joker,
                    suit: Suit::Joker,
                    deck_config: cfg.clone()
                }
            ]
    );
}

#[test]
fn valid_layoff_replace_wildcard_set() {
    let mut cfg = DeckConfig::new();
    cfg.wildcard_rank = Some(Rank::Joker);
    let cfg = Arc::new(cfg);

    let cards = vec![
        Card {
            rank: Rank::Ace,
            suit: Suit::Clubs,
            deck_config: cfg.clone(),
        },
        Card {
            rank: Rank::Ace,
            suit: Suit::Diamonds,
            deck_config: cfg.clone(),
        },
        Card {
            rank: Rank::Ace,
            suit: Suit::Hearts,
            deck_config: cfg.clone(),
        },
        Card {
            rank: Rank::Joker,
            suit: Suit::Joker,
            deck_config: cfg.clone(),
        }, // replace this in the layoff
    ];
    let mut indices = vec![0, 1, 2, 3];
    let mut set = Set::new(&mut cards.clone(), &mut indices).unwrap();
    let mut card = vec![Card {
        rank: Rank::Ace,
        suit: Suit::Spades,
        deck_config: cfg.clone(),
    }];

    assert!(set.layoff_card(&mut card, 0).is_ok());
    assert!(
        card == vec![Card {
            rank: Rank::Joker,
            suit: Suit::Joker,
            deck_config: cfg.clone()
        }]
    );
    assert!(
        set.cards()
            == &vec![
                Card {
                    rank: Rank::Ace,
                    suit: Suit::Clubs,
                    deck_config: cfg.clone()
                },
                Card {
                    rank: Rank::Ace,
                    suit: Suit::Diamonds,
                    deck_config: cfg.clone()
                },
                Card {
                    rank: Rank::Ace,
                    suit: Suit::Hearts,
                    deck_config: cfg.clone()
                },
                Card {
                    rank: Rank::Ace,
                    suit: Suit::Spades,
                    deck_config: cfg.clone()
                }
            ]
    );
}

#[test]
fn valid_layoff_same_cards_set() {
    let mut cfg = DeckConfig::new();
    cfg.wildcard_rank = Some(Rank::Joker);
    let cfg = Arc::new(cfg);

    let cards = vec![
        Card {
            rank: Rank::Ace,
            suit: Suit::Clubs,
            deck_config: cfg.clone(),
        },
        Card {
            rank: Rank::Ace,
            suit: Suit::Diamonds,
            deck_config: cfg.clone(),
        },
        Card {
            rank: Rank::Ace,
            suit: Suit::Hearts,
            deck_config: cfg.clone(),
        },
        Card {
            rank: Rank::Ace,
            suit: Suit::Spades,
            deck_config: cfg.clone(),
        },
    ];
    let mut layoff_cards = cards.clone();
    let mut indices = vec![0, 1, 2, 3];
    let mut set = Set::new(&mut cards.clone(), &mut indices).unwrap();
    let mut set_cards = cards.clone();
    set_cards.append(&mut cards.clone());

    assert!(set.layoff_card(&mut layoff_cards, 0).is_ok()); // should be ok to layoff the same card (ie, if using >1 pack in the deck)
    assert!(set.layoff_card(&mut layoff_cards, 0).is_ok());
    assert!(set.layoff_card(&mut layoff_cards, 0).is_ok());
    assert!(set.layoff_card(&mut layoff_cards, 0).is_ok());
    assert!(layoff_cards.len() == 0);
    assert!(set.cards() == &set_cards);
}
