use std::io;

fn main() {
    println!("Welcome to The Restaurant Menu!");
    println!("P: Poundo Yam / Edinkaiko Soup - ₦3200");
    println!("F: Fried Rice & Chicken - ₦3000");
    println!("A: Amala & Ewedu Soup - ₦2500");
    println!("E: Eba & Egusi Soup - ₦2000");
    println!("W: White Rice & Stew - ₦2500");

    println!("\nEnter the food type (P, F, A, E, W):");
    let mut food_type = String::new();
    io::stdin().read_line(&mut food_type).expect("Failed to read input");
    let food_type = food_type.trim();

    println!("Enter quantity:");
    let mut quantity = String::new();
    io::stdin().read_line(&mut quantity).expect("Failed to read input");
    let quantity: i32 = quantity.trim().parse().expect("Please enter a valid number");

    let price: i32;
    if food_type == "P" {
        price = 3200;
    } else if food_type == "F" {
        price = 3000;
    } else if food_type == "A" {
        price = 2500;
    } else if food_type == "E" {
        price = 2000;
    } else if food_type == "W" {
        price = 2500;
    } else {
        println!("Invalid food type!");
        return;
    }

    let mut total = price * quantity;

    if total > 10000 {
        let discount = total as f32 * 0.05;
        total = (total as f32 - discount) as i32;
        println!("You got a 5% discount!");
    }

    println!("Total charge: ₦{}", total);
}
