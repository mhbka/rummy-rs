use super::state::*;
use super::*;
use crate::game::{actions::*, phases::*};
use crate::{
    cards::meld::{Meld, Meldable},
    player::Player,
};

impl DrawActions for StandardRummy<DrawPhase> {
    type SelfInPlayPhase = StandardRummy<PlayPhase>;

    fn draw_stock(&mut self) {
        let card = &mut self
            .state
            .deck
            .draw(1)
            .expect("Drawing 1 card should never cause an error"); // as we check and replenish below

        self.state.players[self.state.cur_player].cards.append(card);

        if self.state.deck.stock().len() == 0 {
            self.state.deck.turnover_discarded();
        }

        self.phase.has_drawn = true;
    }

    fn draw_discard_pile(&mut self, amount: Option<usize>) -> Result<(), String> {
        self.state.players[self.state.cur_player]
            .cards
            .append(&mut self.state.deck.draw_discard_pile(amount)?);

        self.phase.has_drawn = true;

        Ok(())
    }

    fn to_play_phase(mut self) -> Self::SelfInPlayPhase {
        if !self.phase.has_drawn {
            self.draw_stock();
        }
        StandardRummy {
            phase: PlayPhase { play_count: 0 },
            state: self.state,
        }
    }
}

impl PlayActions for StandardRummy<PlayPhase> {
    type SelfInDiscardPhase = StandardRummy<DiscardPhase>;
    type SelfInRoundEndPhase = StandardRummy<RoundEndPhase>;

    fn form_meld(
        mut self,
        card_indices: Vec<usize>,
    ) -> TransitionResult<Self, Self::SelfInRoundEndPhase, Self, String> {
        if card_indices.len() < 3 {
            return TransitionResult::Error((
                self,
                "card_indices has less than 3 elements; need at least 3 for a meld".into(),
            ));
        }

        let player = &mut self.cur_player();

        match Meld::new(&mut player.cards, &card_indices) {
            Ok(meld) => {
                player.melds.push(meld);
                return TransitionResult::Next(self);
            }
            Err(err) => {
                return TransitionResult::Error((self, err));
            }
        }
    }

    fn layoff_card(
        mut self,
        card_i: usize,
        target_player_i: usize,
        target_meld_i: usize,
    ) -> TransitionResult<Self, Self::SelfInRoundEndPhase, Self, String> {
        let err_string;

        // check that all indices are valid first
        if card_i >= self.cur_player().cards.len() {
            err_string = "card_i is greater than current player's hand size".into();
        } else if target_player_i >= self.state.players.len() {
            err_string = "target_player_i is greater than number of players".into();
        } else if !self.state.players[target_player_i].active {
            err_string = "Target player is not active".into();
        } else if target_meld_i >= self.state.players[target_player_i].melds.len() {
            err_string = "target_meld_i is greater than target player's number of melds".into();
        } else {
            let mut meld = self.state.players[target_player_i]
                .melds
                .remove(target_meld_i); // move so i don't do &mut self simultaneously ...
            let layoff_result = meld.layoff_card(&mut self.cur_player().cards, card_i); // ... with here ...
            self.state.players[target_player_i] // ... then i put it back here.
                .melds
                .insert(target_meld_i, meld);

            match layoff_result {
                Ok(_) => {
                    if self.cur_player().cards.len() == 0 {
                        // if all cards are gone, this player has won
                        return TransitionResult::End(StandardRummy {
                            phase: RoundEndPhase {
                                has_scored_round: false,
                            },
                            state: self.state,
                        });
                    } else {
                        return TransitionResult::Next(self);
                    }
                }

                Err(err) => err_string = err,
            }
        }

        TransitionResult::Error((self, err_string.into()))
    }

    fn to_discard_phase(self) -> Self::SelfInDiscardPhase {
        StandardRummy {
            phase: DiscardPhase {
                has_discarded: false,
            },
            state: self.state,
        }
    }
}

impl DiscardActions for StandardRummy<DiscardPhase> {
    type SelfInDrawPhase = StandardRummy<DrawPhase>;
    type SelfInRoundEndPhase = StandardRummy<RoundEndPhase>;

    fn discard(
        mut self,
        card_i: usize,
    ) -> TransitionResult<Self, Self::SelfInRoundEndPhase, Self, String> {
        if self.phase.has_discarded {
            return TransitionResult::Error((self, "Player has already discarded a card".into()));
        }

        let player_cards = &mut self.state.players[self.state.cur_player].cards;

        let no_player_cards = player_cards.len();

        if card_i >= no_player_cards {
            return TransitionResult::Error((
                self,
                format!(
                    "card_i ({}) is greater than player's hand size ({})",
                    card_i, no_player_cards
                ),
            ));
        }

        let card = player_cards.remove(card_i);
        self.state.deck.add_to_discard_pile(&mut vec![card]);

        if player_cards.len() == 0 {
            TransitionResult::End(StandardRummy {
                phase: RoundEndPhase {
                    has_scored_round: false,
                },
                state: self.state,
            })
        } else {
            self.phase.has_discarded = true;
            TransitionResult::Next(self)
        }
    }

    fn to_next_player(
        mut self,
    ) -> TransitionResult<Self::SelfInDrawPhase, Self::SelfInRoundEndPhase, Self, String> {
        // automatically discard the first card if discard hasn't been called yet,
        if !self.phase.has_discarded {
            match self.discard(0) {
                TransitionResult::Next(s) => self = s,
                TransitionResult::End(e) => return TransitionResult::End(e),
                TransitionResult::Error(_) => unreachable!(), // discarding first card should never error
            }
        }

        let mut state = self.state;

        // find the next active player
        state.cur_player = (state.cur_player + 1) % state.players.len();
        while !state.players[state.cur_player].active {
            state.cur_player = (state.cur_player + 1) % state.players.len();
        }

        TransitionResult::Next(StandardRummy {
            phase: DrawPhase { has_drawn: false },
            state,
        })
    }
}

impl RoundEndActions for StandardRummy<RoundEndPhase> {
    type SelfInDrawPhase = StandardRummy<DrawPhase>;

    fn calculate_score(&mut self) {
        self.phase.has_scored_round = true;

        let scoreable_players = self
            .state
            .players
            .iter()
            .filter(|p| {
                // if forfeiting cards, only look at active players;
                // if not, look at all players with cards
                (self.config().forfeit_cards_on_quit && p.active)
                    || (!self.config().forfeit_cards_on_quit && p.cards.len() > 0)
            })
            .collect();

        self.state.score.calculate(
            &scoreable_players,
            self.state.cur_round,
            self.config().score_winner_only,
        )
    }

    fn to_next_round(mut self) -> Self::SelfInDrawPhase {
        if !self.phase.has_scored_round {
            self.calculate_score();
        }

        let mut state = self.state;
        state.deck.reset();

        // clear all players' cards, set players who just joined to active,
        // and tally up active players
        let mut num_active_players = 0;
        for player in &mut state.players {
            player.melds.clear();
            player.cards.clear();
            if player.joined_in_round == state.cur_round {
                player.active = true;
            }
            if player.active {
                num_active_players += 1;
            }
        }

        let num_deal_cards = get_cards_to_deal(num_active_players, state.deck.config().pack_count);

        state.players.iter_mut().filter(|p| p.active).for_each(|p| {
            let mut deal_cards = state
                .deck
                .draw(num_deal_cards)
                .expect("Drawing pre-determined deal amounts should never cause an error");
            p.cards.append(&mut deal_cards);
        });

        state.cur_round += 1;

        StandardRummy {
            phase: DrawPhase { has_drawn: false },
            state,
        }
    }
}

impl GameEndActions for StandardRummy<GameEndPhase> {}

impl<P: GamePhase> AllActions<StandardRummyConfig, StandardRummyScore> for StandardRummy<P> {
    fn view_state(&self) -> &StandardRummyState {
        &self.state
    }
}

impl<P: GamePhase + PlayablePhase> PlayableActions for StandardRummy<P> {
    type SelfInRoundEndPhase = StandardRummy<RoundEndPhase>;
    type SelfInDrawPhase = StandardRummy<DrawPhase>;

    fn add_player(&mut self, player_id: usize, index: Option<usize>) -> Result<(), String> {
        if !self.state.players.iter().all(|p| p.id != player_id) {
            return Err(format!("Player with ID {player_id} already exists"));
        }

        let player = Player::new(player_id, false, self.state.cur_round);

        if index.is_none() || index.is_some_and(|i| i > self.state.players.len()) {
            self.state.players.push(player);
        } else if let Some(index) = index {
            self.state.players.insert(index, player);
        }

        Ok(())
    }

    fn quit_player(
        mut self,
        player_i: usize,
    ) -> TransitionResult<Self, Self::SelfInRoundEndPhase, Self, String> {
        if player_i == self.state.cur_player || player_i > self.state.players.len() {
            return TransitionResult::Error((
                self,
                format!("player_i {player_i} was either the current player, or greater than number of players")
            ));
        }

        self.cur_player().active = false;

        // end the round if there's only 1 player left
        if self
            .state
            .players
            .iter()
            .fold(0, |acc, p| acc + p.active as usize)
            <= 1
        {
            return TransitionResult::End(StandardRummy {
                phase: RoundEndPhase {
                    has_scored_round: false,
                },
                state: self.state,
            });
        } else {
            return TransitionResult::Next(self);
        }
    }

    fn quit_current_player(mut self) -> Self::SelfInDrawPhase {
        self.cur_player().active = false;

        let mut state = self.state;

        state.cur_player = (state.cur_player + 1) % state.players.len();
        while !state.players[state.cur_player].active {
            // find the next active player
            state.cur_player = (state.cur_player + 1) % state.players.len();
        }

        StandardRummy {
            phase: DrawPhase { has_drawn: true },
            state,
        }
    }

    fn move_card_in_hand(
        &mut self,
        player_i: usize,
        old_pos: usize,
        mut new_pos: usize,
    ) -> Result<(), String> {
        let player_hand = &mut self
            .state
            .players
            .get_mut(player_i)
            .ok_or(format!(
                "player_i {player_i} is greater than number of players"
            ))?
            .cards;

        if old_pos >= player_hand.len() {
            return Err(format!(
                "old_pos {old_pos} is greater than the player's hand's size"
            ));
        }

        if new_pos >= player_hand.len() {
            new_pos = player_hand.len() - 1;
        }

        let card = player_hand.remove(old_pos);
        player_hand.insert(new_pos - 1, card);

        Ok(())
    }

    fn sort_hand(&mut self, player_i: usize) -> Result<(), String> {
        self.state
            .players
            .get_mut(player_i)
            .ok_or("player_i is larger than number of players")?
            .cards
            .sort();

        Ok(())
    }
}
