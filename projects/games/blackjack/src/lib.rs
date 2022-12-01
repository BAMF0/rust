use std::{array, borrow::Borrow, fmt::Display};

#[derive(Clone, Copy)]
enum Rank {
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
            Jack => write!(f, "Jack"),
            Queen => write!(f, "Queen"),
            King => write!(f, "King"),
            Ace => write!(f, "Ace"),
        }
    }
}

#[derive(Clone, Copy)]
enum Suit {
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
struct Card {
    rank: Rank,
    suit: Suit,
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

struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    fn new() -> Deck {
        let mut cards: Vec<Card> = Vec::new();
        let suits = Suit::suits();
        for s in suits {
            cards.append(&mut Card::cards_of(s))
        }
        Deck { cards }
    }
}
