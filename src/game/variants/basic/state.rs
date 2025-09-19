use crate::game::{state::VariantState, variants::basic::{rules::BasicRules, score::BasicScore}};

/// Basic Rummy requires no additional state, so this is an empty struct.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BasicState {}

impl VariantState<BasicScore, BasicRules> for BasicState {}