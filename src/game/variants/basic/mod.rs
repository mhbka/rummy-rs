//! The basic form of Rummy. By default, follows the rules defined [here](https://en.wikipedia.org/wiki/Rummy#Basic_rummy).
//! However, you can override some things with a [`BasicConfig`](config::BasicConfig) and [`DeckConfig`](crate::cards::deck::DeckConfig).
//!
//! You can find the actual game in [`game`].

pub mod config;
pub mod game;
pub mod rules;
pub mod score;
pub mod state;
