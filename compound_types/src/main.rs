fn main() {
    //println!("Hello, world!");
    let numbers: [i32; 5] = [1,2,3,4,5]; // Classic array
    println!("Number Array: {:?}", numbers);
    
    let fruits: [&str; 3] = ["Apple", "Banana", "Orange"]; // String array
    println!("Fruits Array: {:?}", fruits);

    println!("Fruits Array 1St: {}", fruits[0]);
    println!("Fruits Array 2nd: {}", fruits[1]);
    println!("Fruits Array 3rd: {}", fruits[2]);


    // Tuples
    let human: (String, i32, bool) = ("Alice".to_string(), 30, false);
    println!("human Tuple: {:?}", human);

    let mix_tuple = ("Kratos", 23, true, [1,2,3,4,5]);
    println!("My mix Tuple: {:?}", mix_tuple);

    let number_slices: &[i32] = &[1,2,3,4,5];
    println!("Number Slice: {:?}", number_slices);

    let animal_slices: &[&str] = &["Lion", "Elephant", "Crocodile"];
    println!("Animal Slice: {:?}", animal_slices);

    let book_slices: &[&String] = &[&"IT".to_string(), &"Harry Potter".to_string(), &"ZEN".to_string()];
    println!("Book Slice: {:?}", book_slices);


    

    // String vs String Slices (&str)
    // Strings are Growable, Mutable, Owned string type.
    let mut stone_cold: String = String::from("Hell, "); // the mut keyword here make it possible
                                                         // to edit it after creation, with the
                                                         // push_str fn for exemple !
    stone_cold.push_str("Yeah!");
    println!("Stone Cold Say: {}", stone_cold);
    
    let string: String = String::from("Hello World!");
    let slice: &str = &string[6..12];
    println!("Slice Value: {}", slice);
}
