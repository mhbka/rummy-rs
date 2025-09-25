use rummy::{
    game::{
        action::{DiscardAction, DrawDeckAction, FormMeldAction, GameAction},
        game::Game,
    },
    wrappers::replay::Replay,
};

use crate::common::fixtures::create_basic_game_with_history;

#[test]
fn replay_is_correct() {
    let mut game = create_basic_game_with_history(2).unwrap();
    game.next_round().unwrap();

    // draw from deck
    game.execute_action(GameAction::DrawDeck(DrawDeckAction {}))
        .unwrap();
    let state_1 = game.get_state().clone();

    // rearrange hand
    let cur_player = game.get_state().get_current_player().unwrap();
    let mut hand: Vec<_> = cur_player.cards().iter().map(|c| c.data()).collect();
    hand.sort();
    let rearranging_hand_player_id = cur_player.id();
    game.rearrange_player_hand(rearranging_hand_player_id, hand.clone())
        .unwrap();
    let state_2 = game.get_state().clone();

    // form meld
    let indices = vec![1, 2, 3, 4];
    game.execute_action(GameAction::FormMeld(FormMeldAction {
        card_indices: indices.clone(),
    }))
    .unwrap();
    let state_3 = game.get_state().clone();

    // discard a nonexistent card (will fail)
    let failed_discard = game.execute_action(GameAction::Discard(DiscardAction {
        card_index: 10000,
        declare_going_out: None,
    }));
    assert!(failed_discard.is_err());

    // discard card 0
    game.execute_action(GameAction::Discard(DiscardAction {
        card_index: 0,
        declare_going_out: None,
    }))
    .unwrap();
    let state_4 = game.get_state().clone();

    // wrap in replay
    let mut game = Replay::new(game, false);

    // check each state is correct
    game.next().unwrap();
    assert_eq!(game.get_replaying_game().get_state(), &state_1);
    game.next().unwrap();
    assert_eq!(game.get_replaying_game().get_state(), &state_2);
    game.next().unwrap();
    assert_eq!(game.get_replaying_game().get_state(), &state_3);

    // our failed discard shows no change in state
    game.next().unwrap();
    assert_eq!(game.get_replaying_game().get_state(), &state_3);

    game.next().unwrap();
    assert_eq!(game.get_replaying_game().get_state(), &state_4);
}
