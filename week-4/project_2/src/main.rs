use std::io;

fn main() {
    let mut experience_input = String::new();
    let mut age_input = String::new();

    println!("Is the employee experienced? (yes/no): ");
    io::stdin().read_line(&mut experience_input).expect("Failed to read input");
    let is_experienced = experience_input.trim().to_lowercase();

    println!("Enter employee age: ");
    io::stdin().read_line(&mut age_input).expect("Failed to read input");
    let age: u32 = age_input.trim().parse().expect("Failed to input");

    let incentive = if is_experienced == "yes" || is_experienced == "y" {
        if age >= 40 {
            1_560_000 
        } else if age >= 30 && age <= 39 {
            1_480_000 
        } else if age < 28 {
            1_300_000 
        } else {
            1_300_000 
        }
    } else {
        100_000 
    };

    println!("Annual Incentive: N{}", incentive);
}