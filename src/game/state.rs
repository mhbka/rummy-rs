use crate::cards::deck::Deck;
use crate::player::Player;
use std::collections::HashMap;

/// The state of a Rummy game, common across its variants.
///
/// Takes a generic config `C` and score `S: Score`.
pub struct State<C, S: Score> {
    pub config: C,
    pub score: S,
    pub deck: Deck,
    pub players: Vec<Player>,
    pub cur_round: usize,
    pub cur_player: usize,
}

/// Very minimal trait for returning a hashmap representing each round's players' scores.
pub trait Score {
    /// Returns an immutable reference to the score hashmap.
    fn get(&self) -> &HashMap<usize, HashMap<usize, usize>>;
}
