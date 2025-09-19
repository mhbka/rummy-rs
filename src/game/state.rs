use std::collections::HashMap;
use std::fmt::Debug;
use crate::{cards::deck::{Deck, DeckConfig}, game::{action::GameAction, error::{ActionError, FailedActionError, GameError, InternalError}, rules::GameRules, score::{RoundScore, VariantPlayerScore}}, player::Player};

/// The state of the game. Includes state common across all variants like players/deck/current round,
/// as well as `variant_state` for variant-specific state.
/// 
/// ## Note
/// Ensure that mutable references to this are not handed outside of the `Game`.
/// Accidental wrong mutation of the state will probably lead to an invalid gamestate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameState<P: VariantPlayerScore, R: GameRules<VariantScore = P>>   {
    pub phase: GamePhase,
    pub players: Vec<Player>,
    pub deck: Deck,
    pub current_player: usize,
    pub current_round: usize,
    pub round_scores: HashMap<usize, RoundScore<P>>,
    pub variant_state: R::VariantState, 
} 

impl<P: VariantPlayerScore, R: GameRules<VariantScore = P>> GameState<P, R> 
where
    R::VariantState: VariantState<P, R>
{   
    /// Initialize the game state.
    pub fn initialize(player_ids: Vec<usize>, deck_config: DeckConfig, variant_state: R::VariantState) -> Self {
        let players = player_ids
            .into_iter()
            .map(|id| Player {
                id,
                cards: vec![],
                melds: vec![],
                active: true,
                joined_in_round: 0
            })
            .collect();
        let deck = Deck::new(deck_config);
        Self {
            phase: GamePhase::RoundEnd,
            players,
            deck,
            current_round: 0,
            current_player: 0,
            round_scores: HashMap::new(),
            variant_state
        }
    } 

    /// Validate if the action is valid in the current gamestate.
    pub fn validate_action(&self, action: &GameAction) -> Result<(), ActionError> {
        match (self.phase, action) {
            (GamePhase::Draw, GameAction::DrawDeck(_)) => (),
            (GamePhase::Draw, GameAction::DrawDiscardPile(_)) =>(),
            (GamePhase::Play, GameAction::FormMeld(_)) => (),
            (GamePhase::Play, GameAction::FormMelds(_)) => (),
            (GamePhase::Play, GameAction::LayOff(_)) => (),
            (GamePhase::Play, GameAction::Discard(_)) => (),
            _ => {
                let err = FailedActionError::InvalidGamePhase { current_phase: self.phase };
                return Err(ActionError::FailedAction(err));
            },
        };
        R::VariantState::validate_action(self, action)
    }

    /// Sets up a new round by:
    /// - Incrementing `current_round`
    /// - Setting players who joined in the last round as active
    /// - Resetting the deck and dealing new hands
    /// - Setting the current player as `starting_player_index`
    /// 
    /// Returns an `Err` if the game phase isn't `RoundEnded`.
    pub fn start_new_round(&mut self, cards_to_deal: usize, starting_player_index: usize) -> Result<(), GameError> {
        if self.phase != GamePhase::RoundEnd {
            return Err(GameError::WrongGamePhase);
        }

        self.deck.reset();

        for player in &mut self.players {
            if !player.active && player.joined_in_round == self.current_round {
                player.active = true;
            }
            player.melds = Vec::new();
            player.cards = self.deck
                .draw(cards_to_deal)
                .map_err(|err| InternalError::NoCardsInDeckOrDiscardPile)?;
        }

        self.current_player = starting_player_index;
        self.phase = GamePhase::Draw;
        self.current_round += 1;
        
        Ok(())
    }

    /// Get a mutable reference to the current player.
    /// 
    /// Returns an `InternalError` if the `current_player` index is invalid for some reason.
    pub fn get_current_player_mut(&mut self) -> Result<&mut Player, InternalError> {
        self.players
            .get_mut(self.current_player)
            .ok_or(InternalError::InvalidCurrentPlayer { current: self.current_player })
    }

    /// Get a reference to the current player.
    /// 
    /// Returns an `InternalError` if the `current_player` index is invalid for some reason.
    pub fn get_current_player(&self) -> Result<&Player, InternalError> {
        self.players
            .get(self.current_player)
            .ok_or(InternalError::InvalidCurrentPlayer { current: self.current_player })
    }

    /// Increment `current_player` to the next active player.
    pub fn to_next_player(&mut self) {
        let mut next_player = (self.current_player + 1) % self.players.len();
        while !self.players[next_player].active {
            next_player = (self.current_player + 1) % self.players.len();
        }
        self.current_player = next_player;
    }
}

/// Represents the unique state held by a Rummy variant.
/// 
/// The game state consists of the general gamestate, which all Rummy variants have in common,
/// and the variant gamestate, which holds state that the variant may uniquely require.
/// 
/// If a variant doesn't require any unique gamestate, they can simply use an empty struct and implement
/// `VariantState` on it.
pub trait VariantState<P: VariantPlayerScore, R: GameRules<VariantState = Self, VariantScore = P>>: Sized + Clone + Debug + Eq {
    /// Validate if an action is **generally** valid in the current gamestate, for the variant.
    /// 
    /// The default implementation is to just return `Ok(())`. If a variant requires its own validation
    /// for general actions, this function can be overridden.
    /// 
    /// ## Note
    /// This should not be used for validating specific actions (ie, whether forming a meld is valid).
    /// That should be done in the `GameRules` action handler instead.
    fn validate_action(state: &GameState<P, R>, action: &GameAction) -> Result<(), ActionError> {
        Ok(())
    }
}

/// The phases of a Rummy game.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GamePhase {
    Draw,
    Play,
    RoundEnd,
    GameEnd
}

/// A serializable gamestate, purely for serialization purposes.
struct SerializableGameState<P: VariantPlayerScore, R: GameRules<VariantScore = P>>   {
    pub phase: GamePhase,
    pub players: Vec<Player>,
    pub deck: Deck,
    pub current_player: usize,
    pub current_round: usize,
    pub round_scores: HashMap<usize, RoundScore<P>>,
    pub variant_state: R::VariantState, 
} 