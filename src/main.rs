#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other_rect: &Rectangle) -> bool {
        self.width >= other_rect.width && self.height >= other_rect.height
    }
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
         width: dbg!(30 * scale),
         height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    
    let rect2 = Rectangle {
        width: 20,
        height: 30,
    };
    let rect3 = Rectangle {
        width: 80,
        height: 30,
    };
    println!("Can rect1 hold rect2 ? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3 ? {}", rect1.can_hold(&rect3));
}
