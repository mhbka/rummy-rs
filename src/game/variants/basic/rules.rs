use std::collections::HashMap;
use crate::{cards::meld::{Meld, Meldable}, game::{action::*, error::{ActionError, FailedActionError, GameError, InternalError}, rules::GameRules, score::RoundScore, state::{GamePhase, GameState}, variants::basic::{config::{BasicConfig, DrawDiscardPileOverride}, score::BasicScore, state::BasicState}}};

/// The rules for basic Rummy.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BasicRules {
    config: BasicConfig
}

impl BasicRules {
    /// Initialize the rules.
    pub fn new(config: BasicConfig) -> Self {
        Self { config }
    }

    /// The number of cards to deal at the start of a round.
    pub(super) fn cards_to_deal(&self, state: &GameState<BasicScore, BasicRules>) -> usize {
        if let Some(count) = self.config.deal_amount {
            return count;
        }
        let active_players = state.players
            .iter()
            .filter(|p| p.active || p.joined_in_round == state.current_round)
            .count();

        match active_players {
            2 => 10,
            3..=5 => 7,
            6 => 6,
            _ => 10 // NOTE: if >6 players, at least 2 decks are required
        }
    }

    /// The number of cards to draw from the deck.
    pub(super) fn cards_to_draw_from_deck(&self, state: &GameState<BasicScore, BasicRules>) -> usize {
        if let Some(value) = &self.config.draw_deck_amount {
            *value
        } else {
            1
        }
    }

    /// The number of cards to draw from the discard pile.
    pub(super) fn cards_to_draw_from_discard_pile(&self, state: &GameState<BasicScore, BasicRules>, given_amount: usize) -> usize {
        if let Some(value) = &self.config.draw_discard_pile_amount {
            match value {
                DrawDiscardPileOverride::PlayerChooses => given_amount,
                DrawDiscardPileOverride::WholePile => state.deck.discard_pile().len(),
                DrawDiscardPileOverride::Constant(amount) => *amount
            }
        } else {
            1
        }
    }
    
    /// Returns the player index who should start in a round.
    pub(super) fn starting_player_index(&self, state: &GameState<BasicScore, BasicRules>,) -> usize {
        let active_players = state.players
            .iter()
            .filter(|p| p.active)
            .count();
        state.current_round % active_players
    }
}

impl GameRules for BasicRules {
    type VariantState = BasicState;
    type VariantScore = BasicScore;

    fn handle_draw_deck(&self, state: &mut GameState<BasicScore, BasicRules>, action: DrawDeckAction) -> Result<(), ActionError> {
        let mut card = state.deck
            .draw(self.cards_to_draw_from_deck(state))
            .map_err(|err| InternalError::NoCardsInDeckOrDiscardPile)?;
        let player = state.get_current_player_mut()?;
        player.cards.append(&mut card);

        state.phase = GamePhase::Play;

        Ok(())
    }

    fn handle_draw_discard_pile(&self, state: &mut GameState<BasicScore, BasicRules>, action: DrawDiscardPileAction) -> Result<(), ActionError> {
        let requested_amount = if let Some(val) = action.count { val as usize } else { 1 };
        let draw_amount = self.cards_to_draw_from_discard_pile(state, requested_amount);

        let mut card = state.deck
            .draw_discard_pile(draw_amount)
            .map_err(|err| FailedActionError::DiscardPileTooSmall)?;
        let player = state.get_current_player_mut()?;
        player.cards.append(&mut card);
        
        state.phase = GamePhase::Play;

        Ok(())
    }

    fn handle_lay_off(&self, state: &mut GameState<BasicScore, BasicRules>, action: LayOffAction) -> Result<(), ActionError> {
        if action.target_player_index >= state.players.len() {
            return Err(ActionError::FailedAction(FailedActionError::InvalidPlayerIndex));
        }
        if state.current_player >= state.players.len() {
            return Err(ActionError::Internal(
                InternalError::InvalidCurrentPlayer { 
                    current: state.current_player
                }
            ));
        }

        if state.current_player == action.target_player_index {
            // Laying off to own meld
            let current_player = state.get_current_player_mut()?;
            let target_meld = current_player.melds
                .get_mut(action.target_meld_index)
                .ok_or(FailedActionError::InvalidMeldIndex)?;
            target_meld
                .layoff_card(&mut current_player.cards, action.card_index)
                .map_err(|err| FailedActionError::FailedMeld(err))?;
        } else {
            // Different players - use `split_at_mut`, otherwise we get multiple mut references
            let (current_idx, target_idx) = (state.current_player, action.target_player_index);
            let (min_idx, max_idx) = if current_idx < target_idx { 
                (current_idx, target_idx) 
            } else { 
                (target_idx, current_idx) 
            };
            let (left, right) = state.players.split_at_mut(max_idx);
            let (current_player, target_player) = if current_idx < target_idx {
                (&mut left[current_idx], &mut right[0])
            } else {
                (&mut right[0], &mut left[target_idx])
            };
            let target_meld = target_player.melds
                .get_mut(action.target_meld_index)
                .ok_or(FailedActionError::InvalidMeldIndex)?;
            target_meld
                .layoff_card(&mut current_player.cards, action.card_index)
                .map_err(|err| FailedActionError::FailedMeld(err))?;
        }
        
        if state.get_current_player_mut()?.cards.len() == 0 {
            state.phase = GamePhase::RoundEnd;
        }

        Ok(())
    }

    fn handle_form_meld(&self, state: &mut GameState<BasicScore, BasicRules>, action: FormMeldAction) -> Result<(), ActionError> {
        let player = state.get_current_player_mut()?;
        let meld = Meld::new(&mut player.cards, &action.card_indices)
            .map_err(|err| FailedActionError::FailedMeld(err))?;
        player.melds.push(meld);

        if player.cards.len() == 0 {
            state.phase = GamePhase::RoundEnd;
        }

        Ok(())
    }

    fn handle_form_melds(&self, state: &mut GameState<BasicScore, BasicRules>, mut action: FormMeldsAction) -> Result<(), ActionError> {
        let player = state.get_current_player_mut()?;
        let mut melds = Meld::multiple(&mut player.cards, &mut action.melds)
            .map_err(|err| FailedActionError::FailedMeld(err))?;
        player.melds
            .append(&mut melds);

        if player.cards.len() == 0 {
            state.phase = GamePhase::RoundEnd;
        }

        Ok(())
    }

    fn handle_discard(&self, state: &mut GameState<BasicScore, BasicRules>, action: DiscardAction) -> Result<(), ActionError> {
        let player = state.get_current_player_mut()?;
        if action.card_index > player.cards.len() {
            let err = FailedActionError::InvalidCardIndex;
            return Err(
                ActionError::FailedAction(err)
            );
        }
        let discarded_card = player.cards.remove(action.card_index);
        state.deck.add_to_discard_pile(discarded_card);
        match state.get_current_player_mut()?.cards.len() {
            0 => {
                state.phase = GamePhase::RoundEnd;
                Ok(())
            },
            _ => {
                state.phase = GamePhase::Draw;
                state.to_next_player();
                Ok(())
            }
        }
    }

    fn calculate_round_score(&self, state: &GameState<BasicScore, BasicRules>) -> Result<RoundScore<Self::VariantScore>, GameError> {
        if state.phase != GamePhase::RoundEnd {
            return Err(GameError::WrongGamePhase);
        }
        let player_scores: HashMap<_, _> = state.players
            .iter()
            .map(|player| (player.id(), BasicScore::score_player(player)))
            .collect();
        let winner_id = *player_scores
            .iter()
            .find(|(_, score)| score.score() == 0)
            .ok_or(InternalError::RoundHasNoWinner)?
            .0;
        let round_score = RoundScore {
            player_scores,
            winner_id
        };
        Ok(round_score)
    }
}