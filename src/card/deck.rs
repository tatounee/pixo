use rand::{
    seq::{index::sample, SliceRandom},
    Rng,
};

use super::{Ask, Card};

#[derive(Clone)]
pub struct Deck<'a> {
    cards: Vec<Card<'a>>,
    question_index: usize,
}

impl<'a> Deck<'a> {
    pub const fn new(cards: Vec<Card<'a>>) -> Self {
        Self {
            cards,
            question_index: 0,
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.cards.len()
    }

    #[inline]
    pub fn push(&mut self, card: Card<'a>) {
        self.cards.push(card)
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

    #[inline]
    pub fn duplicate(&'a self) -> Self {
        let cards = self
            .cards
            .iter()
            .map(|card| card.duplicate())
            .collect::<Vec<Card>>();
        Self {
            cards,
            question_index: self.question_index,
        }
    }
}

impl<'a> Ask for Deck<'a> {
    fn next_question(&mut self) -> &Card {
        let card = self.cards.get(self.question_index).unwrap();
        self.question_index = (self.question_index + 1) % self.cards.len();
        card
    }
}
