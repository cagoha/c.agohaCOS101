use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Lower bound is :");
    io::stdin().read_line(&mut input1).expect("Failed to input string");
    let lower_bound:i32 = input1.trim().parse().expect("Failed to input");

    println!("Upper bound is :");
    io::stdin().read_line(&mut input2).expect("Failed to input string");
    let upper_bound:i32 = input2.trim().parse().expect("Failed to input");

    for x in lower_bound..upper_bound{
        println!("Count level is {}",x );
    }
}