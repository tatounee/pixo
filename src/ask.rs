mod builder;
mod flip_mode;
mod nulos;

use std::num::NonZeroU32;

use rand::Rng;
use std::io::{self, Stdin};

use crate::{card::Card, deck::Deck};

pub use builder::AskerBuilder;
pub use flip_mode::FlipMode;
use nulos::Nulos;

pub trait Ask {
    fn advance(&mut self) {}
    fn get_card(&self) -> (&Card, usize); // Card / id
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

        println!("{}", card.recto[0]);

        stdin.read_line(&mut user_answer)?;

        loop {
            if card.test(&user_answer) {
                println!();
                self.failed.remove_value(index);
                break;
            } else if user_tries == self.tries.get() {
                println!("Answer : {}\n", card.formated_verso());
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
