use crate::{card::Card, deck::Deck};

use std::num::NonZeroU32;

use rand::Rng;
use std::io::{self, Stdin};

type AllCases = bool;

pub trait Ask {
    fn advance(&mut self) {}
    fn get_card(&self) -> &Card;
}

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
    pub fn deck(&mut self, deck: Deck) {
        self.deck = deck;
    }

    #[inline]
    pub fn rng(&mut self, rng: R) {
        self.rng = rng;
    }

    #[inline]
    pub fn build(mut self) -> Asker<R> {

        match self.flip_mode {
            FlipMode::Verso => self.deck.flip_all(),
            FlipMode::Random(_) => self.deck.flip_all(),
            FlipMode::Recto => (),
        }

        self.deck.suffle(&mut self.rng);

        Asker {
            deck: self.deck,
            all_cases: self.flip_mode.is_all_cases(),
            cycle_counter: 0,
            max_cycle: self.max_cycle,
            tries: self.tries,
            rng: self.rng,
        }
    }
}

pub struct Asker<R: Rng> {
    deck: Deck,
    all_cases: bool,
    cycle_counter: u32,
    max_cycle: NonZeroU32,
    tries: NonZeroU32,
    rng: R,
}

impl<R: Rng> Ask for Asker<R> {
    fn advance(&mut self) {
        if self.deck.question_index() + 1 == self.deck.len() {
            self.cycle_counter += 1;
            if self.all_cases {
                self.deck.flip_all();
            }
        }
        self.deck.advance();
    }

    fn get_card(&self) -> &Card {
        self.deck.get_card()
    }
}


pub enum FlipMode {
    Recto,
    Verso,
    Random(AllCases),
}

impl FlipMode {
    fn is_all_cases(&self) -> bool {
        if let Self::Random(all_cases) = self {
            return *all_cases;
        } else {
            false
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::card::Tip;

//     use super::*;

//     #[test]
//     fn is_it_safe() {
//         let card = Card::new(
//             "The best programming language".into(),
//             "Rust".into(),
//             Tip::None,
//         );

//         let deck = Deck::new(vec![card]);

//         let rng = rand::thread_rng();
//         let collection = Collection::new(deck, rng);

//         let asker = Asker::new(collection, NonZeroU32::new(2).unwrap());

//         asker.run().unwrap();
//     }
// }
