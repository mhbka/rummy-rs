extern crate rummy;

use rprompt;
use rummy::{cards::deck::DeckConfig, game_rewrite::{action::{DrawDeckAction, DrawDiscardPileAction, GameAction}, error::GameError, game::Game, state::GamePhase, variants::basic::game::BasicRummyGame}};


fn main() {
    let game = setup().unwrap();
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
        match gamestate.phase {
            GamePhase::Draw => handle_draw(&mut game),
            GamePhase::Play => handle_play(&mut game),
            GamePhase::RoundEnd => handle_round_end(&mut game),
            GamePhase::GameEnd => handle_game_end(&game),
        }
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

}

fn handle_game_end(game: &BasicRummyGame) {

}

fn handle_layoff(game: &mut BasicRummyGame) {
    {
        let mut card_choice = rprompt::prompt_reply("Choose a card in your hand (0 indexed) to lay off: ").unwrap();
        loop {
            match card_choice.parse::<usize>() {
                Ok(card_index) => {
                    let player_hand_size = game
                        .get_state()
                        .get_current_player()
                        .unwrap()
                        .cards()
                        .len();
                    match card_index > player_hand_size
                    {
                        true => card_choice = rprompt::prompt_reply(format!("Current player only has {player_hand_size} cards. Please try again: ").as_str()).unwrap(),
                        false => break,
                    }
                },
                Err(err) => card_choice = rprompt::prompt_reply("Couldn't parse your answer into a number. Please try again: ").unwrap()
            }
        }
    }
}

fn handle_form_meld(game: &mut BasicRummyGame) {
    
}

fn handle_discard(game: &mut BasicRummyGame) {
    
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