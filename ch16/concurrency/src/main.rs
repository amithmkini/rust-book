use std::{thread, println};
use std::time::Duration;
use std::sync::{Arc, mpsc, Mutex};
use std::rc::Rc;

fn concurrency() {
    // let counter = Rc::new(Mutex::new(0));
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        // let counter = Rc::clone(&counter);
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

#[derive(Debug)]
struct Test {
    a: String,
    b: String,
}

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5 {
        println!("Hi number {i} from the main thread");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();

    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        // thread::sleep(Duration::from_millis(10000));
        let val = String::from("hi");
        tx.send(val).unwrap();
        // println!("Val is {val}");
    });

    // This waits for the thread to execute, so possibly infinite wait?
    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        let vals = vec![
            String::from("Hi"),
            String::from("Hello")
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    for received in rx {
        println!("Received: {}", received);
    }

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let test_var = Test { 
            a: String::from("a"),
            b: String::from("b"),
        };
        tx.send(test_var).unwrap();
        thread::sleep(Duration::from_millis(1));
    });
    
    let received = rx.recv().unwrap();
    println!("Received the number {:?}", received);

    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("m = {:?}", m);

    concurrency();
}
