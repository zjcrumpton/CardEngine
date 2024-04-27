use super::{rank::Rank, suit::Suit};
use std::fmt;

pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} of {:?}", self.rank, self.suit)
    }
}
