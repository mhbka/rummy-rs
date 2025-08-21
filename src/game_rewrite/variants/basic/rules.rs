use crate::{cards::meld::Meldable, game_rewrite::{action::*, error::{ActionError, FailedActionError, InternalError}, game::GameRules, score::RoundScore, state::{GamePhase, GameState}, variants::basic::{score::BasicScore, state::BasicState}}};

/// The rules for basic Rummy.
pub struct BasicRules {}

impl GameRules for BasicRules {
    type VariantState = BasicState;
    type VariantScore = BasicScore;

    fn handle_draw_deck(&mut self, state: &mut GameState<Self>, action: DrawDeckAction) -> Result<ActionOutcome, ActionError> {
        let mut card = state.deck
            .draw(1)
            .map_err(|err| {
                let err = InternalError::NoCardsInDeckOrDiscardPile;
                ActionError::Internal(err)
            })?;
        let player = state.players
            .get_mut(state.current_player)
            .ok_or(ActionError::Internal(
                InternalError::InvalidCurrentPlayer { 
                    current: state.current_player
                }
            ))?;
        player.cards.append(&mut card);
        state.phase = GamePhase::Play;
        Ok(ActionOutcome::Continue)
    }

    fn handle_draw_discard_pile(&mut self, state: &mut GameState<Self>, action: DrawDiscardPileAction) -> Result<ActionOutcome, ActionError> {
        let mut card = state.deck
            .draw_discard_pile(Some(1))
            .map_err(|err| {
                let err = FailedActionError::DiscardPileTooSmall;
                ActionError::FailedAction(err)
            })?;
        let player = state.players
            .get_mut(state.current_player)
            .ok_or(ActionError::Internal(
                InternalError::InvalidCurrentPlayer { 
                    current: state.current_player
                }
            ))?;
        player.cards.append(&mut card);
        state.phase = GamePhase::Play;
        Ok(ActionOutcome::Continue)
    }

    fn handle_lay_off(&mut self, state: &mut GameState<Self>, action: LayOffAction) -> Result<ActionOutcome, ActionError> {
        let players = &mut state.players;
        let cur_player_hand = &mut players
            .get_mut(state.current_player)
            .ok_or(ActionError::Internal(
                InternalError::InvalidCurrentPlayer { 
                    current: state.current_player
                }
            ))?
            .cards;
        let target_player = players
            .get_mut(action.target_player_index)
            .ok_or(ActionError::FailedAction(FailedActionError::InvalidPlayerIndex))?;
        let target_meld = target_player.melds
            .get_mut(action.target_meld_index)
            .ok_or(ActionError::FailedAction(FailedActionError::InvalidMeldIndex))?;
        target_meld
            .layoff_card(cur_player_hand, action.position)
            .map_err(|err| FailedActionError::FailedMeld(err))?;
        Ok(ActionOutcome::Continue)
    }

    fn handle_form_meld(&mut self, state: &mut GameState<Self>, action: FormMeldAction) -> Result<ActionOutcome, ActionError> {
        todo!()
    }

    fn handle_form_melds(&mut self, state: &mut GameState<Self>, action: FormMeldsAction) -> Result<ActionOutcome, ActionError> {
        todo!()
    }

    fn handle_discard(&mut self, state: &mut GameState<Self>, action: DiscardAction) -> Result<ActionOutcome, ActionError> {
        todo!()
    }

    fn calculate_round_score(&mut self, state: &GameState<Self>) -> Result<RoundScore<Self::VariantScore>, ActionError> {
        todo!()
    }
    
    fn cards_to_deal(&self, state: &GameState<Self>) -> usize {
        todo!()
    }
    
    fn starting_player_index(&self, state: &GameState<Self>) -> usize {
        todo!()
    }
}