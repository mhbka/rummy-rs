use std::collections::HashMap;
use crate::{cards::meld::{Meld, Meldable}, game_rewrite::{action::*, error::{ActionError, FailedActionError, GameError, InternalError}, game::GameRules, score::{RoundScore, VariantPlayerScore}, state::{GamePhase, GameState}, variants::basic::{score::BasicScore, state::BasicState}}};

/// We always draw 1 card from the deck in Rummy.
const DECK_DRAW_AMT: usize = 1;

/// We always draw 1 card from the discard pile in Rummy.
const DISCARD_PILE_DRAW_AMT: usize = 1;


/// The rules for basic Rummy.
pub struct BasicRules {}

impl GameRules for BasicRules {
    type VariantState = BasicState;
    type VariantScore = BasicScore;

    /// Draws a single card from the deck, ignoring `action`.
    fn handle_draw_deck(&mut self, state: &mut GameState<BasicScore, BasicRules>, action: DrawDeckAction) -> Result<(), ActionError> {
        let mut card = state.deck
            .draw(DECK_DRAW_AMT)
            .map_err(|err| InternalError::NoCardsInDeckOrDiscardPile)?;
        let player = state.get_current_player_mut()?;
        player.cards.append(&mut card);

        state.phase = GamePhase::Play;

        Ok(())
    }

    /// Draws a single card from the discard pile, ignoring `action`.
    fn handle_draw_discard_pile(&mut self, state: &mut GameState<BasicScore, BasicRules>, action: DrawDiscardPileAction) -> Result<(), ActionError> {
        let mut card = state.deck
            .draw_discard_pile(Some(DISCARD_PILE_DRAW_AMT))
            .map_err(|err| FailedActionError::DiscardPileTooSmall)?;

        let player = state.get_current_player_mut()?;
        player.cards.append(&mut card);
        
        state.phase = GamePhase::Play;

        Ok(())
    }

    fn handle_lay_off(&mut self, state: &mut GameState<BasicScore, BasicRules>, action: LayOffAction) -> Result<(), ActionError> {
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
                .layoff_card(&mut current_player.cards, action.position)
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
                .layoff_card(&mut current_player.cards, action.position)
                .map_err(|err| FailedActionError::FailedMeld(err))?;
        }
        
        if state.get_current_player_mut()?.cards.len() == 0 {
            state.phase = GamePhase::RoundEnd;
        }

        Ok(())
    }

    fn handle_form_meld(&mut self, state: &mut GameState<BasicScore, BasicRules>, action: FormMeldAction) -> Result<(), ActionError> {
        let player = state.get_current_player_mut()?;
        let meld = Meld::new(&mut player.cards, &action.card_indices)
            .map_err(|err| FailedActionError::FailedMeld(err))?;
        player.melds.push(meld);

        if player.cards.len() == 0 {
            state.phase = GamePhase::RoundEnd;
        }

        Ok(())
    }

    fn handle_form_melds(&mut self, state: &mut GameState<BasicScore, BasicRules>, mut action: FormMeldsAction) -> Result<(), ActionError> {
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

    fn handle_discard(&mut self, state: &mut GameState<BasicScore, BasicRules>, action: DiscardAction) -> Result<(), ActionError> {
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

    fn calculate_round_score(&mut self, state: &GameState<BasicScore, BasicRules>) -> Result<RoundScore<Self::VariantScore>, GameError> {
        if state.phase != GamePhase::RoundEnd {
            return Err(
                GameError::RoundHasntEnded
            );
        }
        let player_scores: HashMap<_, _> = state.players
            .iter()
            .map(|player| (player.id(), BasicScore::score_player(player)))
            .collect();
        let winner_id = *player_scores
            .iter()
            .find(|(_, score)| score.score_value() == 0)
            .ok_or(GameError::RoundHasNoWinner)?
            .0;
        let round_score = RoundScore {
            player_scores,
            winner_id
        };
        Ok(round_score)
    }
}