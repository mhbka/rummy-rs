use crate::game_rewrite::{state::VariantState, variants::basic::{rules::BasicRules, score::BasicScore}};

/// Basic Rummy requires no additional state, so this is an empty struct.
pub struct BasicState {}

impl VariantState<BasicRules> for BasicState {}