use rummy::{cards::{card::{Card, CardData}, meld::{self, Meldable}, suit_rank::{Rank, Suit}}, game::{action::{DiscardAction, DrawDeckAction, FormMeldAction, GameAction}, game::Game, variants::basic::game::BasicRummyGame}};
use crate::common::fixtures::create_basic_game;

/// Returns the game at a point where a set can be made.
fn set_game() -> BasicRummyGame {
    let mut game = create_basic_game(2).unwrap();
    game.next_round().unwrap();
    game.execute_action(GameAction::DrawDeck(DrawDeckAction {})).unwrap();

    let cur_player = game.get_state().get_current_player().unwrap();
    let mut hand = cur_player.cards().clone();
    hand.sort();
    game.rearrange_player_hand(cur_player.id(), hand).unwrap();

    game
}

/// Returns the game at a point where a run can be made.
fn run_game() -> BasicRummyGame {
    let mut game = set_game();

    game.execute_action(GameAction::Discard(DiscardAction { card_index: 0, declare_going_out: None })).unwrap();

    game.execute_action(GameAction::DrawDeck(DrawDeckAction {})).unwrap();
    let cur_player = game.get_state().get_current_player().unwrap();
    let mut hand = cur_player.cards().clone();
    hand.sort();
    game.rearrange_player_hand(cur_player.id(), hand).unwrap();

    game
}

#[test]
fn form_set() {
    let mut game = set_game();
    let indices = vec![1,2,3,4];
    game.execute_action(GameAction::FormMeld(FormMeldAction { card_indices: indices })).unwrap();

    let meld_cards: Vec<_> = game
        .get_state()
        .get_current_player()
        .unwrap()
        .melds()[0]
        .cards()
        .iter()
        .map(|c| c.data())
        .collect();
    let expected_cards = vec![
        CardData { rank: Rank::Three, suit: Suit::Clubs },
        CardData { rank: Rank::Three, suit: Suit::Diamonds },
        CardData { rank: Rank::Three, suit: Suit::Hearts },
        CardData { rank: Rank::Three, suit: Suit::Spades },
    ];
    assert_eq!(meld_cards, expected_cards);
}

#[test]
fn form_set_successful_in_any_order() {
    let mut game = set_game();
    let indices = vec![3,4,1,2];
    let result = game.execute_action(GameAction::FormMeld(FormMeldAction { card_indices: indices }));
    assert!(result.is_ok());
}

#[test]
fn form_run() {
    let mut game = run_game();
    let indices = vec![8, 9, 10];
    game.execute_action(GameAction::FormMeld(FormMeldAction { card_indices: indices })).unwrap();

    let meld_cards: Vec<_> = game
        .get_state()
        .get_current_player()
        .unwrap()
        .melds()[0]
        .cards()
        .iter()
        .map(|c| c.data())
        .collect();
    let expected_cards = vec![
        CardData { rank: Rank::Jack, suit: Suit::Spades },
        CardData { rank: Rank::Queen, suit: Suit::Spades },
        CardData { rank: Rank::King, suit: Suit::Spades },
    ];
    assert_eq!(meld_cards, expected_cards);
}

#[test]
fn form_run_fails_with_wrong_order_of_indices() {
    let mut game = run_game();

    // the correct order, as above, is `8, 9, 10`
    let indices = vec![10, 9, 8];
    let result = game.execute_action(GameAction::FormMeld(FormMeldAction { card_indices: indices }));
    assert!(result.is_err());
}

#[test]
fn form_meld_fails_with_duplicate_indices() {
    let mut game = set_game();
    let indices = vec![1,2,3,4,1];
    let result = game.execute_action(GameAction::FormMeld(FormMeldAction { card_indices: indices }));
    assert!(result.is_err());
}