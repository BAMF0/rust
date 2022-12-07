use rand::Rng;
use std::{array, fmt::Display};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Rank {
    Numeric(usize),
    Jack,
    Queen,
    King,
    Ace,
}

impl Rank {
    fn numerics() -> [Rank; 9] {
        array::from_fn(|n| Numeric(n))
    }
}

use Rank::{Ace, Jack, King, Numeric, Queen};
impl Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Numeric(n) => write!(f, "{}", n),
            Jack => write!(f, "J"),
            Queen => write!(f, "Q"),
            King => write!(f, "K"),
            Ace => write!(f, "A"),
        }
    }
}

#[derive(Clone, Copy)]
pub enum Suit {
    Hearts,
    Tiles,
    Clovers,
    Pikes,
}

impl Suit {
    fn suits() -> [Suit; 4] {
        [Hearts, Tiles, Clovers, Pikes]
    }
}

use Suit::{Clovers, Hearts, Pikes, Tiles};
impl Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Hearts => write!(f, "♡"),
            Tiles => write!(f, "♢"),
            Clovers => write!(f, "♣"),
            Pikes => write!(f, "♠"),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl Card {
    fn new(rank: Rank, suit: Suit) -> Card {
        Card { rank, suit }
    }

    fn cards_of(suit: Suit) -> Vec<Card> {
        let mut cards: Vec<Card> = Vec::new();
        cards.append(&mut Card::numerics_of(suit));
        cards.append(&mut Card::faces_of(suit));
        cards.push(Card::new(Ace, suit));
        cards
    }

    fn faces_of(suit: Suit) -> Vec<Card> {
        let jack = Card::new(Jack, suit);
        let queen = Card::new(Queen, suit);
        let king = Card::new(King, suit);
        vec![jack, queen, king]
    }

    fn numerics_of(suit: Suit) -> Vec<Card> {
        let numerics: [Card; 9] = array::from_fn(|n| Card::new(Numeric(n), suit));
        Vec::from(numerics)
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}

pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut cards: Vec<Card> = Vec::new();
        let suits = Suit::suits();
        for s in suits {
            cards.append(&mut Card::cards_of(s))
        }
        Deck { cards }
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn shuffle(&mut self) {
        for i in 0..self.len() {
            let mut rng = rand::thread_rng();
            let j: usize = rng.gen_range(0..self.len());
            self.cards.swap(i, j)
        }
    }
}

impl Display for Deck {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}", self.cards.first().unwrap())?;
        for c in &self.cards[1..] {
            write!(f, ", {}", c)?;
        }
        write!(f, "]")
    }
}
