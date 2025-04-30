mod book;

use std::collections::HashMap;
use crate::book::{Book, BookErrors};
use utils::read_valid_stdin;
use crate::book::BookErrors::{NotEnoughCopies, CriticalError, NotValidField};

///
/// Livraria 2.0
///
/// Para implementar esta iteração do exercício, deve copiar a versão anterior livraria 1.0
/// E fazer todas as alterações pedidas pelo enunciado.
///
/// Devem manter ambas as versões do exercício.
struct Library {
    lib: HashMap<String, Book>, // ISBN to Book
    title_book: HashMap<String, String>, // Title to ISBN
    keywords_books: HashMap<String, Vec<String>>, // Keyword to Books
    author_books: HashMap<String, Vec<String>>, // Author to Books
    requested: HashMap<String, Vec<String>>, // Person to Books
}

impl Library {
    fn new() -> Self {
        Library {
            lib: HashMap::new(),
            title_book: HashMap::new(),
            keywords_books: HashMap::new(),
            author_books: HashMap::new(),
            requested: HashMap::new(),
        }
    }
    fn help(&self) {
        println!("Operations:\nadd\nremove\nlist library\nlist requested\nsearch\nrequest\nreturn\n");
    }
    
    fn add_book(&mut self, book: Book) -> Result<&Book, BookErrors> {
        let isbn = book.get_isbn().clone();
        self.lib.insert(isbn.clone(), book);
        let book = self.lib.get(&isbn).ok_or(CriticalError)?;
        self.title_book.insert(book.get_title().clone(), book.get_isbn().clone());

        book.get_keywords().iter().for_each(|keyword| {
            if self.keywords_books.contains_key(keyword) {
                self.keywords_books.get_mut(keyword).unwrap().push(book.get_isbn().clone());
            }else {
                self.keywords_books.insert(keyword.clone(), vec![book.get_isbn().clone()]);
            }
        });

        self.lib.get(&book.get_isbn().clone()).ok_or(CriticalError)
    }

    fn create_book(&mut self, isbn: String) -> Result<&Book, BookErrors> {
        let title = read_valid_stdin("Title>", |input| Ok(input.trim().to_string()));
        let author = read_valid_stdin("Author>", |input| Ok(input.trim().to_string()));
        let keywords = read_valid_stdin("Keywords (separated by comma)>", |input| Ok(input.trim().to_string()));
        let keywords: Vec<String> = keywords.split(",")
            .map(|s| s.to_string()).collect::<Vec<String>>();

        let copies = read_valid_stdin("Copies>", |input| Ok(input.trim().parse::<u32>().unwrap()));

        if title == *"" || author == *"" || keywords.is_empty() {
            return Err(NotValidField);
        }

        self.add_book(Book::new(isbn.clone(), title.clone(), author.clone(), keywords.clone(), copies)) 
    }
    
    fn apply(&mut self, op: &String) -> Result<&'static str, &'static str> {
        match op.as_ref() {
            "add" => {

                let isbn = read_valid_stdin("ISBN>", |input| Ok(input.trim().to_string()));
                let has_book = self.lib.iter().filter(|(isbn_i, _)| **isbn_i == isbn ).count() > 0;
                
                #[allow(clippy::is_digit_ascii_radix)]
                if isbn.chars().all(|x| x.is_digit(10)) {
                    println!("{:?}", isbn.chars());
                    return Err("ISBN not valid");
                }
                
                if !has_book {
                    self.create_book(isbn.to_string()).map_err(| x| {
                        match x {
                            NotValidField => "Field not valid",
                            _ => "This should not happen"
                        }
                    })?;
                }else {
                    let book = self.lib.get_mut(&isbn).unwrap();

                    let copies = read_valid_stdin("Copies>", |input| Ok(input.trim().parse::<u32>().unwrap()));

                    book.add_n_copies(copies).expect("Failed to add copies");
                }
                Ok("Added with success")
            },
            "remove" => {
                let isbn = read_valid_stdin("ISBN>", |input| Ok(input.trim().to_string()));

                if !self.lib.contains_key(&isbn) {
                    return Err("E> Book does not exist in library")
                }

                let copies = read_valid_stdin("Use number of copies or -1 to fully remove from the library\nCopies>",
                                              |input| Ok(input.trim().parse::<i32>().unwrap()));
                
                if copies == -1 {
                    let book = self.lib.remove(&isbn).ok_or("C>This should not happen")?;
                    self.title_book.remove(book.get_title());
                    let author_books = self.author_books.get_mut(book.get_author()).ok_or("C>This should not happen")?;
                    let index_author_books = author_books.iter().position(|x| x.clone() == isbn).ok_or("C>This should not happen")?;
                    author_books.swap_remove(index_author_books);
                    for keywork in book.get_keywords() {
                        let books_vec = self.keywords_books.get_mut(keywork)
                            .ok_or("This should not happen")?;
                        
                        let index = books_vec.iter().position(|x| x.clone() == isbn).ok_or("C>This should not happen")?;
                        
                        books_vec.swap_remove(index);
                    }
                } else if copies <= 0 {
                    return Err("E>Number of copies must be greater than zero");
                }
                else{
                    self.lib.get_mut(&isbn).ok_or("E>This should not happen")?.remove_n_copies(copies as u32).map_err(|x|{
                        match x {
                            NotEnoughCopies => "E>Not enough copies",
                            _ => "C>This should not happen"
                        }
                    } )?;
                }
                Ok("Book removed with success")
            },
            "list library" => {
                println!("List ({}):", self.lib.len());
                for (_,book) in self.lib.iter() {
                    println!("{book}");
                }
                Ok("Listed with success")
            },
            "list requested" => {
                println!("Requested ({}):", self.requested.len());
                for (i, (person, books)) in self.requested.iter().enumerate() {
                    println!("{}: {person} ({}):", i+1, books.len());
                    for (j, book) in books.iter().enumerate() {
                        println!("\t{}.{}: {}", i+1, j+1, self.lib.get(book).ok_or("This should not happen")?);
                    }
                }
                Ok("Listed with success")
            },
            "request" => {
                let isbn = read_valid_stdin("ISBN>", |input| Ok(input.trim().to_string()));
                let name = read_valid_stdin("NAME>", |input| Ok(input.trim().to_string()));

                let book = self.lib.get_mut(&isbn).ok_or("Book does not exist in library")?;
                
                book.decrement_copies().map_err(|x| {
                    match x {
                        NotEnoughCopies => "Not enough copies",
                        _ => "This should not happen"
                    }
                })?;
                
                if self.requested.contains_key(&name) {
                    self.requested.get_mut(&name).ok_or("This should not happen")?.push(isbn.clone());
                }else{
                    self.requested.insert(name.clone(), vec!(isbn.clone()));
                }
                
                Ok("Book requested with success")
            },
            "return" => {
                self.apply(&"list requested".to_string())?;
                let name = read_valid_stdin("NAME>", |input| Ok(input.trim().to_string()));
                if name.is_empty() {
                    return Err("Name cannot be empty");
                }
                if !self.requested.contains_key(&name) {
                    return Err("Not a valid name");
                }
                
                let number = read_valid_stdin("ID>", | input| {
                    let input = input.trim();
                    let number = input.parse::<usize>().map_err(|_| "E> ID needs to be a number ID")?;
                    if number > 0 && number <= self.requested.get(&name).ok_or("This should not happen")?.len() {
                        Ok(number-1)
                    }else {
                        Err("E> Needs to be a number in the list")
                    }
                });

                let user_requested = self.requested.get_mut(&name).ok_or("This should not happen")?;
                let isbn = user_requested.swap_remove(number);
                
                if user_requested.is_empty() {
                    self.requested.remove(&name);
                }
                
                self.lib.get_mut(&isbn).ok_or("This should not happen")?.increment_copies().map_err(|_| "This should not happen")?;
                
                Ok("Book returned with success")
            }
            _ => Err("Command not valid")
        }
    }
}

fn main() {
    
    let mut library = Library::new(); 

    library.add_book(Book::new("123".to_string(), "Teste123".to_string(), "Andre Silva".to_string(), vec!["test1".to_string(), "test2".to_string()], 1)).expect("Couldn't add book");
    library.add_book(Book::new("234".to_string(), "Teste123".to_string(), "Andre Silva".to_string(), vec!["test1".to_string(), "test2".to_string()], 2)).expect("Couldn't add book");
    library.add_book(Book::new("345".to_string(), "Teste123".to_string(), "Andre Silva".to_string(), vec!["test1".to_string(), "test2".to_string()], 3)).expect("Couldn't add book");
    
    let mut input: String;
    loop {
        library.help();
        input = read_valid_stdin(">", |input| Ok(input.trim().to_string()));

        if input.eq_ignore_ascii_case("exit") {
            break;
        }
        match library.apply(&input) {
            Ok(ok) => println!("{}", ok), 
            Err(err) => println!("{}", err)
        }
    }
    
}
