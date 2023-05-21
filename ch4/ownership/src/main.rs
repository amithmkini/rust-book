fn main() {
    let mut s = String::from("hello");
    append_world(&mut s);
    let len = calculate_length(&mut s);
    println!("{s}, length: {len}");
    
    let word = first_word(&s[..]);
    println!("first word: {word}");
}

fn calculate_length(s: &mut String) -> usize {
    let length = s.len();
    length
}

fn append_world(s: &mut String) {
    s.push_str(" world");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
