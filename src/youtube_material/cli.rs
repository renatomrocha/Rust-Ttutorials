use std::vec;

use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    println!("Args 2 is: {:?}", args[2]);
    let name = args[2].clone();
    let status = "100%";

    if command == "hello" {
        println!("Hi {}, how are you?", name);
    } else if command == "status" {
        println!("Status is {}", status);
    } else {
        println!("Not a valid command");
    }

    // println!("Command: {:?}", command);

}