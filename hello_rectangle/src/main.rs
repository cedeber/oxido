// An "interface" to implement
trait Geometry {
    fn area(&self) -> u32;
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Common way to do a "constructor"
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

impl Geometry for Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle::new(20, 20);

    println!(
        "The area of the rectangle 1 is {} square pixels.",
        rect1.area()
    );

    println!(
        "The area of the rectangle 2 is {} square pixels.",
        rect2.area()
    );
}
