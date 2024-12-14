fn main() {
    let s1 = String::from("hello");

    let len = cal_len(&s1);

    println!("The length of '{s1}' is {len}.");

    change(&s1);

    let mut s2 = String::from("s2, hello");
    change2(&mut s2);
    println!("s2 is {s2}");

    fn1();
}

fn cal_len(s: &String) -> usize{ // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of 
  // what it refers to, it is not dropped.

fn change(some_string: &String){
    // some_string.push_str(", world."); // Does not work.
}

fn change2(some_string: &mut String){
    some_string.push_str(", world!");
}

fn fn1(){
    let refrence_to_nothing = dangle();
}

fn dangle() -> &String{ // dangle returns a reference to a String
    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!