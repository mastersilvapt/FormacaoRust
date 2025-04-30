use std::collections::HashMap;
use crate::items::*;
use crate::library::LibraryError::{BookAlreadyExists, CriticalError, ItemNotFound, KeywordDoesNotExist, TitleAlreadyExists, TitleDoesNotExist};

#[allow(dead_code)]
enum LibraryError {
    CriticalError,
    TitleAlreadyExists,
    ItemNotFound,
    BookAlreadyExists,
    KeywordDoesNotExist,
    TitleDoesNotExist,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Library {
    items: Vec<Item>,
    title_to_work: HashMap<String, usize>,
    author_to_work: HashMap<String, Vec<usize>>,
    keywords_to_work: HashMap<String, Vec<usize>>,
    isbn_to_work: HashMap<String, usize>,
}

#[allow(dead_code)]
impl Library {
    pub fn new() -> Self {
        Library { 
            items: vec![], 
            title_to_work: HashMap::new(), 
            author_to_work: HashMap::new(), 
            keywords_to_work: HashMap::new(),
            isbn_to_work: HashMap::new(),
        }
    }
    
    pub fn add_item(&mut self, item: Item) -> Result<(), LibraryError> {
        let base = match &item {
            Item::Book(b) => &b.base,
            Item::AudioBook(ab) => &ab.base,
            Item::Statue(s) => &s.base,
            Item::Painting(p) => &p.base,
        };
        let base= base.clone();
        
        let idx = self.items.len();
        
        self.items.push(item);
        
        let item = self.items.get_mut(idx).ok_or(CriticalError)?;
        
        if let Item::Book(b) = item {
            if self.isbn_to_work.contains_key(&b.isbn){
                return Err(BookAlreadyExists);
            }
            self.isbn_to_work.insert(b.isbn.clone(), idx);
        }
        
        self.author_to_work.entry(base.author.clone()).or_default().push(idx);
        if self.title_to_work.contains_key(&base.title) {
            return Err(TitleAlreadyExists);
        }
        self.title_to_work.insert(base.title.clone(), idx);
        
        base.keywords.iter().for_each(|k| self.keywords_to_work.entry(k.clone()).or_default().push(idx));
        
        Ok(())
    }
    
    pub fn search_by_isbn(&mut self, isbn: &str) -> Result<&Item, LibraryError> {
        self.items.get(*self.isbn_to_work.get(isbn).ok_or(ItemNotFound)?).ok_or(CriticalError)
    }
    
    pub fn search_by_keyword(&mut self, keyword: &str) -> Result<Vec<&Item>, LibraryError> {
        self.keywords_to_work.get(keyword).ok_or(KeywordDoesNotExist)?
            .iter()
            .map(|&idx| self.items.get(idx).ok_or(CriticalError))
            .collect::<Result<Vec<&Item>, LibraryError>>()
    }
    
    pub fn search_by_title(&mut self, title: &str) -> Result<&Item, LibraryError> {
        self.items.get(*self.title_to_work.get(title).ok_or(TitleDoesNotExist)?).ok_or(CriticalError)
    }
    
    pub fn search_by_author(&mut self, author: &str) -> Result<Vec<&Item>, LibraryError> {
        self.author_to_work.get(author).ok_or(ItemNotFound)?
            .iter()
            .map(|&idx| self.items.get(idx).ok_or(CriticalError))
            .collect::<Result<Vec<&Item>, LibraryError>>()
    }
}