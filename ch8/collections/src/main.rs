#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn vectors() {
    let mut v = Vec::new();
    // let v = vec![1, 2, 3];
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    println!("v = {:?}", v);

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    // let fifth: Option<&i32> = v.get(4);
    match v.get(4) {
        Some(fifth) => println!("The fifth element is {}", fifth),
        None => println!("There is no fifth element."),
    }

    for i in &v {
        println!("{i}");
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("{i}");
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue")),
    ];

    println!("row = {:?}", row);
}


fn strings() {
    let kannada = String::from("ಗಡವ");
    println!("{kannada}");

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{s}");

    s.push('!');
    println!("{s}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 has been moved here and can no longer be used
    println!("{s3}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");
    println!("{s}");

    
    // for c in "नमस्ते".bytes() {
    for c in "Зд".chars() {
        println!("{c}");
    }
}

fn hashmaps() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    scores.entry(String::from("Yellow")).or_insert(100);
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    scores.insert(String::from("Red"), 100);

    let mut map: HashMap<String, i32> = HashMap::new();
    map.insert(String::from("Blue"), 10);
    map.insert(String::from("Yellow"), 50);
    
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let map: HashMap<_, _> = 
        teams.into_iter().zip(initial_scores.into_iter()).collect();

    for (key, value) in &map {
        println!("{key}: {value}");
    }
}

fn main() {
    vectors();
    strings();
    hashmaps();
}
