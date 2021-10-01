pub mod deck;
pub mod collection;

use std::mem;

use crate::ask::Ask;

pub struct Card {
    recto: String,
    verso: String,
    only_recto: bool,
}

impl Card {
    pub const fn new(recto: String, verso: String) -> Self {
        Self {
            recto: recto,
            verso: verso,
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