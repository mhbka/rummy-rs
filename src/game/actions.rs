use super::state::{Score, State};

/// A result for methods which may branch into different game phases:
/// - `Next`: Transition to the intended next phase, `N`
/// - `End`: Transition to `RoundEndPhase` as `E` if the round ends due to some condition
/// - `Error`: Fail the transition, returning a tuple of `S` (game in current phase) and an error `Err`
pub enum TransitionResult<N, E: RoundEndActions, S, Err> {
    Next(N),
    End(E),
    Error((S, Err)),
}

/// Trait for actions during DrawPhase.
pub trait DrawActions {
    // `Self` in `PlayPhase`.
    type SelfInPlayPhase: PlayActions;

    /// Draw from the stock for the current player.
    ///
    /// Resets the stock if it is empty after drawing, or if not enough cards are present.
    fn draw_stock(&mut self);

    /// Draw from the discard pile for the current player.
    ///
    /// `amount` is the number of cards to draw, where `None` means draw the entire discard pile.
    ///
    /// This is provided as some variants of Rummy (ie [Basic Rummy](https://en.wikipedia.org/wiki/Rummy))
    /// may allow the player to choose how many discard cards to draw.
    ///
    /// If the variant doesn't allow this, its implementation can just ignore `amount` and use a default value.
    fn draw_discard_pile(&mut self, amount: Option<usize>) -> Result<(), String>;

    /// Transition to next state where the current player can make plays. 
    /// 
    /// Automatically calls `draw_stock` if it hasn't been called at this point.
    fn to_play_phase(self) -> Self::SelfInPlayPhase;
}

/// Trait for actions during PlayPhase.
pub trait PlayActions: Sized {
    // `Self` in `DiscardPhase` and `RoundEndPhase`.
    type SelfInDiscardPhase: DiscardActions;
    type SelfInRoundEndPhase: RoundEndActions;

    /// Form a meld from a Vec of indices from the current player's hand.
    ///
    /// If the game ends with this play, returns an `End`; else, returns `Next` containing `Self`.
    ///
    /// If
    fn form_meld(
        self,
        card_indices: Vec<usize>,
    ) -> TransitionResult<Self, Self::SelfInRoundEndPhase, Self, String>;

    /// Layoff `card_i` card in the current player's hand,
    /// to `target_player_i` player's `target_meld_i` meld.
    ///
    /// If the game ends with this play, returns an `End`; else, returns `Next` containing `Self`.
    ///
    /// If any of the indices are invalid, or the layoff is invalid, returns an `Error`.
    fn layoff_card(
        self,
        card_i: usize,
        target_player_i: usize,
        target_meld_i: usize,
    ) -> TransitionResult<Self, Self::SelfInRoundEndPhase, Self, String>;

    /// Transition to the next state where the current player can discard.
    ///
    /// **NOTE**: Ensure any required actions are taken by the time/during this call,
    /// as it is infallible.
    fn to_discard_phase(self) -> Self::SelfInDiscardPhase;
}

/// Trait for actions during DiscardPhase.
pub trait DiscardActions: Sized {
    // `Self` in `PlayPhase` and `RoundEndPhase`.
    type SelfInDrawPhase: DrawActions;
    type SelfInRoundEndPhase: RoundEndActions;

    /// Discard a card for current player at given index in their hand.
    ///
    /// If the game ends with this discard, returns an `End`; else, returns `Next` containing `Self`.
    ///
    /// If `card_i` is invalid, returns an `Error`.
    fn discard(
        self,
        card_i: usize,
    ) -> TransitionResult<Self, Self::SelfInRoundEndPhase, Self, String>;

    /// Transition to the next state by going to the next active player
    /// where they can draw.
    ///
    /// **NOTE**: As this function calls `discard` automatically if it hasn't been called yet,
    /// it also returns a `TransitionResult`.
    fn to_next_player(
        self,
    ) -> TransitionResult<Self::SelfInDrawPhase, Self::SelfInRoundEndPhase, Self, String>;
}

/// Trait for actions during RoundEndPhase.
pub trait RoundEndActions {
    // `Self` in `PlayPhase` and `RoundEndPhase`.
    type SelfInDrawPhase: DrawActions;

    /// Calculate the round's score for players who are active,
    /// or who have just quit.
    fn calculate_score(&mut self);

    /// Start a new round. Typically includes:
    /// - Incrementing the round number
    /// - Set players who joined in previous round to active
    /// - Reset all cards and deal to players
    /// - Start at `DrawPhase`
    ///
    /// **NOTE**: Ensure that score is automatically calculated if it hasn't been when this is called,
    /// as the transition is infallible.
    fn to_next_round(self) -> Self::SelfInDrawPhase;
}

/// Trait for actions during GameEndPhase.
pub trait GameEndActions {
    // TODO: what makes sense here?
}

/// Trait for actions during any phase.
pub trait AllActions<C, S: Score> {
    /// View the game's state through an immutable reference.
    fn view_state(&self) -> &State<C, S>;
}

/// Trait for actions during any playable phase.
pub trait PlayableActions: Sized {
    type SelfInRoundEndPhase: RoundEndActions;
    type SelfInDrawPhase: DrawActions;

    /// Add a player to the game.
    ///
    /// If `index` is given, add them at that index in `players`;
    /// else, or if `index` is greater than the number of players,
    /// add them at the last position of `players`.
    ///
    /// If the player was added while a round is ongoing, add them as inactive,
    /// and start them next round.
    ///
    /// If a player with `player_id` already exists, return `Err`.
    ///
    /// If there are too many players (depending on the variant's ruling), return `Err`.
    fn add_player(&mut self, player_id: usize, index: Option<usize>) -> Result<(), String>;

    /// Sets a (non-current) player as having quit.
    /// If only 1 active player is left, ends the round.
    ///
    /// Returns `Error` if `player_i` is the current player.
    /// To quit the current player, use `quit_current_player` instead.
    fn quit_player(
        self,
        player_i: usize,
    ) -> TransitionResult<Self, Self::SelfInRoundEndPhase, Self, String>;

    /// Sets the current player as having quit, advancing to the next player
    /// and going to `DrawPhase`.
    fn quit_current_player(self) -> Self::SelfInDrawPhase;

    /// Moves the specified player's hand's card at `old_pos` to `new_pos`.
    ///
    /// If `player_i` or `old_pos` is invalid, an `Err` is returned.
    /// If `new_pos` is greater than the player's hand size, the card is moved to the rightmost position.
    fn move_card_in_hand(
        &mut self,
        player_i: usize,
        old_pos: usize,
        new_pos: usize,
    ) -> Result<(), String>;

    /// Sort a player's hand by rank, then suit.
    ///
    /// If `player_i` is invalid, an `Err` is returned.
    fn sort_hand(&mut self, player_i: usize) -> Result<(), String>;
}
