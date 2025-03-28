/**
 * Adding properties to a struct using the derive attribute.
 * In this case, the Debug trait allows to define how to print
 * the struct to the standard output (the terminal)
 */
#[derive(Debug)]
pub struct Deck {
    // private field
    cards: Vec<String>
}

impl Deck {
    pub fn new(cards: Vec<String>) -> Self {
        Self {cards}
    }

    pub fn shuffle(&self) {}
}