#[derive(Debug)] // derive trait
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32); // tuple struct
struct Point(i32, i32, i32); // tuple struct

#[derive(Debug)] // derive trait
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 { // method
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool { // method
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle { // associated function
        Rectangle { width: size, height: size }
    }
}

#[derive(Debug)] // derive trait
struct UnitStruct; // unit struct

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // field init shorthand
        email,
        sign_in_count: 1,
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    println!("rect1: {:?}", rect1);
    println!("rect1 area: {}", rect1.area());
    let user1 = User {
        active: true,
        username: String::from("user1"),
        email: String::from("user@example.com"),
        sign_in_count: 1,
    };

    println!("User1: {:?}", user1);
    dbg!(&user1);
    let user2 = build_user(
        String::from("test@example.com"),
        String::from("user2"),
    );

    let user3 = User {
        username: String::from("user3"),
        email: String::from("kini@example.com"),
        ..user1 // struct update syntax
    };

    
    println!("user2: {}, email: {}, active: {}, sign_in_count: {}",
        user2.username, user2.email, user2.active, user2.sign_in_count);

    println!("user3: {}, email: {}, active: {}, sign_in_count: {}",
        user3.username, user3.email, user3.active, user3.sign_in_count);

    let red = Color(255, 0, 0);
    let origin = Point(0, 0, 0);

    println!("red: {}, {}, {}", red.0, red.1, red.2);
    println!("origin: {}, {}, {}", origin.0, origin.1, origin.2);

    let unit = UnitStruct;
    println!("unit: {:?}", unit);
}
