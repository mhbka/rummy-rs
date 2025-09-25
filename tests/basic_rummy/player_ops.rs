use rummy::game::{error::{GameError, GameSetupError}, game::Game};
use crate::common::fixtures::create_basic_game;

#[test]
fn add_player_during_new_round() {
    let mut game = create_basic_game(2).unwrap();
    game.add_player(2).unwrap();
    game.next_round().unwrap();
    for player in game.get_state().players() {
        assert!(player.active());
    }
}

#[test]
fn add_player_during_round() {
    let mut game = create_basic_game(2).unwrap();
    game.next_round().unwrap();
    game.add_player(2).unwrap();
    for player in game.get_state().players() {
        if player.id() == 2 {
            assert!(!player.active());
        } else {
            assert!(player.active());
        }
    }
}

#[test]
fn add_existing_player_id_fails() {
    let mut game = create_basic_game(2).unwrap();
    let result = game.add_player(1);
    assert!(matches!(result, Err(GameError::AddedPlayerAlreadyExists)));
}

#[test]
fn add_too_many_players_fails() {
    let mut game = create_basic_game(6).unwrap();
    game.add_player(6).unwrap();
    assert!(matches!(game.next_round(), Err(GameError::FailedRoundSetup(_))));
}

#[test]
fn quit_player_during_round() {
    let mut game = create_basic_game(3).unwrap();
    game.next_round().unwrap();
    game.quit_player(0).unwrap();
    let state = game.get_state();
    assert!(!state.players()[0].active());
    assert!(state.players()[1].active());
    assert!(state.players()[2].active());
}

#[test]
fn quit_nonexistent_player_fails() {
    let mut game = create_basic_game(2).unwrap();
    assert!(matches!(game.quit_player(3), Err(GameError::PlayerDoesntExist)));
}