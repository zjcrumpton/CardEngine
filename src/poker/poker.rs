use super::player::Player;

// Texas Hold'em
pub struct Poker {
    players: Vec<Player>,
    max_players: u8,
}

#[derive(Debug, PartialEq)]
pub enum PlayerCountError {
    InsufficientFunds(u32),
}

impl Poker {
    pub fn new(max_players: u8) -> Self {
        // it is impossible to play
        // poker with more than 22 people
        if max_players > 22 {
            panic!();
        }

        let players: Vec<Player> = vec![];
        Poker {
            max_players,
            players,
        }
    }

    pub fn max_players(&self) -> u8 {
        self.max_players
    }

    pub fn players(&self) -> &Vec<Player> {
        self.players.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::Poker;

    #[test]
    fn starts_with_no_players() {
        let poker: Poker = Poker::new(9);

        assert_eq!(poker.players().len(), 0);
    }

    #[test]
    fn initializes_max_players() {
        let poker: Poker = Poker::new(9);

        assert_eq!(poker.max_players(), 9);
    }

    #[test]
    #[should_panic]
    fn panics_when_invalid_max_player_count_too_high() {
        Poker::new(23);
    }
}
