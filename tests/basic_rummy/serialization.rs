#[test]
#[cfg(feature = "serde")]
fn serialization_works() {
    use crate::common::fixtures::create_basic_game;
    use rummy::game::r#trait::Game;

    let mut game = create_basic_game(2).unwrap();
    game.next_round().unwrap();

    let serialized = serde_json::to_string(&game).unwrap();
    let deserialized_game = serde_json::from_str(&serialized).unwrap();
    assert_eq!(game, deserialized_game);
}
