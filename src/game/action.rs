use crate::cards::card::CardData;

/// The possible interactions with a `Game`.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GameInteractions {
    Action(GameAction),
    PlayerJoin {
        player_id: usize,
    },
    PlayerQuit {
        player_id: usize,
    },
    HandRearrangement {
        player_id: usize,
        new_arrangement: Vec<CardData>,
    },
}

/// The possible actions taken in a Rummy game.
///
/// Each action aims to include all possible data required by any (mainstream) Rummy variant.
/// Each variant can use just the data that it requires, and ignore/return errors for unnecessary/invalid data.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DrawDeckAction {}

/// Represents drawing from the discard pile.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DrawDiscardPileAction {
    /// For variants that allow taking a chosen number of cards from the discard pile.
    pub count: Option<u8>,
}

/// Represents laying off a card into an existing meld.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LayOffAction {
    /// The index of the card in the current player's hand to lay off.
    pub card_index: usize,
    /// The index of the user owning the targeted meld.
    pub target_player_index: usize,
    /// The index of the meld on the table to add the card to.
    pub target_meld_index: usize,
}

/// Represents forming a single meld.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FormMeldAction {
    /// The indices of cards in the current player's hand to form into a meld.
    pub card_indices: Vec<usize>,
}

/// Represents forming multiple melds at once.
///
/// ## Note
/// The user is responsible for ensuring no overlapping of card indexes for each meld.
/// If there are overlaps, an error will be returned when attempting to execute this action.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FormMeldsAction {
    /// The list of indices of cards to each form into a meld.
    pub melds: Vec<Vec<usize>>,
}

/// Represents discarding a card.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DiscardAction {
    /// The index of the card in the current player's hand to discard.
    pub card_index: usize,
    /// For variants that require declaring when going out with this discard.
    pub declare_going_out: Option<bool>,
}
