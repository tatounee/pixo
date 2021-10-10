use rand::{prelude::SliceRandom, Rng};

#[derive(Debug)]
pub struct Nulos {
    failed1: Vec<usize>,
    failed2: Vec<usize>,
    which_one: Failed,
    index: usize,
    cancel_advance: bool,
}

impl Nulos {
    pub const fn new() -> Self {
        Self {
            failed1: Vec::new(),
            failed2: Vec::new(),
            which_one: Failed::One,
            index: 0,
            cancel_advance: false,
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.failed1.is_empty() && self.failed2.is_empty()
    }

    #[inline]
    pub fn shuffle<R: Rng>(&mut self, rng: &mut R) {
        self.failed1.shuffle(rng);
        self.failed2.shuffle(rng);
    }

    #[inline]
    pub fn push(&mut self, value: usize) -> Option<()> {
        // We store the value into the vec which is NOT used
        if self.which_one.is_one() {
            self.failed2.push(value);
        } else {
            self.failed1.push(value);
        }

        Some(())
    }

    #[inline]
    pub fn get(&self) -> Option<usize> {
        self.get_vec().get(self.index).copied()
    }

    #[inline]
    pub fn advance<R: Rng>(&mut self, rng: &mut R) {
        if self.cancel_advance {
            self.cancel_advance = false;
        } else {
            self.index += 1;
        }

        if self.index >= self.get_vec().len() {
            *self.get_mut_vec() = Vec::new();

            self.which_one.switch();
            self.shuffle(rng);
            self.index = 0;
        }
    }

    #[inline]
    pub fn remove_value(&mut self, value: usize) -> Option<()> {
        let mut failed_len = self.get_vec().len();

        let index = self.get_vec().iter().position(|v| *v == value)?;

        if index < failed_len {
            self.get_mut_vec().remove(index);
            failed_len -= 1;

            if index <= self.index || self.index == failed_len {
                self.cancel_advance = true;
            }
        }

        Some(())
    }

    const fn get_vec(&self) -> &Vec<usize> {
        if self.which_one.is_one() {
            &self.failed1
        } else {
            &self.failed2
        }
    }

    #[inline]
    fn get_mut_vec(&mut self) -> &mut Vec<usize> {
        if self.which_one.is_one() {
            &mut self.failed1
        } else {
            &mut self.failed2
        }
    }
}

#[derive(Debug)]
enum Failed {
    One,
    Two,
}

impl Failed {
    const fn is_one(&self) -> bool {
        matches!(self, Self::One)
    }

    #[inline]
    fn switch(&mut self) {
        if self.is_one() {
            *self = Self::Two
        } else {
            *self = Self::One
        }
    }
}
