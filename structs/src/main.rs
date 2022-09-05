#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn square(side: i32) -> Self {
        Self {
            width: side,
            height: side,
        }
    }

    fn area(&self) -> i32 {
        self.width * self.height
    }

    fn can_hold(&self, cmp_rect: &Self) -> bool {
        self.width > cmp_rect.width && self.height > cmp_rect.height
    }
}

fn main() {
    let rect: Rectangle = Rectangle {
        width: dbg!(10 * 20),
        height: 20,
    };

    let rect1: Rectangle = Rectangle {
        width: dbg!(10 * 30),
        height: 25,
    };

    let square = dbg!(Rectangle::square(13));

    println!("Rect: {:#?}", rect);
    println!("Area of givven rectangle: {}", area(&rect));
    println!("Area of rect1: {}", rect.area());

    println!("Can rect1 holde rect: {}", rect1.can_hold(&rect));
}

fn area(rect: &Rectangle) -> i32 {
    rect.width * rect.height
}
