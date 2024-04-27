use crate::card::{card::Card, rank::Rank, suit::Suit};
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug, Clone)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let mut cards: Vec<Card> = Vec::new();
        let suits: [Suit; 4] = [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades];
        let ranks: [Rank; 13] = [
            Rank::Two,
            Rank::Three,
            Rank::Four,
            Rank::Five,
            Rank::Six,
            Rank::Seven,
            Rank::Eight,
            Rank::Nine,
            Rank::Ten,
            Rank::Jack,
            Rank::Queen,
            Rank::King,
            Rank::Ace,
        ];

        for &suit in &suits {
            for &rank in &ranks {
                cards.push(Card { rank, suit })
            }
        }

        Deck { cards }
    }

    pub fn shuffle(&mut self) {
        let mut rng: rand::prelude::ThreadRng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    pub fn deal(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn len(&mut self) -> usize {
        self.cards.len()
    }

    pub fn cards(&self) -> &[Card] {
        &self.cards
    }
}

mod tests {
    use crate::card::card::Card;
    use crate::deck::deck::Deck;

    #[test]
    fn deck_contains_52_cards() {
        let mut deck = Deck::new();
        assert_eq!(deck.len(), 52)
    }

    #[test]
    fn deal_reduces_deck_size() {
        let mut deck = Deck::new();
        deck.deal();
        assert_eq!(deck.len(), 51);
    }

    #[test]
    fn shuffle_changes_order() {
        let mut deck: Deck = Deck::new();
        let before_shuffle_state: Vec<Card> = deck.cards.clone();
        deck.shuffle();
        assert_ne!(before_shuffle_state, deck.cards())
    }
}
