/// The possible actions taken in a Rummy game.
/// 
/// Each action aims to include all possible data required by any (mainstream) Rummy variant.
/// Each variant can use just the data that it requires, and ignore/return errors for unnecessary/invalid data.
#[derive(Debug, Clone)]
pub enum GameAction {
    /// Draw from the deck.
    DrawDeck(DrawDeckAction),
    /// Draw from the discard pile.
    DrawDiscardPile(DrawDiscardPileAction),
    /// Layoff a card onto a meld.
    LayOff(LayOffAction),
    /// Form a single meld.
    FormMeld(FormMeldAction),
    /// Form multiple melds at once.
    FormMelds(FormMeldsAction),
    /// Discard and end the turn.
    Discard(DiscardAction),
}

/// Represents drawing from the deck.
#[derive(Debug, Clone)]
pub struct DrawDeckAction {
    /// For variants that allow drawing multiple cards from the deck.
    pub count: Option<u8>,
}

/// Represents drawing from the discard pile.
#[derive(Debug, Clone)]
pub struct DrawDiscardPileAction {
    /// For variants that allow taking multiple cards from the discard pile.
    pub count: Option<u8>,
    /// For variants that impose restrictions on what can be done after drawing from the discard pile.
    pub must_use_immediately: Option<bool>,
}

/// Represents laying off a card into an existing meld.
#[derive(Debug, Clone)]
pub struct LayOffAction {
    /// The index of the card in the current player's hand to lay off.
    pub card_index: usize,
    /// The ID of the meld on the table to add the card to.
    pub target_meld_id: usize,
    /// For run melds, specifies where in the sequence to place the card.
    pub position: Option<MeldPosition>,
}

#[derive(Debug, Clone)]
pub enum MeldPosition {
    Start,
    End,
    /// For variants that allow insertion at specific positions within a meld.
    Index(usize),
}

/// Represents forming a single meld.
#[derive(Debug, Clone)]
pub struct FormMeldAction {
    /// The indices of cards in the current player's hand to form into a meld.
    pub card_indices: Vec<usize>,
}

/// Represents forming multiple melds at once.
#[derive(Debug, Clone)]
pub struct FormMeldsAction {
    /// The individual meld formations to execute simultaneously.
    pub melds: Vec<FormMeldAction>,
}

/// Represents discarding a card.
#[derive(Debug, Clone)]
pub struct DiscardAction {
    /// The index of the card in the current player's hand to discard.
    pub card_index: usize,
    /// For variants that require declaring when going out with this discard.
    pub declare_going_out: Option<bool>,
}

/// The possible outcomes of a `GameAction`.
#[derive(Debug, Clone, PartialEq)]
pub enum ActionOutcome {
    Continue,
    PlayerWentOut,
    RoundEnded,
    GameEnded,
}