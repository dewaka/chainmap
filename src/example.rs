mod lib;

use lib::ChainMap;
use std::collections::HashMap;

fn example() {
    let mut toys = HashMap::new();
    toys.insert("Blocks", 30);
    toys.insert("Monopoly", 20);

    let mut computers = HashMap::new();
    computers.insert("iMap", 1000);
    computers.insert("Chromebook", 800);
    computers.insert("PC", 400);

    let mut clothing = HashMap::new();
    computers.insert("Jeans", 40);
    computers.insert("T-Shirt", 10);

    let inventory = ChainMap::new(vec![toys, computers, clothing]);

    println!("{:?}", inventory.get(&"Monopoly"));
    println!("{:?}", inventory.get(&"Mario Bros."));
    println!("{:?}", inventory.get(&"Chromebook"));
    println!("{:?}", inventory.get(&"Jeans"));
}

fn main() {
    println!("ChainMap inventory example");
    example();
}
