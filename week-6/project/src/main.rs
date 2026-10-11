use std::io;

fn trapezium_area(){
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("\nEnter height");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let height:i32 = input1.trim().parse().expect("Invalid input");

    println!("\nEnter base A ");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let base_a:i32  = input2.trim().parse().expect("Invalid input");

    println!("\nEnter base B");
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let base_b:i32 = input3.trim().parse().expect("Invalid input");

    let area = height / 2 * (base_a + base_b);
    println!("Area is {}",area );
}

fn rhombus_area(){
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("\nEnter first diagonal");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let diagonal1:i32 = input1.trim().parse().expect("Invalid input");

    println!("\nEnter second diagonal");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let diagonal2:i32 = input2.trim().parse().expect("Invalid input");

    let area = (diagonal1 * diagonal2) / 2;
    println!("Area is {}",area );
}
fn parrallelogram_area(){
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("\n Enter the base");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let base:i32 = input1.trim().parse().expect("Invalid input");

    println!("\n Enter the altitude");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let altitude:i32 = input2.trim().parse().expect("Invalid input");

    let area = base * altitude;
    println!("Area is {}",area );
}
fn cube_surfacearea(){
    let mut input = String::new();

    println!("\nEnter the cube side ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let side:i32 = input.trim().parse().expect("Invalid input");

    let surface_area = 6 * side * side;
    println!("Surface area is {}",surface_area );

}
fn cylinder_volume(){
    let mut input1 = String::new(); 
    let mut input2 = String::new();
    let pi = 22 / 7;

    println!("\nEnter the value for radius");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let radius:i32 = input1.trim().parse().expect("Invalid input");

    println!("\nEnter the value for height");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let height:i32 = input2.trim().parse().expect("Invalid input");

    let volume = pi * radius * radius * height;
    println!("Volume is {}",volume );
}
fn main() {
    println!("area of trapezium");
    println!("area of rhombus");
    println!("area of parallelogram");
    println!("surface area of a cube");
    println!("volume of cylinder");
    println!("\nWhich of the following would you like to find ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let user_input:String = input.trim().to_lowercase();

    if user_input == "area of trapezium"{
        trapezium_area();
    }
    else if user_input == "area of rhombus" {
        rhombus_area();
    }
    else if user_input == "area of parallelogram"{
        parrallelogram_area();
    }
    else if user_input == "surface area of a cube"{
        cube_surfacearea();
    }
    else if user_input == "volume of cylinder"{
        cylinder_volume();
    }
    else {
        println!("You have to pick from the list");
    }
}