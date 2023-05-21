use std::fmt;
use std::println;
use std::slice;

#[allow(unused_variables)]

static mut COUNTER: u32 = 0;

extern "C" {
    fn abs(input: i32) -> i32;
}

fn add_to_counter(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {}

type Thunk = Box<dyn Fn() + Send + 'static>;

fn add_once(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is {}", *r1);
        println!("r2 is {}", *r2);
        *r2 += 1;
        println!("r2 is {}", *r2);
    }

    // This will cause SegFault
    // let address = 0x01234usize;
    // let r = address as *mut i32;

    // let values: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };
    // for value in values {
    // println!("{}", value);
    // }
    // let address = 0x012345usize;
    // let r = address as *const i32;

    unsafe {
        println!("Absolute value of -3 is {}", abs(-3));
    }

    add_to_counter(30);
    unsafe {
        println!("Counter value: {}", COUNTER);
    }

    let point = Point { x: 4, y: 5 };
    point.outline_print();

    #[allow(unused_variables)]
    let f: Thunk = Box::new(|| println!("Hello"));

    let answer = do_twice(add_once, 5);
    println!("The answer is: {}", answer);
}

#[allow(unused_variables)]
#[allow(dead_code)]
fn takes_long_type(f: Thunk) {}

#[allow(dead_code)]
fn returns_long_type() -> Thunk {
    Box::new(|| println!("Test"))
}
