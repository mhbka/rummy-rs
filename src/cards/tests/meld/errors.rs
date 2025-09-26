#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn test_insufficient_cards_error() {
        let cfg = basic_config();
        let mut cards = vec![
            create_card(Rank::Ace, Suit::Clubs, cfg.clone()),
            create_card(Rank::Ace, Suit::Spades, cfg.clone()),
        ];
        let indices = vec![0, 1]; // Only 2 cards

        let result = Meld::new(&mut cards, &indices);
        assert!(matches!(
            result,
            Err(MeldError::InsufficientCards {
                provided: 2,
                minimum: 3
            })
        ));
    }

    #[test]
    fn test_invalid_index_error() {
        let cfg = basic_config();
        let mut cards = vec![create_card(Rank::Ace, Suit::Clubs, cfg.clone())];
        let indices = vec![0, 1, 5]; // Index 5 is out of bounds

        let result = Meld::new(&mut cards, &indices);
        assert!(matches!(result, Err(MeldError::InvalidCardIndex)));
    }

    #[test]
    fn test_empty_indices() {
        let cfg = basic_config();
        let mut cards = vec![create_card(Rank::Ace, Suit::Clubs, cfg.clone())];
        let indices = vec![];

        let result = Meld::new(&mut cards, &indices);
        assert!(matches!(
            result,
            Err(MeldError::InsufficientCards {
                provided: 0,
                minimum: 3
            })
        ));
    }

    #[test]
    fn test_layoff_invalid_index() {
        let cfg = basic_config();
        let mut cards = create_set_cards(
            Rank::Ace,
            &[Suit::Clubs, Suit::Diamonds, Suit::Hearts],
            cfg.clone(),
        );
        let indices = vec![0, 1, 2];
        let mut set = Set::new(&mut cards, &indices).unwrap();

        let mut layoff_hand = vec![create_card(Rank::Ace, Suit::Spades, cfg.clone())];

        let result = set.layoff_card(&mut layoff_hand, 5); // Invalid index
        assert!(matches!(result, Err(MeldError::InvalidCardIndex)));
    }

    #[test]
    fn test_invalid_set_error() {
        let cfg = basic_config();
        let mut cards = vec![
            create_card(Rank::Ace, Suit::Clubs, cfg.clone()),
            create_card(Rank::Ace, Suit::Spades, cfg.clone()),
            create_card(Rank::Two, Suit::Clubs, cfg.clone()),
        ];
        let indices = vec![0, 1, 2];

        let result = Set::new(&mut cards, &indices);
        assert!(matches!(result, Err(MeldError::InvalidSet)));
    }

    #[test]
    fn test_invalid_run_error() {
        let cfg = basic_config();
        let mut cards = vec![
            create_card(Rank::Ace, Suit::Clubs, cfg.clone()),
            create_card(Rank::Two, Suit::Clubs, cfg.clone()),
            create_card(Rank::Three, Suit::Spades, cfg.clone()), // Wrong suit
        ];
        let indices = vec![0, 1, 2];

        let result = Run::new(&mut cards, &indices);
        assert!(matches!(result, Err(MeldError::InvalidRun)));
    }

    #[test]
    fn test_invalid_set_layoff() {
        let cfg = basic_config();
        let mut cards = create_set_cards(
            Rank::Ace,
            &[Suit::Clubs, Suit::Diamonds, Suit::Hearts],
            cfg.clone(),
        );
        let indices = vec![0, 1, 2];
        let mut meld = Meld::new(&mut cards, &indices).unwrap();

        let mut layoff_card = vec![create_card(Rank::Two, Suit::Clubs, cfg.clone())];

        let result = meld.layoff_card(&mut layoff_card, 0);
        assert!(matches!(result, Err(MeldError::InvalidLayoff)));
    }

    #[test]
    fn test_invalid_run_layoff() {
        let cfg = basic_config();
        let mut cards = vec![
            create_card(Rank::Ace, Suit::Clubs, cfg.clone()),
            create_card(Rank::Two, Suit::Clubs, cfg.clone()),
            create_card(Rank::Three, Suit::Clubs, cfg.clone()),
        ];
        let indices = vec![0, 1, 2];
        let mut meld = Meld::new(&mut cards, &indices).unwrap();

        let mut layoff_cards = vec![
            create_card(Rank::Five, Suit::Clubs, cfg.clone()), // Gap in sequence
            create_card(Rank::Four, Suit::Spades, cfg.clone()), // Wrong suit
        ];

        assert!(matches!(
            meld.layoff_card(&mut layoff_cards, 0),
            Err(MeldError::InvalidLayoff)
        ));
        assert!(matches!(
            meld.layoff_card(&mut layoff_cards, 1),
            Err(MeldError::InvalidLayoff)
        ));
    }
}
