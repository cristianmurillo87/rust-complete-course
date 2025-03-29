// enums in rust can hold values of any type
// and similar to structs traits can be implemented or derived for them
// sructs might be convenient to structs in the cases when
// it is needed to implement the same method, but it behaves differently
// depending of the enum variants
#[derive(Debug)]
enum Media {
    Book {title: String, author: String},
    Movie {title: String, director: String},
    Audiobook {title: String},
    Series(String),
    Podcast(u32),
    Placeholder
}

impl Media {
    fn description(&self) -> String {
        match self {
            Media::Book {title, author } => format!("Book: {} {}", title, author),
            Media::Audiobook { title } => format!("Audiobook: {}", title),
            Media::Movie { title, director } => format!("Movie: {} {}", title, director),
            Media::Series(series_name) => format!("Series: {}", series_name),
            Media::Podcast(chapter) => format!("Podcast chapter: {}", chapter),
            Media::Placeholder => "Placeholder".to_string()

        }
    }
}

#[derive(Debug)]
struct Catalog {
    items: Vec<Media>
}

impl Catalog {
    fn new() -> Self {
        Catalog { items: vec![] }
    }

    fn add(&mut self, media: Media) {
        self.items.push(media);
    }

    fn get_by_index(&self, index: usize) -> Option<&Media> {
        if self.items.len() > index {
           return  Option::Some(&self.items[index]); // Some(&self.items[index]) is also valid
        }
        None
        
    }
}

fn print_media(media: Media) {
    println!("{:#?}", media);
}

fn print_catalog_item(catalog: &Catalog, item_index: usize) {
    match catalog.get_by_index(item_index) {
        Option::Some(item) => {
            println!("Media item at index {}:\n{:#?}", item_index, item);
        }
        Option::None => println!("Media item not found at index {}!", item_index)
    }
}

#[derive(Debug)]
struct Account {
    balance: i32
}


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

    let item = catalog.get_by_index(0);

    // unwrap - gives back the value wrapped inside the option if it exists
    // otherwhise, it the value is None, it cases the program to panic (crash)
    // use only for quick debugging (not recommended in production)
    println!("{:#?}", item.unwrap());

    let second_item = catalog.get_by_index(40);

    // if second_item is Option::None it crashes with the error message
    // provided inside expect()
    // similar to unwrap, it's used mostly for debugging or demonstration
    println!("{:#?}", second_item.expect("Item not found at the specified index!"));

    let third_item = catalog.get_by_index(2);
    let placeholder = Media::Placeholder;

    // unwrap_or provides a fallback value in case the unwrapped value is an Option::None
    println!("{:#?}", third_item.unwrap_or(&placeholder));

    let mut accounts = vec![
        Account { balance: 0},
        Account {balance: 10}
    ];

    let first_acc = accounts.first_mut();

    match first_acc {
        Some(account) => {
            account.balance = 30;
            println!("{:#?}", account);
        }
        None => println!("No account found")
    }
}
