/*
    Chapter 6
    
    1. Enums and Pattern Matching
    
    https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html

*/



// Enums allow for us to bind data to each variant 
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String)
}

struct Car {}

struct Bike {}

struct Plane {}

enum Vehicle {
    Car(Car),
    Bike(Bike),
    Plane(Plane)
}


pub fn run() {

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));


}