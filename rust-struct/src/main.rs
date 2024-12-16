fn main() {
    println!("Hello, world!");

    let mut user1 = User {
        active: true,
        userName: String::from("someusername"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    let email = user1.email;
    println!("1 email is {email}");

    user1.email = String::from("another@example.com");
    let email = user1.email;
    println!("2 email is {email}");

    let user2 = User {
        active: user1.active,
        userName: user1.userName,
        email: String::from("test@t.com"),
        sign_in_count: user1.sign_in_count,
    };

    let user3 = User {
        email: String::from("xx@test.com"),
        ..user2
    };

}

fn build_user(email: String, userName: String) -> User {
    User {
        active: true,
        email,
        userName,
        sign_in_count: 1,
    }
}

struct User {
    active: bool,
    userName: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32,i32,i32);
struct Point(i32,i32,i32);

fn test1(){
    let black = Color(0,0,0);
    let origin = Point(0,0,0);
}

struct AlwaysEqual;

fn test2(){
    let subject = AlwaysEqual;
}