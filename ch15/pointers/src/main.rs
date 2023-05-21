use std::cell::RefCell;
use std::{ops::Deref, println};
use std::rc::{Rc, Weak};

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

enum List2 {
    Cons(i32, Rc<List2>),
    Nil,
}

use std::assert_eq;

use crate::List::{Cons,Nil};

fn hello(name: &str) {
    println!("Hello {name}");
}


struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data {}!", self.data);
    }
}

// This creates circular reference
#[derive(Debug)]
enum List3 {
    Cons(i32, RefCell<Rc<List3>>),
    Nil,
}

impl List3 {
    fn tail(&self) -> Option<&RefCell<Rc<List3>>> {
        match self {
            List3::Cons(_, item) => Some(item),
            List3::Nil => None,
        }
    }
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}



fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    let c = CustomSmartPointer{ data: String::from("my stuff") };
    drop(c);
    let d = CustomSmartPointer{ data: String::from("other stuff") };

    let a = Rc::new(List2::Cons(5, Rc::new(List2::Cons(10, Rc::new(List2::Nil)))));
    let second_list = List2::Cons(10, Rc::clone(&a));
    let third_list  = List2::Cons(11, Rc::clone(&a));

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}
