use crate::{card::card::Card, deck::deck::Deck};

mod card;
mod deck;

fn main() {
    let mut deck: Deck = Deck::new();
    deck.shuffle();

    let card: Option<Card> = deck.deal();
    match card {
        Some(card) => println!("You drew a/an {}", card),
        None => println!("The deck was empty!"),
    }
}
