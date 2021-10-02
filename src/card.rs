use std::{fmt, mem};

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
            mem::swap(&mut self.recto, &mut self.verso);
            self.tip.flip()
        }
    }
}

impl Ask for Card {
    fn get_card(&self) -> &Card {
        self
    }
}

pub enum Tip {
    None,
    One(String),
    RectoVerso(String, String),
}

impl Tip {
    #[inline]
    fn flip(&mut self) {
        if let Self::RectoVerso(a, b) = self {
            mem::swap(a, b)
        }
    }
}

impl fmt::Display for Tip {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => f.write_str("Wrong answer."),
            Self::One(tip) => f.write_fmt(format_args!("Tip : {}.", tip)),
            Self::RectoVerso(tip, _) => f.write_fmt(format_args!("Tip : {}.", tip))
        }
    }
}