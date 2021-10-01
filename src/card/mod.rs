pub mod collection;
pub mod deck;

use std::mem;

use crate::ask::Ask;

pub struct Card {
    pub recto: String,
    pub verso: String,
    pub tip: Tip,
    pub only_recto: bool,
}

impl Card {
    pub const fn new(recto: String, verso: String, tip: Tip) -> Self {
        Self {
            recto,
            verso,
            tip,
            only_recto: false,
        }
    }

    #[inline]
    pub fn only_recto(&mut self, only: bool) {
        self.only_recto = only
    }

    #[inline]
    pub fn flip(&mut self) {
        if !self.only_recto {
            mem::swap(&mut self.recto, &mut self.verso)
        }
    }
}

impl Ask for Card {
    fn next_question(&mut self) -> &Card {
        self
    }
}

pub enum Tip {
    None,
    One(String),
    RectoVerso(String, String),
}
