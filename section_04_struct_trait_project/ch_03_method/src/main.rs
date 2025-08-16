#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    // Method ที่ใช้ &self (immutable reference)
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    // Method ที่ใช้ &mut self (mutable reference)
    fn double_size(&mut self) {
        self.width *= 2;
        self.height *= 2;
    }
    
    // Associated function (ไม่ใช้ self)
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50
    };

    let rect2 = Rectangle {
        width: 20,
        height: 40
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
}

fn main_demo() {
    // ต้องเป็น mut เพื่อให้สามารถเรียก double_size() ได้
    let mut rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("Area of rect1: {}", rect1.area());

    rect1.double_size();
    println!("After doubled size: {}", rect1.area());

    let sq = Rectangle::square(25);
    println!("Create a new square: {:?}", sq);
}
