
// Loops - Used to iterate until a conditional is met

pub fn run() {

    let mut count = 0;

    // Infinite loop
    loop {
        count +=1;
        println!("Current number: {}", count);
        if count > 10 {
            println!("Reached limit");
            break;
        }
    }


    // While Loop (FizBuzz)
    // count = 0;
    // while count <= 100 {

    //     if count % 15 == 0 {
    //         println!("FizzBuzz");
    //     } else if count % 3 == 0 {
    //         println!("Fizz");
    //     } else if count % 5 == 0 {
    //         println!("Buzz");
    //     } else {
    //         println!("{}", count);
    //     }
    //     count +=1;
    // }


    // For range (Fizzbuzz)
    for x in 0..100 {
        if x % 15 == 0 {
            println!("FizzBuzz");
        } else if x % 3 == 0 {
            println!("Fizz");
        } else if x % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", x);
        }
    }


}