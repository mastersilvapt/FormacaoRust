
pub struct Stack<T> {
    elems: Vec<T>,
}

#[allow(dead_code)]
impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack { elems: Vec::new() }
    }
    
    pub fn push(&mut self, elem: T) {
        self.elems.push(elem);
    }
    
    pub fn pop(&mut self) -> Option<T> {
        self.elems.pop()
    }
    
    pub fn peek(&self) -> Option<&T> {
        self.elems.last()
    }
    
    pub fn is_empty(&self) -> bool {
        self.elems.is_empty()
    }
    
    pub fn size(&self) -> usize {
        self.elems.len()
    }
    
    pub fn clear(&mut self) {
        self.elems.clear();
    }
}