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
}

#[cfg(test)]
mod tests {
    use crate::poker::player::Player;

    #[test]
    fn starts_with_empty_hand() {
        let player: Player = Player::new();
        assert_eq!(player.hand(), None);
    }
}
