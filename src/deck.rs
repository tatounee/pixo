use rand::{
    seq::{index::sample, SliceRandom},
    Rng,
};

use crate::ask::Ask;
use crate::card::Card;

pub struct Deck {
    cards: Vec<Card>,
    question_index: usize,
}

impl Deck {
    pub const fn new(cards: Vec<Card>) -> Self {
        Self {
            cards,
            question_index: 0,
        }
    }

    pub const fn question_index(&self) -> usize {
        self.question_index
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.cards.len()
    }

    #[inline]
    pub fn suffle<R: Rng>(&mut self, rng: &mut R) {
        self.cards.shuffle(rng);
    }

    #[inline]
    pub fn flip_all(&mut self) {
        self.cards.iter_mut().for_each(|card| card.flip())
    }

    pub fn flip_random<R: Rng>(&mut self, rng: &mut R) {
        let indexes = sample(rng, self.cards.len(), self.cards.len() >> 2);
        for i in indexes {
            self.flip_nth(i);
        }
    }

    #[inline]
    pub fn flip_nth(&mut self, nth: usize) -> Option<()> {
        let card = self.cards.get_mut(nth)?;
        card.flip();

        Some(())
    }
}

impl Ask for Deck {
    fn advance(&mut self) {
        self.question_index = (self.question_index + 1) % self.cards.len();
    }

    fn get_card(&self) -> &Card {
        self.cards.get(self.question_index).unwrap()
    }
}
