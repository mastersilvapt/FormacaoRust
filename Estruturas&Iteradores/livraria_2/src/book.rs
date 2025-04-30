
use std::fmt::Display;
use std::ops::Add;
use crate::book::BookErrors::{NotEnoughCopies};

#[derive(Clone, Copy, Debug)]
pub(crate) enum BookErrors {
    NotEnoughCopies,
    NotValidField,
    CriticalError,
}


#[derive(Clone, Debug)]
pub(crate) struct Book {
    isbn: String,
    title: String,
    author: String,
    keywords: Vec<String>,
    copies: u32,
}

impl Book {
    pub(crate) fn new(isbn: String, title: String, author: String, keywords: Vec<String>, copies: u32) -> Self {
        Book {
            isbn,
            title,
            author,
            keywords,
            copies,
        }
    }
    
    pub fn remove_n_copies(&mut self, n: u32) -> Result<(), BookErrors> {
        if self.copies < n {
            return Err(NotEnoughCopies)
        }
        self.copies -= n;
        Ok(())
    }
    pub fn add_n_copies(&mut self, n: u32) -> Result<(), BookErrors> {
        self.copies += n;
        Ok(())
    }
    pub fn increment_copies(&mut self) -> Result<(), BookErrors> {
        self.copies += 1;
        Ok(())
    }
    
    pub fn decrement_copies(&mut self) -> Result<(), BookErrors> {
        self.copies -= 1;
        Ok(())
    }
    
    pub fn get_title(&self) -> &String {
        &self.title
    }
    
    pub fn get_author(&self) -> &String {
        &self.author
    }
    
    pub fn get_keywords(&self) -> &Vec<String> {
        &self.keywords
    }
    
    pub fn get_isbn(&self) -> &String {
        &self.isbn
    }
}

impl Display for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = String::from("ISBN: ")
            .add(&self.isbn.clone())
            .add(" | TITLE: ")
            .add(&self.title.clone())
            .add(" | AUTHOR: ")
            .add(&self.author.clone())
            .add(" | COPIES:")
            .add(&self.copies.to_string());
        write!(f, "{}", str)
    }
}