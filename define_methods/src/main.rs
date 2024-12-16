fn main() {
    println!("Hello, world!");

    let rect1 = Rect {
        width: 30,
        height: 50,
    };
    println!("The area is {}.", rect1.area());

    let rect2 = Rect {
        width: 20,
        height: 40,
    };
    let rect3 = Rect {
        width: 50,
        height: 80,
    }; 

    println!("rect1 can hold rect2: {}.", rect1.can_hold(&rect2));
    println!("rect1 can hold rect3: {}.", rect1.can_hold(&rect3));

    let rect4 = Rect::square(10);
    println!("rect1 can hold rect3: {rect4:#?}.");
}

#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rect: &Rect) -> bool {
        self.width >= rect.width && self.height >= rect.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}