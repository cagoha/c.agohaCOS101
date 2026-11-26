use std::fs::File;
use std::io::{self, Read};

fn read_file(filename: &str) {
    let mut file = File::open(filename).unwrap();

    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    println!("{}", contents);
}

fn administrator() {
    println!("DATABASE STRUCTURE");
    read_file("globacom_dbase.sql");
}

fn customer() {
    println!("CUSTOMER TABLE");
    read_file("customer_tb.sql");
}

fn vendor() {
    println!("DATAPLAN TABLE");
    read_file("dataplan_tb.sql");
}
fn project(){
    println!("PROJECT MANAGEMENT TABLE");
    read_file("project_tb.sql");
}
fn staff(){
    println!("STAFF TABLE");
    read_file("staff_tb.sql");
}

fn main() {
    let mut input = String::new();

    println!("1. Administrator");
    println!("2. Customer");
    println!("3. Vendor");
    println!("4. Project");
    println!("5. Staff");
    println!("Choose a role:");

    io::stdin().read_line(&mut input).unwrap();
    let choice:i32 = input.trim().parse().unwrap();

    if choice == 1{
        administrator();
    }
    else if choice == 2{
        customer();
    }
    else if choice == 3{
        vendor();
    }
    else if choice == 4{
            project();
        }
    else if choice == 5{
            staff();
        }
    else {
        println!("You have to choose from the list");
    }
}