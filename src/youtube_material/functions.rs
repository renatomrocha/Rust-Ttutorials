
pub fn run() {


    greeting("Hello", "Renato");   
    
    let a: i32 = add(3, 8);

    println!("Sum result is {}", a);

    // Closure
    let n3: i32 = 10;
    let add_nums = |n1:i32, n2: i32| n1 + n2 + n3; // Closure -> Accessing outer scope variable inside function
    println!("C Sum: {}", add_nums(3,3));

}



fn greeting(greet: &str, name: &str) {
    println!("{} {}", greet, name);
}


fn add(n1: i32, n2: i32) -> i32 {

    n1 + n2 // Skip semicolon -> returns
}