fn main() {
    hello_world();
    tell_height(185);
    human_id("Ibrah", 20, 185.3);

    let _x: i32 = {
        let price: i32 = 5;
        let qty: i32 = 10;
        price*qty // or return price*qty;
    };

    println!("Result is: {}", _x);

    let y: i32 = add(4, 6);
    println!("Value of y is: {}", y);


    println!("My Body Mass Index: {}", calculate_bmi(80.0, 1.85));
}

fn hello_world() // Thanks to hoisting I can define the fn after using it
{
    println!("Hello, world!");
}

fn tell_height(height: u32)
{
    println!("My Height: {}", height);
}

fn human_id(name: &str, age: u32, height: f32)
{
    println!("My name is {}, I am {} years old, and my height is {} cm.", name, age, height);
}


fn add(a: i32, b: i32) -> i32 // arrow and type is for function returning values
{
    a + b
}

// Expressions and Statements
// Expression; Anything that returns a value.
// Statement Anything that does not return a value.
// Almost all statements in Rust end with ;
// let y = let x = 10;


fn calculate_bmi(weight: f64, height: f64) -> f64
{
    weight/(height*height)
}



