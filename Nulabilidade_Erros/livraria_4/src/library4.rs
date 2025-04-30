use crate::items::{Base, Item};

#[allow(dead_code)]
impl Item {
    
    fn get_base(&self) -> &Base {
        match self {
            Item::Book(b) => &b.base,
            Item::AudioBook(ab) => &ab.base,
            Item::Statue(s) => &s.base,
            Item::Painting(p) => &p.base,
        }
    }
    pub fn get_title(&self) -> Option<String> {
        Some(self.get_base().title.clone())
    }
    
    pub fn get_author(&self) -> Option<String> {
        Some(self.get_base().author.clone())
    }
    
    pub fn get_keywords(&self) -> Option<Vec<String>> {
        Some(self.get_base().keywords.iter().map(|k| k.to_string()).collect::<Vec<String>>())
    }
    
    pub fn get_copies(&self) -> Option<usize> {
        Some(self.get_base().copies)
    }
    
    pub fn get_isbn(&self) -> Option<String> {
        if let Item::Book(b) = self {
            Some(b.isbn.clone())
        }else {
            None
        }
    }
    
    pub fn get_duration(&self) -> Option<u32> {
       if let Item::AudioBook(ab) = self {
           Some(ab.duration)
       } else {
           None
       }
    }
    
    pub fn get_dimension_statue(&self) -> Option<(f32, f32, f32)> {
        if let Item::Statue(s) = self {
            Some(s.dimensions)
        }else { 
            None
        }
    }
    
    pub fn get_dimension_painting(&self) -> Option<(f32, f32)> {
       if let Item::Painting(p) = self {
           Some(p.dimensions)
       } else { 
           None
       }
    }
}