use crate::game_rewrite::{action::*, error::ActionError, game::GameRules, score::RoundScore, state::GameState, variants::basic::{score::BasicScore, state::BasicState}};

pub struct BasicRules {}

impl GameRules for BasicRules {
    type VariantState = BasicState;
    type VariantScore = BasicScore;

    fn handle_draw_deck(&mut self, state: &mut GameState<Self>, action: DrawDeckAction) -> Result<ActionOutcome, ActionError> {
        todo!()
    }

    fn handle_draw_discard_pile(&mut self, state: &mut GameState<Self>, action: DrawDiscardPileAction) -> Result<ActionOutcome, ActionError> {
        todo!()
    }

    fn handle_lay_off(&mut self, state: &mut GameState<Self>, action: LayOffAction) -> Result<ActionOutcome, ActionError> {
        todo!()
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

    fn calculate_round_score(&mut self, state: &mut GameState<Self>) -> Result<RoundScore<Self::VariantScore>, ActionError> {
        todo!()
    }
}