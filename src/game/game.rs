use super::action::*;
use crate::{cards::card::CardData, game::{error::{ActionError, GameError}, rules::GameRules, state::GameState}};

/// Represents a Rummy game.
pub trait Game {
    /// The `GameRules` that this game follows.
    type Rules: GameRules;

    /// Attempt to execute the `GameAction` for the current player.
    /// 
    /// Returns an `Err` if the action couldn't be executed for some reason.
    fn execute_action(&mut self, action: GameAction) -> Result<(), ActionError>;

    /// Inspect the game's current state.
    fn get_state(&self) -> &GameState<<<Self as Game>::Rules as GameRules>::VariantScore, Self::Rules>;

    /// Quit a player whose ID is `player_id`.
    /// 
    /// Returns an `Err` if such player doesn't exist, or the game already ended.
    fn quit_player(&mut self, player_id: usize) -> Result<(), GameError>;

    /// Add a player using the given `player_id`.
    /// 
    /// Returns an `Err` if the maximum number of players has been reached,
    /// or a player with that ID already exists.
    fn add_player(&mut self, player_id: usize) -> Result<(), GameError>;

    /// Rearranges the player's cards according to what is present in `new_arrangement`.
    /// 
    /// Returns an `Err` if such player doesn't exist, 
    /// `new_arrangement` doesn't contain the exact cards currently in that player's hand,
    /// or the game phase isn't `Draw` or `Play`.
    fn rearrange_player_hand(&mut self, player_id: usize, new_arrangement: Vec<CardData>) -> Result<(), GameError>;

    /// Calculate and store round scores and start the next round.
    /// 
    /// Returns an `Err` if the game phase is not `RoundEnd`,
    /// or the setup failed for some reason.
    fn next_round(&mut self) -> Result<(), GameError>;
}

#[cfg(feature = "serde")]
/// Supertraits for a `Game`.
pub trait GameTraits: Sized + Clone + serde::Serialize + for<'a> serde::Deserialize<'a> {}

#[cfg(not(feature = "serde"))]
/// Supertraits for a `Game`.
pub trait GameTraits: Sized + Clone {}

#[cfg(feature = "serde")]
impl<T> GameTraits for T where T: Sized + Clone + serde::Serialize + for<'a> serde::Deserialize<'a> {}

#[cfg(not(feature = "serde"))]
impl<T> GameTraits for T where T: Sized + Clone {}
