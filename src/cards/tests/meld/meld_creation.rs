#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn test_valid_set_creation() {
        let cfg = basic_config();
        let mut cards = create_set_cards(
            Rank::Ace,
            &[Suit::Clubs, Suit::Diamonds, Suit::Hearts],
            cfg.clone(),
        );
        let indices = vec![0, 1, 2];

        let meld = Meld::new(&mut cards, &indices);
        assert!(meld.is_ok());
        assert!(meld.unwrap().is_set());
    }

    #[test]
    fn test_valid_run_creation() {
        let cfg = basic_config();
        let mut cards = vec![
            create_card(Rank::Ace, Suit::Clubs, cfg.clone()),
            create_card(Rank::Two, Suit::Clubs, cfg.clone()),
            create_card(Rank::Three, Suit::Clubs, cfg.clone()),
        ];
        let indices = vec![0, 1, 2];

        let meld = Meld::new(&mut cards, &indices);
        assert!(meld.is_ok());
        assert!(meld.unwrap().is_run());
    }

    #[test]
    fn test_max_length_set() {
        let cfg = basic_config();
        let mut cards = create_set_cards(
            Rank::Queen,
            &[Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades],
            cfg.clone(),
        );
        let indices = vec![0, 1, 2, 3];

        let set = Set::new(&mut cards, &indices).unwrap();
        assert_eq!(set.cards().len(), 4);
        assert_eq!(set.rank(), Rank::Queen);
    }

    #[test]
    fn test_long_run_creation() {
        let cfg = basic_config();
        let mut cards = vec![
            create_card(Rank::Two, Suit::Hearts, cfg.clone()),
            create_card(Rank::Three, Suit::Hearts, cfg.clone()),
            create_card(Rank::Four, Suit::Hearts, cfg.clone()),
            create_card(Rank::Five, Suit::Hearts, cfg.clone()),
            create_card(Rank::Six, Suit::Hearts, cfg.clone()),
            create_card(Rank::Seven, Suit::Hearts, cfg.clone()),
        ];
        let indices = vec![0, 1, 2, 3, 4, 5];

        let run = Run::new(&mut cards, &indices).unwrap();
        assert_eq!(run.cards().len(), 6);
        assert_eq!(run.suit(), Suit::Hearts);
    }

    #[test]
    fn test_cards_removed_from_hand_on_creation() {
        let cfg = basic_config();
        let mut cards = vec![
            create_card(Rank::Ace, Suit::Clubs, cfg.clone()),
            create_card(Rank::Ace, Suit::Diamonds, cfg.clone()),
            create_card(Rank::Ace, Suit::Hearts, cfg.clone()),
            create_card(Rank::Two, Suit::Clubs, cfg.clone()),
        ];
        let indices = vec![0, 1, 2];
        let original_len = cards.len();

        let _meld = Meld::new(&mut cards, &indices).unwrap();

        assert_eq!(cards.len(), original_len - 3);
        assert_eq!(cards[0].rank, Rank::Two); // Only remaining card
    }

    #[test]
    fn test_meld_chooses_set_over_run_when_both_possible() {
        // Test case where cards could form either a set or run
        // This documents current behavior - adjust based on your preferences
        let cfg = basic_config();
        let mut cards = vec![
            create_card(Rank::Seven, Suit::Clubs, cfg.clone()),
            create_card(Rank::Seven, Suit::Diamonds, cfg.clone()),
            create_card(Rank::Seven, Suit::Hearts, cfg.clone()),
        ];
        let indices = vec![0, 1, 2];

        let meld = Meld::new(&mut cards, &indices).unwrap();
        // Based on your implementation, Set::new is tried first
        assert!(meld.is_set());
    }

    #[test]
    fn test_unordered_indices() {
        let cfg = basic_config();
        let mut cards = create_set_cards(
            Rank::King,
            &[Suit::Clubs, Suit::Diamonds, Suit::Hearts],
            cfg.clone(),
        );
        let indices = vec![2, 0, 1]; // Out of order

        let set = Set::new(&mut cards, &indices).unwrap();
        assert_eq!(set.cards().len(), 3);
        assert_eq!(set.rank(), Rank::King);
    }
}
