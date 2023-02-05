struct Library {
    books: Vec<Book>,
}

struct Book {
    title: String,
    year: u16,
}

impl Book {
    fn new(title: &str, year: u16) -> Book {
        Book {
            title: String::from(title),
            year,
        }
    }
}

// This makes it possible to print Book values with {}.
impl std::fmt::Display for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.title, self.year)
    }
}


impl Library {
    fn new() -> Library {
        Library {
            books: Vec::new(),
        }
    }

    fn len(&self) -> usize {
        self.books.len()
    }

    fn is_empty(&self) -> bool {
        self.books.is_empty()
    }

    fn add_book(&mut self, book: Book) {
       self.books.push(book)
    }

    fn print_books(&self) {
        for book in &self.books {
            println!("{}", book);
        }
    }

    fn oldest_book(&self) -> Option<&Book> {
        self.books.iter().min_by_key(|book| book.year)
    }
}

#[test]
fn test_library_len() {
    let mut library = Library::new();
    assert_eq!(library.len(), 0);
    assert!(library.is_empty());
    library.add_book(Book::new("PI adventures", 1565));
    library.add_book(Book::new("Lord of the Rings", 1954));
    library.add_book(Book::new("Alice's Adventures in Wonderland", 1865));
    assert_eq!(library.len(), 3);
    assert!(!library.is_empty());
}

#[test]
fn test_library_oldest_book() {
    let mut library = Library::new();

    assert!(library.oldest_book().is_none());

    library.add_book(Book::new("Lord of the Rings", 1954));
    assert_eq!(
        library.oldest_book().map(|b| b.title.as_str()),
        Some("Lord of the Rings")
    );

    library.add_book(Book::new("Alice's Adventures in Wonderland", 1865));
    library.add_book(Book::new("PI", 1565));
    assert_eq!(
        library.oldest_book().map(|b| b.title.as_str()),
        Some("PI")
    );
}

fn main() {
    let mut library = Library::new();

    println!("Our library is empty: {}", library.is_empty());
    library.add_book(Book::new("PI adventures", 1565));
    library.add_book(Book::new("Lord of the Rings", 1954));
    library.add_book(Book::new("Alice's Adventures in Wonderland", 1865));

    println!("Our library is empty: {}", library.is_empty());

    library.print_books();

    match library.oldest_book() {
        Some(book) if book.year == 1566 => println!("especial case PI"),
        Some(book) => println!("My oldest book is {book}"),
        None => println!("My library is empty!"),
    }

    println!("Our library has {} books", library.len());
}