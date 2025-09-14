/// Holds override configuration values for basic Rummy.
#[derive(Clone, Debug)]
pub struct BasicConfig {
    /// Overrides the default number of cards to deal at the start of each round.
    pub deal_amount: Option<usize>,
    /// Overrides the default number of cards a player draws from the deck.
    pub draw_deck_amount: Option<usize>,
    /// Overrides the default number of cards a player draws from the discard pile.
    pub draw_discard_pile_amount: Option<DrawDiscardPileOverride>
}

/// The type of discard pile draw behaviour.
#[derive(Clone, Debug)]
pub enum DrawDiscardPileOverride {
    /// The player can choose how many cards they wish to draw from the discard pile.
    PlayerChooses,
    /// The player must draw the whole discard pile.
    WholePile,
    /// The player always draws this number of cards from the discard pile.
    Constant(usize)
}