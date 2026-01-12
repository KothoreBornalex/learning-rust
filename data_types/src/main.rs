fn main() {
    //println!("Hello, world!");

    let x: i32 = -42; // i32 is either positive or negative it's signed.
    let y: u64 = 100; // u64 is strictly positive it's unsigned.
                      //
    println!("Signed Integer: {}", x);
    println!("Unsigned Integer: {}", y);

    let e: i32 = 2147483647; // largest i32 positive or negative value.
    let i: i64 = 9223372036854775807; // larget u64 positive or negative value.
                      
    println!("Maximum value of i32: {}", e);
    println!("Maximum value of i64: {}", i);


    let pi: f64 = 3.14; // i32 is either positive or negative it's signed.
    println!("Value of pi: {}", pi);
    

    let is_snowing: bool = true; // bool value either true of false
    println!("Is it snowing ? {}", is_snowing);

    let letter: char = 'a';
    println!("First letter of the alphabet is: {}", letter);

}

