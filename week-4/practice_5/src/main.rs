use std::io;

fn main() {
    let mut input = String::new();

    println!("\nEnter your height(in centimeters):");
    io::stdin().read_line(&mut input).expect("Invalid String");
    let height:f32 = input.trim().parse().expect("Invalid number");

    if height >= 150.00 && height <= 170.0
    { println!("You are of average height");
}
else if height > 170.0 && height <= 195.0
{
    println!("You are Tall asf");
}
else if height < 150.00 && height >= 100.00
{
    println!("You're A Dwarf!");
}
else {
    println!("Abnormal Height");
}
}