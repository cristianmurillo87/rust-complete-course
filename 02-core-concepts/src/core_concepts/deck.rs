use rand::{rng, seq::SliceRandom};

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

    // &mut - argument is a mutable variable
    // indicates that it will be modified
    pub fn shuffle(&mut self) {
        // declaring a mutable variable
        let mut rng = rng();
        // if a function requires a mutable reference
        // is must be passed as &mut variable_name
        self.cards.shuffle(&mut rng);
    }

    pub fn deal(&mut self, num_cards: usize)-> Vec<String> {
        // split_of -> splits a vector at a specified index (given as the at argument)
        // vec![1, 2, 3, 4, 5].split_off(2) would leave the original vector
        // as [1, 2, 3]  and return the vector [4, 5]
        self.cards.split_off(self.cards.len() - num_cards)
    }
}