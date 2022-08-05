/*
    Chapter 4

    3. The slice type
    
    https://doc.rust-lang.org/book/ch04-03-slices.html
*/



pub fn run() {

    let s = String::from("hello my name is renato");

    let l = first_word(&s);

    println!("First word of string {} has length {}", s, l);

    // Second approach with slices

    let s1 = String::from("hello world");

    let hello = &s1[0..5];
    let world = &s1[6..11];

    let s2 = String::from("hello");

    let slice = &s2[0..2];
    let equivalet_slice = &s2[..2];

    let are_equivalent = slice == equivalet_slice;

    println!("Are slices equivalent? {}", are_equivalent);

    let end_slice = &s2[3..s2.len()];
    let equivalent_end_slice = &s2[3..];

    println!("Are end slices equivalent? {}", end_slice == equivalent_end_slice);

    let entire_slice = &s2[..];

    println!("s2 equivalent to entire_slice? {}", entire_slice == s2);


    let l1 = first_word_with_slices(&s);

    println!("First word of string '{}' (processed in slices) is '{}'", s, l1);

    // NOTE: string literals are slices!!!!

    let my_string_literal = "hello world";

    let word = first_word_with_slices(&my_string_literal[0..6]);
    println!("Word: {}", word);
    let word = first_word_with_slices(&my_string_literal[..]);
    println!("Word: {}", word);
    let word = first_word_with_slices(my_string_literal);
    println!("Word: {}", word);


    // Other slices

    let a = [1,2,3,4,5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2,3]);
    


}

// Function to iterate through a string and find the index of the end of the first word
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() { // Iterator through array of bytes
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_with_slices(s: &str) -> &str { // Better use &str argument type since it works for both String references and string slices
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}