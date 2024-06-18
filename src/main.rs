pub mod cards;
pub mod game;
pub mod player;

use game::{
    actions::{
        AllActions, DiscardActions, DrawActions, PlayActions, PlayableActions, RoundEndActions,
        TransitionResult,
    },
    phases::{DiscardPhase, DrawPhase, GamePhase, PlayPhase, PlayablePhase, RoundEndPhase},
    state::{Score, State},
    variants::standard::{StandardRummy, StandardRummyGame},
};
use rprompt;

fn main() {
    let player_ids = vec![1, 2, 3, 4];
    let game = StandardRummyGame::quickstart(player_ids).to_next_round();
    handle_round(game);
}

fn handle_round(mut game: StandardRummy<DrawPhase>) -> StandardRummy<RoundEndPhase> {
    loop {
        handle_draw(&mut game);

        let mut play_game = game.to_play_phase();

        play_game = match handle_play(play_game) {
            Ok(game) => game,
            Err(game) => {
                println!("Round has ended; returning...");
                return game;
            }
        };

        let discard_game = play_game.to_discard_phase();

        let discard_game = match handle_discard(discard_game) {
            Ok(game) => game,
            Err(game) => {
                println!("Round has ended; returning...");
                return game;
            }
        };

        game = match discard_game.to_next_player() {
            TransitionResult::Next(game) => game,
            TransitionResult::End(_) => unreachable!(), // already discarded above, so no auto-discard; thus, round cannot end here
            TransitionResult::Error(_) => unreachable!(), // no error occurs here
        };
    }
}

fn print_state<C, S: Score>(state: &State<C, S>) {
    println!("---------------");
    println!("Current player: {}", state.players[state.cur_player].id());
    println!("Hand: {:?} ({} cards)", state.players[state.cur_player].cards(), state.players[state.cur_player].cards().len());
    println!("Deck size: {}", state.deck.stock().len());
    println!(
        "Top discard card and size: {:?}, {}",
        state.deck.peek_discard_pile(),
        state.deck.discard_pile().len()
    );
    println!("Melds: ");

    for player in &state.players {
        println!("Player: {}", player.id());
        for meld in &player.melds {
            println!("{meld:?}");
        }
        println!("\n");
    }

    println!("---------------");
}

fn handle_draw(game: &mut StandardRummy<DrawPhase>) {
    let state = game.view_state();

    print_state(state);

    if state.deck.discard_pile().len() == 0 {
        println!("No discard pile, drawing from stock...");
        game.draw_stock();
    } else {
        match rprompt::prompt_reply(
            r#"
        1. Draw stock
        2. Attempt to draw from discard pile
        "#,
        )
        .unwrap()
        .as_str()
        {
            "1" => game.draw_stock(),
            "2" => {
                let amount = rprompt::prompt_reply("Draw how many?: ")
                    .unwrap()
                    .parse()
                    .ok();
                if game.draw_discard_pile(amount).is_err() {
                    println!("Not enough cards in pile; drawing from stock...");
                    game.draw_stock();
                }
            }
            _ => {
                println!("Invalid; drawing from stock...");
                game.draw_stock();
            }
        }
    }
}

fn handle_play(
    mut game: StandardRummy<PlayPhase>,
) -> Result<StandardRummy<PlayPhase>, StandardRummy<RoundEndPhase>> {
    let state = game.view_state();
    print_state(state);

    loop {
        let play_result = match rprompt::prompt_reply(
            r#"
        1. Form meld
        2. Layoff card
        3. Sort hand
        4. Discard
        "#,
        )
        .unwrap()
        .as_str()
        {
            "1" => play_meld(game),
            "2" => play_layoff(game),
            "3" => {
                sort_hand(&mut game);
                TransitionResult::Next(game)
            }
            "4" => {
                println!("Continuing...");
                return Ok(game);
            }
            _ => {
                println!("Invalid input, continuing...");
                return Ok(game);
            }
        };

        game = match play_result {
            TransitionResult::Next(game) => game,
            TransitionResult::End(game) => return Err(game),
            TransitionResult::Error((game, err)) => {
                println!("Error: {err}");
                game
            }
        };

        match rprompt::prompt_reply("Make another play? (Y/N): ")
            .unwrap()
            .as_str()
        {
            "Y" | "y" => print_state(game.view_state()),
            "N" | "n" => return Ok(game),
            _ => {
                println!("Not valid input; going to discard...");
                return Ok(game);
            }
        }
    }
}

fn play_meld(
    game: StandardRummy<PlayPhase>,
) -> TransitionResult<
    StandardRummy<PlayPhase>,
    StandardRummy<RoundEndPhase>,
    StandardRummy<PlayPhase>,
    String,
> {
    let cur_player = &game.view_state().players[game.view_state().cur_player];
    let mut indices = Vec::new();

    loop {
        match rprompt::prompt_reply("Enter index of card to put in meld (-1 to stop): ")
            .unwrap()
            .parse::<i32>()
        {
            Ok(i) => {
                if i < 0 {
                    println!("Collecting...");
                    break;
                } else if i as usize > cur_player.cards.len() {
                    println!("Greater than player's hand size. Try again.");
                } else {
                    println!("Chosen card: {:?}", cur_player.cards[i as usize]);
                    indices.push(i as usize);
                }
            }
            Err(_) => println!("Invalid input."),
        }
    }

    game.form_meld(indices)
}

fn play_layoff(
    game: StandardRummy<PlayPhase>,
) -> TransitionResult<
    StandardRummy<PlayPhase>,
    StandardRummy<RoundEndPhase>,
    StandardRummy<PlayPhase>,
    String,
> {
    let card_i = match rprompt::prompt_reply("Enter index of card to layoff: ")
        .unwrap()
        .parse()
    {
        Ok(i) => i,
        Err(_) => {
            println!("Invalid input; returning...");
            return TransitionResult::Next(game);
        }
    };

    let target_player_i = match rprompt::prompt_reply("Enter index of targeted player: ")
        .unwrap()
        .parse()
    {
        Ok(i) => i,
        Err(_) => {
            println!("Invalid input; returning...");
            return TransitionResult::Next(game);
        }
    };

    let target_meld_i = match rprompt::prompt_reply("Enter index of targeted meld: ")
        .unwrap()
        .parse()
    {
        Ok(i) => i,
        Err(_) => {
            println!("Invalid input; returning...");
            return TransitionResult::Next(game);
        }
    };

    game.layoff_card(card_i, target_player_i, target_meld_i)
}

fn sort_hand<P: GamePhase + PlayablePhase>(game: &mut StandardRummy<P>) {
    match rprompt::prompt_reply("Choose player to sort hand for: ")
        .unwrap()
        .parse()
    {
        Ok(i) => match game.sort_hand(i) {
            Ok(_) => println!("Sorted player {i}'s hand."),
            Err(err) => println!("Error: {err}"),
        },
        Err(_) => {
            println!("Invalid input.");
        }
    }
}

fn handle_discard(
    mut game: StandardRummy<DiscardPhase>,
) -> Result<StandardRummy<DiscardPhase>, StandardRummy<RoundEndPhase>> {
    let state = game.view_state();

    print_state(state);

    loop {
        let i = match rprompt::prompt_reply("Choose a card to discard: ")
            .unwrap()
            .parse()
        {
            Ok(i) => i,
            Err(_) => {
                println!("Invalid; try again...");
                continue;
            }
        };

        game = match game.discard(i) {
            TransitionResult::Next(game) => {
                println!("Discarded a card.");
                return Ok(game);
            }
            TransitionResult::End(game) => {
                println!("Last card discarded; round ending...");
                return Err(game);
            }
            TransitionResult::Error((game, _)) => {
                println!("Invalid index; try again.");
                game
            }
        }
    }
}
