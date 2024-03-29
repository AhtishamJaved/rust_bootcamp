use std::collections::HashMap;
use std::io;

#[derive(Debug)]
struct Product {
    name: String,
    description: String,
    price: f64,
    quantity: u32,
}

enum Action {
    Add,
    Edit,
    Delete,
}

struct InventoryManager {
    inventory: HashMap<String, Product>,
}

impl InventoryManager {
    fn new() -> Self {
        InventoryManager {
            inventory: HashMap::new(),
        }
    }

    fn add_product(&mut self, name: String, description: String, price: f64, quantity: u32) {
        let product = Product {
            name: name.clone(),
            description,
            price,
            quantity,
        };
        self.inventory.insert(name, product);
    }

    fn edit_product(&mut self, name: &str, price: f64, quantity: u32) {
        if let Some(product) = self.inventory.get_mut(name) {
            product.price = price;
            product.quantity = quantity;
        } else {
            println!("Oops! Can't find that product.");
        }
    }

    fn delete_product(&mut self, name: &str) {
        self.inventory.remove(name);
    }

    fn generate_report(&self) {
        println!("===== Inventory Report =====");
        for (name, product) in &self.inventory {
            println!(
                "{}: {} - ${:.2} - {} in stock",
                name, product.description, product.price, product.quantity
            );
        }
    }

    fn search_product(&self, name: &str) {
        if let Some(product) = self.inventory.get(name) {
            println!("===== Product Details =====");
            println!(
                "Name: {}\nDescription: {}\nPrice: ${:.2}\nQuantity: {}",
                product.name, product.description, product.price, product.quantity
            );
        } else {
            println!("Product not found.");
        }
    }

    fn view_low_stock_products(&self, threshold: u32) {
        println!("===== Low Stock Products =====");
        for (name, product) in &self.inventory {
            if product.quantity < threshold {
                println!(
                    "{}: {} - ${:.2} - {} in stock",
                    name, product.description, product.price, product.quantity
                );
            }
        }
    }

    fn sort_products(&self, by: &str) {
        let mut products: Vec<_> = self.inventory.values().collect();
        match by {
            "name" => products.sort_by_key(|p| p.name.clone()),
            "price" => products.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap()),
            "quantity" => products.sort_by(|a, b| a.quantity.cmp(&b.quantity)),
            _ => {
                println!("Invalid sorting option.");
                return;
            }
        }
        println!("===== Sorted Products =====");
        for product in products {
            println!(
                "{}: {} - ${:.2} - {} in stock",
                product.name, product.description, product.price, product.quantity
            );
        }
    }
}

fn main() {
    let mut inventory_manager = InventoryManager::new();


    inventory_manager.add_product(
        "Apple".to_string(),
        "Fresh Red Apple".to_string(),
        1.25,
        100,
    );
    inventory_manager.add_product(
        "Banana".to_string(),
        "Ripe Yellow Banana".to_string(),
        0.75,
        150,
    );

    println!("Welcome to Rusty's Amazing Inventory Manager!");

    loop {
        println!("What do you want to do today?");
        println!("1. Add a new product");
        println!("2. Edit an existing product");
        println!("3. Delete a product");
        println!("4. Generate an inventory report");
        println!("5. Search for a product");
        println!("6. View low stock products");
        println!("7. Sort products");
        println!("8. Exit");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim().parse() {
            Ok(1) => {
                let mut name = String::new();
                let mut description = String::new();
                let mut price = String::new();
                let mut quantity = String::new();

                println!("Tell me about this new product!");
                println!("Name:");
                io::stdin()
                    .read_line(&mut name)
                    .expect("Failed to read line");

                println!("Description:");
                io::stdin()
                    .read_line(&mut description)
                    .expect("Failed to read line");

                println!("Price:");
                io::stdin()
                    .read_line(&mut price)
                    .expect("Failed to read line");
                let price: f64 = price.trim().parse().unwrap();

                println!("Quantity:");
                io::stdin()
                    .read_line(&mut quantity)
                    .expect("Failed to read line");
                let quantity: u32 = quantity.trim().parse().unwrap();

                inventory_manager.add_product(
                    name.trim().to_string(),
                    description.trim().to_string(),
                    price,
                    quantity,
                );
            }
            Ok(2) => {
                let mut name = String::new();
                let mut price = String::new();
                let mut quantity = String::new();

                println!("Which product do you want to edit?");
                io::stdin()
                    .read_line(&mut name)
                    .expect("Failed to read line");

                println!("What's the new price?");
                io::stdin()
                    .read_line(&mut price)
                    .expect("Failed to read line");
                let price: f64 = price.trim().parse().unwrap();

                println!("How many are left?");
                io::stdin()
                    .read_line(&mut quantity)
                    .expect("Failed to read line");
                let quantity: u32 = quantity.trim().parse().unwrap();

                inventory_manager.edit_product(name.trim(), price, quantity);
            }
            Ok(3) => {
                let mut name = String::new();
                println!("Which product do you want to delete?");
                io::stdin()
                    .read_line(&mut name)
                    .expect("Failed to read line");
                inventory_manager.delete_product(&name.trim().to_string());
            }
            Ok(4) => {
                inventory_manager.generate_report();
            }
            Ok(5) => {
                let mut name = String::new();
                println!("Enter the name of the product you want to search for:");
                io::stdin()
                    .read_line(&mut name)
                    .expect("Failed to read line");
                inventory_manager.search_product(name.trim());
            }
            Ok(6) => {
                let mut threshold = String::new();
                println!("Enter the threshold for low stock products:");
                io::stdin()
                    .read_line(&mut threshold)
                    .expect("Failed to read line");
                let threshold: u32 = threshold.trim().parse().unwrap();
                inventory_manager.view_low_stock_products(threshold);
            }
            Ok(7) => {
                let mut by = String::new();
                println!("Sort products by (name/price/quantity):");
                io::stdin()
                    .read_line(&mut by)
                    .expect("Failed to read line");
                inventory_manager.sort_products(by.trim());
            }
            Ok(8) => {
                println!("See you next time!");
                break;
            }
            _ => println!("Invalid option. Please choose again."),
        }
    }
}
