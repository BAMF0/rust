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
            if !(c.rank == Ace && sum > 10) {
                sum += c.eval();
            } else {
                sum += 1;
            }
        }
        sum
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

struct Player {
    name: String,
    hand: Hand,
    marks: usize,
    wager: usize,
}

impl Player {
    fn bet(&mut self, bet: usize) {
        assert!(bet <= self.marks, "bet must not exceed player's marks");
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

    fn surrender(&mut self) {
        self.marks += self.wager / 2;
        self.wager = 0;
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} with hand: {}", self.name, self.hand)
    }
}
