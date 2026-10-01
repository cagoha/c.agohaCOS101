fn main() {
    let A:i32 = 10;
    let B:i32 = 20;

    println!("Value of A: {}",A );
    println!("Value of B: {}",B );

    let mut result = A > B ;
    println!("A greater then B is {}",result );

    result = A < B ; 
    println!("A lesser than B is {}",result );

    result = A >= B ;
    println!("A greater than or equal to B is {}",result );

    result = A <= B ;
    println!("A less than or equal to B is {}",result );

    result = A == B;
    println!("A is equal to B: {}",result );

    result = A != B;
    println!("A is not equal to B: {}",result );
}