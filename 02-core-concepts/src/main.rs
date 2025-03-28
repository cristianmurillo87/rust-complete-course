
mod core_concepts;

use core_concepts::deck::Deck;

fn main() {
    // Both arrays and vectors act as data collections
    // Arrays have a fixed size and cannot be modified once defined
    // Vectors can be resized, but only if they are declared as mutable
    let suits_arr = ["Hearts", "Spades", "Diamonds"]; // array
    let dec_cards_vec = vec!["Ace", "Two", "Three"]; // vector (immnutable)

    let mut cards = Vec::new(); // vec![] is also a valid sintax for declaring a vector

    for suit in suits_arr {
        for card in &dec_cards_vec {
            let item = format!("{} of {}", card, suit);
            cards.push(item);
        }
    }



    let deck = Deck::new(cards);

    // {:?} prints structs that implement the Debug trait
    // {:#?} prints in a nicer format
    println!("Here is your deck {:#?}", deck);
}
