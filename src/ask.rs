mod flip_mode;
mod nulos;

use std::fmt;
use std::num::NonZeroU32;

use rand::Rng;
use std::io::{self, Stdin};

use crate::{card::Card, deck::Deck};

pub use flip_mode::FlipMode;
use nulos::Nulos;

pub trait Ask {
    fn advance(&mut self) {}
    fn get_card(&self) -> (&Card, usize); // Card / id
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

    // #[inline]
    // pub fn deck(&mut self, deck: Deck) {
    //     self.deck = deck;
    // }

    // #[inline]
    // pub fn rng(&mut self, rng: R) {
    //     self.rng = rng;
    // }

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

pub struct Asker<R: Rng> {
    deck: Deck,
    failed: Nulos,
    all_cases: bool,
    cycle_counter: u32,
    max_cycle: NonZeroU32,
    tries: NonZeroU32,
    stat: Stat,
    rng: R,
}

impl<R: Rng> Ask for Asker<R> {
    fn advance(&mut self) {
        if matches!(self.stat, Stat::New) {
            if self.deck.question_index() + 1 == self.deck.len() {
                if self.failed.is_empty() {
                    self.cycle_counter += 1;
                    self.deck.suffle(&mut self.rng);
                    if self.all_cases {
                        self.deck.flip_all();
                    }
                    self.deck.advance();
                } else {
                    self.stat = Stat::Failed;
                    self.failed.advance(&mut self.rng);
                }
            } else {
                self.deck.advance()
            }
        } else if self.failed.is_empty() {
            self.stat = Stat::New;
            self.advance();
        } else {
            self.failed.advance(&mut self.rng);
        }
    }

    fn get_card(&self) -> (&Card, usize) {
        if matches!(self.stat, Stat::New) {
            self.deck.get_card()
        } else {
            let index = self.failed.get().unwrap();
            (self.deck.get_card_by_index(index).unwrap(), index)
        }
    }
}

impl<R: Rng> Asker<R> {
    pub fn run(mut self) -> Result<(), io::Error> {
        let mut stdin = io::stdin();

        while self.cycle_counter < self.max_cycle.get() {
            self.ask(&mut stdin)?;
            self.advance();
        }

        Ok(())
    }

    fn ask(&mut self, stdin: &mut Stdin) -> Result<(), io::Error> {
        let (card, index) = self.get_card();

        let mut user_answer = String::new();
        let mut user_tries = 1;

        println!("{}", card.recto);

        stdin.read_line(&mut user_answer)?;

        loop {
            if user_answer.trim() == card.verso.trim() {
                println!();
                self.failed.remove_value(index);
                break;
            } else if user_tries == self.tries.get() {
                println!("Answer : {}\n", card.verso);
                self.failed.push(index).unwrap();
                break;
            } else {
                println!("{}", card.tip);

                user_answer = String::new();
                stdin.read_line(&mut user_answer)?;
            }

            user_tries += 1;
        }

        Ok(())
    }
}

enum Stat {
    New,
    Failed,
}
