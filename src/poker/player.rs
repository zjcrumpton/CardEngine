use crate::card::card::Card;

pub struct Player {
    hand: Option<[Card; 2]>,
    stack: u32,
}

#[derive(Debug, PartialEq)]
pub enum StackError {
    InsufficientFunds(u32),
}

impl Player {
    pub fn new(starting_stack: u32) -> Self {
        let hand: Option<[Card; 2]> = None;

        Player {
            hand,
            stack: starting_stack,
        }
    }

    pub fn hand(&self) -> Option<&[Card; 2]> {
        self.hand.as_ref()
    }

    pub fn stack(&self) -> u32 {
        self.stack
    }

    pub fn assign_hand(&mut self, new_hand: [Card; 2]) {
        self.hand = Some(new_hand);
    }

    pub fn decrease_stack(&mut self, amount: u32) -> Result<u32, StackError> {
        if amount > self.stack {
            Err(StackError::InsufficientFunds(amount - self.stack))
        } else {
            self.stack -= amount;
            Ok(self.stack)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        card::{card::Card, rank::Rank, suit::Suit},
        poker::player::{Player, StackError},
    };

    #[test]
    fn starts_with_empty_hand() {
        let player: Player = Player::new(0);
        assert_eq!(player.hand(), None);
    }

    #[test]
    fn can_be_assigned_a_hand() {
        let mut player: Player = Player::new(0);
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

    #[test]
    fn assigns_initial_starting_stack() {
        let player: Player = Player::new(1000);

        assert_eq!(player.stack(), 1000);
    }

    #[test]
    fn can_decrease_stack() {
        let mut player: Player = Player::new(1000);

        match player.decrease_stack(500) {
            Ok(new_amount) => assert_eq!(new_amount, 500),
            Err(_) => panic!("Did not expect an error result"),
        }

        assert_eq!(player.stack(), 500);
    }

    #[test]
    fn can_not_go_negative() {
        let mut player: Player = Player::new(1000);

        match player.decrease_stack(2000) {
            Ok(_) => panic!("Expected an error result"),
            Err(StackError::InsufficientFunds(short)) => assert_eq!(short, 1000),
        }

        assert_eq!(player.stack(), 1000);
    }
}
