fn main() {
    println!("Hello, world!");

    let width1 = 30;
    let height1 = 50;
    println!("1 The area of the rect is {} square pixels.", area1(width1, height1));

    // refactoring with Tuples
    let rect1 = (30, 50);
    println!("2 The area of the rect is {} square pixels.", area2(rect1));

    // refactoring with Struct
    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("3 The area of the rect is {} square pixels.", area3(&rect2));

    let scale = 2;
    let pointer = Pointer {
        x: dbg!(10 * scale),
        y: 20,
    };
    // println!("pointer is {pointer:?}");
    println!("pointer is {pointer:#?}");
}

fn area1(width: u32, height: u32) -> u32{
    width * height
}

fn area2(dimensions: (u32,u32)) -> u32 {
    dimensions.0 * dimensions.1
}

struct Rectangle {
    width: u32,
    height: u32,
}

fn area3(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

#[derive(Debug)]
struct Pointer {
    x: u32,
    y: u32,
}