use task_2::{Coin, Product, VendingMachine};

fn main() {
    let mut machine = VendingMachine::new(5, 10);

    machine
        .add_product(
            0,
            Product {
                name: "Soda".to_string(),
                price: 45,
            },
            5,
        )
        .unwrap();
    machine
        .add_product(
            1,
            Product {
                name: "Chips".to_string(),
                price: 30,
            },
            10,
        )
        .unwrap();

    machine.add_coins(Coin::Ten, 5);
    machine.add_coins(Coin::Five, 5);
    machine.add_coins(Coin::One, 10);

    match machine.purchase_product(0, "Soda", &vec![Coin::Fifty]) {
        Ok(msg) => println!("{}", msg),
        Err(err) => println!("Error: {}", err),
    };

    match machine.purchase_product(
        0,
        "Soda",
        &vec![Coin::Twenty, Coin::Ten, Coin::Ten, Coin::Five, Coin::One],
    ) {
        Ok(msg) => println!("{}", msg),
        Err(err) => println!("Error: {}", err),
    };
}
