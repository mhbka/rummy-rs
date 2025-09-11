use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{
        block::Title, Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap,
    },
    Frame, Terminal,
};
use std::io;
use rummy::{
    cards::{deck::DeckConfig, meld::Meldable}, 
    game_rewrite::{
        action::{DiscardAction, DrawDeckAction, DrawDiscardPileAction, FormMeldAction, GameAction, LayOffAction}, 
        error::GameError, 
        game::Game, 
        state::GamePhase, 
        variants::basic::game::BasicRummyGame
    }
};

#[derive(Debug, Clone, PartialEq)]
enum AppState {
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
enum InputMode {
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

struct App {
    game: Option<BasicRummyGame>,
    state: AppState,
    input_mode: InputMode,
    input_buffer: String,
    selected_cards: Vec<usize>,
    layoff_data: LayOffData,
    error_message: Option<String>,
    should_quit: bool,
}

#[derive(Default)]
struct LayOffData {
    card_index: Option<usize>,
    target_player_index: Option<usize>,
    target_meld_index: Option<usize>,
    position: Option<usize>,
}

impl App {
    fn new() -> Self {
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
        let player_ids = vec![1, 2, 3, 4];
        let deck_config = DeckConfig { 
            shuffle_seed: None, 
            pack_count: 1, 
            high_rank: None, 
            wildcard_rank: None 
        };
        let mut game = BasicRummyGame::new(player_ids, deck_config)?;
        game.next_round()?;
        self.game = Some(game);
        self.state = AppState::GamePlay;
        Ok(())
    }

    fn handle_input(&mut self, key: KeyCode) {
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
                    let action = GameAction::DrawDeck(DrawDeckAction { count: Some(1) });
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

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),    // Title
            Constraint::Min(10),      // Main content
            Constraint::Length(3),    // Status/Input
            Constraint::Length(3),    // Instructions
        ])
        .split(f.size());

    // Title
    let title = Paragraph::new("🃏 Rummy Game 🃏")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    match app.state {
        AppState::MainMenu => render_main_menu(f, chunks[1]),
        AppState::DrawPhase | AppState::PlayPhase | AppState::GamePlay => {
            render_game_state(f, chunks[1], app)
        }
        AppState::LayOffInput => render_layoff_input(f, chunks[1], app),
        AppState::FormMeldInput => render_meld_input(f, chunks[1], app),
        AppState::DiscardInput => render_discard_input(f, chunks[1], app),
        AppState::RoundEnd => render_round_end(f, chunks[1], app),
        AppState::GameEnd => render_game_end(f, chunks[1], app),
        AppState::Error(ref msg) => render_error(f, chunks[1], msg),
    }

    // Status bar
    let status = if let Some(ref msg) = app.error_message {
        msg.clone()
    } else {
        match app.input_mode {
            InputMode::Normal => "Ready".to_string(),
            _ => format!("Input: {}", app.input_buffer),
        }
    };
    
    let status_paragraph = Paragraph::new(status)
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL).title("Status"));
    f.render_widget(status_paragraph, chunks[2]);

    // Instructions
    let instructions = match app.state {
        AppState::MainMenu => "Press Enter to start new game • Q to quit",
        AppState::DrawPhase => "Press 1 for deck, 2 for discard pile • ESC to cancel • Q to quit",
        AppState::PlayPhase => "Press 1 for layoff, 2 for meld, 3 for discard • Q to quit",
        AppState::LayOffInput => "Enter card index, then target player, meld, position • ESC to cancel",
        AppState::FormMeldInput => "Enter card indices, press D when done • ESC to cancel",
        AppState::DiscardInput => "Enter card index to discard • ESC to cancel",
        AppState::RoundEnd => "Press Enter to continue to next round • Q to quit",
        AppState::GameEnd => "Press Enter to return to main menu • Q to quit",
        _ => "ESC to go back • Q to quit",
    };

    let instructions_paragraph = Paragraph::new(instructions)
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title("Instructions"));
    f.render_widget(instructions_paragraph, chunks[3]);
}

fn render_main_menu(f: &mut Frame, area: Rect) {
    let welcome_text = vec![
        Line::from(""),
        Line::from(Span::styled("Welcome to Rummy!", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))),
        Line::from(""),
        Line::from("A classic card game for 2-4 players."),
        Line::from(""),
        Line::from("Press Enter to start a new game!"),
    ];

    let paragraph = Paragraph::new(welcome_text)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title("Main Menu"));
    f.render_widget(paragraph, area);
}

fn render_game_state(f: &mut Frame, area: Rect, app: &App) {
    if let Some(ref game) = app.game {
        let gamestate = game.get_state();
        
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        // Left side - Current player info
        let current_player = gamestate.get_current_player().unwrap();
        let hand_text = current_player.cards()
            .iter()
            .enumerate()
            .map(|(i, card)| card.to_string())
            .collect::<Vec<_>>()
            .join(", ");

        let left_content = vec![
            Line::from(Span::styled(format!("Round: {}", gamestate.current_round), Style::default().fg(Color::Cyan))),
            Line::from(Span::styled(format!("Current Player: {}", gamestate.current_player), Style::default().fg(Color::Yellow))),
            Line::from(""),
            Line::from(Span::styled("Your Hand:", Style::default().add_modifier(Modifier::BOLD))),
            Line::from(hand_text),
            Line::from(""),
            Line::from(format!("Deck size: {}", gamestate.deck.stock().len())),
            Line::from(format!("Discard pile size: {}", gamestate.deck.discard_pile().len())),
            Line::from(format!("Top discard: {:?}", gamestate.deck.peek_discard_pile())),
        ];

        let left_paragraph = Paragraph::new(left_content)
            .block(Block::default().borders(Borders::ALL).title("Game State"))
            .wrap(Wrap { trim: true });
        f.render_widget(left_paragraph, chunks[0]);

        // Right side - All players info
        let mut players_text = Vec::new();
        for player in &gamestate.players {
            if player.active() {
                players_text.push(Line::from(format!("Player {}: {} cards", player.id(), player.cards().len())));
                for (i, meld) in player.melds().iter().enumerate() {
                    players_text.push(Line::from(format!("  Meld {}: {:?}", i, meld.cards())));
                }
                players_text.push(Line::from(""));
            }
        }

        let right_paragraph = Paragraph::new(players_text)
            .block(Block::default().borders(Borders::ALL).title("All Players"))
            .wrap(Wrap { trim: true });
        f.render_widget(right_paragraph, chunks[1]);
    }
}

fn render_layoff_input(f: &mut Frame, area: Rect, app: &App) {
    let prompt = match app.input_mode {
        InputMode::LayOffCardIndex => "Enter card index to lay off:",
        InputMode::LayOffTargetPlayer => "Enter target player index:",
        InputMode::LayOffTargetMeld => "Enter target meld index:",
        InputMode::LayOffPosition => "Enter position in meld:",
        _ => "Lay off input:",
    };

    let content = vec![
        Line::from(Span::styled("Lay Off Action", Style::default().add_modifier(Modifier::BOLD))),
        Line::from(""),
        Line::from(prompt),
        Line::from(format!("> {}", app.input_buffer)),
    ];

    let paragraph = Paragraph::new(content)
        .block(Block::default().borders(Borders::ALL).title("Lay Off"))
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}

fn render_meld_input(f: &mut Frame, area: Rect, app: &App) {
    let content = vec![
        Line::from(Span::styled("Form Meld Action", Style::default().add_modifier(Modifier::BOLD))),
        Line::from(""),
        Line::from("Enter card indices to add to meld (press D when done):"),
        Line::from(format!("> {}", app.input_buffer)),
        Line::from(""),
        Line::from(format!("Selected cards: {:?}", app.selected_cards)),
    ];

    let paragraph = Paragraph::new(content)
        .block(Block::default().borders(Borders::ALL).title("Form Meld"))
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}

fn render_discard_input(f: &mut Frame, area: Rect, app: &App) {
    let content = vec![
        Line::from(Span::styled("Discard Action", Style::default().add_modifier(Modifier::BOLD))),
        Line::from(""),
        Line::from("Enter card index to discard:"),
        Line::from(format!("> {}", app.input_buffer)),
    ];

    let paragraph = Paragraph::new(content)
        .block(Block::default().borders(Borders::ALL).title("Discard"))
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}

fn render_round_end(f: &mut Frame, area: Rect, app: &App) {
    if let Some(ref game) = app.game {
        let gamestate = game.get_state();
        let latest_score = gamestate.round_scores
            .get(&gamestate.current_round);

        let content = vec![
            Line::from(Span::styled("Round Ended!", Style::default().add_modifier(Modifier::BOLD).fg(Color::Green))),
            Line::from(""),
            Line::from(format!("Round {} complete", gamestate.current_round)),
            Line::from(format!("Scores: {:?}", latest_score)),
            Line::from(""),
            Line::from("Press Enter to continue to next round..."),
        ];

        let paragraph = Paragraph::new(content)
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).title("Round Complete"));
        f.render_widget(paragraph, area);
    }
}

fn render_game_end(f: &mut Frame, area: Rect, app: &App) {
    let content = vec![
        Line::from(Span::styled("Game Over!", Style::default().add_modifier(Modifier::BOLD).fg(Color::Red))),
        Line::from(""),
        Line::from("Thanks for playing!"),
        Line::from(""),
        Line::from("Press Enter to return to main menu..."),
    ];

    let paragraph = Paragraph::new(content)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title("Game Over"));
    f.render_widget(paragraph, area);
}

fn render_error(f: &mut Frame, area: Rect, error_msg: &str) {
    let content = vec![
        Line::from(Span::styled("Error!", Style::default().add_modifier(Modifier::BOLD).fg(Color::Red))),
        Line::from(""),
        Line::from(error_msg),
    ];

    let paragraph = Paragraph::new(content)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title("Error"));
    f.render_widget(paragraph, area);
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let mut app = App::new();

    loop {
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                app.handle_input(key.code);
                if app.should_quit {
                    break;
                }
            }
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run the app
    let res = run_app(&mut terminal);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}