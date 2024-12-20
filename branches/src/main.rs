fn main() {
    println!("Hello, world!");

    let number = 3;
    if number > 5 {
        println!("condition is true");
    } else if number > 2 {
        println!("condition is 2")
    } else {
        println!("condition is false");
    }

    let condition = true;
    let num = if condition {5} else {6};
    println!("num is {num}");

    loop_fn();

    loop_break();

    while_fn();

    for_fn();
}

fn loop_fn(){
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10{
            break counter * 2;
        }
    };

    println!("result is {result}");
}

fn loop_break(){
    let mut count = 0;

    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");
}

fn while_fn(){
    let mut number = 3;

    while number != 0 {
        println!("{number}");

        number -= 1;
    }

    println!("over!");
}

fn for_fn(){
    let a = [1,2,3,4,5];
    for element in a {
        println!("the value is: {element}");
    }
}