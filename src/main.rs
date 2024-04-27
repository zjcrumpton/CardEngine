use crate::card::{card::Card, rank::Rank, suit::Suit};

mod card;

fn main() {
    let card = Card {
        rank: Rank::Ace,
        suit: Suit::Hearts,
    };

    println!("You have drawn a/an {}!", card);
}
