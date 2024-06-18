use crate::{
    cards::suit_rank::Rank::*,
    game::state::{Score, State},
    player::Player,
};
use std::collections::HashMap;

/// State for a standard Rummy game.
pub type StandardRummyState = State<StandardRummyConfig, StandardRummyScore>;

/// Keeps the score of a standard Rummy game.
#[derive(Debug)]
pub struct StandardRummyScore {
    score: HashMap<usize, HashMap<usize, usize>>,
}

impl Score for StandardRummyScore {
    fn get(&self) -> &HashMap<usize, HashMap<usize, usize>> {
        &self.score
    }
}

impl StandardRummyScore {
    /// Initialize a new score struct.
    pub(super) fn new() -> Self {
        StandardRummyScore {
            score: HashMap::new(),
        }
    }

    /// Scores a set of players using the card values found [here](https://en.wikipedia.org/wiki/Rummy),
    /// and sets it for the current round.
    ///
    /// If `score_winner_only`, all other players' hand's values will be added as the winner's score;
    /// else, each player is scored individually on their own hand's value.
    pub(super) fn calculate(
        &mut self,
        scoreable_players: &Vec<&Player>,
        round: usize,
        score_winner_only: bool,
    ) {
        let individual_scores = StandardRummyScore::score_all(scoreable_players);

        let round_score = match self.score.get_mut(&round) {
            Some(round_score) => round_score,
            None => {
                self.score.insert(round, HashMap::new());
                self.score.get_mut(&round).unwrap()
            }
        };

        if !score_winner_only {
            for i in 0..scoreable_players.len() {
                round_score.insert(scoreable_players[i].id, individual_scores[i]);
            }
        } else {
            let winner_score = individual_scores.iter().fold(0, |acc, &s| acc + s);
            let &winner = scoreable_players
                .iter()
                .find(|p| p.cards.len() == 0)
                .expect("The game must have a winner with 0 cards in hand");
            scoreable_players.iter().for_each(|&p| {
                // give winner his score, and everyone else 0
                if std::ptr::eq(winner, p) {
                    round_score.insert(winner.id, winner_score);
                } else {
                    round_score.insert(p.id, 0);
                }
            })
        }
    }

    /// Return a `Vec` where each element is the corresponding player's score.
    fn score_all(scoreable_players: &Vec<&Player>) -> Vec<usize> {
        scoreable_players
            .iter()
            .map(|&p| {
                p.cards.iter().fold(0, |score, card| {
                    score
                        + match card.rank {
                            Ace => 15,
                            King | Queen | Jack | Ten => 10,
                            Joker => 0,
                            rank => rank as usize,
                        }
                })
            })
            .collect()
    }
}

/// The configurable options of a standard Rummy game.
#[derive(Debug)]
pub struct StandardRummyConfig {
    /// Whether only the winner is scored by the total of all other players' hands,
    ///
    /// where the **overall winner has the highest score**,
    ///
    /// or all players are scored by their own hand,
    ///
    /// where the **overall winner has the lowest score**.
    pub score_winner_only: bool,

    /// Whether a player forfeits their cards and score if they quit, or keep the cards
    /// and get scored on the current state.
    pub forfeit_cards_on_quit: bool,

    /// Whether, once the deck stock is depleted and the discard pile is added into it,
    /// to shuffle the stock or just leave it turnt over.
    pub shuffle_stock_upon_depletion: bool,

    /// Whether or not to use a rank as a wildcard, which increases on each round.
    /// (for eg, Round 1 -> 2, Round 2 -> 3, Round 3 -> 4 ...)
    pub increasing_wildcard_rank: bool,

    /// How much of the discard pile can be drawn.
    /// - If `None`, the player can choose how many to draw.
    /// - If `Some(usize::MAX)`, the player must always take the entire discard pile.
    /// - Else, the player draws the specified amount (or the entire pile, if its size is smaller).
    pub discard_pile_draw_amount: Option<usize>,
}

impl StandardRummyConfig {
    /// Configure the game based on the rules [here](https://en.wikipedia.org/wiki/Rummy).
    ///
    /// To initialize with your own settings, simply create this struct with its fields.
    pub fn new() -> Self {
        StandardRummyConfig {
            score_winner_only: true,
            forfeit_cards_on_quit: true,
            shuffle_stock_upon_depletion: false,
            increasing_wildcard_rank: false,
            discard_pile_draw_amount: Some(1),
        }
    }
}
