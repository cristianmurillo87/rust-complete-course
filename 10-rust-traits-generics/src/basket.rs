// example of generic struct
use super::container::Container;

pub struct Basket<T> {
    item: Option<T>,
}

impl<T> Basket<T> {
    pub fn new(item: T) -> Self {
        Self { item: Some(item) }
    }
}

impl<T> Container<T> for Basket<T> {
    // mutable reference because take() will move the value
    // (change ownership) and pack it into an Option
    fn get(&mut self) -> Option<T> {
        self.item.take()
    }

    fn put(&mut self, item: T) {
        self.item = Some(item);
    }

    fn is_empty(&self) -> bool {
        self.item.is_none()
    }
}
