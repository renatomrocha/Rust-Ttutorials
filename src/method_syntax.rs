/*
    Chapter 5

    3. Method syntax
    
    https://doc.rust-lang.org/book/ch05-03-method-syntax.html
*/


struct Rectangle {
    width: u32,
    height: u32
}

// Method implementation bonded to Rectangle struct
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn extend(&mut self, dw: u32, dh: u32) {
        self.width += dw;
        self.height += dh;
    }

    // Example getters
    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }

    fn dimensions(&self) -> (u32, u32) {
        (self.width(), self.height())

    }

    fn can_hold(&self, rectangle: &Rectangle) -> bool {
        (rectangle.width() < self.width() && rectangle.height() < self.height()) || (rectangle.height() < self.width() && rectangle.width() < self.height())
    }

    // Associated functions -> Functions that don't require an instance of the type
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }

}




pub fn run() {
    let mut rect = Rectangle {
        width: 30,
        height: 40
    };

    println!("The area of the rectangle is {} square pixels", rect.area());

    rect.extend(10, 5);

    println!("After extension, rectangle has width {}, heigh {} and area {}", rect.width(), rect.height(), rect.area());

    println!("Rectangle dimensions: {:?}", rect.dimensions());

    let rect2: Rectangle = Rectangle { width: 45, height: 20 };

    println!("Does rect2 fits inside rect? {}", rect.can_hold(&rect2));
    println!("Does rect1 fits inside rect2? {}", rect2.can_hold(&rect));

    // Calling an associated function 
    let square = Rectangle::square(10);

    println!("Square has dimensions: {:?}", square.dimensions());
}