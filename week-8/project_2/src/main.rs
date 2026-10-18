use std::io;

fn main() {
    let mut candidates: Vec<(String, u32)> = Vec::new();

    println!("Enter the number of candidates:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let num: usize = input.trim().parse().expect("Invalid input");

    for i in 1..=num {
        println!("Enter candidate {}'s name:", i);
        let mut input1 = String::new();
        io::stdin().read_line(&mut input1).expect("Failed to read input");
        let name = input1.trim().to_string();

        println!("Enter years of programming experience for {}:", name);
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).expect("Failed to read input");
        let years: u32 = input2.trim().parse().expect("Invalid input");

        candidates.push((name, years));
    }

    let mut highest = &candidates[0];
    for candidate in &candidates {
        if candidate.1 > highest.1 {
            highest = candidate;
        }
    }

    println!(
        "\nThe candidate with the highest experience is {} with {} years.",
        highest.0, highest.1
    );
}
