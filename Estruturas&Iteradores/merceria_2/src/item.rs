#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub(crate) struct ProductId(pub u32);

#[derive(Clone, Debug)]
pub(crate) struct Product {
    id: ProductId,
    name: String,
    expiration_date: String,
    price: f32,
    quantity: u32,
}

impl Product {
    pub(crate) fn new(id: ProductId, name: String, expiration_date: String, price: f32, quantity: u32) -> Self {
        Product { id, name, expiration_date, price, quantity }
    }
    
    pub fn get_id(&self) -> ProductId {
        self.id.clone()
    }
    
    pub fn get_name(&self) -> &String {
        &self.name
    }
    
    pub fn get_expiration_date(&self) -> &String {
        &self.expiration_date
    }
    
    pub fn get_price(&self) -> f32 {
        self.price
    }
    
    pub fn get_quantity(&self) -> u32 {
        self.quantity
    }
    
    pub fn get_quantity_mut(&mut self) -> &mut u32 {
        &mut self.quantity
    }
    
    pub fn set_price(&mut self, price: f32) {
        self.price = price;
    }
    
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}