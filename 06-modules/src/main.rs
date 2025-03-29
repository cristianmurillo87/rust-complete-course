mod content;
use content::{catalog::Catalog, media::Media, print_catalog_item};


fn main() {
    let audiobook = Media::Audiobook { title: String::from("An Audiobook") };
    let good_movie = Media::Movie { title: String::from("A Good Movie"), director: String::from("A Good Director") };
    let bad_book = Media::Book { title: "Some Bad Book".to_string(), author: "Some Bad Author".to_string() };

    println!("{}", audiobook.description());
    println!("{}", good_movie.description());
    println!("{}", bad_book.description());

    let mut catalog = Catalog::new();

    catalog.add(audiobook);
    catalog.add(good_movie);
    catalog.add(bad_book);
    catalog.add(Media::Series(String::from("A very good series!!")));
    catalog.add(Media::Podcast(3));
    catalog.add(Media::Placeholder);

    print_catalog_item(&catalog, 0);
    print_catalog_item(&catalog, 4);
    print_catalog_item(&catalog, 100);
}
