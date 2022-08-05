pub struct Lesson {
    pub title : String,
    pub script: fn()
}

impl Lesson {
    fn start_lesson(&self) {
        println!("--------------------------- {} ------------------------", self.title);
    }

    pub fn run_lesson(&self) {
        self.start_lesson();
        (self.script)();
        self.end_lesson();
    }

    fn end_lesson(&self) {
        println!("----- End of {}...", self.title);
    }

}
