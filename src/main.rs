/* Tutorials followed from the Rust official documentation:
    https://doc.rust-lang.org/book
*/



mod ownership;
mod references_and_borrowing;
mod slices;
mod structs;
mod method_syntax;
mod lesson_utils;
mod enums;

//:: allows us to namespace the particular from function
fn main() {

    
    let lesson_1 = lesson_utils::Lesson {
        title: String::from("Ownership Lesson"),
        script: ownership::run
    };

    let lesson_2 = lesson_utils::Lesson {
        title: String::from("References and Borrowing"),
        script: references_and_borrowing::run
    };

    let lesson_3 = lesson_utils::Lesson {
        title: String::from("Slices"),
        script: slices::run
    };


    let lesson_4 = lesson_utils::Lesson {
        title: String::from("Structs"),
        script: structs::run
    };


    let lesson_5 = lesson_utils::Lesson {
        title: String::from("Method syntax"),
        script: method_syntax::run
    };

    let lesson_6 = lesson_utils::Lesson {
        title: String::from("Enums and Pattern matching"),
        script: enums::run
    };


    let lessons : Vec<lesson_utils::Lesson> = vec![lesson_1, lesson_2, lesson_3, lesson_4, lesson_5]; 


    for lesson in lessons {
        lesson.run_lesson();
    }



}


