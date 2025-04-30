use std::fmt::Display;
use std::io::{stdin, stdout, Write};
use std::ops::Add;

struct Library {
    lib: Vec<(Book, u32)>,
    requested: Vec<(Book, String)>
}

impl Library {
    fn add_book(&mut self, book: Book) {
        for bk in self.lib.iter_mut() {
            if bk.0.isbn == book.isbn {
                bk.1 += 1;
                return;
            }
        }
        self.lib.push((book, 1));
    }

    fn delete_book(&mut self, isbn: &String) {
        let mut idx: Option<usize> = None;
        for (i, bk) in self.lib.iter_mut().enumerate() {
            if bk.0.isbn == *isbn {
                bk.1 -= 1;
                if bk.1 == 0 {
                    idx = Some(i);
                }
                break;
            }
        }
        if  let Some(idx) = idx {
            self.lib.remove(idx);
        }
    }

    fn list_books(&self) -> Vec<(Book, u32)> {
        self.lib.clone()
    }

    fn help(&self) {
        println!("Oprations:");
        println!("add");
        println!("remove");
        println!("list");
        println!("request");
        println!("return");
    }
    fn apply(&mut self, op: &String) -> bool {
        match op.as_ref() {
            "add" => {
                
                let isbn = read_valid_stdin("ISBN>", |input| Ok(input.trim().to_string()));
                let title = read_valid_stdin("Title>", |input| Ok(input.trim().to_string()));
                let author = read_valid_stdin("Author>", |input| Ok(input.trim().to_string()));
                let keywords = read_valid_stdin("Keywords (separated by comma)>", |input| Ok(input.trim().to_string()));
                let keywords: Vec<String> = keywords.split(",")
                    .map(|s| s.to_string()).collect::<Vec<String>>();
                
                let book = Book { isbn, title, author, keywords };
                
                if book.isbn == *"" || book.title == *"" ||
                    book.author == *"" || book.keywords.is_empty() {
                    return false;
                }
                self.add_book(book);
                true
            },
            "remove" => {
                let isbn = read_valid_stdin("ISBN>", |input| Ok(input.trim().to_string()));
                
                let count = match &self.list_books().iter()
                    .find_map(|(b,c)| if b.isbn == isbn { Some(c) } else { None }){
                    None => return false,
                    Some(0) => return false,
                    Some(c) => **c,
                };
                self.delete_book(&isbn);
                self.list_books().iter().filter(|(b, c)| b.isbn == isbn && *c == count-1).count() == 1
            },
            "list" => {
                println!("List ({}):", self.list_books().len());
                for (bk,i) in self.list_books(){
                    println!("{bk} | Number of books -> {i}");
                }
                
                println!("Requested ({}):", self.requested.len());
                for (i, (bk, name)) in self.requested.iter().enumerate() {
                    println!("{}. {bk} - {name}", i+1);
                }
                true
            },
            "request" => {
                let isbn = read_valid_stdin("ISBN>", |input| Ok(input.trim().to_string()));
                let name = read_valid_stdin("NAME>", |input| Ok(input.trim().to_string()));
                
                let book = self.lib.iter_mut().find(|(x,_)| x.isbn == isbn);
                if book.is_none(){
                    return false;
                }
                let book = book.unwrap();
                if book.1 == 0 {
                    return false;
                }
                book.1 -= 1;
                
                self.requested.push((book.0.clone(), name));
                
                true
            },
            "return" => {
                let number = read_valid_stdin("ID>", |input| {
                    let input = input.trim();
                    let number = input.parse::<usize>().map_err(|_| "Invalid ID")?;
                    if number == 0 || number > self.requested.len() {
                        Err("Out of Bounds")
                    }else {
                        Ok(number)
                    }
                });
                
                let request = self.requested.remove(number-1);
                
                let book = self.lib.iter_mut().find(|(x, _n)| x.isbn == request.0.isbn);
                if book.is_none(){
                    return false;
                }
                let book = book.unwrap();
                book.1 += 1;
                
                true
            }
            _ => {
                false
            }
        }
    }
}

#[derive(Clone, Debug)]
struct Book {
    isbn: String,
    title: String,
    author: String,
    keywords: Vec<String>,
}

impl Display for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = String::from("ISBN: ")
            .add(&self.isbn.clone())
            .add(" | TITLE: ")
            .add(&self.title.clone())
            .add(" | AUTHOR: ")
            .add(&self.author.clone());
        write!(f, "{}", str)
    }
}

fn main() {
    let mut library = Library { lib: vec![], requested: vec![] };
    
    library.add_book(Book { isbn: "123".to_string(), title: "Teste123".to_string(), author: "Andre Silva".to_string(), keywords: vec!["teste1".to_string(), "teste2".to_string()]});
    library.add_book(Book { isbn: "123".to_string(), title: "Teste123".to_string(), author: "Andre Silva".to_string(), keywords: vec!["teste1".to_string(), "teste2".to_string()]});
    library.add_book(Book { isbn: "234".to_string(), title: "Teste234".to_string(), author: "Filipe Silva".to_string(), keywords: vec!["teste3".to_string(), "teste4".to_string()]});
    library.add_book(Book { isbn: "345".to_string(), title: "Teste345".to_string(), author: "Jose Silva".to_string(), keywords: vec!["teste5".to_string(), "teste6".to_string()]});
    library.add_book(Book { isbn: "345".to_string(), title: "Teste345".to_string(), author: "Jose Silva".to_string(), keywords: vec!["teste5".to_string(), "teste6".to_string()]});
    library.add_book(Book { isbn: "345".to_string(), title: "Teste345".to_string(), author: "Jose Silva".to_string(), keywords: vec!["teste5".to_string(), "teste6".to_string()]});
    
    let mut input: String;
    loop {
        library.help();
        input = read_valid_stdin(">", |input| Ok(input.trim().to_string()));

        if input.eq_ignore_ascii_case("exit") {
            break;
        }
        library.apply(&input);
    }
}

fn read_valid_stdin<F, T>(prompt: &str, maplidator: F) -> T
where
    F: Fn(String) -> Result<T, &'static str>,
{
    loop {
        let mut input = String::new();
        print!("{}", prompt);
        stdout().flush().unwrap();
        stdin().read_line(&mut input).unwrap();
        match maplidator(input) {
            Ok(value) => break value,
            Err(msg) => println!("{}", msg),
        }
    }
}