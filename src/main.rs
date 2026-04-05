use remyan::Card;

fn main() {
    let deck = Card::generate_deck(false);
    println!("{:#?}", deck);
    println!("{}", deck.len());
}
