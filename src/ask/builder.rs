use std::{fmt, num::NonZeroU32};

use rand::Rng;

use crate::deck::Deck;

use super::{nulos::Nulos, Asker, FlipMode, Stat};

pub struct AskerBuilder<R: Rng> {
    deck: Deck,
    max_cycle: NonZeroU32,
    tries: NonZeroU32,
    flip_mode: FlipMode,
    rng: R,
}

impl<R: Rng> AskerBuilder<R> {
    #[inline]
    pub fn new(deck: Deck, rng: R) -> Self {
        Self {
            deck,
            max_cycle: NonZeroU32::new(1).unwrap(),
            tries: NonZeroU32::new(1).unwrap(),
            flip_mode: FlipMode::Recto,
            rng,
        }
    }

    #[inline]
    pub fn max_cycle(&mut self, max_cycle: NonZeroU32) {
        self.max_cycle = max_cycle;
    }

    #[inline]
    pub fn tries(&mut self, tries: NonZeroU32) {
        self.tries = tries;
    }

    #[inline]
    pub fn flip_mode(&mut self, verso_mode: FlipMode) {
        self.flip_mode = verso_mode;
    }

    #[inline]
    pub fn build(mut self) -> Asker<R> {
        match self.flip_mode {
            FlipMode::Verso => self.deck.flip_all(),
            FlipMode::Random(_) => self.deck.flip_random(&mut self.rng),
            FlipMode::Recto => (),
        }

        self.deck.suffle(&mut self.rng);

        Asker {
            deck: self.deck,
            failed: Nulos::new(),
            all_cases: self.flip_mode.is_all_cases(),
            cycle_counter: 0,
            max_cycle: self.max_cycle,
            tries: self.tries,
            stat: Stat::New,
            rng: self.rng,
        }
    }
}

impl<R: Rng> fmt::Debug for AskerBuilder<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AskerBuilder")
            .field("deck_len", &self.deck.len())
            .field("max_cycle", &self.max_cycle)
            .field("tries", &self.tries)
            .field("flip_mode", &self.flip_mode)
            .finish_non_exhaustive()
    }
}
