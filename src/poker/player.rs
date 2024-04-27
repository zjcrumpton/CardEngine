use crate::card::card::Card;

pub struct Player {
    hand: Option<[Card; 2]>,
}

impl Player {
    pub fn new() -> Self {
        let hand: Option<[Card; 2]> = None;

        Player { hand }
    }

    pub fn hand(&self) -> Option<&[Card; 2]> {
        self.hand.as_ref()
    }

    pub fn assign_hand(&mut self, new_hand: [Card; 2]) {
        self.hand = Some(new_hand);
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        card::{card::Card, rank::Rank, suit::Suit},
        poker::player::Player,
    };

    #[test]
    fn starts_with_empty_hand() {
        let player: Player = Player::new();
        assert_eq!(player.hand(), None);
    }

    #[test]
    fn can_be_assigned_a_hand() {
        let mut player: Player = Player::new();
        let first_card: Card = Card {
            rank: Rank::Ace,
            suit: Suit::Spades,
        };
        let second_card: Card = Card {
            rank: Rank::King,
            suit: Suit::Diamonds,
        };
        let new_hand: [Card; 2] = [first_card, second_card];

        player.assign_hand(new_hand);

        assert_eq!(player.hand().unwrap(), &new_hand);
    }
}
