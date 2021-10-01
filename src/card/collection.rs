use std::num::NonZeroU32;

use rand::Rng;

use super::{deck::Deck, Card};
use crate::ask::Ask;

pub struct Collection<R: Rng> {
    deck: Deck,
    deck_index: usize,
    all_cases: bool,
    cycle: u32,
    max_cycle: u32,
    rng: R
    max_cycle: NonZeroU32,
}

impl<R: Rng> Collection<R> {
    #[inline]
    pub fn new(deck: Deck, rng: R) -> Self {
        Self {
            deck_index: deck.len(),
            deck,
            all_cases: false,
            cycle: 0,
            rng
            max_cycle: NonZeroU32::new(1).unwrap(),
        }
    }

    #[inline]
    pub fn verso_mode(&mut self) {
        self.deck.flip_all();
    }

    #[inline]
    pub fn all_cases_mode(&mut self) {
        self.all_cases = true
    }

    #[inline]
    pub fn random_mode(&mut self) {
        self.deck.flip_random(&mut self.rng);
    }

    #[inline]
    pub fn pass(&mut self, pass: NonZeroU32) {
        self.max_cycle = pass;
    }
}

impl<R: Rng> Ask for Collection<R> {
    fn next_question(&mut self) -> &Card {
        self.deck_index += 1;

        if self.deck_index == self.deck.len() {
            self.deck_index = 0;
            self.cycle += 1;
            if self.all_cases {
                self.deck.flip_all();
            }
        }

        self.deck.next_question()
    }
}
