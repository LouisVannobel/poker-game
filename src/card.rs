#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Suit {
    Hearts,
    Spades,
    Diamonds,
    Clubs,
}


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Card {
    pub rank: String,
    pub suit: Suit,
}

impl Card {
    pub fn to_string(&self) -> String {
        let suit_symbol = match self.suit {
            Suit::Hearts => "♥️",
            Suit::Spades => "♠️",
            Suit::Diamonds => "♦️",
            Suit::Clubs => "♣️",
        };
        format!("{}{}", self.rank, suit_symbol)
    }

    pub fn rank_value(&self) -> u32 {
        match self.rank.as_str() {
            "2" => 2,
            "3" => 3,
            "4" => 4,
            "5" => 5,
            "6" => 6,
            "7" => 7,
            "8" => 8,
            "9" => 9,
            "10" => 10,
            "J" => 11,
            "Q" => 12,
            "K" => 13,
            "A" => 14,
            _ => 0,
        }
    }
}
