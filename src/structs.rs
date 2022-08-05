/*
    Chapter 5

    1. Defining and instantiating structs
    
    https://doc.rust-lang.org/book/ch05-01-defining-structs.html
*/


struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}


// Tuple structs

struct Color(i32, i32, i32);

struct Point(i32, i32, i32);

// Unit like structs

struct AlwaysEqual; // Used to implement traits on some type, but there's no data associated with it's type


fn build_user(email:String, username: String) -> User {
    User { active: true, username, email, sign_in_count: 0 }
}

fn move_point(point: &mut Point, dx:i32, dy: i32, dz: i32) {

    point.0 += dx;
    point.1 +=dy;
    point.2 += dz;
}


pub fn run() {
    let mut user1 = build_user(String::from("useremail@email.pt"), String::from("user1"));
    user1.email = String::from("updatedemail@example.com");

    // Instantiating new instances with struct update syntax

    let user2 = User{
        email: String::from("user2.email.com"),
        ..user1 // Variable user1.username is moved to user2, user2 is now unaccessible
    };

    let user3 = User {
        email: String::from("user3.email.com"),
        username: String::from("user3"),
        ..user2 // In this case user2 will be available, because the only copied variables are of simple type, therefore they are copied, not moved
    };

    println!("User 3 is {}, logged {} times and is active={}", user3.username, user3.sign_in_count, user3.active);
    println!("User 2 still exists: {}", user2.username);
    // assert_eq!(user1.username, user2.username); This won't work because user1 is not accessible



    let black = Color(0,0,0);
    let mut point = Point(0,0,0);

    println!("Initial point coords -> (x:{}, y:{}, z:{})", point.0,point.1,point.2);

    move_point(&mut point, 5, 10, -2);

    println!("New point coords -> (x:{}, y:{}, z:{})", point.0,point.1,point.2);


    let subject = AlwaysEqual;


}



#[derive(Debug)] // This attribute allows for macro println! to know the default format of the struct when printed
struct Rectangle {
    width : u32,
    height: u32
}

pub fn rectangle_prog() {

    let rect = Rectangle {
        width: 10,
        height: 20
    };

    println!("Rect is {:#?}", rect);
    // Another way to debug a value
    dbg!(&rect);

    println!("Rectangle area is: {}", rectangle_area(&rect));

}

fn rectangle_area(rectangle: &Rectangle) -> u32  {

    rectangle.width * rectangle.height
}
