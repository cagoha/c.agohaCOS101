use std::io;

fn add(a:i32 ,b:i32){
    let sum = a + b;
    println!("sum of A and B is {} ",sum);
}

fn main() {
    println!("Enter input for parameter A");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let a = input1.trim().parse().expect("Invalid input");

    println!("Enter input for parameter B");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let b = input2.trim().parse().expect("Invalid input");

    add(a,b);
}