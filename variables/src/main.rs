fn main() {
    println!("Hello, world!");

    // 可变变量
    let mut x = 5;
    println!("x is {x}");

    x = 6;
    println!("x is {x}");

    // 常量
    const THREE_HOURS_IN_SECONDS: u32 = 3 * 60 * 60;
    println!("THREE_HOURS_IN_SECONDS is {THREE_HOURS_IN_SECONDS}");

    // shadow
    let y = 7;
    let y = 10;
    {
        let y = 20;
        println!("line 1 y is {y}");
    }
    println!("line 2 y is {y}");

    // 元组与解构，数据访问
    let tup: (i32, f64, u8) = (-500, 64.0, 1);
    let (a, b ,c) = tup;
    println!("a: {a}, b: {b}, c: {c}");

    let first = tup.0;
    let second = tup.1;
    let third = tup.2;
    println!("first is {first}, second is {second}, third is {third}");

    // 数组：类型固定，长度固定
    let ary = [1,2,3,4,5];
    let ary: [i32; 3] = [1,2,3];
    let ary = [2; 5]; // 等同于 [2,2,2,2,2]
    let ary1 = ary[1];
    println!("ary1 is {ary1}");
    // let ary2 = ary[10]; // 访问超界限

    another_fu(4);
}

fn another_fu(x: i32){
    println!("another_fu! {x}");

    let y = {
        let x = 3;
        x + 1
    };
    print!("y is {y} \n");

    let res = five();
    print!("res is {res}");
}

fn five() -> i32{
    5  // 不加分号表示为表达式；加分号是语句，将会报错： expect i32
}
