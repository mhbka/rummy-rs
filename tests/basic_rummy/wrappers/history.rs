use crate::common::fixtures::create_basic_game_with_history;
use rummy::game::{
    action::{DiscardAction, DrawDeckAction, FormMeldAction, GameAction, GameInteractions},
    game::Game,
};

#[test]
fn history_is_correct() {
    let mut game = create_basic_game_with_history(2).unwrap();
    game.next_round().unwrap();

    // draw from deck
    game.execute_action(GameAction::DrawDeck(DrawDeckAction {}))
        .unwrap();

    // rearrange hand
    let cur_player = game.get_state().get_current_player().unwrap();
    let mut hand: Vec<_> = cur_player.cards().iter().map(|c| c.data()).collect();
    hand.sort();
    let rearranging_hand_player_id = cur_player.id();
    game.rearrange_player_hand(rearranging_hand_player_id, hand.clone())
        .unwrap();

    // form meld
    let indices = vec![1, 2, 3, 4];
    game.execute_action(GameAction::FormMeld(FormMeldAction {
        card_indices: indices.clone(),
    }))
    .unwrap();

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

    // validate the actions were recorded + whether they failed
    let round_history = game.get_histories().get(&1).unwrap();
    assert_eq!(
        round_history[0].entry,
        GameInteractions::Action(GameAction::DrawDeck(DrawDeckAction {}))
    );
    assert_eq!(round_history[0].successful, true);

    assert_eq!(
        round_history[1].entry,
        GameInteractions::HandRearrangement {
            player_id: rearranging_hand_player_id,
            new_arrangement: hand
        }
    );
    assert_eq!(round_history[1].successful, true);

    assert_eq!(
        round_history[2].entry,
        GameInteractions::Action(GameAction::FormMeld(FormMeldAction {
            card_indices: indices
        }))
    );
    assert_eq!(round_history[2].successful, true);

    assert_eq!(
        round_history[3].entry,
        GameInteractions::Action(GameAction::Discard(DiscardAction {
            card_index: 10000,
            declare_going_out: None
        }))
    );
    assert_eq!(round_history[3].successful, false);

    assert_eq!(
        round_history[4].entry,
        GameInteractions::Action(GameAction::Discard(DiscardAction {
            card_index: 0,
            declare_going_out: None
        }))
    );
    assert_eq!(round_history[4].successful, true);

    assert_eq!(round_history.len(), 5);
}
