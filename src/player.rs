use crate::card::Card;

#[derive(Debug, Clone)]
pub struct Player { 
    pub name: String,
    pub chips: u32,
    pub hand: Vec<Card>,
    pub is_human: bool,
    pub is_active: bool,
    pub current_bet: u32,
}

impl Player {
    pub fn new(name: String, is_human: bool) -> Self {
        Self {
            name,
            chips: 100,
            hand: Vec::new(),
            is_human,
            is_active: true,
            current_bet: 0,
        }
    }
}
