/*
    Chapter 4
    
    1. Ownership
    
    https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html

*/


pub fn run() {

    //Since Rust has no GC, we need to free the memory space once the variable is no longer required
    println!("- Since Rust has no GC, we need to free the memory space once the variable is no longer required\n");
    // Simple values with known size at compile time are always stored on stack
    println!("- Simple values with known size at compile time are always stored on stack\n");
    println!("For the piece of code: \n\nlet x = 5;\nlet mut y =x;\ny = 6;\n");
    let x = 5;
    let mut y =x;
    y = 6;
    println!("x is {} // X remains the same", x); // X remains the same
    println!("y is {} // Y is changed to 6 since it's a copy of a simple value with a known, fixed size", y); // Y is changed to 6 since it's a copy of a simple value with a known, fixed size

    // Ways Variables and Data Interact

    // 1. ---------- Move ---------- Similar to shallow copy, but invalidates the variable from which the new one is assigned
    
    /* A String varibale is made of 3 components (stored in the stack) pointer to value, length and capacity.
    Actual content is stored on the heap */
    let s1 = String::from("hello");
    
    /* When assigning s1 to s2, the value on the stack (pointer, length and capacity) is copied  to s2 meaning we copied the referennce to the value on the heap*/
    let s2 = s1;
    
    /*After s2 =s1, Rust considers s1 as no longer valid to prevent double free errors,
        so println("{}, world!", s1) no longer works, this is Rust preventing you from using the invalidated reference
        This is called a MOVE -> s1 was MOVED to s2
    */

    
    // 2. ---------- Ways Variables and Data Interact: Clone ---------- Similar to deep copy
    
    /* With this approach, Rust creates a brand new stack and heap copy of the variable, no reference, it's and independent but equal in value. s3 keeps valid as a variable, contraty to the previous example */
    let s3 = String::from("hello");
    let mut s4 = s3.clone();

    s4.push('!');
    println!("s3 string is {}, and it's clone is {}", s3, s4);



    // ---------------------- Ownership and Functions -------------------------


    let s5 = String::from("taking my ownership");

    take_ownership(s5); // s's value moves into the function and so is no longer valid here
    // println!("{}",s5) will no longer work

    let x1 = 5;

    makes_copy(x1); // x would move into function, but i32 is Copy, so it's okay to still use x afterward

    println!("Usable var x1 is {}", x1);


    // ------------------------ Return Values and Scope --------------------------
    /*Returning values can also transfer ownership. */
    
    let sa = give_ownership(); // give_ownership moves its return value into sa

    println!("{}", sa);

    let sb = String::from("hello");

    let sc = take_and_give_back(sb); // sb is moved into take_and_give_back, which moves its return value into sc

    println!("Final string: {}", sc);


} // Here, sc goes out of scope and is dropped. sb was moved, so nothing
  // happens. sa goes out of scope and is dropped.



fn take_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn give_ownership() -> String {
    let some_string = String::from("Returned by give_ownership");

    some_string
}

fn take_and_give_back(owned_string: String) -> String {
    owned_string
}

