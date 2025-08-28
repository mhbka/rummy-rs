extern crate rummy;

use rprompt;
use rummy::{cards::deck::DeckConfig, game_rewrite::variants::basic::game::BasicRummyGame};


fn main() {
    let game = setup();
    run(game);
}

fn setup() -> BasicRummyGame {
    let player_ids = vec![1, 2, 3, 4];
    let deck_config = DeckConfig { 
        shuffle_seed: None, 
        pack_count: 1, 
        high_rank: None, 
        wildcard_rank: None 
    };
    BasicRummyGame::new(player_ids, deck_config)
}

fn run(game: BasicRummyGame) {
}
