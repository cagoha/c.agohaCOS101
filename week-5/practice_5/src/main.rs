fn main() {
    let fullname = " Pan-Atlantic University ";
    println!();
    println!("Name: {}",fullname );
    println!("\nbefore trim");
    println!("fullname length is {}",fullname.len() );
    println!("After trim");
    println!("fullname length is {}",fullname.trim().len() );
}