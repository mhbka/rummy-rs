#[cfg(test)]    
mod tests {    
    use super::super::*;

    #[test]
    fn test_valid_set_layoff() {
        let cfg = basic_config();
        let mut cards = create_set_cards(Rank::Ace, &[Suit::Clubs, Suit::Diamonds, Suit::Hearts], cfg.clone());
        let indices = vec![0, 1, 2];
        let mut set = Set::new(&mut cards, &indices).unwrap();
        
        let mut layoff_card = vec![create_card(Rank::Ace, Suit::Spades, cfg.clone())];
        assert!(set.layoff_card(&mut layoff_card, 0).is_ok());
    }

    #[test]
    fn test_layoff_removes_card_from_hand() {
        let cfg = basic_config();
        let mut cards = create_set_cards(Rank::Ace, &[Suit::Clubs, Suit::Diamonds, Suit::Hearts], cfg.clone());
        let indices = vec![0, 1, 2];
        let mut set = Set::new(&mut cards, &indices).unwrap();
        
        let mut layoff_hand = vec![
            create_card(Rank::Ace, Suit::Spades, cfg.clone()),
            create_card(Rank::Two, Suit::Clubs, cfg.clone()),
        ];
        
        assert!(set.layoff_card(&mut layoff_hand, 0).is_ok());
        assert_eq!(layoff_hand.len(), 1); // Card should be removed
        assert_eq!(layoff_hand[0].rank, Rank::Two); // Only remaining card
        assert_eq!(set.cards().len(), 4); // Meld should have one more card
    }

    #[test]
    fn test_run_layoff_at_bottom() {
        let cfg = basic_config();
        let mut cards = vec![
            create_card(Rank::Five, Suit::Diamonds, cfg.clone()),
            create_card(Rank::Six, Suit::Diamonds, cfg.clone()),
            create_card(Rank::Seven, Suit::Diamonds, cfg.clone()),
        ];
        let indices = vec![0, 1, 2];
        let mut run = Run::new(&mut cards, &indices).unwrap();
        
        let mut layoff_hand = vec![
            create_card(Rank::Four, Suit::Diamonds, cfg.clone()),
        ];
        assert!(run.layoff_card(&mut layoff_hand, 0).is_ok());
        assert_eq!(run.cards().len(), 4);
    }

    #[test]
    fn test_run_layoff_at_top() {
        let cfg = basic_config();
        let mut cards = vec![
            create_card(Rank::Five, Suit::Diamonds, cfg.clone()),
            create_card(Rank::Six, Suit::Diamonds, cfg.clone()),
            create_card(Rank::Seven, Suit::Diamonds, cfg.clone()),
        ];
        let indices = vec![0, 1, 2];
        let mut run = Run::new(&mut cards, &indices).unwrap();
        
        let mut layoff_hand = vec![
            create_card(Rank::Eight, Suit::Diamonds, cfg.clone()),
        ];
        assert!(run.layoff_card(&mut layoff_hand, 0).is_ok());
        assert_eq!(run.cards().len(), 4);
    }

    #[test]
    fn test_run_layoff_at_both_ends() {
        let cfg = basic_config();
        let mut cards = vec![
            create_card(Rank::Five, Suit::Diamonds, cfg.clone()),
            create_card(Rank::Six, Suit::Diamonds, cfg.clone()),
            create_card(Rank::Seven, Suit::Diamonds, cfg.clone()),
        ];
        let indices = vec![0, 1, 2];
        let mut run = Run::new(&mut cards, &indices).unwrap();
        
        // Test laying off at the bottom
        let mut layoff_hand = vec![
            create_card(Rank::Four, Suit::Diamonds, cfg.clone()),
        ];
        assert!(run.layoff_card(&mut layoff_hand, 0).is_ok());
        assert_eq!(run.cards().len(), 4);
        
        // Test laying off at the top
        layoff_hand = vec![
            create_card(Rank::Eight, Suit::Diamonds, cfg.clone()),
        ];
        assert!(run.layoff_card(&mut layoff_hand, 0).is_ok());
        assert_eq!(run.cards().len(), 5);
    }

    #[test]
    fn test_multiple_layoffs_on_same_meld() {
        let cfg = basic_config();
        let mut cards = vec![
            create_card(Rank::Two, Suit::Clubs, cfg.clone()),
            create_card(Rank::Three, Suit::Clubs, cfg.clone()),
            create_card(Rank::Four, Suit::Clubs, cfg.clone()),
            create_card(Rank::Five, Suit::Clubs, cfg.clone()),
        ];
        let indices = vec![0, 1, 2, 3];
        let mut meld = Meld::new(&mut cards, &indices).unwrap();

        let mut layoff_cards = vec![
            create_card(Rank::Ace, Suit::Clubs, cfg.clone()), // at bottom
            create_card(Rank::Six, Suit::Clubs, cfg.clone()), // at top
        ];
        
        assert!(meld.layoff_card(&mut layoff_cards, 0).is_ok());
        assert_eq!(layoff_cards.len(), 1); // First card removed
        
        assert!(meld.layoff_card(&mut layoff_cards, 0).is_ok()); 
        assert_eq!(layoff_cards.len(), 0); // Second card removed
    }

    #[test]
    fn test_layoff_wrong_rank_on_set() {
        let cfg = basic_config();
        let mut cards = create_set_cards(Rank::King, &[Suit::Clubs, Suit::Diamonds, Suit::Hearts], cfg.clone());
        let indices = vec![0, 1, 2];
        let mut set = Set::new(&mut cards, &indices).unwrap();
        
        let mut layoff_hand = vec![
            create_card(Rank::Queen, Suit::Spades, cfg.clone()), // Wrong rank
        ];
        
        let result = set.layoff_card(&mut layoff_hand, 0);
        assert!(matches!(result, Err(MeldError::InvalidLayoff)));
        assert_eq!(layoff_hand.len(), 1); // Card should remain in hand
    }

    #[test]
    fn test_layoff_gap_in_run() {
        let cfg = basic_config();
        let mut cards = vec![
            create_card(Rank::Two, Suit::Spades, cfg.clone()),
            create_card(Rank::Three, Suit::Spades, cfg.clone()),
            create_card(Rank::Four, Suit::Spades, cfg.clone()),
        ];
        let indices = vec![0, 1, 2];
        let mut run = Run::new(&mut cards, &indices).unwrap();
        
        let mut layoff_hand = vec![
            create_card(Rank::Six, Suit::Spades, cfg.clone()), // Gap - should be 5
        ];
        
        let result = run.layoff_card(&mut layoff_hand, 0);
        assert!(matches!(result, Err(MeldError::InvalidLayoff)));
    }

    #[test]
    fn test_layoff_wrong_suit_on_run() {
        let cfg = basic_config();
        let mut cards = vec![
            create_card(Rank::Two, Suit::Hearts, cfg.clone()),
            create_card(Rank::Three, Suit::Hearts, cfg.clone()),
            create_card(Rank::Four, Suit::Hearts, cfg.clone()),
        ];
        let indices = vec![0, 1, 2];
        let mut run = Run::new(&mut cards, &indices).unwrap();
        
        let mut layoff_hand = vec![
            create_card(Rank::Five, Suit::Diamonds, cfg.clone()), // Wrong suit
        ];
        
        let result = run.layoff_card(&mut layoff_hand, 0);
        assert!(matches!(result, Err(MeldError::InvalidLayoff)));
    }
}