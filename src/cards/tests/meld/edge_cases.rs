#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn test_duplicate_indices_behavior() {
        let cfg = basic_config();
        let mut cards = create_set_cards(Rank::Ace, &[Suit::Clubs, Suit::Diamonds, Suit::Hearts], cfg.clone());
        let indices = vec![0, 1, 1]; // Duplicate index
        
        // This test documents current behavior - you might want to decide if this should be an error
        let result = Meld::new(&mut cards, &indices);
        
        // The behavior depends on implementation - this will show what actually happens
        // You might want to make this an error condition in the future
        match result {
            Ok(meld) => {
                println!("Duplicate indices created meld: {:?}", meld);
                // Document what type of meld was created
            }
            Err(e) => {
                println!("Duplicate indices failed with: {:?}", e);
                // Document what error was returned
            }
        }
    }

    #[test]
    fn test_empty_hand_layoff() {
        let cfg = basic_config();
        let mut cards = create_set_cards(Rank::King, &[Suit::Clubs, Suit::Diamonds, Suit::Hearts], cfg.clone());
        let indices = vec![0, 1, 2];
        let mut set = Set::new(&mut cards, &indices).unwrap();
        
        let mut empty_hand: Vec<Card> = vec![];
        let result = set.layoff_card(&mut empty_hand, 0);
        
        assert!(matches!(result, Err(MeldError::InvalidIndex { index: 0, .. })));
    }

    #[test]
    fn test_single_card_hand_layoff() {
        let cfg = basic_config();
        let mut cards = create_set_cards(Rank::Jack, &[Suit::Clubs, Suit::Diamonds, Suit::Hearts], cfg.clone());
        let indices = vec![0, 1, 2];
        let mut set = Set::new(&mut cards, &indices).unwrap();
        
        let mut single_card_hand = vec![create_card(Rank::Jack, Suit::Spades, cfg.clone())];
        let result = set.layoff_card(&mut single_card_hand, 0);
        
        assert!(result.is_ok());
        assert!(single_card_hand.is_empty()); // Card should be removed
    }

    #[test]
    fn test_large_indices_vector() {
        let cfg = basic_config();
        let mut cards: Vec<Card> = (0..10)
            .map(|i| create_card(Rank::Ace, if i % 2 == 0 { Suit::Clubs } else { Suit::Diamonds }, cfg.clone()))
            .collect();
        
        // Try to create a meld with more cards than typical
        let indices: Vec<usize> = (0..8).collect();
        let result = Set::new(&mut cards, &indices);
        
        // This might work (if all cards have same rank) or fail - documents behavior
        match result {
            Ok(set) => assert_eq!(set.cards().len(), 8),
            Err(e) => println!("Large set creation failed: {:?}", e),
        }
    }

    #[test]
    fn test_out_of_order_run_indices() {
        let cfg = basic_config();
        let mut cards = vec![
            create_card(Rank::Seven, Suit::Hearts, cfg.clone()),
            create_card(Rank::Five, Suit::Hearts, cfg.clone()),
            create_card(Rank::Six, Suit::Hearts, cfg.clone()),
            create_card(Rank::Eight, Suit::Hearts, cfg.clone()),
        ];
        
        // Indices that would form a valid run if sorted
        let indices = vec![1, 2, 0, 3]; // Points to 5,6,7,8 of Hearts
        
        let result = Run::new(&mut cards, &indices);
        // Should work because Run::new should sort the cards
        assert!(result.is_ok());
        
        if let Ok(run) = result {
            assert_eq!(run.cards().len(), 4);
            // Verify the run is properly ordered
            // (This assumes your implementation sorts cards within the run)
        }
    }

    #[test] 
    fn test_ace_edge_cases_in_runs() {
        let cfg = basic_config();
        
        // Test Ace-low run (A-2-3)
        let mut low_ace_cards = vec![
            create_card(Rank::Ace, Suit::Clubs, cfg.clone()),
            create_card(Rank::Two, Suit::Clubs, cfg.clone()),
            create_card(Rank::Three, Suit::Clubs, cfg.clone()),
        ];
        let indices = vec![0, 1, 2];
        let low_result = Run::new(&mut low_ace_cards, &indices);
        
        // Test Ace-high run (Q-K-A) - if your game supports this
        let mut high_ace_cards = vec![
            create_card(Rank::Queen, Suit::Spades, cfg.clone()),
            create_card(Rank::King, Suit::Spades, cfg.clone()),
            create_card(Rank::Ace, Suit::Spades, cfg.clone()),
        ];
        let indices = vec![0, 1, 2];
        let high_result = Run::new(&mut high_ace_cards, &indices);
        
        // Document what your implementation does with Aces
        println!("Ace-low run result: {:?}", low_result);
        println!("Ace-high run result: {:?}", high_result);
        
        // Test wraparound (K-A-2) - this should fail in most Rummy variants
        let mut wraparound_cards = vec![
            create_card(Rank::King, Suit::Diamonds, cfg.clone()),
            create_card(Rank::Ace, Suit::Diamonds, cfg.clone()),
            create_card(Rank::Two, Suit::Diamonds, cfg.clone()),
        ];
        let indices = vec![0, 1, 2];
        let wraparound_result = Run::new(&mut wraparound_cards, &indices);
        
        // This should typically fail
        assert!(wraparound_result.is_err());
    }

    #[test]
    fn test_minimum_meld_sizes() {
        let cfg = basic_config();
        
        // Test exactly 3 cards for set
        let mut set_cards = create_set_cards(Rank::Four, &[Suit::Clubs, Suit::Diamonds, Suit::Hearts], cfg.clone());
        let indices = vec![0, 1, 2];
        let result = Set::new(&mut set_cards, &indices);
        assert!(result.is_ok());
        
        // Test exactly 3 cards for run
        let mut run_cards = vec![
            create_card(Rank::Nine, Suit::Spades, cfg.clone()),
            create_card(Rank::Ten, Suit::Spades, cfg.clone()),
            create_card(Rank::Jack, Suit::Spades, cfg.clone()),
        ];
        let indices = vec![0, 1, 2];
        let result = Run::new(&mut run_cards, &indices);
        assert!(result.is_ok());
    }

    #[test]
    fn test_layoff_index_exactly_at_boundary() {
        let cfg = basic_config();
        let mut cards = create_set_cards(Rank::Ten, &[Suit::Clubs, Suit::Diamonds, Suit::Hearts], cfg.clone());
        let indices = vec![0, 1, 2];
        let mut set = Set::new(&mut cards, &indices).unwrap();
        
        let mut layoff_hand = vec![
            create_card(Rank::Ten, Suit::Spades, cfg.clone()),
        ];
        
        // Test with index exactly at the last valid position
        let result = set.layoff_card(&mut layoff_hand, 0);
        assert!(result.is_ok());
        
        // Test with index exactly one past the last valid position
        let mut another_hand = vec![
            create_card(Rank::Ten, Suit::Hearts, cfg.clone()),
        ];
        let result = set.layoff_card(&mut another_hand, 1);
        assert!(matches!(result, Err(MeldError::InvalidIndex { index: 1, max_valid: 0 })));
    }

    #[test]
    fn test_meld_with_identical_cards() {
        let cfg = basic_config();
        // Create identical cards (same rank and suit)
        let mut cards = vec![
            create_card(Rank::Five, Suit::Hearts, cfg.clone()),
            create_card(Rank::Five, Suit::Hearts, cfg.clone()),
            create_card(Rank::Five, Suit::Hearts, cfg.clone()),
        ];
        let indices = vec![0, 1, 2];
        
        // This tests how your implementation handles duplicate cards
        // In a real deck, you shouldn't have identical cards, but this tests robustness
        let result = Set::new(&mut cards, &indices);
        
        // Document behavior with identical cards
        match result {
            Ok(set) => {
                assert_eq!(set.cards().len(), 3);
                println!("Identical cards formed valid set");
            }
            Err(e) => println!("Identical cards failed: {:?}", e),
        }
    }

    #[test]
    fn test_mixed_deck_configs() {
        // Test what happens if cards have different deck configs
        let cfg1 = basic_config();
        let cfg2 = Arc::new(DeckConfig::new());
        
        let mut cards = vec![
            create_card(Rank::Seven, Suit::Clubs, cfg1.clone()),
            create_card(Rank::Seven, Suit::Diamonds, cfg2.clone()), // Different config
            create_card(Rank::Seven, Suit::Hearts, cfg1.clone()),
        ];
        let indices = vec![0, 1, 2];
        
        let result = Set::new(&mut cards, &indices);
        
        // This documents behavior when cards have different configurations
        // You might want to add validation to ensure all cards use the same config
        match result {
            Ok(_) => println!("Mixed configs allowed"),
            Err(e) => println!("Mixed configs rejected: {:?}", e),
        }
    }
}
