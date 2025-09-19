#[cfg(test)]
mod tests {    
    use crate::cards::deck::DeckConfig;
    use crate::cards::{
        card::Card,
        suit_rank::{Rank, Suit},
    };
    use std::sync::Arc;

    #[test]
    /// Cards have the expected ordering.
    fn normal_ordering_card() {
        let cfg = Arc::new(DeckConfig::new());

        // cards are ordered by rank, then suit
        let card1 = Card {
            rank: Rank::Ace,
            suit: Suit::Clubs,
            deck_config: cfg.clone(),
        };
        let card2 = Card {
            rank: Rank::Ace,
            suit: Suit::Diamonds,
            deck_config: cfg.clone(),
        };
        let card3 = Card {
            rank: Rank::Two,
            suit: Suit::Clubs,
            deck_config: cfg.clone(),
        };

        assert!(card2 > card1);
        assert!(card3 > card2);
    }

    #[test]
    /// If the deck config specifies a custom high rank,
    /// ordering will decrease from that rank onwards.
    ///
    /// For eg, `high_rank = 3` means `3 > 2 > Ace > King > Queen > ...`
    fn custom_ordering_card() {
        let cfg = Arc::new(DeckConfig {
            shuffle_seed: None,
            pack_count: 1,
            high_rank: Some(Rank::Three),
            wildcard_rank: None,
        });

        // Rank::Three should be the highest now
        let card1 = Card {
            rank: Rank::King,
            suit: Suit::Spades,
            deck_config: cfg.clone(),
        };
        let card2 = Card {
            rank: Rank::Two,
            suit: Suit::Spades,
            deck_config: cfg.clone(),
        };
        let card3 = Card {
            rank: Rank::Three,
            suit: Suit::Clubs,
            deck_config: cfg.clone(),
        };

        assert!(card2 > card1);
        assert!(card3 > card2);

        // Suit ordering should remain the same
        let card4 = Card {
            rank: Rank::Three,
            suit: Suit::Spades,
            deck_config: cfg.clone(),
        };
        assert!(card4 > card1);
    }
}