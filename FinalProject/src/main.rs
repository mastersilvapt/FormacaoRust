use std::cmp::Ordering;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::str::FromStr;
use serde_derive::{Deserialize, Serialize};
use time::{Duration, UtcDateTime};
use coords::StoreCoords;
use warehouse::{Product, ProductCategory, Warehouse, WarehouseAllocator, WarehouseEntry};

mod warehouse;
mod free_map;
mod coords;

#[derive(Serialize,Deserialize)]
struct AnyOldProduct {
    identifier: i64,
    name: String,
    amount: u64,
    quality: ProductCategory,
    timestamp: UtcDateTime,
}

impl Default for AnyOldProduct {
    fn default() -> Self {
        AnyOldProduct {
            identifier: i64::default(),
            name: String::default(),
            amount: u64::default(),
            quality: ProductCategory::Normal,
            timestamp: UtcDateTime::MIN
        }
    }
}

impl PartialEq for AnyOldProduct {
    fn eq(&self, other: &Self) -> bool {
        self.identifier == other.identifier
    }
}

impl Eq for AnyOldProduct {}

impl PartialOrd for AnyOldProduct {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

impl Ord for AnyOldProduct {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.identifier.cmp(&other.identifier)
    }
}

impl Product for AnyOldProduct {
    fn identifier(&self) -> &i64 {
        &self.identifier
    }
    fn name(&self) -> &String {
        &self.name
    }

    fn amount(&self) -> u64 {
        self.amount
    }
    
    fn quality(&self) -> &ProductCategory { &self.quality }

    fn timestamp(&self) -> time::UtcDateTime { self.timestamp }

    fn set_timestamp(&mut self, timestamp: time::UtcDateTime) { self.timestamp = timestamp; }
}

impl Display for AnyOldProduct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Product ID {}:", self.identifier)?;
        writeln!(f, "\tName: {}", self.name)?;
        writeln!(f, "\tAmount: {}", self.amount)?;
        writeln!(f, "\tQuality: {}", self.quality)?;
        write!(f, "\tEntered the warehouse at: {}", self.timestamp)
    }
}

impl AnyOldProduct {
    fn new(identifier: i64, name: String, amount: u64, quality: ProductCategory) -> Self {
        let timestamp = UtcDateTime::now();
        AnyOldProduct { identifier, name, amount, quality, timestamp }
    }
}

struct WarehouseAllocatorClosestFirst;
struct WarehouseAllocatorClosestFirstEfficient;

impl<I: Product> WarehouseAllocator<I> for WarehouseAllocatorClosestFirst {
    fn next(&mut self, warehouse: &Warehouse<I>, product: &I) -> Option<StoreCoords> {
        let mut oversized_count = if let ProductCategory::Oversized { zone_count } = product.quality() {
            *zone_count
        } else { 0 };
        let max_row = if let ProductCategory::Fragile { max_row, .. } = product.quality() {
            Some(*max_row)
        } else { None };
        
        for (i,row) in warehouse.store().iter().enumerate() {
            if let Some(max_row) = max_row {
                if i > max_row {
                    break;
                }
            }
            for (j, shelf) in row.iter().enumerate() {
                let oversized_count_store = oversized_count;
                for (k, zone) in shelf.iter().enumerate() {
                    match zone {
                        WarehouseEntry::Some(_) | WarehouseEntry::OversizedPlaceholder => {}
                        WarehouseEntry::None => {
                            if oversized_count == 0 {
                                return Some((i, j, k - oversized_count_store).into())
                            }
                            oversized_count -= 1;
                        }
                    }
                }
                if oversized_count_store != oversized_count {
                    oversized_count = oversized_count_store;
                }
            }
        }

        None
    }
}

impl<I: Product> WarehouseAllocator<I> for WarehouseAllocatorClosestFirstEfficient {
    fn next(&mut self, warehouse: &Warehouse<I>, product: &I) -> Option<StoreCoords> {
        while let Some(range) = warehouse.free_map().iter().next() {
            if let ProductCategory::Fragile { max_row, .. } = product.quality() {
                if range.start().0 > *max_row {
                    continue
                }
            }
            
            if let ProductCategory::Oversized { zone_count } = product.quality() {
                let mut try_end = range.start().clone();
                if try_end.2 + zone_count >= warehouse.store_max_idx() - 1 {
                    return None
                }
                try_end.2 += zone_count;
                if !range.contains(&try_end) {
                    return None
                }
            }
            
            return Some(range.start().clone())
        }
        None
    }
}

struct WarehouseAllocatorRoundRobin {
    last_coords: StoreCoords,
}

impl WarehouseAllocatorRoundRobin {
    fn new() -> Self {
        WarehouseAllocatorRoundRobin {
            last_coords: (0,0,0).into()
        }
    }
}

impl<I: Product> WarehouseAllocator<I> for WarehouseAllocatorRoundRobin {
    fn next(&mut self, warehouse: &Warehouse<I>, product: &I) -> Option<StoreCoords> {
        let mut oversized_count = if let ProductCategory::Oversized { zone_count } = product.quality() {
            *zone_count
        } else { 0 };
        let max_row = if let ProductCategory::Fragile { max_row, .. } = product.quality() {
            Some(*max_row)
        } else { None };
        let (mut i, mut j, mut k) = (&self.last_coords).into();
        
        while i < warehouse.store().len() {
            if let Some(max_row) = max_row {
                if i > max_row {
                    break;
                }
            }
            let row = &warehouse.store()[i];
            while j < row.len() {
                let shelf = &row[j];
                let oversized_count_store = oversized_count;
                while k < shelf.len() {
                    let zone = &shelf[k];
                    
                    match zone {
                        WarehouseEntry::Some(_) | WarehouseEntry::OversizedPlaceholder => {}
                        WarehouseEntry::None => {
                            if oversized_count == 0 {
                                let coords: StoreCoords = (i, j, k).into();
                                self.last_coords = coords.clone();
                                return Some(coords);
                            }
                            oversized_count -= 1;
                        }
                    }
                    
                    k += 1;
                }
                
                j += 1;
                k = 0;
                if oversized_count_store != oversized_count {
                    oversized_count = oversized_count_store;
                }
            }
            
            i += 1;
            j = 0;
        }

        None
    }
}

struct WarehouseAllocatorRoundRobinEfficient {
    last_coords: StoreCoords,
}

impl WarehouseAllocatorRoundRobinEfficient {
    fn new() -> Self {
        WarehouseAllocatorRoundRobinEfficient {
            last_coords: (0,0,0).into()
        }
    }
}

impl<I: Product> WarehouseAllocator<I> for WarehouseAllocatorRoundRobinEfficient {
    fn next(&mut self, warehouse: &Warehouse<I>, product: &I) -> Option<StoreCoords> {
        let next_coords = self.last_coords.next(warehouse.store_max_idx())?;
        let iter = warehouse.free_map().iter_from(next_coords);
        for range in iter {
            // We don't care about partial overlaps here, since last_coords should always be pointing at the last allocation
            if let ProductCategory::Fragile { max_row, .. } = product.quality() {
                if range.start().0 > *max_row {
                    continue
                }
            }

            if let ProductCategory::Oversized { zone_count } = product.quality() {
                let mut try_end = range.start().clone();
                if try_end.2 + zone_count >= warehouse.store_max_idx() - 1 {
                    return None
                }
                try_end.2 += zone_count;
                if !range.contains(&try_end) {
                    return None
                }
            }

            return Some(range.start().clone())
        }

        None
    }
}

fn main() {
    let mut warehouse = Warehouse::new(20);
    //let mut warehouse_allocator = WarehouseAllocatorClosestFirst;
    let mut warehouse_allocator = WarehouseAllocatorClosestFirstEfficient;
    //let mut warehouse_allocator = WarehouseAllocatorRoundRobin::new();
    //let mut warehouse_allocator = WarehouseAllocatorRoundRobinEfficient::new();
    
    // Filter setup goes here
    
    println!("The grocery store is open.");
    loop {
        print_command_list();
        let command = read_valid_stdin("Command: ", maplidator_int_index_limit(11));
        
        match command {
            1 => { // Add product 
                let identifier = read_valid_stdin("Product identifier: ", |input| {
                    let input = input.trim();
                    input.parse().map_err(|_| "Failed to parse into number")
                });
                let name = read_valid_stdin("Product name: ", maplidator_identity_trim);
                let amount: u64 = read_valid_stdin("Amount in stack: ", |input| {
                    let input = input.trim();
                    input.parse().map_err(|_| "Failed to parse into number")
                });

                let quality = read_valid_stdin(
                    "Select a category:\n1) Fragile\n2) Oversized\n3)Normal\nYour choice: ",
                |input| {
                        let input = maplidator_int_index_limit(3)(input)?;
                        match input {
                            1 => {
                                let expiry_date = read_valid_stdin("Expiry date (YYYY-MM-DD): ", |input| {
                                    let mut input = input.trim().split('-');
                                    let year= get_from_iterator_and_parse(&mut input).map_err(|_| "Could not find a valid year")?;
                                    let month: u8 = get_from_iterator_and_parse(&mut input).map_err(|_| "Could not find a valid month")?;
                                    let month = month.try_into().map_err(|_| "Invalid month")?;
                                    let day = get_from_iterator_and_parse(&mut input).map_err(|_| "Could not find a valid day")?;

                                    if input.next().is_some() {
                                        return Err("Extra data found during parsing");
                                    }

                                    time::Date::from_calendar_date(year, month, day).map_err(|_| "Invalid date")
                                });
                                let max_row = read_valid_stdin("Max row: ", maplidator_int_index_limit(warehouse.store_max_idx()));
                                
                                Ok(ProductCategory::Fragile { expiry_date, max_row })
                            }
                            2 => {
                                let zone_count = read_valid_stdin("Zones occupied: ", maplidator_int_index_limit(warehouse.store_max_idx()));
                                
                                Ok(ProductCategory::Oversized { zone_count })
                            }
                            3 => {
                                Ok(ProductCategory::Normal)
                            }
                            _ => unreachable!()
                        }
                    }
                );
                
                let product = AnyOldProduct::new(identifier, name, amount, quality);
                
                match warehouse.add_product(product, &mut warehouse_allocator) {
                    Ok(()) => println!("Product added"),
                    Err(e) => println!("Failed to add product: {}", e),
                }
            }
            2 => { // Remove Product
                println!("Browse the store and select an item to remove");
                let coords = match warehouse.store_browse() {
                    Ok(x) => {x}
                    Err(_) => {
                        println!("Browse cancelled. No changes were made.");
                        continue
                    }
                };
                
                if let WarehouseEntry::Some(product) = warehouse.get_product_ref(&coords) {
                    println!("Deleting item at row {}, shelf {}, zone {}:\n{}\n", coords.0+1, coords.1+1, coords.2+1, product);
                    let confirm = read_valid_stdin("Confirm delete [y/n]: ", maplidator_yes_or_no);
                    if confirm {
                        warehouse.remove_product(coords).expect("Should have already checked product exists");
                    } else {
                        println!("Cancelled. No changes were made.");
                    }
                } else { 
                    println!("Cannot delete empty product\nCancelled");
                }
            }
            3 => { // List by name
                let keys = warehouse.list_by_name();
                println!("There are {} products in the warehouse", keys.len());
                
                for (_,val) in keys {
                    let val = warehouse.get_product_ref(&val[0])
                        .expect_ref("Only Some values in map");
                    println!("{}", val)
                }
            }
            4 => { // Search ID
                let identifier = read_valid_stdin("Product identifier to search for: ", |input| {
                    let input = input.trim();
                    input.parse().map_err(|_| "Failed to parse into number")
                });
                
                match warehouse.search_by_id(&identifier) {
                    None => println!("Not found"),
                    Some(val) => {
                        println!("We have {} items with identifier {}", val.len(), identifier);
                    }
                }
            }
            5 => { // Search Name
                let name = read_valid_stdin("Product name to search for: ", maplidator_identity_trim);
                
                match warehouse.search_by_name(&name) {
                    None => println!("Not found"),
                    Some(val) => {
                        println!("We have {} items with name {}", val.len(), name);
                    }
                }
            } 
            6 => { // Search All Locations
                let identifier = read_valid_stdin("Product identifier to search for: ", |input| {
                    let input = input.trim();
                    input.parse().map_err(|_| "Failed to parse into number")
                });
                
                match warehouse.search_by_id(&identifier) {
                    None => println!("Not found"),
                    Some(val) => {
                        println!("Locations for item with identifier {}:", identifier);
                        for coord in val {
                            println!("\tRow {}, Shelf {}, Zone {}", coord.0, coord.1, coord.2);
                        }
                    }
                }
            }
            7 => { // Browse store
                let coords = match warehouse.store_browse() {
                    Err(_) => {
                        println!("Browse cancelled.");
                        continue
                    }
                    Ok(x) => {x}
                };
                
                let product = match warehouse.get_product_ref(&coords) {
                    WarehouseEntry::None | WarehouseEntry::OversizedPlaceholder => {
                        println!("No details available.");
                        continue
                    }
                    WarehouseEntry::Some(x) => {x}
                };
                
                println!("Product detail:\n{}", product);
            }
            8 => { // Search Expiry Dates
                let date = read_valid_stdin("Date to search (YYYY-MM-DD): ", |input| {
                    let mut input = input.trim().split('-');
                    let year= get_from_iterator_and_parse(&mut input).map_err(|_| "Could not find a valid year")?;
                    let month: u8 = get_from_iterator_and_parse(&mut input).map_err(|_| "Could not find a valid month")?;
                    let month = month.try_into().map_err(|_| "Invalid month")?;
                    let day = get_from_iterator_and_parse(&mut input).map_err(|_| "Could not find a valid day")?;

                    if input.next().is_some() {
                        return Err("Extra data found during parsing");
                    }

                    time::Date::from_calendar_date(year, month, day).map_err(|_| "Invalid date")
                });
                
                println!("{} items have expired", warehouse.search_expiry_dates(..date).count());
                println!("{} items will expire within 3 days", warehouse.search_expiry_dates(date..=date+Duration::days(3)).count());
            }
            9 => { break }
            10 => { // import
                let filename = read_valid_stdin("File to read: ", maplidator_identity_trim);
                let file = File::open(filename);
                let mut reader = BufReader::new(file.unwrap());
                
                warehouse = Warehouse::from_json(&mut reader).unwrap();
                println!("Done")
            }
            11 => { // export
                let filename = read_valid_stdin("File to write: ", maplidator_identity_trim);
                let file = File::create(filename);
                let mut writer = BufWriter::new(file.unwrap());
                
                warehouse.to_json(&mut writer);
                println!("Done")
            }
            _ => { unreachable!() }
        }
    }
    
    println!("The warehouse is closed. Bye!");
}

fn print_command_list() {
    println!("Available commands:");
    println!("1) Add product");
    println!("2) Remove product");
    println!("3) List by name");
    println!("4) Search by ID");
    println!("5) Search by name");
    println!("6) Search all locations for item");
    println!("7) Browse store");
    println!("8) Search for expiry dates");
    println!("9) Quit");
    println!("10) Import from JSON (Testing)");
    println!("11) Export to JSON (Testing)");
}

/*
#[derive(Copy, Clone)]
enum MySliceIndex {
    Single(usize),
    Range(usize, usize),
}

fn vec_get_or_insert<I, T: Default>(vec: &mut Vec<T>, index: &MySliceIndex) -> &mut [T]
{
    match index {
        MySliceIndex::Single(index) => {
            if vec.get_mut(*index).is_some() {
                // Nao posso usar match nem retornar o resultado de get_mut aqui, por limitação do compilador de rust
                // Ver caso similar em: 
                // https://github.com/rust-lang/rfcs/blob/master/text/2094-nll.md#problem-case-3-conditional-control-flow-across-functions 
                return vec.get_mut(*index..*index).unwrap();
            }

            vec.resize_with(index + 1, Default::default);
            &mut vec.get_mut(*index..*index).unwrap()
        }
        MySliceIndex::Range(s_i, e_i) => {
            if vec.get_mut(*s_i..*e_i).is_some() {
                return vec.get_mut(*s_i..*e_i).unwrap();
            }
            
            let delta = e_i + vec.len();
            vec.resize_with(delta + 1, Default::default);
            &mut vec.get_mut(*s_i..*e_i).unwrap()
        }
    }
}*/

fn maplidator_yes_or_no(input: String) -> Result<bool, &'static str> {
    let input = input.trim();
    if input == "y" {
        Ok(true)
    } else if input == "n" {
        Ok(false)
    } else {
        Err("Use y or n to specify command")
    }
}

fn maplidator_identity_trim(input: String) -> Result<String, &'static str> {
    Ok(input.trim().to_string())
}

fn maplidator_int_index_limit(index_limit: usize) -> impl Fn(String) -> Result<usize, &'static str> {
    move |input| {
        let input = input.trim();
        let input: usize = input.parse().map_err(|_| "Failed to parse into number")?;
        if input < 1 || input > index_limit {
            return Err("Not a valid number in context");
        }
        Ok(input)
    }
}

fn maplidator_int_index_limit_zero(index_limit: usize) -> impl Fn(String) -> Result<usize, &'static str> {
    move |input| {
        let input = input.trim();
        let input: usize = input.parse().map_err(|_| "Failed to parse into number")?;
        if input >= index_limit {
            return Err("Not a valid number in context");
        }
        Ok(input)
    }
}

fn read_valid_stdin<F, T>(prompt: &str, maplidator: F) -> T
where
    F: Fn(String) -> Result<T, &'static str>,
{
    loop {
        let mut input = String::new();
        print!("{}", prompt);
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).unwrap();
        match maplidator(input) {
            Ok(value) => break value,
            Err(msg) => println!("{}", msg),
        }
    }
}

fn get_from_iterator_and_parse<'a, T: FromStr>(it: &mut impl Iterator<Item = &'a str>) -> Result<T, ()> {
    match it.next() {
        None => {
            Err(())
        }
        Some(x) => {
            match x.parse::<T>() {
                Ok(x) => { Ok(x) }
                Err(_) => {
                    Err(())
                }
            }
        }
    }
}