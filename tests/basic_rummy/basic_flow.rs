use rummy::game::{action::{DrawDeckAction, FormMeldAction, GameAction}, game::Game};
use crate::common::fixtures::create_basic_game;

#[test]
fn basic_flow() {
    // this flow tests creating a default game, going to the first round,
    // drawing a card, forming a meld, and discarding a card.
    //
    // the card states were checked beforehand with the preset shuffle seed.
    //
    // this is very basic but the best I can do manually inputting cards.
    //
    // TODO: record some known-valid games and run them in a test harness
    let mut game = create_basic_game(2).unwrap();
    game.next_round().unwrap();
    game.execute_action(GameAction::DrawDeck(DrawDeckAction {})).unwrap();

    let cur_player = game.get_state().get_current_player().unwrap();
    let mut hand = cur_player.cards().clone();
    hand.sort();
    game.rearrange_player_hand(cur_player.id(), hand).unwrap();
    let indices = vec![1,2,3,4];
    game.execute_action(GameAction::FormMeld(FormMeldAction { card_indices: indices })).unwrap();   
}