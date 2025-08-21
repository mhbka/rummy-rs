use super::*;

#[test]
fn test_set_properties() {
    let cfg = basic_config();
    let mut cards = create_set_cards(Rank::King, &[Suit::Clubs, Suit::Diamonds, Suit::Hearts], cfg.clone());
    let indices = vec![0, 1, 2];
    
    let set = Set::new(&mut cards, &indices).unwrap();
    
    assert_eq!(set.rank(), Rank::King);
    assert_eq!(set.cards().len(), 3);
    
    // Verify all cards in the set have the correct rank
    for card in set.cards() {
        assert!(card.rank == Rank::King || card.is_wildcard());
    }
}

#[test]
fn test_run_properties() {
    let cfg = basic_config();
    let mut cards = vec![
        create_card(Rank::Seven, Suit::Hearts, cfg.clone()),
        create_card(Rank::Eight, Suit::Hearts, cfg.clone()),
        create_card(Rank::Nine, Suit::Hearts, cfg.clone()),
    ];
    let indices = vec![0, 1, 2];
    
    let run = Run::new(&mut cards, &indices).unwrap();
    
    assert_eq!(run.suit(), Suit::Hearts);
    assert_eq!(run.cards().len(), 3);
    
    // Verify all cards in the run have the correct suit (excluding wildcards)
    for card in run.cards() {
        assert!(card.suit == Suit::Hearts || card.is_wildcard());
    }
}

#[test]
fn test_meld_type_identification() {
    let cfg = basic_config();
    
    // Test set identification
    let mut set_cards = create_set_cards(Rank::Ten, &[Suit::Clubs, Suit::Diamonds, Suit::Hearts], cfg.clone());
    let indices = vec![0, 1, 2];
    let set_meld = Meld::new(&mut set_cards, &indices).unwrap();
    
    assert!(set_meld.is_set());
    assert!(!set_meld.is_run());
    
    // Test run identification
    let mut run_cards = vec![
        create_card(Rank::Jack, Suit::Spades, cfg.clone()),
        create_card(Rank::Queen, Suit::Spades, cfg.clone()),
        create_card(Rank::King, Suit::Spades, cfg.clone()),
    ];
    let indices = vec![0, 1, 2];
    let run_meld = Meld::new(&mut run_cards, &indices).unwrap();
    
    assert!(run_meld.is_run());
    assert!(!run_meld.is_set());
}

#[test]
fn test_set_cards_immutable_reference() {
    let cfg = basic_config();
    let mut cards = create_set_cards(Rank::Nine, &[Suit::Clubs, Suit::Diamonds, Suit::Hearts], cfg.clone());
    let indices = vec![0, 1, 2];
    
    let set = Set::new(&mut cards, &indices).unwrap();
    let set_cards = set.cards();
    
    // Verify we get an immutable reference
    assert_eq!(set_cards.len(), 3);
    // This shouldn't compile if we tried: set_cards.push(...)
}

#[test]
fn test_run_cards_immutable_reference() {
    let cfg = basic_config();
    let mut cards = vec![
        create_card(Rank::Three, Suit::Diamonds, cfg.clone()),
        create_card(Rank::Four, Suit::Diamonds, cfg.clone()),
        create_card(Rank::Five, Suit::Diamonds, cfg.clone()),
    ];
    let indices = vec![0, 1, 2];
    
    let run = Run::new(&mut cards, &indices).unwrap();
    let run_cards = run.cards();
    
    // Verify we get an immutable reference
    assert_eq!(run_cards.len(), 3);
    // This shouldn't compile if we tried: run_cards.push(...)
}

#[test]
fn test_meld_preserves_card_properties() {
    let cfg = basic_config();
    let mut cards = vec![
        create_card(Rank::Ace, Suit::Clubs, cfg.clone()),
        create_card(Rank::Ace, Suit::Diamonds, cfg.clone()),
        create_card(Rank::Ace, Suit::Hearts, cfg.clone()),
    ];
    let indices = vec![0, 1, 2];
    
    // Store original card properties
    let original_suits: Vec<Suit> = cards.iter().map(|c| c.suit).collect();
    
    let set = Set::new(&mut cards, &indices).unwrap();
    
    // Verify cards in meld maintain their original properties
    let meld_suits: Vec<Suit> = set.cards().iter().map(|c| c.suit).collect();
    
    // Note: order might be different due to removal/insertion logic
    for suit in original_suits {
        assert!(meld_suits.contains(&suit));
    }
}

#[test]
fn test_layoff_increases_meld_size() {
    let cfg = basic_config();
    let mut cards = create_set_cards(Rank::Seven, &[Suit::Clubs, Suit::Diamonds, Suit::Hearts], cfg.clone());
    let indices = vec![0, 1, 2];
    let mut set = Set::new(&mut cards, &indices).unwrap();
    
    let original_size = set.cards().len();
    
    let mut layoff_hand = vec![create_card(Rank::Seven, Suit::Spades, cfg.clone())];
    set.layoff_card(&mut layoff_hand, 0).unwrap();
    
    assert_eq!(set.cards().len(), original_size + 1);
}

#[test]
fn test_meld_debug_representation() {
    let cfg = basic_config();
    let mut cards = create_set_cards(Rank::Queen, &[Suit::Clubs, Suit::Diamonds, Suit::Hearts], cfg.clone());
    let indices = vec![0, 1, 2];
    
    let meld = Meld::new(&mut cards, &indices).unwrap();
    
    // Verify that Debug is implemented (this will compile-fail if not)
    let debug_str = format!("{:?}", meld);
    assert!(!debug_str.is_empty());
}