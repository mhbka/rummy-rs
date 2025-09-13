use rummy::{cards::deck::DeckConfig, game::variants::basic::game::BasicRummyGame};

const SHUFFLE_SEED: u64 = 1;

/// Create a basic Rummy game with a fixed shuffling seed.
pub fn create_basic_game(player_count: usize) -> BasicRummyGame {
    let player_ids: Vec<usize> = (0..player_count).collect();
    let deck_config = DeckConfig {
        shuffle_seed: Some(SHUFFLE_SEED),
        pack_count: 1,
        high_rank: None,
        wildcard_rank: None
    };
    BasicRummyGame::new(player_ids, deck_config).unwrap()
}

/// Create a basic Rummy game with your own shuffling seed (if `None`, the deck isn't shuffled).
pub fn create_basic_game_with_seed(player_count: usize, seed: Option<usize>) -> BasicRummyGame {

}