// Import the random crate
use rand;

// enum IpAddrKind {
//     V4,
//     V6,
// }
// 
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 }, // anonymous struct
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            // Randomly print if the penny is lucky
            if rand::random() {
                println!("Lucky penny!");
            }
            1 // return value
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn is_it_a_penny(coin: Option<Coin>) {
    if let Some(Coin::Penny) = coin {
        println!("Lucky penny!");
    } else {
        println!("Not a penny");
    }
}

fn main() {
    let some_number = Some(5);
    println!("some_number: {:?}", some_number);
    let some_string = Some("a string");
    println!("some_string: {:?}", some_string);
    let absent_number: Option<i32> = None;
    if absent_number.is_none() {
        println!("absent_number is none");
    } else {
        println!("absent_number is not none");
    }
    println!("Value of a penny: {}", value_in_cents(Coin::Penny));
    is_it_a_penny(Some(Coin::Nickel));
    is_it_a_penny(Some(Coin::Quarter));
    is_it_a_penny(Some(Coin::Dime));
}
