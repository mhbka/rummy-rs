use crate::game_rewrite::{state::VariantState, variants::basic::{rules::BasicRules, score::BasicScore}};

pub struct BasicState {}

impl VariantState<BasicRules> for BasicState {}