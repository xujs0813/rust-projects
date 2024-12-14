fn main() {
    let s = "hello";

    s_string();

    fn1();

    fn_clone();

    fn2();

    fn3();

}

fn s_char(){
                     // s is not valid here, it's not yet dclared
    let s = "hello"; // s is valid from this point forward

    // do something
}                    // this scope is now over, and s is no longer valid

fn s_string(){
    let s = String::from("hello");

    let mut ss = String::from("hello");
    ss.push_str(", world!");
    println!("ss is {ss}")
}

fn fn1(){
    let s1 = String::from("hello");
    let s2 = s1;   // Move

    // println!("s1 is {s1}"); // borrow of moved value: s1
}

fn fn_clone(){
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {s1}, s2 = {s2}");
}

fn fn2(){
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into this function...
    // println!("s is {s}"); // s's no longer valid here.

    let x = 4;          // x comes into scope
    makes_copy(x);      // x would move into the function
    println!("x is {x}"); // but i32 is Copy. So it's okay to still use x afterward

} // here, x goes out of scope, then s.
  // but s's value was Moved, so nothing special happens.

fn takes_ownership(some_string: String){ // some_string comes into scope
    println!("{some_string}");
} // here, some_string goes out of scope and Drop is called.
  // The backing memory is freed.

fn makes_copy(some_integer: i32){ // some_integer comes into scope
    println!("{some_integer}");
} // here, some_integer goes out of scope. Nothing special happens.



fn fn3() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}