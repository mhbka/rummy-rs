extern crate rummy;

use rprompt;
use rummy::{cards::{deck::DeckConfig, meld::Meldable}, game_rewrite::{action::{DiscardAction, DrawDeckAction, DrawDiscardPileAction, FormMeldAction, GameAction, LayOffAction}, error::GameError, game::Game, state::GamePhase, variants::basic::game::BasicRummyGame}};


fn main() {
    let mut game = setup().unwrap();
    game.next_round().unwrap();
    run(game);
}

fn setup() -> Result<BasicRummyGame, GameError> {
    let player_ids = vec![1, 2, 3, 4];
    let deck_config = DeckConfig { 
        shuffle_seed: None, 
        pack_count: 1, 
        high_rank: None, 
        wildcard_rank: None 
    };
    BasicRummyGame::new(player_ids, deck_config)
}

fn run(mut game: BasicRummyGame) {
    loop {
        print_game_state(&game);
        let gamestate = game.get_state();
        println!("-------------");
        match gamestate.phase {
            GamePhase::Draw => handle_draw(&mut game),
            GamePhase::Play => handle_play(&mut game),
            GamePhase::RoundEnd => handle_round_end(&mut game),
            GamePhase::GameEnd => handle_game_end(&game),
        }
        println!("-------------");
    }
}

fn handle_draw(game: &mut BasicRummyGame) {
    if game.get_state().deck.discard_pile().len() == 0 {
        println!("Discard pile empty; drawing from deck...");
        let draw_action = GameAction::DrawDeck(DrawDeckAction { count: Some(1) });
        game.execute_action(draw_action).unwrap();
        return;
    }

    let mut draw_choice = rprompt::prompt_reply("Draw from deck stock (1) or discard pile (2): ").unwrap();
    loop {
        match draw_choice.as_str() {
            "1" => {
                let draw_action = GameAction::DrawDeck(DrawDeckAction { count: Some(1) });
                game.execute_action(draw_action).unwrap();
                break;
            },
            "2" => {
                let draw_action = GameAction::DrawDiscardPile(DrawDiscardPileAction { count: Some(1) });
                game.execute_action(draw_action).unwrap();
                break;
            },
            _ => {
                draw_choice = rprompt::prompt_reply("Invalid choice. Please try again: ").unwrap();
            }
        }
    }
}

fn handle_play(game: &mut BasicRummyGame) {
    let mut action_choice = rprompt::prompt_reply("Lay off a card (1), form a meld (2), or discard and end your turn (3): ").unwrap();
    loop {
        match action_choice.as_str() {
            "1" => {
                handle_layoff(game);
                break;
            },
            "2" => {
                handle_form_meld(game);
                break;
            },
            "3" => {
                handle_discard(game);
                break;
            }
            _ => action_choice = rprompt::prompt_reply("Invalid choice. Please try again: ").unwrap()
        }
    }
}

fn handle_round_end(game: &mut BasicRummyGame) {
    let gamestate = game.get_state();
    let latest_score = gamestate.round_scores
        .get(&gamestate.current_round)
        .unwrap();
    println!("Round {} score: {:?}", gamestate.current_round, latest_score);

    println!("Going to next round...");
    game.next_round().unwrap();
}

fn handle_game_end(game: &BasicRummyGame) {

}

fn handle_layoff(game: &mut BasicRummyGame) {
    let gamestate = game.get_state();

    let card_index;
    let target_player_index;
    let target_meld_index;
    let position;

    let mut card_choice = rprompt::prompt_reply("Choose a card in your hand (0 indexed) to lay off: ").unwrap();
    loop {
        match card_choice.parse::<usize>() {
            Ok(chosen_card_index) => {
                let player_hand_size = gamestate
                    .get_current_player()
                    .unwrap()
                    .cards()
                    .len();
                match chosen_card_index > player_hand_size
                {
                    true => card_choice = rprompt::prompt_reply(format!("Current player only has {player_hand_size} cards. Please try again: ").as_str()).unwrap(),
                    false => {
                        card_index = chosen_card_index;
                        break;
                    },
                }
            },
            Err(err) => card_choice = rprompt::prompt_reply("Couldn't parse your answer into a number. Please try again: ").unwrap()
        }
    }
    
    let mut player_choice = rprompt::prompt_reply("Choose a player (0 indexed) whose meld you want to lay off to: ").unwrap();
    loop {
        match player_choice.parse::<usize>() {
            Ok(chosen_player_index) => {
                let players = gamestate.players.len();
                match chosen_player_index > players
                {
                    true => player_choice = rprompt::prompt_reply(format!("Only {players} players in game. Please try again: ").as_str()).unwrap(),
                    false => {
                        target_player_index = chosen_player_index;
                        break;
                    },
                }
            },
            Err(err) => player_choice = rprompt::prompt_reply("Couldn't parse your answer into a number. Please try again: ").unwrap()
        }
    }

    let mut meld_choice = rprompt::prompt_reply("Choose the player's meld (0 indexed) which you want to lay off to: ").unwrap();
    loop {
        match meld_choice.parse::<usize>() {
            Ok(chosen_meld_index) => {
                let target_player_melds = gamestate
                    .players[target_player_index]
                    .melds()
                    .len();
                match chosen_meld_index > target_player_melds
                {
                    true => meld_choice = rprompt::prompt_reply(format!("Only {target_player_melds} melds under that player. Please try again: ").as_str()).unwrap(),
                    false => {
                        target_meld_index = chosen_meld_index;
                        break;
                    },
                }
            },
            Err(err) => meld_choice = rprompt::prompt_reply("Couldn't parse your answer into a number. Please try again: ").unwrap()
        }
    }

    let mut position_choice = rprompt::prompt_reply("Choose the position within the meld (0 indexed) to place the laid off card at: ").unwrap();
    loop {
        match position_choice.parse::<usize>() {
            Ok(chosen_position_index) => {
                let meld_cards = gamestate
                    .players[target_player_index]
                    .melds()[target_player_index]
                    .cards()
                    .len();
                match chosen_position_index > meld_cards
                {
                    true => position_choice = rprompt::prompt_reply(format!("Only {meld_cards} cards in that meld. Please try again: ").as_str()).unwrap(),
                    false => {
                        position = chosen_position_index;
                        break;
                    },
                }
            },
            Err(err) => position_choice = rprompt::prompt_reply("Couldn't parse your answer into a number. Please try again: ").unwrap()
        }
    }

    let layoff_action = LayOffAction { 
        card_index,
        target_player_index,
        target_meld_index,
        position
    };
    match game.execute_action(GameAction::LayOff(layoff_action)) {
        Ok(ok) => println!("Layoff successful!"),
        Err(err) => println!("Failed to layoff (error: {err:?})")
    }
}

fn handle_form_meld(game: &mut BasicRummyGame) {
    let mut card_indices = Vec::new();
    
    let mut card_choice = rprompt::prompt_reply("Choose a card (0 indexed) to add into your meld (D to stop): ").unwrap();
    loop {
        if card_choice.as_str() == "D" {
            println!("Cards chosen: {card_indices:?}");
            break;
        }
        match card_choice.parse::<usize>() {
            Ok(card_index) => {
                let hand_size = game
                    .get_state()
                    .get_current_player()
                    .unwrap()
                    .cards()
                    .len();
                match card_index > hand_size || card_indices.iter().find(|&&i| i == card_index).is_some()
                {
                    true => card_choice = rprompt::prompt_reply(format!("Either out-of-bounds, or card index is already added. Please try again: ").as_str()).unwrap(),
                    false => card_indices.push(card_index)
                }
            },
            Err(err) => card_choice = rprompt::prompt_reply("Couldn't parse your answer into a number. Please try again: ").unwrap()
        }
    }

    let meld_action = FormMeldAction {
        card_indices
    };
    match game.execute_action(GameAction::FormMeld(meld_action)) {
        Ok(ok) => println!("Meld successful!"),
        Err(err) => println!("Failed to form meld (error: {err:?})")
    }
}

fn handle_discard(game: &mut BasicRummyGame) {
    let card_index;
    let mut card_choice = rprompt::prompt_reply("Choose a card (0 indexed) to discard: ").unwrap();
    loop {
        match card_choice.parse::<usize>() {
            Ok(index) => {
                let hand_size = game
                    .get_state()
                    .get_current_player()
                    .unwrap()
                    .cards()
                    .len();
                match index > hand_size
                {
                    true => card_choice = rprompt::prompt_reply(format!("Either out-of-bounds, or card index is already added. Please try again: ").as_str()).unwrap(),
                    false => {
                        card_index = index;
                        break;
                    }
                }
            },
            Err(err) => card_choice = rprompt::prompt_reply("Couldn't parse your answer into a number. Please try again: ").unwrap()
        }
    }

    let discard_action = DiscardAction { 
        card_index,
        declare_going_out: None
    };
    match game.execute_action(GameAction::Discard(discard_action)) {
        Ok(ok) => println!("Meld successful!"),
        Err(err) => println!("Failed to discard (error: {err:?})")
    }
}

fn print_game_state(game: &BasicRummyGame) {
    let gamestate = game.get_state();
    println!("---------------------");
    println!("
        Round: {}
        Current player: {}
        Current player's hand: {:?}
        Deck size: {}
        Discard pile size: {}
        Top discard card: {:?}
        ",
        gamestate.current_round,
        gamestate.current_player,
        gamestate.get_current_player().unwrap().cards(),
        gamestate.deck.stock().len(),
        gamestate.deck.discard_pile().len(),
        gamestate.deck.peek_discard_pile(),
    );
    for player in &gamestate.players {
        if player.active() {
            println!("
            ------
            Player: {}
            Cards in hand: {}
            Melds: {:?}
            ------
            ",
            player.id(),
            player.cards().len(),
            player.melds()
            );
        }
    }
    println!("---------------------");
}