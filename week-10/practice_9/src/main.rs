struct Rectangle{
    height:u32,
    width:u32,
}
impl Rectangle{
    fn area(&self)->u32{
        self.width*self.height
    }
}
fn main() {
    let small = Rectangle{
        width:10,
        height:20
    };
    println!("\nwidth is: {}\nheight is: {}\nThe area of the Rectangle is: {} ",small.width, small.height, small.area() );
}