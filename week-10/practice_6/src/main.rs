fn add_one(x:&mut i32){
    *x+= 1;
}
fn main() {
    let mut i = 3;
    add_one(&mut i);
    println!("{}",i );
}