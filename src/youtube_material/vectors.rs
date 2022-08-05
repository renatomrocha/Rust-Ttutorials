//Vectors are resizeable arrays
use std::mem;

pub fn run() {

    let mut numbers : Vec<i32> = vec![1,2,3,4,5];

    // Re-assign value
    numbers[2] = 1000000000;


    //Pushing to vector
    numbers.push(10);
    println!("Whole vector is: {:?}", numbers);

    numbers.pop();

    // Popping last value
    println!("After pop {:?}", numbers);

    println!(" Index value from vector: {}", numbers[0]);

    // Get array length
    println!("Vector length {}", numbers.len());

    // Arrays are stack allocated
    println!("Memory used by vector in bytes: {}", mem::size_of_val(&numbers));

    // Get Slice
    let slice: &[i32] = &numbers[1..3];
    println!("Slice {:?}", slice);


    // Iterating vector
    for x in numbers.iter() {
        println!(" Number: {}", x);
    }

    // Loop and mutate values

    for x in numbers.iter_mut() {
        *x *=2;
    }

    println!("After loop mutation: {:?}", numbers);

}