use std::println;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    let mut list = vec![1,2,3];
    println!("Before closure: {:?}", list);
    
    let mut borrow_mutably = || list.push(7);
    // println!("After closure: {:?}", list);
    borrow_mutably();
    borrow_mutably();
    println!("After closure: {:?}", list);

    let list_iter = list.iter();
    let total: i32 = list_iter.sum();
    println!("The total: {total}");

    let another_list: Vec<i32> = list.iter().map(|x| x + 1).collect();
    println!("Another list: {:?}", another_list);

    let odd_numbers: Vec<i32> = list.into_iter().filter(|x| x % 2 == 1).collect();
    println!("Odd numbers from the list: {:?}", odd_numbers);
}
