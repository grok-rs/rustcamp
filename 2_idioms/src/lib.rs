use std::collections::{hash_map, HashMap};

#[derive(Debug, Clone)]
pub struct Product {
    pub name: String,
    pub price: u32, // price in "cents" or smallest currency unit
}

#[derive(Debug, Clone)]
pub struct Shelf {
    pub capacity: usize,
    pub products: HashMap<String, (Product, usize)>, // (Product, current_stock)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum Coin {
    One = 1,
    Two = 2,
    Five = 5,
    Ten = 10,
    Twenty = 20,
    Fifty = 50,
}

impl Coin {
    pub fn value(&self) -> u32 {
        *self as u32
    }
}

#[derive(Debug)]
pub struct VendingMachine {
    pub shelves: Vec<Shelf>,
    pub coins: HashMap<Coin, u32>, // Coin -> count
}

impl VendingMachine {
    pub fn new(shelf_count: usize, shelf_capacity: usize) -> Self {
        VendingMachine {
            shelves: vec![
                Shelf {
                    capacity: shelf_capacity,
                    products: HashMap::new(),
                };
                shelf_count
            ],
            coins: HashMap::new(),
        }
    }

    pub fn add_product(
        &mut self,
        shelf_index: usize,
        product: Product,
        quantity: usize,
    ) -> Result<(), String> {
        if shelf_index >= self.shelves.len() {
            return Err("Invalid shelf index".to_string());
        }

        let shelf = &mut self.shelves[shelf_index];
        match shelf.products.entry(product.name.clone()) {
            hash_map::Entry::Occupied(mut entry) => {
                let (_, current_stock) = entry.get_mut();
                if *current_stock + quantity <= shelf.capacity {
                    *current_stock += quantity;
                    Ok(())
                } else {
                    Err("Exceeds shelf capacity".to_string())
                }
            }
            hash_map::Entry::Vacant(entry) => {
                if quantity <= shelf.capacity {
                    entry.insert((product, quantity));
                    Ok(())
                } else {
                    Err("Exceeds shelf capacity".to_string())
                }
            }
        }
    }

    pub fn add_coins(&mut self, coin: Coin, count: u32) {
        *self.coins.entry(coin).or_insert(0) += count;
    }

    /// Checks if we *can* give exact `change` (does not modify internal state).
    fn can_give_change(&self, mut change: u32) -> bool {
        // Create a temporary copy of available coins.
        let mut available_coins = self.coins.clone();

        // Sort coins in descending value order so we try to give biggest coins first.
        let mut denominations = vec![
            Coin::Fifty,
            Coin::Twenty,
            Coin::Ten,
            Coin::Five,
            Coin::Two,
            Coin::One,
        ];
        denominations.sort_by(|a, b| b.value().cmp(&a.value()));

        for &coin in &denominations {
            while change >= coin.value() {
                if let Some(count) = available_coins.get_mut(&coin) {
                    if *count > 0 {
                        *count -= 1;
                        change -= coin.value();
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
        }

        change == 0
    }

    fn update_coins(&mut self, inserted_coins: &[Coin], mut change: u32) {
        for &coin in inserted_coins {
            *self.coins.entry(coin).or_insert(0) += 1;
        }

        let mut denominations = vec![
            Coin::Fifty,
            Coin::Twenty,
            Coin::Ten,
            Coin::Five,
            Coin::Two,
            Coin::One,
        ];
        denominations.sort_by(|a, b| b.value().cmp(&a.value()));

        for &coin in &denominations {
            while change >= coin.value() {
                if let Some(count) = self.coins.get_mut(&coin) {
                    if *count > 0 {
                        *count -= 1;
                        change -= coin.value();
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
        }
    }

    /// Attempts to purchase one unit of the specified product from the given shelf
    /// using the provided `inserted_coins`. Returns the total amount of change
    /// (in cents) if successful.
    pub fn purchase_product(
        &mut self,
        shelf_index: usize,
        product_name: &str,
        inserted_coins: &[Coin],
    ) -> Result<u32, String> {
        if shelf_index >= self.shelves.len() {
            return Err("Invalid shelf index".to_string());
        }

        let shelf_immutable = &self.shelves[shelf_index];
        let (product, stock) = shelf_immutable
            .products
            .get(product_name)
            .ok_or_else(|| "Product not found on shelf".to_string())?;

        if *stock == 0 {
            return Err("Product is out of stock".to_string());
        }

        let price = product.price;

        let total_inserted: u32 = inserted_coins.iter().map(|coin| coin.value()).sum();
        if total_inserted < price {
            return Err("Not enough coins inserted".to_string());
        }
        let change = total_inserted - price;

        if change > 0 && !self.can_give_change(change) {
            return Err("Cannot provide exact change.".to_string());
        }

        {
            let shelf_mutable = &mut self.shelves[shelf_index];
            let (_, stock_mut) = shelf_mutable
                .products
                .get_mut(product_name)
                .expect("Product missing despite earlier checks");

            *stock_mut -= 1;
        }

        self.update_coins(inserted_coins, change);

        Ok(change)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_product() {
        let mut vm = VendingMachine::new(3, 10);
        let product = Product {
            name: "Water".to_string(),
            price: 10,
        };
        assert!(vm.add_product(0, product.clone(), 5).is_ok());
        assert_eq!(vm.shelves[0].products["Water"].1, 5);
    }

    #[test]
    fn test_add_product_exceed_capacity() {
        let mut vm = VendingMachine::new(3, 10);
        let product = Product {
            name: "Water".to_string(),
            price: 10,
        };
        assert!(vm.add_product(0, product.clone(), 15).is_err());
    }

    #[test]
    fn test_add_coins() {
        let mut vm = VendingMachine::new(3, 10);
        vm.add_coins(Coin::Ten, 3);
        vm.add_coins(Coin::Five, 5);
        assert_eq!(*vm.coins.get(&Coin::Ten).unwrap(), 3);
        assert_eq!(*vm.coins.get(&Coin::Five).unwrap(), 5);
    }

    #[test]
    fn test_can_give_change_true() {
        let mut vm = VendingMachine::new(3, 10);
        vm.add_coins(Coin::Ten, 1);
        vm.add_coins(Coin::Five, 1);
        assert!(vm.can_give_change(15));
    }

    #[test]
    fn test_can_give_change_false() {
        let mut vm = VendingMachine::new(3, 10);
        vm.add_coins(Coin::Ten, 1);
        assert!(!vm.can_give_change(15));
    }

    #[test]
    fn test_can_give_change_large_amount() {
        let mut vm = VendingMachine::new(3, 10);
        vm.add_coins(Coin::Fifty, 2);
        vm.add_coins(Coin::Twenty, 1);
        vm.add_coins(Coin::Ten, 1);
        assert!(vm.can_give_change(130));
    }

    #[test]
    fn test_purchase_product_exact_price() {
        let mut vm = VendingMachine::new(1, 10);
        let product = Product {
            name: "Soda".to_string(),
            price: 10,
        };
        vm.add_product(0, product.clone(), 5).unwrap();
        vm.add_coins(Coin::Ten, 1);

        let inserted_coins = vec![Coin::Ten];
        let result = vm.purchase_product(0, "Soda", &inserted_coins);

        assert_eq!(result, Ok(0));
        assert_eq!(vm.shelves[0].products["Soda"].1, 4);
        assert_eq!(*vm.coins.get(&Coin::Ten).unwrap(), 2);
    }

    #[test]
    fn test_purchase_product_with_change() {
        let mut vm = VendingMachine::new(1, 10);
        let product = Product {
            name: "Juice".to_string(),
            price: 7,
        };
        vm.add_product(0, product.clone(), 5).unwrap();
        vm.add_coins(Coin::One, 5);

        let inserted_coins = vec![Coin::Ten];
        let result = vm.purchase_product(0, "Juice", &inserted_coins);

        assert_eq!(result, Ok(3));
        assert_eq!(vm.shelves[0].products["Juice"].1, 4);
        assert_eq!(*vm.coins.get(&Coin::One).unwrap(), 2);
        assert_eq!(*vm.coins.get(&Coin::Ten).unwrap(), 1);
    }
}
