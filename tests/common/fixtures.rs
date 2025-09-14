use rummy::{cards::deck::DeckConfig, game::{error::GameSetupError, variants::basic::{config::BasicConfig, game::BasicRummyGame}}};

/// A constant shuffle seed for testing.
const SHUFFLE_SEED: u64 = 1;

/// Create a basic Rummy game with a fixed shuffling seed and default settings.
pub fn create_basic_game(player_count: usize) -> Result<BasicRummyGame, GameSetupError> {
    let player_ids: Vec<usize> = (0..player_count).collect();
    let deck_config = DeckConfig {
        shuffle_seed: Some(SHUFFLE_SEED),
        pack_count: 1,
        high_rank: None,
        wildcard_rank: None
    };
    let game_config = BasicConfig {
        deal_amount: None,
        draw_deck_amount: None,
        draw_discard_pile_amount: None
    };
    BasicRummyGame::new(player_ids, game_config, deck_config)
}

/// Create a basic Rummy game with your own shuffling seed (if `None`, the deck isn't shuffled) and configs.
pub fn create_basic_game_with_config(
    player_count: usize, 
    seed: Option<u64>, 
    game_config: Option<BasicConfig>, 
    deck_config: Option<DeckConfig>
) -> Result<BasicRummyGame, GameSetupError> {
    let player_ids: Vec<usize> = (0..player_count).collect();
    let deck_config = match deck_config {
        Some(config) => config,
        None => DeckConfig {
            shuffle_seed: seed,
            pack_count: 1,
            high_rank: None,
            wildcard_rank: None
        }
    };
    let game_config = match game_config {
        Some(config) => config,
        None => BasicConfig {
            deal_amount: None,
            draw_deck_amount: None,
            draw_discard_pile_amount: None
        }
    };
    BasicRummyGame::new(player_ids, game_config, deck_config)
}