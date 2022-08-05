// Arrays - Fixed list where elements are the same data types

use std::mem;

pub fn run() {

    let mut numbers : [i32; 5] = [1,2,3,4,5];

    // Re-assign value
    numbers[2] = 1000000000;

    println!("Whole array is: {:?}", numbers);

    println!(" Index value from array: {}", numbers[0]);

    // Get array length
    println!("Array length {}", numbers.len());

    // Arrays are stack allocated
    println!("Memory used by array in bytes: {}", mem::size_of_val(&numbers));

    // Get Slice
    let slice: &[i32] = &numbers[1..3];
    println!("Slice {:?}", slice);

}