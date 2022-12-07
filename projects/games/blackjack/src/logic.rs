use crate::cards::Card;
use crate::cards::Rank::{Ace, Numeric};

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
