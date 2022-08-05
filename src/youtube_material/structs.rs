// Structs - Used to create custom data types

// Traditional struct
struct Color {
    red: u8,
    green: u8,
    blue: u8
}

// Tuple struct
struct TupleColor(u8,u8,u8);


struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    // Construct person
    fn new(first: &str, last: &str) -> Person {
       return Person { first_name: first.to_string(), last_name: last.to_string() };
    }

    // Get full name
    fn full_name(&self) -> String {
        return format!("{} {}", self.first_name, self.last_name);
    }


    // Set last name
    fn change_last_name(&mut self, new_last_name: &str) {
        self.last_name = new_last_name.to_string();
    }

    // Name to tuple
    fn to_tuple(self) -> (String, String) {
        // return format!("{:?}", (self.first_name, self. last_name))
        return (self.first_name, self. last_name);
    }

}


pub fn run() {

    let mut c = Color { red: 255, green: 0, blue: 0};

    println!("Color is red: {}, green: {}, blue: {}", c.red, c.green, c.blue);

    c.green = 100;

    println!("Updated color is red: {}, green: {}, blue: {}", c.red, c.green, c.blue);


    let mut tc: TupleColor = TupleColor(255,0,0);

    tc.1 = 100;

    println!("Tupple Color: {} {} {}", tc.0, tc.1, tc.2);


    let mut p = Person::new("John", "Doe");

    println!("Person {} {}", p.first_name, p.last_name);

    p.change_last_name("Dooowe");
    println!("Person {}", p.full_name());

    println!("To tuple {:?}", p.to_tuple());

}