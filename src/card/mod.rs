pub mod deck;
pub mod collection;

use std::mem;
use std::borrow::Cow;

pub trait Ask {
    fn next_question(&mut self) -> &Card;
}

#[derive(Clone)]
pub struct Card<'a> {
    recto: Cow<'a, String>,
    verso: Cow<'a, String>,
    recto_only: bool,
}

impl<'a> Card<'a> {
    pub const fn new(recto: String, verso: String) -> Self {
        Self {
            recto: Cow::Owned(recto),
            verso: Cow::Owned(verso),
            recto_only: false,
        }
    }

    #[inline]
    pub fn only_recto(&mut self, only: bool) {
        self.recto_only = only
    }

    #[inline]
    pub fn duplicate(&'a self) -> Self {
        Self {
            recto: Cow::Borrowed(self.recto.as_ref()),
            verso: Cow::Borrowed(self.verso.as_ref()),
            recto_only: self.recto_only,
        }
    }

    #[inline]
    pub fn flip(&mut self) {
        if !self.recto_only {
            mem::swap(&mut self.recto, &mut self.verso)
        }
    }
}

impl<'a> Ask for Card<'a> {
    fn next_question(&mut self) -> &Card {
        self
    }
}