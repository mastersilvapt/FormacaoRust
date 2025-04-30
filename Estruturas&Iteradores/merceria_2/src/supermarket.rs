use std::collections::HashMap;
use crate::item::{Product, ProductId};
use crate::supermarketerrors::SuperMarketError;
use crate::supermarketerrors::SuperMarketError::{CommandNotFound, CriticalError, NoItemInStock, PositionAlreadyFilled, NoSuchItem};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub(crate) struct Pos {
    x: usize,
    y: usize,
    z: usize,
}

pub(crate) struct SuperMarket {
    inv: HashMap<ProductId, Product>,
    localization: HashMap<ProductId, Vec<Pos>>,
    warehouse: HashMap<Pos, ProductId>
}

impl SuperMarket {
    pub(crate) fn new() -> Self {
        SuperMarket {
            inv: HashMap::new(),
            localization: HashMap::new(),
            warehouse: HashMap::new()
        }
    }

    #[allow(dead_code)]
    pub(crate) fn add(&mut self, pos: Pos, product: Product) -> Result<(), SuperMarketError> {
        let id = product.get_id();

        if self.warehouse.contains_key(&pos) {
            return Err(PositionAlreadyFilled);
        }

        self.warehouse.insert(pos.clone(), id.clone());

        self.inv.entry(id.clone()).or_insert(product);

        self.localization.entry(id.clone()).or_default().push(pos);

        Ok(())
    }

    #[allow(dead_code)]
    pub(crate) fn restock(&mut self, id: ProductId, quantity: u32) -> Result<(), SuperMarketError> {
        
        if !self.inv.contains_key(&id) {
            return Err(NoSuchItem);
        }
        
        *self.inv.get_mut(&id).ok_or(CriticalError)?.get_quantity_mut() += quantity;
        
        Ok(())
    }

    #[allow(dead_code)]
    pub(crate) fn remove(&mut self, id: ProductId) -> Result<(), SuperMarketError> {

        if !self.inv.contains_key(&id) {
            return Err(NoSuchItem);
        }

        let product = self.inv.remove(&id).ok_or(CriticalError)?;

        let pos = self.localization.remove(&product.get_id()).ok_or(CriticalError)?;

        for p in pos {
            self.warehouse.remove(&p).ok_or(CriticalError)?;
        }

        Ok(())
    }

    #[allow(dead_code)]
    pub(crate) fn buy(&mut self, id: ProductId, quantity: u32) -> Result<(), SuperMarketError> {

        if !self.inv.contains_key(&id) {
            return Err(NoSuchItem);
        }

        if self.inv.get(&id).ok_or(CriticalError)?.get_quantity() < quantity {
            return Err(NoItemInStock);
        }

        *self.inv.get_mut(&id).ok_or(CriticalError)?.get_quantity_mut() -= quantity;

        Ok(())
    }

    #[allow(dead_code)]
    pub(crate) fn move_(&mut self, old_pos: Pos, new_pos: Pos) -> Result<(), SuperMarketError> {

        let prod_id = self.warehouse.remove(&old_pos).ok_or(CriticalError)?;

        let idx = self.localization.get_mut(&prod_id).ok_or(CriticalError)?
            .iter().position(|x| *x == old_pos).ok_or(CriticalError)?;

        self.localization.get_mut(&prod_id).ok_or(CriticalError)?.remove(idx);

        self.localization.get_mut(&prod_id).ok_or(CriticalError)?.push(new_pos.clone());

        self.warehouse.insert(new_pos, prod_id).ok_or(CriticalError)?;

        Ok(())
    }

    pub(crate) fn print_inventory(&self) {
        for (id, products) in self.localization.iter() {
            println!("{:?} at:", self.inv.get(id).expect("This should not happen"));
            for pos in products.iter() {
                println!("\t{:?}", pos);
            }
        }
    }
    
    #[allow(dead_code)]
    pub(crate) fn change_price(&mut self, id: ProductId, new_price: f32) -> Result<(), SuperMarketError> {
        self.inv.get_mut(&id).ok_or(CriticalError)?.set_price(new_price);
        Ok(())
    }

    #[allow(dead_code)]
    pub(crate) fn change_name(&mut self, id: ProductId, new_name: String) -> Result<(), SuperMarketError> {
        self.inv.get_mut(&id).ok_or(CriticalError)?.set_name(new_name);
        Ok(())
    }
    
    #[allow(dead_code)]
    fn apply(&mut self, command: String) -> Result<String, &'static str> {
        let command = command.trim();
        match command {
            "add" => {
                Ok("Added".to_string())
            },
            "restock" => {
                Ok("Restock".to_string())
            }
            "remove" => {
                Ok("Abandon".to_string())
            }
            "buy" => {
                Ok("Removed".to_string())
            },
            "move" => {
                Ok("Moved".to_string())
            }
            "change price" => {
                Ok("Change price".to_string())
            }
            "change name" => {
                Ok("Change name".to_string())
            }
            _ => Err(CommandNotFound.to_str())
        }
    }

    pub(crate) fn tests(&mut self) {
        self.add(Pos {x: 0, y: 0, z: 0}, Product::new(ProductId(1), "item1".to_string(), "2025.05.12".to_string(), 5.43, 5)).expect("Error adding to market in testing environment");
        self.add(Pos {x: 0, y: 5, z: 0}, Product::new(ProductId(1), "item1".to_string(), "2025.05.12".to_string(), 5.43, 5)).expect("Error adding to market in testing environment");
        self.add(Pos {x: 0, y: 0, z: 1}, Product::new(ProductId(2), "item2".to_string(), "2025.06.12".to_string(), 6.43, 2)).expect("Error adding to market in testing environment");
        self.add(Pos {x: 0, y: 1, z: 0}, Product::new(ProductId(3), "item3".to_string(), "2025.07.12".to_string(), 7.43, 1)).expect("Error adding to market in testing environment");
        self.add(Pos {x: 0, y: 1, z: 1}, Product::new(ProductId(4), "item4".to_string(), "2025.08.12".to_string(), 2.43, 3)).expect("Error adding to market in testing environment");
        self.add(Pos {x: 5, y: 1, z: 1}, Product::new(ProductId(5), "item5".to_string(), "2025.09.12".to_string(), 1.43, 9)).expect("Error adding to market in testing environment");
        self.add(Pos {x: 1, y: 0, z: 0}, Product::new(ProductId(6), "item6".to_string(), "2025.10.12".to_string(), 0.43, 0)).expect("Error adding to market in testing environment");
        self.add(Pos {x: 1, y: 0, z: 1}, Product::new(ProductId(7), "item7".to_string(), "2025.12.12".to_string(), 6.43, 15)).expect("Error adding to market in testing environment");
        self.add(Pos {x: 1, y: 1, z: 0}, Product::new(ProductId(8), "item8".to_string(), "2025.05.13".to_string(), 1.00, 200)).expect("Error adding to market in testing environment");
        self.add(Pos {x: 1, y: 1, z: 1}, Product::new(ProductId(9), "item9".to_string(), "2025.05.14".to_string(), 100.52, 52)).expect("Error adding to market in testing environment");
        self.add(Pos {x: 1, y: 0, z: 2}, Product::new(ProductId(10), "item10".to_string(), "2025.02.10".to_string(), 9.99, 42)).expect("Error adding to market in testing environment");
        self.add(Pos {x: 5, y: 2, z: 1}, Product::new(ProductId(11), "item11".to_string(), "2025.11.02".to_string(), 1.63, 8)).expect("Error adding to market in testing environment");
    }
}


