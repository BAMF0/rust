use blackjack::Deck;

fn main() {
    let mut deck = Deck::new();
    println!("{deck}");
    deck.shuffle();
    println!("{deck}");
}
