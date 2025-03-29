pub mod media;
pub mod catalog;

pub fn print_catalog_item(catalog: &catalog::Catalog, item_index: usize) {
    match catalog.get_by_index(item_index) {
        Option::Some(item) => {
            println!("Media item at index {}:\n{:#?}", item_index, item);
        }
        Option::None => println!("Media item not found at index {}!", item_index)
    }
}