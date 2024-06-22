# rummy
This crate models the card game Rummy. It supports configuration and different popular variants (WIP).

**NOTE: this crate is in early development; expect nothing to stay stable.**

## Use

### Modules
A typical import of this crate will look like this:
```rust 
use rummy::game::{
    actions::{
        AllActions, DiscardActions, DrawActions, PlayActions, PlayableActions, RoundEndActions,
        TransitionResult,
    },
    phases::{DiscardPhase, DrawPhase, GamePhase, PlayPhase, PlayablePhase, RoundEndPhase},
    state::{Score, State},
    variants::standard::{StandardRummy, StandardRummyGame},
};
```

A breakdown of the modules:
- **actions**: Traits that split up possible actions for each phase of a Rummy game.
- **phases**: Different phases of a Rummy game; used as state for the game's typestate.
- **state**: State of a Rummy game, common across all variants.
- **variants**: The game itself; contains different variants of Rummy.

### Starting a game
```rust
// initialize a list of player IDs
let player_ids = vec![1, 2, 3, 4];

// pass it into a variant's `quickstart`
let game = StandardRummyGame::quickstart(player_ids); 

// we initialize at round 0 in `RoundEndPhase`, so we must advance to the next round
let game = game.to_next_round();
```

Alternatively, you can configure the game by setting a config:
```rust
// the config struct for standard Rummy
let game_config = StandardRummyConfig { /* ... */ };

// and for the deck itself
let deck_config = DeckConfig { /* ... */ };

// we pass both into the variant's `new`
let game = StandardRummyGame::quickstart(player_ids, game_config, deck_config); 
```

### Transitions
Certain actions, such as forming melds, can result in immediate transitions to a different gamephase. Calling these functions consumes the game and returns a `TransitionResult`.

WIP

## Example
Use `cargo run --examples rummy` to run a command-line implementation of this crate.
