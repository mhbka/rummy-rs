use std::{collections::HashMap, sync::Arc};
use serde::{Deserialize, Serialize};
use crate::{cards::deck::DeckConfig, game::{game::Game, rules::GameRules, score::{RoundScore, VariantPlayerScore}, state::{GamePhase, GameState}, variants::basic::{game::BasicRummyGame, rules::BasicRules, score::BasicScore}}, serialization::{cards::SerializableDeck, player::SerializablePlayer}};

/// A serializable version of a `BasicRummyGame`.
#[derive(Serialize, Deserialize)]
pub(super) struct SerializableBasicRummyGame {
    state: SerializableGameState<BasicScore, BasicRules>,
    rules: BasicRules,
}

impl SerializableBasicRummyGame {
    /// Convert this from a `BasicRummyGame`.
    pub fn from_game(game: &BasicRummyGame) -> Self {
        Self {
            state: SerializableGameState::from_gamestate(&game.state),
            rules: game.rules.clone()
        }
    }

    
    /// Convert this to a `BasicRummyGame`.
    pub fn to_game(self) -> BasicRummyGame {
        BasicRummyGame { 
            state: self.state.to_gamestate(), 
            rules: self.rules 
        }
    }
}

// to serialize `BasicRummyGame`, we convert to its serializable form...
impl Serialize for BasicRummyGame {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer 
    {
        let serializable_game = SerializableBasicRummyGame::from_game(self);
        serializable_game.serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for BasicRummyGame {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de>,
    {
        let serialized_game = SerializableBasicRummyGame::deserialize(deserializer)?;
        let game = serialized_game.to_game();
        Ok(game)
    }
}

/// A serializable version of a `GameState`.
#[derive(Serialize, Deserialize)]
pub(super) struct SerializableGameState<P: VariantPlayerScore, R: GameRules<VariantScore = P>>   {
    pub phase: GamePhase,
    pub players: Vec<SerializablePlayer>,
    pub deck: SerializableDeck,
    pub deck_config: DeckConfig,
    pub current_player: usize,
    pub current_round: usize,
    pub round_scores: HashMap<usize, RoundScore<P>>,
    pub variant_state: R::VariantState, 
}

impl<P: VariantPlayerScore, R: GameRules<VariantScore = P>> SerializableGameState<P, R> {
    /// Convert this to an actual `GameState`.
    pub fn to_gamestate(self) -> GameState<P, R> {
        let deck_config = Arc::new(self.deck_config);
        let players = self.players
            .into_iter()
            .map(|p| p.to_player(deck_config.clone()))
            .collect();
        let deck = self.deck.to_deck(deck_config);
        GameState {
            phase: self.phase,
            players,
            deck,
            current_player: self.current_player,
            current_round: self.current_round,
            round_scores: self.round_scores,
            variant_state: self.variant_state
        }
    }

    /// Convert this from a `GameState`.
    pub fn from_gamestate(state: &GameState<P, R>) -> Self {
        let players = state.players
            .iter()
            .map(|p| SerializablePlayer::from_player(p))
            .collect();
        let deck = SerializableDeck::from_deck(&state.deck);
        Self {
            phase: state.phase,
            players,
            deck,
            deck_config: (*state.deck.config).clone(),
            current_player: state.current_player,
            current_round: state.current_round,
            round_scores: state.round_scores.clone(),
            variant_state: state.variant_state.clone()
        }
    }
}