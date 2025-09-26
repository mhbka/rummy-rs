use crate::common::fixtures::{create_basic_game, create_basic_game_with_config};
use rummy::{
    cards::deck::DeckConfig,
    game::{error::GameSetupError, r#trait::Game},
};

#[test]
fn deal_2_players() {
    let mut game = create_basic_game(2).unwrap();
    game.next_round().unwrap();
    let state = game.get_state();
    for player in state.players() {
        assert_eq!(player.cards().len(), 10);
    }
}

#[test]
fn deal_3_to_5_players() {
    for player_count in 3..=5 {
        let mut game = create_basic_game(player_count).unwrap();
        game.next_round().unwrap();
        let state = game.get_state();
        for player in state.players() {
            assert_eq!(player.cards().len(), 7);
        }
    }
}

#[test]
fn deal_6_players() {
    let mut game = create_basic_game(6).unwrap();
    game.next_round().unwrap();
    let state = game.get_state();
    for player in state.players() {
        assert_eq!(player.cards().len(), 6);
    }
}

#[test]
fn deal_7_players_requires_two_decks() {
    // Should fail with 1 deck
    let game = create_basic_game(7);
    assert!(matches!(game, Err(GameSetupError::NotEnoughCards)));

    // Should work with 2 decks
    let deck_config = DeckConfig {
        pack_count: 2,
        ..Default::default()
    };
    let mut game = create_basic_game_with_config(7, None, None, Some(deck_config)).unwrap();
    game.next_round().unwrap();
    let state = game.get_state();
    for player in state.players() {
        assert_eq!(player.cards().len(), 10);
    }
}

#[test]
fn deal_10_players_two_decks_fails() {
    let deck_config = DeckConfig {
        pack_count: 2,
        ..Default::default()
    };
    let game = create_basic_game_with_config(10, None, None, Some(deck_config));
    assert!(matches!(game, Err(GameSetupError::NotEnoughCards)));
}
