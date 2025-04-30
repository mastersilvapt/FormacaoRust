use std::fmt::Display;
use std::collections::BTreeMap;
use std::mem;
use std::ops::RangeBounds;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_derive::{Deserialize, Serialize};
use thiserror::Error;
use crate::coords::StoreCoords;
use crate::free_map::FreeMap;

#[derive(Hash, Serialize, Deserialize)]
pub enum ProductCategory {
    Normal,
    Fragile {
        expiry_date: time::Date,
        max_row: usize
    },
    Oversized {
        // num of extra zones, cannot be higher than warehouse's max_idx
        zone_count: usize,
    }
}

impl Display for ProductCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProductCategory::Normal => {
                write!(f, "Normal")
            }
            ProductCategory::Fragile { expiry_date, max_row } => {
                write!(f, "Fragile - Expires {}, cannot be located above row {}", expiry_date, max_row)
            }
            ProductCategory::Oversized { zone_count } => {
                write!(f, "Oversized - Occupies {} zones", zone_count)
            }
        }
    }
}

pub trait Product: Ord + Display + Serialize + DeserializeOwned + Default {
    fn identifier(&self) -> &i64;
    fn name(&self) -> &String;
    fn amount(&self) -> u64;
    fn quality(&self) -> &ProductCategory;
    fn timestamp(&self) -> time::UtcDateTime;

    fn set_timestamp(&mut self, timestamp: time::UtcDateTime);
}

#[derive(Default, Serialize, Deserialize)]
pub enum WarehouseEntry<I> {
    Some(I),
    #[default]
    None,
    OversizedPlaceholder
}

impl<I> WarehouseEntry<I> {
    fn is_some(&self) -> bool {
        matches!(*self, WarehouseEntry::Some(_))
    }

    fn is_none(&self) -> bool {
        matches!(self, WarehouseEntry::None | WarehouseEntry::OversizedPlaceholder)
    }

    fn expect(self, msg: &str) -> I {
        match self {
            WarehouseEntry::Some(val) => {val}
            _ => panic!("{}", msg)
        }
    }
    
    pub fn expect_ref(&self, msg: &str) -> &I {
        match self {
            WarehouseEntry::Some(val) => val,
            _ => panic!("{}", msg)
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Warehouse<I> {
    store_max_idx: usize,
    store: Vec<Vec<Vec<WarehouseEntry<I>>>>,
    // Não é preciso nenhuma trait aqui, mas a especificação diz trait
    #[serde(skip)]
    filters: Vec<Box<dyn WarehouseAdmissionFilter<I>>>,
    store_index_by_name: BTreeMap<String, Vec<StoreCoords>>,
    store_index_by_id: BTreeMap<i64, Vec<StoreCoords>>,
    store_index_expiry_dates: BTreeMap<time::Date, Vec<i64>>,
    free_map: crate::free_map::FreeMap,
}

impl<I: Product> Warehouse<I> {
    pub fn new(store_max_idx: usize) -> Warehouse<I> {
        let store = Vec::from_iter(
            (0..store_max_idx)
                .map(|_| {
                    Vec::from_iter(
                        (0..store_max_idx)
                            .map(|_| {
                                Vec::from_iter(
                                    (0..store_max_idx)
                                        .map(|_| {
                                            WarehouseEntry::<I>::None
                                        })
                                )
                            })
                    )
                }));
        
        Warehouse {
            store_max_idx,
            store,
            filters: Vec::new(),
            store_index_by_name: BTreeMap::new(),
            store_index_by_id: BTreeMap::new(),
            store_index_expiry_dates: BTreeMap::new(),
            free_map: FreeMap::new(store_max_idx),
        }
    }

    fn verify_product_filters(&mut self, product: &I) -> bool {
        let mut filters = mem::take(&mut self.filters);
        let result = filters.iter_mut().all(|filter| filter.check(self, product));
        self.filters = filters;
        result
    }
    
    pub fn add_product(&mut self, product: I, allocator: &mut impl WarehouseAllocator<I>) -> Result<(), ModificationError> {
        if !self.verify_product_filters(&product) {
            return Err(ModificationError::NotAllowed);
        }

        let store_coords = match allocator.next(self, &product) {
            Some(store_coords) => store_coords,
            None => return Err(ModificationError::Full)
        };
        if let ProductCategory::Fragile { max_row, .. } = product.quality() {
            if store_coords.0 > *max_row {
                return Err(ModificationError::Fragile);
            }
        }
        let row = self.store.get_mut(store_coords.0).expect("Allocator output invalid");
        let shelf = row.get_mut(store_coords.1).expect("Allocator output invalid");
        
        match product.quality() {
            ProductCategory::Oversized { zone_count } => {
                if zone_count >= &self.store_max_idx {
                    return Err(ModificationError::TooBig)
                }
                
                let zone = shelf.get_mut(store_coords.2..=store_coords.2+zone_count).expect("Allocator output invalid");
                
                // do all checks first
                for z in &*zone {
                    match z {
                        WarehouseEntry::Some(_) | WarehouseEntry::OversizedPlaceholder => {
                            return Err(ModificationError::Occupied);
                        }
                        WarehouseEntry::None => {}
                    }
                }
                
                let (zone, placeholders) = zone.split_first_mut().expect("Allocator output invalid");
                
                self.store_index_by_name.entry(product.name().clone()).or_default().push(store_coords.clone());
                self.store_index_by_id.entry(*product.identifier()).or_default().push(store_coords.clone());
                if let ProductCategory::Fragile { expiry_date, .. } = product.quality() {
                    self.store_index_expiry_dates.entry(*expiry_date).or_default().push(*product.identifier());
                }
                self.free_map.occupy_range(store_coords.clone()..=(store_coords.0,store_coords.1,store_coords.2+zone_count).into());
                *zone = WarehouseEntry::Some(product);
                
                for place in placeholders {
                    *place = WarehouseEntry::OversizedPlaceholder
                }
                
                Ok(())
            }
            _ => {
                let zone = shelf.get_mut(store_coords.2).expect("Allocator output invalid");

                match zone {
                    WarehouseEntry::Some(_) | WarehouseEntry::OversizedPlaceholder => {
                        Err(ModificationError::Occupied)
                    }
                    WarehouseEntry::None => {
                        self.store_index_by_name.entry(product.name().clone()).or_default().push(store_coords.clone());
                        self.store_index_by_id.entry(*product.identifier()).or_default().push(store_coords.clone());
                        if let ProductCategory::Fragile { expiry_date, .. } = product.quality() {
                            self.store_index_expiry_dates.entry(*expiry_date).or_default().push(*product.identifier());
                        }
                        self.free_map.occupy_single(store_coords);
                        *zone = WarehouseEntry::Some(product);
                        Ok(())
                    }
                }
            }
        }
    }
    
    pub fn remove_product(&mut self, store_coords: StoreCoords) -> Result<(), ModificationError> {
        let shelf = &mut self.store[store_coords.0][store_coords.1];
        let place = &mut shelf[store_coords.2];
        let mut oversized_count = 0_usize;
        let result_1 = match place {
            WarehouseEntry::None => {
                Err(ModificationError::NotFound)
            }
            WarehouseEntry::Some(p) => {
                let map_entry = self.store_index_by_name.get_mut(p.name())
                    .expect("Existing product should be indexed in map");
                map_entry.remove(map_entry.iter().position(|x| {
                    *x == store_coords
                }).expect("Existing product should be indexed in map"));
                if map_entry.is_empty() {
                    self.store_index_by_name.remove(p.name());
                }
                let map_entry = self.store_index_by_id.get_mut(p.identifier())
                    .expect("Existing product should be indexed in map");
                map_entry.remove(map_entry.iter().position(|x| {
                    *x == store_coords
                }).expect("Existing product should be indexed in map"));
                if map_entry.is_empty() {
                    self.store_index_by_id.remove(p.identifier());
                }
                
                if let ProductCategory::Fragile { expiry_date, .. } = p.quality() {
                    let map_entry = self.store_index_expiry_dates.get_mut(expiry_date)
                        .expect("Existing product should be indexed in map");
                    map_entry.remove(map_entry.iter().position(|x| {
                        *x == *p.identifier()
                    }).expect("Existing product should be indexed in map"));
                    if map_entry.is_empty() {
                        self.store_index_expiry_dates.remove(expiry_date);
                    }
                }
                
                if let ProductCategory::Oversized { zone_count } = p.quality() {
                    oversized_count = *zone_count;
                    self.free_map.free_range(store_coords.clone()..=(store_coords.0,store_coords.1, store_coords.2+zone_count).into());
                } else {
                    self.free_map.free_single(store_coords.clone());
                }
                
                *place = WarehouseEntry::None;
                Ok(())
            }
            WarehouseEntry::OversizedPlaceholder => {
                Err(ModificationError::Placeholder)
            }
        };
        
        if oversized_count > 0 {
            let places = shelf.get_mut(store_coords.2+1..=store_coords.2+oversized_count)
                .expect("Store in invalid state. <AfterAdd,CaughtOnRemove>");
            
            for place in places {
                *place = WarehouseEntry::None;
            }
        }
        
        result_1
    }
    
    pub fn get_product_ref(&self, store_coords: &StoreCoords) -> &WarehouseEntry<I> {
        &self.store[store_coords.0][store_coords.1][store_coords.2]
    }
    
    fn validate_coords(&self, store_coords: &StoreCoords) -> bool {
        self.store.get(store_coords.0)
            .and_then(|row| row.get(store_coords.1))
            .and_then(|shelf| shelf.get(store_coords.2)).is_some()
    }
    
    // find a product interactively, returns coordinates for the product
    // Exclusive for existing products
    // Does not handle gaps
    pub fn store_browse(&self) -> Result<StoreCoords, BrowserError> {
        if self.store.is_empty() {
            println!("No rows to browse");
            return Err(BrowserError);
        }
        
        println!("Select a row ({} rows in warehouse)", self.store.len());
        let row_number = crate::read_valid_stdin("Row number: ", crate::maplidator_int_index_limit_zero(self.store.len()));
        
        let row = &self.store[row_number];
        
        if row.is_empty() {
            println!("Empty row");
            return Err(BrowserError);
        }
        
        println!("Row {}, Select a shelf ({} shelves in row)", row_number, row.len());
        let shelf_number = crate::read_valid_stdin("Shelf number: ", crate::maplidator_int_index_limit_zero(row.len()));
        
        let shelf = &row[shelf_number];
        
        if shelf.is_empty() {
            println!("Empty shelf");
            return Err(BrowserError);
        }
        
        println!("Row {}, Shelf {}, Select a product zone", row_number, shelf_number);
        println!("Products in shelf by zone number:");
        for (index, product) in shelf.iter().enumerate() {
            match product {
                WarehouseEntry::Some(p) => {
                    println!("  {}: {}", index, p.name());
                }
                WarehouseEntry::None => {
                    println!("  {}: <empty>", index);
                }
                WarehouseEntry::OversizedPlaceholder => {
                    println!("  {}: <oversized>", index);
                }
            }
        }
        
        let zone_number = crate::read_valid_stdin("Zone number: ", crate::maplidator_int_index_limit_zero(shelf.len()));
        
        Ok((row_number, shelf_number, zone_number).into())
    }
    
    pub fn search_by_name(&self, name: &str) -> Option<&Vec<StoreCoords>> {
        self.store_index_by_name.get(name)
    }
    
    pub fn search_by_id(&self, id: &i64) -> Option<&Vec<StoreCoords>> {
        self.store_index_by_id.get(id)
    }
    
    pub fn search_expiry_dates<R>(&self, range: R) -> impl Iterator<Item = (&time::Date, &Vec<i64>)>
    where
        R: RangeBounds<time::Date>
    {
        self.store_index_expiry_dates.range(range)    
    }
    
    pub fn store(&self) -> &Vec<Vec<Vec<WarehouseEntry<I>>>> {
        &self.store
    }
    
    pub fn store_max_idx(&self) -> usize {
        self.store_max_idx
    }
    
    pub fn list_by_name(&self) -> impl ExactSizeIterator<Item = (&String,&Vec<StoreCoords>)> {
        self.store_index_by_name.iter()
    }
    
    pub fn free_map(&self) -> &FreeMap {
        &self.free_map
    }
    
    pub fn to_json(&self, writer: &mut impl std::io::Write) {
        serde_json::to_writer_pretty(writer, self).unwrap()
    }
    
    pub fn from_json(reader: &mut impl std::io::BufRead) -> Result<Self, ()> {
        serde_json::from_reader(reader).map_err(|_| ())
    }
}

pub trait WarehouseAllocator<I: Product> {
    fn next(&mut self, warehouse: &Warehouse<I>, product: &I) -> Option<StoreCoords>;
}

trait WarehouseAdmissionFilter<I: Product> {
    /// Returns true if allowed, false if not
    /// This method will not have access to the active filters in the warehouse (The list will always be empty)
    fn check(&mut self, warehouse: &Warehouse<I>, product: &I) -> bool;
}

impl<I: Product> WarehouseAdmissionFilter<I> for dyn FnMut(&Warehouse<I>, &I) -> bool {
    fn check(&mut self, warehouse: &Warehouse<I>, product: &I) -> bool {
        self(warehouse, product)
    }
}

#[derive(Debug, Error)]
pub enum ModificationError {
    #[error("Location in store already has a product")]
    Occupied,
    #[error("No product in location, but one is required")]
    NotFound,
    #[error("Could not find a place for the product")]
    Full,
    #[error("Product is not allowed in due to current filters")]
    NotAllowed,
    #[error("Placeholders cannot be manipulated directly, operate on the item instead")]
    Placeholder,
    #[error("Fragile item cannot be placed at location specified")]
    Fragile,
    #[error("Oversized item does not fit inside warehouse")]
    TooBig
}

pub struct BrowserError;