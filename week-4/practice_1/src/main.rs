use std::io;

fn main() {
    println!("\nStudents Information Management System");

    println!("\nEnter your name");
    let mut name = String::new();

    io::stdin()
    .read_line(&mut name)
    .expect("Failed to read input");
    println!("Your name is {}",name );

    println!("Enter Your Age");
    let mut age =String::new();

    io::stdin()
    .read_line(&mut age)
    .expect("Failed to read input");
    let age:u8 = age.trim().parse().expect("Age is not an integer");
    println!("You are {} years old",age );
}   