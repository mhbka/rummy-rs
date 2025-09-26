#![doc = include_str!("../README.md")]

pub mod cards;
pub mod game;
pub mod player;
pub mod wrappers;

#[cfg(feature = "serde")]
pub mod serialization;
