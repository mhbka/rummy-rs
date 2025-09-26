# rummy
`rummy` is a Rust crate for running Rummy games. 
It supports:
- **Deck configuration**: Configure the pack count, shuffle seed, wildcards, and even high cards (ever wanted `Ten` to be the high rank instead of `King`?) 
- **Game configuration**: Override the default amount of cards to draw from the deck or discard pile, or the number of cards to discard on each turn.
- **Useful wrappers**: Comes with `History` and `Replay` wrappers, allowing you to view all the actions of the game and even replay the game step-by-step.
- **Serializable**: With the `serde` feature, you can (de)serialize games, allowing you to store them with ease!

## Basic usage
```rust
fn main() {
    // Create a basic Rummy game with 4 players and default configuration 
    let player_ids: Vec<usize> = vec![1, 2, 3, 4];
    let deck_config = DeckConfig {
        shuffle_seed: None,
        pack_count: 1,
        high_rank: None,
        wildcard_rank: None
    };
    let game_config = BasicConfig {
        deal_amount: None,
        draw_deck_amount: None,
        draw_discard_pile_amount: None
    };
    let mut game = BasicRummyGame::new(player_ids, game_config, deck_config).unwrap();

    // Advance to the next round (the game starts at round 0)
    game.next_round().unwrap();

    // Current player draws a card from the deck...
    game.execute_action(GameAction::DrawDeck(DrawDeckAction {})).unwrap();
    
    // ... then discards the 3rd card from their hand
    game.execute_action(GameAction::Discard(DiscardAction { card_index: 2, declare_going_out: None })).unwrap();

    // Inspect the game state
    let state = game.get_state();

    // Add a player with ID 5 (they'll only start playing the next round)
    game.add_player(5).unwrap();

    // Or quit the player with ID 3
    game.quit_player(3).unwrap();
}
```

## Wrappers
### History
This allows you to record and view the complete history of all interactions with the game:
```rust
// Create a basic Rummy game with history
let player_ids: Vec<usize> = vec![1, 2, 3, 4];
let deck_config = DeckConfig {
    shuffle_seed: None,
    pack_count: 1,
    high_rank: None,
    wildcard_rank: None
};
let game_config = BasicConfig {
    deal_amount: None,
    draw_deck_amount: None,
    draw_discard_pile_amount: None
};
let mut game = History::new(player_ids, game_config, deck_config).unwrap();

// play the game as usual...

// Get the history of interactions for round 1
let round_history = game
    .get_histories()
    .get(&1)
    .unwrap();
```

### Replay
Going a step further, you can replay a game (including undo/redo), and inspect its state after each action:
```rust
// say you already have a `History<BasicRummyGame>`...
let mut game_with_replay = Replay::new(game, false);

// Replay maintains an internal "replaying game" which starts at round 0;
// we call `next()` to advance with the game's history of actions,
// which returns an `Option` with the just-executed action.
let action = game_with_replay.next().unwrap();

// We can also check the state of the replaying game
let replaying_state = game_with_replay.get_replaying_game().get_state(); 

// If we want, we can also go backwards!
game_with_replay.previous();
```

## Examples
A `basic_rummy` example has been included, where you can play basic Rummy in a neat terminal GUI (thanks to `ratatui`!).
To play it, just run:
```Powershell
cargo run --example basic_rummy
```

## Progress
While this crate is still a work-in-progress, the intention is to (as much as possible) only extend the API. 
These are the currently planned additions:

### Variants
- [ ] Gin Rummy
- [ ] 500 Rum 
- [ ] Canasta
- [ ] Indian Rummy

### Wrappers
- [ ] Possible Plays - A wrapper that can return the possible plays (meld/layoffs etc) the current player can take

### Others
- [ ] Recorded game testing + harness - Record some known valid games and implement a harness to run them in tests
- [ ] Bot player - A purely algorithmic bot player with configurable difficulty

Any help with contributing towards these (or with your own ideas) is always welcome!

## License
Licensed under the [MIT license](http://opensource.org/licenses/MIT).