use crate::common::fixtures::create_basic_game_with_config;
use rummy::game::{error::GameSetupError, game::Game, variants::basic::config::BasicConfig};

#[test]
fn override_deal_amount_is_correct() {
    let game_config = BasicConfig {
        deal_amount: Some(20),
        draw_deck_amount: None,
        draw_discard_pile_amount: None,
    };
    let mut game = create_basic_game_with_config(2, None, Some(game_config), None).unwrap();
    game.next_round().unwrap();
    for player in game.get_state().players() {
        assert_eq!(player.cards().len(), 20);
    }
}

#[test]
fn override_is_correct_with_many_players() {
    let game_config = BasicConfig {
        deal_amount: Some(1),
        draw_deck_amount: None,
        draw_discard_pile_amount: None,
    };
    let mut game = create_basic_game_with_config(20, None, Some(game_config), None).unwrap();
    game.next_round().unwrap();
    for player in game.get_state().players() {
        assert_eq!(player.cards().len(), 1);
    }
}

#[test]
fn override_fails_if_not_enough_cards() {
    let game_config = BasicConfig {
        deal_amount: Some(1),
        draw_deck_amount: None,
        draw_discard_pile_amount: None,
    };
    let game = create_basic_game_with_config(27, None, Some(game_config), None);
    assert!(matches!(game, Err(GameSetupError::NotEnoughCards)));
}

#[test]
fn override_fails_if_one_player() {
    let game_config = BasicConfig {
        deal_amount: Some(1),
        draw_deck_amount: None,
        draw_discard_pile_amount: None,
    };
    let game = create_basic_game_with_config(1, None, Some(game_config), None);
    assert!(matches!(game, Err(GameSetupError::TooFewPlayers)));
}
