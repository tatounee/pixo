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

impl<R: Rng> Asker<R> {
    pub fn run(mut self) -> Result<(), io::Error> {
        let mut stdin = io::stdin();

        while self.cycle_counter < self.max_cycle.get() {
            self.ask(&mut stdin)?;
            self.advance();
        }

        Ok(())
    }

    fn ask(&self, stdin: &mut Stdin) -> Result<(), io::Error> {
        let card = self.get_card();

        let mut user_answer = String::new();
        let mut user_tries = 1;

        println!("{}", card.recto);

        stdin.read_line(&mut user_answer)?;

        loop {
            if user_answer.trim() == card.verso {
                break;
            } else if user_tries == self.tries.get() {
                println!("Answer : {}", card.verso);
                break;
            } else {
                println!("{}", card.tip);

                stdin.read_line(&mut user_answer)?;
            }

            user_tries += 1;
        }

        Ok(())
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
