use crate::card::Card;

pub trait Ask {
    fn next_question(&mut self) -> &Card;
}
