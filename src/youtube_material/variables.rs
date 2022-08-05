// Variables hold primitive data or references to data
// Variables are immutable by default -> Variables cannot be reassigned by default (const by default)
// Rust is a block-scoped language

pub fn run() {

    let name = "Renato";

    let mut age = 28; // This variable is mutable
    println!("My name is {} and I am {}", name, age);

    age = 29;
    println!("My name is {} and I turned {}", name, age);

    // Define constant

    const ID: i32 = 001;

    println!("ID: {}", ID);

    // Assign multiple variables

    let ( my_name, my_age) = ("Renato", 28);

    println!("{} is {}", my_name, my_age);


}