#[derive(Debug)]
pub enum Media {
    Book {title: String, author: String},
    Movie {title: String, director: String},
    Audiobook {title: String},
    Series(String),
    Podcast(u32),
    Placeholder
}

impl Media {
    pub fn description(&self) -> String {
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