
// Conditionals - Used to check 

fn serve_drink(age: &i32, check_id: bool) {


    if !check_id {
        println!("I'm gonna need to check your id");
        if *age > 21 && check_id {
            println!("Bartender says: What would you like to drink?");
        }  else {
            println!("You're not allowed to drink alcohol until you are 21");
        }

    }
    

    // Shorthand if

    let is_of_age = if age >= &21 {true} else {false};

    println!("Is person of age? {}", is_of_age);

}


pub fn run() {

    let mut age: i32 = 28;
    let check_id: bool = false;

    serve_drink(&age, check_id);

    println!("After drink served has id been checked? {}", check_id);

}