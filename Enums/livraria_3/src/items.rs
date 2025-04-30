use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct Base {
    pub title: String,
    pub author: String,
    pub keywords: HashSet<String>,
    pub copies: usize,
}

#[derive(Clone, Debug)]
pub struct Book {
    pub base: Base,
    pub isbn: String,
}

#[derive(Clone, Debug)]
pub struct AudioBook {
    pub base: Base,
    pub duration: u32,
}

#[derive(Clone, Debug)]
pub struct Statue {
    pub base: Base,
    pub dimensions: (f32, f32, f32),
}

#[derive(Clone, Debug)]
pub struct Painting {
    pub base: Base,
    pub dimensions: (f32, f32),
}

#[derive(Clone, Debug)]
pub enum Item {
    Book(Book),
    AudioBook(AudioBook),
    Statue(Statue),
    Painting(Painting),
}