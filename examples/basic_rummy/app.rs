use crossterm::event::KeyCode;
use rummy::{
    cards::{deck::DeckConfig, meld::Meldable}, 
    game::{
        action::{DiscardAction, DrawDeckAction, DrawDiscardPileAction, FormMeldAction, GameAction, LayOffAction}, 
        error::GameError, 
        game::Game, 
        state::GamePhase, 
        variants::basic::{config::BasicConfig, game::BasicRummyGame}
    }
};

#[derive(Debug, Clone, PartialEq)]
pub enum AppState {
    MainMenu,
    GamePlay,
    DrawPhase,
    PlayPhase,
    LayOffInput,
    FormMeldInput,
    DiscardInput,
    RoundEnd,
    GameEnd,
    Error(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum InputMode {
    Normal,
    DrawChoice,
    PlayChoice,
    LayOffCardIndex,
    LayOffTargetPlayer,
    LayOffTargetMeld,
    LayOffPosition,
    MeldCardSelection,
    DiscardCardIndex,
}

pub struct App {
    pub game: Option<BasicRummyGame>,
    pub state: AppState,
    pub input_mode: InputMode,
    pub input_buffer: String,
    pub selected_cards: Vec<usize>,
    pub layoff_data: LayOffData,
    pub error_message: Option<String>,
    pub should_quit: bool,
}

#[derive(Default)]
pub struct LayOffData {
    pub card_index: Option<usize>,
    pub target_player_index: Option<usize>,
    pub target_meld_index: Option<usize>,
    pub position: Option<usize>,
}

impl App {
    pub fn new() -> Self {
        Self {
            game: None,
            state: AppState::MainMenu,
            input_mode: InputMode::Normal,
            input_buffer: String::new(),
            selected_cards: Vec::new(),
            layoff_data: LayOffData::default(),
            error_message: None,
            should_quit: false,
        }
    }

    fn setup_game(&mut self) -> Result<(), GameError> {
        let player_ids = vec![1, 2];
        let deck_config = DeckConfig { 
            shuffle_seed: Some(1), 
            pack_count: 1, 
            high_rank: None, 
            wildcard_rank: None 
        };
        let game_config = BasicConfig {
            deal_amount: None,
            draw_deck_amount: None,
            draw_discard_pile_amount: None,
        };
        let mut game = BasicRummyGame::new(player_ids, game_config, deck_config).unwrap();
        game.next_round()?;
        self.game = Some(game);
        self.state = AppState::GamePlay;
        Ok(())
    }

    pub fn handle_input(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('q') => self.should_quit = true,
            KeyCode::Esc => {
                self.input_buffer.clear();
                self.input_mode = InputMode::Normal;
                if let Some(ref game) = self.game {
                    match game.get_state().phase {
                        GamePhase::Draw => self.state = AppState::DrawPhase,
                        GamePhase::Play => self.state = AppState::PlayPhase,
                        GamePhase::RoundEnd => self.state = AppState::RoundEnd,
                        GamePhase::GameEnd => self.state = AppState::GameEnd,
                    }
                }
            }
            _ => {
                match self.state {
                    AppState::MainMenu => self.handle_main_menu_input(key),
                    AppState::DrawPhase => self.handle_draw_input(key),
                    AppState::PlayPhase => self.handle_play_input(key),
                    AppState::LayOffInput => self.handle_layoff_input(key),
                    AppState::FormMeldInput => self.handle_meld_input(key),
                    AppState::DiscardInput => self.handle_discard_input(key),
                    AppState::RoundEnd => self.handle_round_end_input(key),
                    AppState::GameEnd => self.handle_game_end_input(key),
                    _ => {}
                }
            }
        }
    }

    fn handle_main_menu_input(&mut self, key: KeyCode) {
        match key {
            KeyCode::Enter => {
                if let Err(e) = self.setup_game() {
                    self.state = AppState::Error(format!("Failed to setup game: {:?}", e));
                } else {
                    self.update_game_state();
                }
            }
            _ => {}
        }
    }

    fn handle_draw_input(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('1') => {
                if let Some(ref mut game) = self.game {
                    let action = GameAction::DrawDeck(DrawDeckAction {});
                    if let Err(e) = game.execute_action(action) {
                        self.error_message = Some(format!("Draw failed: {:?}", e));
                    }
                    self.update_game_state();
                }
            }
            KeyCode::Char('2') => {
                if let Some(ref mut game) = self.game {
                    if game.get_state().deck.discard_pile().len() > 0 {
                        let action = GameAction::DrawDiscardPile(DrawDiscardPileAction { count: Some(1) });
                        if let Err(e) = game.execute_action(action) {
                            self.error_message = Some(format!("Draw failed: {:?}", e));
                        }
                        self.update_game_state();
                    } else {
                        self.error_message = Some("Discard pile is empty!".to_string());
                    }
                }
            }
            _ => {}
        }
    }

    fn handle_play_input(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('1') => {
                self.state = AppState::LayOffInput;
                self.input_mode = InputMode::LayOffCardIndex;
                self.layoff_data = LayOffData::default();
            }
            KeyCode::Char('2') => {
                self.state = AppState::FormMeldInput;
                self.input_mode = InputMode::MeldCardSelection;
                self.selected_cards.clear();
            }
            KeyCode::Char('3') => {
                self.state = AppState::DiscardInput;
                self.input_mode = InputMode::DiscardCardIndex;
            }
            KeyCode::Char('4') => {
                self.execute_sort_hand();
            }
            _ => {}
        }
    }

    fn handle_layoff_input(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char(c) if c.is_ascii_digit() => {
                self.input_buffer.push(c);
            }
            KeyCode::Backspace => {
                self.input_buffer.pop();
            }
            KeyCode::Enter => {
                if let Ok(value) = self.input_buffer.parse::<usize>() {
                    match self.input_mode {
                        InputMode::LayOffCardIndex => {
                            if let Some(ref game) = self.game {
                                let hand_size = game.get_state().get_current_player().unwrap().cards().len();
                                if value < hand_size {
                                    self.layoff_data.card_index = Some(value);
                                    self.input_mode = InputMode::LayOffTargetPlayer;
                                    self.input_buffer.clear();
                                } else {
                                    self.error_message = Some("Card index out of bounds".to_string());
                                }
                            }
                        }
                        InputMode::LayOffTargetPlayer => {
                            if let Some(ref game) = self.game {
                                let player_count = game.get_state().players.len();
                                if value < player_count {
                                    self.layoff_data.target_player_index = Some(value);
                                    self.input_mode = InputMode::LayOffTargetMeld;
                                    self.input_buffer.clear();
                                } else {
                                    self.error_message = Some("Player index out of bounds".to_string());
                                }
                            }
                        }
                        InputMode::LayOffTargetMeld => {
                            if let Some(ref game) = self.game {
                                let target_player = self.layoff_data.target_player_index.unwrap();
                                let meld_count = game.get_state().players[target_player].melds().len();
                                if value < meld_count {
                                    self.layoff_data.target_meld_index = Some(value);
                                    self.input_mode = InputMode::LayOffPosition;
                                    self.input_buffer.clear();
                                } else {
                                    self.error_message = Some("Meld index out of bounds".to_string());
                                }
                            }
                        }
                        InputMode::LayOffPosition => {
                            self.layoff_data.position = Some(value);
                            self.execute_layoff();
                        }
                        _ => {}
                    }
                } else {
                    self.error_message = Some("Invalid number".to_string());
                }
            }
            _ => {}
        }
    }

    fn handle_meld_input(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char(c) if c.is_ascii_digit() => {
                self.input_buffer.push(c);
            }
            KeyCode::Backspace => {
                self.input_buffer.pop();
            }
            KeyCode::Enter => {
                if let Ok(card_index) = self.input_buffer.parse::<usize>() {
                    if let Some(ref game) = self.game {
                        let hand_size = game.get_state().get_current_player().unwrap().cards().len();
                        if card_index < hand_size && !self.selected_cards.contains(&card_index) {
                            self.selected_cards.push(card_index);
                            self.input_buffer.clear();
                        } else {
                            self.error_message = Some("Invalid or duplicate card index".to_string());
                        }
                    }
                }
            }
            KeyCode::Char('d') | KeyCode::Char('D') => {
                self.execute_form_meld();
            }
            _ => {}
        }
    }

    fn handle_discard_input(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char(c) if c.is_ascii_digit() => {
                self.input_buffer.push(c);
            }
            KeyCode::Backspace => {
                self.input_buffer.pop();
            }
            KeyCode::Enter => {
                if let Ok(card_index) = self.input_buffer.parse::<usize>() {
                    self.execute_discard(card_index);
                }
            }
            _ => {}
        }
    }

    fn handle_round_end_input(&mut self, key: KeyCode) {
        match key {
            KeyCode::Enter => {
                if let Some(ref mut game) = self.game {
                    if let Err(e) = game.next_round() {
                        self.state = AppState::Error(format!("Failed to start next round: {:?}", e));
                    } else {
                        self.update_game_state();
                    }
                }
            }
            _ => {}
        }
    }

    fn handle_game_end_input(&mut self, key: KeyCode) {
        match key {
            KeyCode::Enter => {
                self.state = AppState::MainMenu;
                self.game = None;
            }
            _ => {}
        }
    }

    fn execute_sort_hand(&mut self) {
        if let Some(ref mut game) = self.game {
            let current_player = game.get_state()
                .get_current_player()
                .unwrap();
            let mut current_player_hand = current_player
                .cards()
                .clone();
            current_player_hand.sort();
            game
                .rearrange_player_hand(current_player.id(), current_player_hand)
                .expect(&format!("{:?}", game.get_state().phase));
            self.update_game_state();
        }
    }

    fn execute_layoff(&mut self) {
        if let Some(ref mut game) = self.game {
            let action = LayOffAction {
                card_index: self.layoff_data.card_index.unwrap(),
                target_player_index: self.layoff_data.target_player_index.unwrap(),
                target_meld_index: self.layoff_data.target_meld_index.unwrap(),
                position: self.layoff_data.position.unwrap(),
            };
            
            match game.execute_action(GameAction::LayOff(action)) {
                Ok(_) => {
                    self.error_message = Some("Layoff successful!".to_string());
                }
                Err(e) => {
                    self.error_message = Some(format!("Layoff failed: {:?}", e));
                }
            }
            self.update_game_state();
        }
    }

    fn execute_form_meld(&mut self) {
        if let Some(ref mut game) = self.game {
            let action = FormMeldAction {
                card_indices: self.selected_cards.clone(),
            };
            
            match game.execute_action(GameAction::FormMeld(action)) {
                Ok(_) => {
                    self.error_message = Some("Meld formed successfully!".to_string());
                }
                Err(e) => {
                    self.error_message = Some(format!("Meld failed: {:?}", e));
                }
            }
            self.selected_cards.clear();
            self.update_game_state();
        }
    }

    fn execute_discard(&mut self, card_index: usize) {
        if let Some(ref mut game) = self.game {
            let hand_size = game.get_state().get_current_player().unwrap().cards().len();
            if card_index >= hand_size {
                self.error_message = Some("Card index out of bounds".to_string());
                return;
            }

            let action = DiscardAction {
                card_index,
                declare_going_out: None,
            };
            
            match game.execute_action(GameAction::Discard(action)) {
                Ok(_) => {
                    self.error_message = Some("Discard successful!".to_string());
                }
                Err(e) => {
                    self.error_message = Some(format!("Discard failed: {:?}", e));
                }
            }
            self.update_game_state();
        }
    }

    fn update_game_state(&mut self) {
        if let Some(ref game) = self.game {
            match game.get_state().phase {
                GamePhase::Draw => self.state = AppState::DrawPhase,
                GamePhase::Play => self.state = AppState::PlayPhase,
                GamePhase::RoundEnd => self.state = AppState::RoundEnd,
                GamePhase::GameEnd => self.state = AppState::GameEnd,
            }
            self.input_mode = InputMode::Normal;
            self.input_buffer.clear();
        }
    }
}