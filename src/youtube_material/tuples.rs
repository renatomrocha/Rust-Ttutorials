// Tuples group together values pf different types
// Max 12 elements

pub fn run() {

    let example_tuple:(i32,i32,&str, f32, bool) = (1,2,"Shit", 1.3, true);

    println!("{}, {}, {}", example_tuple.0, example_tuple.2, example_tuple.4);


}