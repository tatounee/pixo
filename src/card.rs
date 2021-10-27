use std::{fmt, mem};

use crate::ask::Ask;

pub struct Card {
    pub recto: Vec<String>,
    pub verso: Vec<String>,
    pub tip: Tip,
    pub only_recto: bool,
}

impl Card {
    pub const fn new(recto: Vec<String>, verso: Vec<String>, tip: Tip, only_recto: bool) -> Self {
        Self {
            recto,
            verso,
            tip,
            only_recto,
        }
    }

    #[inline]
    pub fn flip(&mut self) {
        if !self.only_recto {
            mem::swap(&mut self.recto, &mut self.verso);
            self.tip.flip()
        }
    }

    #[inline]
    pub fn formated_verso(&self) -> String {
        self.verso.join(" OR ")
    }

    // Return true if the answer was correct
    #[inline]
    pub fn test(&self, answer: &str) -> bool {
        self.verso.iter().any(|true_answer| true_answer.trim() == answer.trim())
    }
}

impl Ask for Card {
    fn get_card(&self) -> (&Card, usize) {
        (self, 0)
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
            Self::RectoVerso(tip, _) => f.write_fmt(format_args!("Tip : {}.", tip)),
        }
    }
}
