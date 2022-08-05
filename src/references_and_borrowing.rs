/*
    Chapter 4
    
    2. References and Borrowing
    
    https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html

*/



pub fn run() {

    /* Instead of working around with giving and taking back ownership,
     one can use references to keep using a value after this being processed by some function.
     A reference is like a pointer in that it's an address we can follow to access the data stored at that address.
     Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the lif of that reference */

     let s1 = String::from("hello");

     let len = calculate_length(&s1); // References allow you to refer to some value without taking ownership of it.

     println!("The length of '{}' is {}.", s1, len);


     // Just as variable, references are immutable by default 
     let mut s2 = String::from("Lender: I can borrow you this string!\n"); // Creating mutable variable

    let muts2 = &mut s2; // Creating mutable reference to that variable

     println!("After string was borrowed...");

     change_borrowed(muts2); // Passing a MUTABLE REFERENCE

     println!("{}",s2);

     /*NOTE: Only one mutable reference to a value is allowed at a time 
        The benefit of having this restriction is that Rust can prevent data races at compiler time.
        A data race is similar to a race condition and happens when these three behaviors occur:
            - Two or more pointers access the same data at the same time
            - At least one of the pointers is being used to write to the data
            - There's no mechanism being used to synchronize the access to the data
     */

     // We can use curly braces to create a new scopr, allowing for multiple references, just not simultaneous ones

     let mut s3 = String::from("Mutable string changing in");

     {
        let scope = String::from(" -Isolated scope");
        let r1 = &mut s3;
        change(r1, &scope);
     }

     let r2 = &mut s3;
     let scope = String::from(" -Outter scope");
     change(r2, &scope);

     println!("{}",s3);




    /*`No mutable refs are allowed while there's an immutable ref in use (multiple immutable refs are allowed tho) */
    /*
        let mut s = String::from("hello");
        let r1 = &s; // no problem
        let r2 = &s; // no problem
        let r3 = &mut s; // BIG PROBLEM
        println!("{}, {}, and {}", r1, r2, r3);
    */
    //This will work tho
    let mut s = String::from("hello");
    let r1 = &s; 
    let r2 = &s; 
    println!("{} and {}", r1, r2);
    // Scope for immutable references ends here, they are no longer used after this println
    let r3 = &mut s; 
    println!("{}", r3);


  




}

fn change_borrowed(some_string: &mut String) {
    some_string.push_str("Borrower: Thanks for borrowing this string to me!");
}

fn change(some_string: &mut String, scope: &String) {
    some_string.push_str(scope);
}


fn calculate_length(s: &String) -> usize {
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped


// Dangling references
// fn dangle() -> &String { // dangle returns a reference to a String

//     let s = String::from("hello"); // s is a new String

//     &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!


fn no_dangle() -> String {
    let s = String::from("hello");

    s
} // This works, because the ownership of s is moved out