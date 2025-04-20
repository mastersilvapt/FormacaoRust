use std::io::Write;

#[derive(Clone, Debug)]
struct Product {
    id: u32,
    name: String,
    expiration: String,
    price: f64,
    quantity: usize,
}

// Fileira -> Prateleira -> Zona
enum Command {
    AddProduct(usize, usize, usize, Product),
    Resupply(u32, usize),
    RemoveProduct(u32, usize),
    ChangePrice(u32, f64),
    MoveTo(u32, usize, usize, usize),
    Show(),
}

#[derive(Clone, Debug)]
struct Supermarket {
    products: Vec<Vec<Vec<Option<Product>>>>,
}

impl Supermarket {
    fn new() -> Self {
        let  products: Vec<Vec<Vec<Option<Product>>>> = vec![vec![vec![]]];
        Supermarket { products }
    }

    fn find_coords(&self, id: u32) -> Result<(usize, usize, usize), &'static str> {
        for (i, fileira) in self.products.iter().enumerate() {
            for (j, prateleira) in fileira.iter().enumerate() {
                for(k, opt) in prateleira.iter().enumerate() {
                    match opt {  
                        Some(prod) => {
                           if(prod.id == id) {
                               return Ok((i, j, k));
                           } 
                        }
                        None => {
                            continue;
                        }
                    }
                    
                }
            }
        }
        Err("Product not found")
    }
    
    fn make_position(&mut self, (i, j, k): (usize, usize, usize)) {
        while self.products.len() <= i {
            self.products.push(vec![]);
        }
        while self.products[i].len() <= j {
            self.products[i].push(vec![]);
        }
        self.products[i][j].resize(k+1, None);
    }
    
    fn apply(&mut self, command: Command) -> Result<&str, &'static str>{
        match command {
            Command::AddProduct(fileira, prateleira, zone, product) => {
                match self.find_coords(product.id){
                    Ok(_) => {
                         Err("Product already exists")          
                    }
                    Err(_) => {
                        self.make_position((fileira, prateleira, zone));
                        self.products[fileira][prateleira][zone] = Some(product);
                        Ok("Added product")
                    }
                }
            }
            Command::Resupply(id, quantity) => {
                let (i,j,k) = self.find_coords(id)?;
                let product = &mut self.products[i][j][k];
                if product.is_some() {
                    product.as_mut().unwrap().quantity += quantity;
                    Ok("Resupplied")
                }else { 
                    Err("Product not found")
                }
            }
            Command::RemoveProduct(id, quantity) => {
                let (i,j,k) = self.find_coords(id)?;
                let product = &mut self.products[i][j][k];
                if product.as_mut().unwrap().quantity < quantity { 
                    return Err("Cannot remove more than in ");
                }
                if product.as_mut().unwrap().quantity - quantity == 0 {
                    self.products[i][j].remove(k);
                }else {
                    product.as_mut().unwrap().quantity -= quantity;
                }
                Ok("Product Removed")
            }
            Command::ChangePrice(id, new_price) => {
                let (i,j,k) = self.find_coords(id)?;
                let product = &mut self.products[i][j][k];
                product.as_mut().unwrap().price = new_price;
                Ok("New price set")
            }

            Command::MoveTo(id, fileira, prateleira, zone) => {
                let (i, j, k) = self.find_coords(id)?;
                if self.products[fileira][prateleira][zone].is_some(){
                    return Err("Product already occupied");
                }

                let product = self.products[i][j].remove(k);
                self.make_position((fileira, prateleira, zone));
                self.products[fileira][prateleira][zone] = product;
                
                Ok("Product moved")
            }
            
            Command::Show() => {
                self.products.iter().enumerate().for_each(|(i, fileira)| {
                    fileira.iter().enumerate().for_each(|(j, prateleira)| {
                        prateleira.iter().enumerate().for_each(|(k, product)| {
                            if product.is_some() {
                                println!("{} {} {} -> {:?}", i, j, k, product.clone().unwrap());
                            }
                        })
                    })
                });
                
                
                Ok("Showed")
            }
        }

    }

    fn help(&self){
        println!("Usage:");
        println!("   addProduct Fila Prateleira Zona product.id product.name product.expiration_date product.price product.quantity");
        println!("   resupply product.id product.quantity");
        println!("   removeProduct product.id product.quantity");
        println!("   changePrice product.id product.new_price");
        println!("   moveTo product.id Fila Prateleira Zona");
        println!("   show");
        println!();
    }

    fn read_command(&self, command: &str) -> Result<Command, &'static str> {
        println!("Command: {}", command);
        let mut splits = command.split_ascii_whitespace();
        match splits.next().unwrap() {
            "addProduct" => {
                let fila = splits.next().unwrap_or("Expecting Fila");
                let fila = fila.parse::<usize>().or(Err("Failed to parse 'Fila'"))?;

                let prateleira = splits.next().unwrap_or("Expecting Prateleira");
                let prateleira = prateleira.parse::<usize>().or(Err("Failed to parse 'Prateleira'"))?;

                let zona = splits.next().unwrap_or("Expecting Zona");
                let zona = zona.parse::<usize>().or(Err("Failed to parse 'Zona'"))?;

                let product_id = splits.next().unwrap_or("Expecting Product ID");
                let product_id = product_id.parse::<u32>().map_err(|_| "Invalid ID")?;

                let product_name = splits.next().unwrap_or("Expecting Product Name");

                let product_date = splits.next().unwrap_or("Expecting Product Expiration Date");

                let product_price = splits.next().unwrap_or("Expecting Product Price");
                let product_price = product_price.parse::<f64>().map_err(|_| "Invalid Price")?;

                let product_quantity = splits.next().unwrap_or("Expecting Product Quantity");
                let product_quantity = product_quantity.parse::<usize>().map_err(|_| "Invalid quantity")?;

                Ok(Command::AddProduct(fila, prateleira, zona, Product { 
                    id: product_id, 
                    name: product_name.to_string(), 
                    expiration: product_date.to_string(), 
                    price: product_price,
                    quantity: product_quantity 
                }))
            },
            "resupply" => {
                let product_id = splits.next().unwrap_or("Expecting Product ID");
                let product_id = product_id.parse::<u32>().map_err(|_| "Invalid ID")?;
                let product_quantity = splits.next().unwrap_or("Expecting Product Quantity");
                let product_quantity = product_quantity.parse::<usize>().map_err(|_| "Invalid quantity")?;
                Ok(Command::Resupply(product_id, product_quantity))
            }
            "removeProduct" => {
                let product_id = splits.next().unwrap_or("Expecting Product ID");
                let product_id = product_id.parse::<u32>().map_err(|_| "Invalid ID")?;
                let product_quantity = splits.next().unwrap_or("Expecting Product Quantity");
                let product_quantity = product_quantity.parse::<usize>().map_err(|_| "Invalid quantity")?;
                Ok(Command::RemoveProduct(product_id, product_quantity))
            },
            "changePrice" => {
                let product_id = splits.next().unwrap_or("Expecting Product ID");
                let product_id = product_id.parse::<u32>().map_err(|_| "Invalid ID")?;
                let product_price = splits.next().unwrap_or("Expecting Product Price");
                let product_price = product_price.parse::<f64>().map_err(|_| "Invalid price")?;
                Ok(Command::ChangePrice(product_id, product_price))
            },
            "moveTo" => {
                let product_id = splits.next().unwrap_or("Expecting Product ID");
                let product_id = product_id.parse::<u32>().map_err(|_| "Invalid ID")?;
                let fila = splits.next().unwrap_or("Expecting Fila");
                let fila = fila.parse::<usize>().or(Err("Failed to parse 'Fila'"))?;
                let prateleira = splits.next().unwrap_or("Expecting Prateleira");
                let prateleira = prateleira.parse::<usize>().or(Err("Failed to parse 'Prateleira'"))?;
                let zona = splits.next().unwrap_or("Expecting Zona");
                let zona = zona.parse::<usize>().or(Err("Failed to parse 'Zona'"))?;
                Ok(Command::MoveTo(product_id, fila, prateleira, zona))
            },
            "show" => {
                Ok(Command::Show())
            }
            _ => Err("Command Not valid"),
        }
    }
}

fn main() {
    let mut supermarket = Supermarket::new();
    
    let _ = supermarket.apply(Command::AddProduct(1,5,6, Product { id: 123, name: "test1".to_string(), expiration: "12/05/25".to_string(), price: 5.32, quantity: 1 }));
    let _ = supermarket.apply(Command::AddProduct(6,2,4, Product { id: 234, name: "test2".to_string(), expiration: "13/06/26".to_string(), price: 15.32, quantity: 5 }));
    let _ = supermarket.apply(Command::AddProduct(2,6,2, Product { id: 345, name: "test3".to_string(), expiration: "14/07/27".to_string(), price: 7.87, quantity: 2 }));
    
    loop{
        supermarket.help();
        let command = read_valid_stdin(">", |x| supermarket.read_command(x.trim()));
        match supermarket.apply(command){
            Ok(result) => println!("{}", result),
            Err(result) => println!("{}", result),
        }
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