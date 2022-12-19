use crate::cards::Card;
use crate::cards::Rank::{Ace, Numeric};
use std::fmt::Display;

enum Choices {
    Hit,
    Stand,
    DoubleDown,
    Surrender,
}

impl Card {
    fn eval(&self) -> usize {
        match self.rank {
            Numeric(n) => n,
            Ace => 11,
            _ => 10,
        }
    }
}

struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    fn eval(&self) -> usize {
        let mut sum = 0;
        for c in &self.cards {
            sum += c.eval();
        }
        if sum > 21 && self.has_ace() {
            sum -= 10
        }
        sum
    }

    fn has_ace(&self) -> bool {
        match self.cards.iter().find(|c| c.rank == Ace) {
            Some(_) => true,
            None => false,
        }
    }

    fn new(card: Card) -> Hand {
        Hand { cards: vec![card] }
    }

    fn put(&mut self, card: Card) {
        self.cards.push(card)
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for c in &self.cards {
            write!(f, ", {}", c)?;
        }
        write!(f, "]")
    }
}

pub struct Player {
    name: String,
    hand: Hand,
    marks: usize,
    wager: usize,
}

impl Player {
    fn bet(&mut self, bet: usize) {
        assert!(bet <= self.marks, "Bet must not exceed player's marks!");
        self.marks -= bet;
        self.wager += bet;
    }
    fn deal(&mut self, card: Card) {
        self.hand.put(card)
    }

    fn marks(&self) -> usize {
        self.marks
    }

    fn new(hand: Hand, name: String, marks: usize) -> Player {
        Player {
            hand,
            name,
            marks,
            wager: 0,
        }
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} with hand: {}", self.name, self.hand)
    }
}
