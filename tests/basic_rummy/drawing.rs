use rummy::game::{action::{DiscardAction, DrawDeckAction, DrawDiscardPileAction, GameAction}, game::Game, variants::basic::config::{BasicConfig, DrawDiscardPileOverride}};
use crate::common::fixtures::{create_basic_game, create_basic_game_with_config};

#[test]
fn default_draw_one_card_from_deck() {
    let mut game = create_basic_game(2).unwrap();
    game.next_round().unwrap();
    game.execute_action(GameAction::DrawDeck(DrawDeckAction {})).unwrap();
    assert_eq!(game.get_state().players()[0].cards().len(), 11);
    assert_eq!(game.get_state().players()[1].cards().len(), 10);
}

#[test]
fn override_draw_multiple_cards_from_deck() {
    let game_config = BasicConfig {
        deal_amount: None,
        draw_deck_amount: Some(5),
        draw_discard_pile_amount: None
    };
    let mut game = create_basic_game_with_config(2, None, Some(game_config), None).unwrap();
    game.next_round().unwrap();
    game.execute_action(GameAction::DrawDeck(DrawDeckAction {})).unwrap();
    assert_eq!(game.get_state().players()[0].cards().len(), 15);
    assert_eq!(game.get_state().players()[1].cards().len(), 10);
}

#[test]
fn default_draw_one_card_from_discard_pile() {
    let mut game = create_basic_game(2).unwrap();
    game.next_round().unwrap();
    game.execute_action(GameAction::DrawDeck(DrawDeckAction {})).unwrap();
    game.execute_action(GameAction::Discard(DiscardAction { card_index: 0, declare_going_out: None })).unwrap();

    // since we didn't override, this should just draw 1
    game.execute_action(GameAction::DrawDiscardPile(DrawDiscardPileAction { count: Some(250) })).unwrap();

    assert_eq!(game.get_state().players()[1].cards().len(), 11);
}

#[test]
fn override_draw_multiple_from_discard_pile() {
    let game_config = BasicConfig {
        deal_amount: None,
        draw_deck_amount: None,
        draw_discard_pile_amount: Some(DrawDiscardPileOverride::Constant(2))
    };
    let mut game = create_basic_game_with_config(2, None, Some(game_config), None).unwrap();
    game.next_round().unwrap();

    // 3 in discard pile
    game.execute_action(GameAction::DrawDeck(DrawDeckAction {})).unwrap();
    game.execute_action(GameAction::Discard(DiscardAction { card_index: 0, declare_going_out: None })).unwrap();
    game.execute_action(GameAction::DrawDeck(DrawDeckAction {})).unwrap();
    game.execute_action(GameAction::Discard(DiscardAction { card_index: 0, declare_going_out: None })).unwrap();
    game.execute_action(GameAction::DrawDeck(DrawDeckAction {})).unwrap();
    game.execute_action(GameAction::Discard(DiscardAction { card_index: 0, declare_going_out: None })).unwrap();

    // config to draw a constant value of 2 from discard pile, so this value shouldn't affect it
    game.execute_action(GameAction::DrawDiscardPile(DrawDiscardPileAction { count: Some(250) })).unwrap();

    assert_eq!(game.get_state().players()[1].cards().len(), 12);
}

#[test]
fn override_draw_chosen_amount_from_discard_pile() {
    let game_config = BasicConfig {
        deal_amount: None,
        draw_deck_amount: None,
        draw_discard_pile_amount: Some(DrawDiscardPileOverride::PlayerChooses)
    };
    let mut game = create_basic_game_with_config(2, None, Some(game_config), None).unwrap();
    game.next_round().unwrap();

    // 3 in discard pile
    game.execute_action(GameAction::DrawDeck(DrawDeckAction {})).unwrap();
    game.execute_action(GameAction::Discard(DiscardAction { card_index: 0, declare_going_out: None })).unwrap();
    game.execute_action(GameAction::DrawDeck(DrawDeckAction {})).unwrap();
    game.execute_action(GameAction::Discard(DiscardAction { card_index: 0, declare_going_out: None })).unwrap();
    game.execute_action(GameAction::DrawDeck(DrawDeckAction {})).unwrap();
    game.execute_action(GameAction::Discard(DiscardAction { card_index: 0, declare_going_out: None })).unwrap();

    // should fail since there aren't that many cards in the discard pile
    assert!(game.execute_action(GameAction::DrawDiscardPile(DrawDiscardPileAction { count: Some(4) })).is_err());

    // should be fine
    game.execute_action(GameAction::DrawDiscardPile(DrawDiscardPileAction { count: Some(3) })).unwrap();

    assert_eq!(game.get_state().players()[1].cards().len(), 13);
}

#[test]
fn override_draw_whole_discard_pile() {
    let game_config = BasicConfig {
        deal_amount: None,
        draw_deck_amount: None,
        draw_discard_pile_amount: Some(DrawDiscardPileOverride::WholePile)
    };
    let mut game = create_basic_game_with_config(2, None, Some(game_config), None).unwrap();
    game.next_round().unwrap();

    // 3 in discard pile
    game.execute_action(GameAction::DrawDeck(DrawDeckAction {})).unwrap();
    game.execute_action(GameAction::Discard(DiscardAction { card_index: 0, declare_going_out: None })).unwrap();
    game.execute_action(GameAction::DrawDeck(DrawDeckAction {})).unwrap();
    game.execute_action(GameAction::Discard(DiscardAction { card_index: 0, declare_going_out: None })).unwrap();
    game.execute_action(GameAction::DrawDeck(DrawDeckAction {})).unwrap();
    game.execute_action(GameAction::Discard(DiscardAction { card_index: 0, declare_going_out: None })).unwrap();

    // since override to draw entire discard pile, this value shouldn't affect it
    game.execute_action(GameAction::DrawDiscardPile(DrawDiscardPileAction { count: Some(0) })).unwrap();

    assert_eq!(game.get_state().players()[1].cards().len(), 13);
}
