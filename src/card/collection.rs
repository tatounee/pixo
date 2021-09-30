use rand::Rng;

use super::{Ask, deck::Deck, Card};

pub struct Collection<'a, R: Rng> {
    deck: Deck<'a>,
    deck_index: usize,
    all_cases: bool,
    cycle: u32,
    max_cycle: u32,
    rng: R
}

impl<'a, R: Rng> Collection<'a, R> {
    #[inline]
    pub fn new(deck: Deck<'a>, rng: R) -> Self {
        Self {
            deck_index: deck.len(),
            deck,
            all_cases: false,
            cycle: 0,
            max_cycle: 1,
            rng
        }
    }

    #[inline]
    pub fn all_cases(&mut self, all: bool) {
        self.all_cases = all
    }

    #[inline]
    pub fn active_random(&mut self) {
        self.deck.flip_random(&mut self.rng);
    }

    #[inline]
    pub fn extend(&'a mut self, by: u32) {
        self.max_cycle += by;
    }
}

impl<'a, R: Rng> Ask for Collection<'a, R> {
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
