// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data


pub fn run() {

    // String literal
    let hello = "Hello";

    // Dynamic string
    let mut dynamic_string = String::from("Hello");

    dynamic_string.push('!');

    println!("Dynamic string is {}", dynamic_string);

    dynamic_string.push_str(" My name is Renato");

    println!("Updated string {}", dynamic_string);


    // Get length
    println!("Length {}", dynamic_string.len());


    // Capacity in bytes
    println!("Capacity: {}", dynamic_string.capacity());

    // Is empty
    println!("Is empty? {}", dynamic_string.is_empty());

    // Contains
    println!("Contains 'Renato' ? {}", dynamic_string.contains("Renato"));

    // Replace
    println!("Replace: {}", dynamic_string.replace("Renato", "Tony"));

    // Loop through string by whitespace
    for word in dynamic_string.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);

    s.push('A');
    s.push('B');

    println!("{}", s);

    // Assertion testing
    assert_eq!(2,s.len());
    assert_eq!(10,s.capacity());



}