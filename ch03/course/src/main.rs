fn main() {
    let hugging_face = '🤗';
    println!("{}", hugging_face);
    
    // Named loops 
    'outer: for x in 0..10 {
        'inner: for y in 0..10 {
            if x % 2 == 0 { continue 'outer; } // Continues the loop over x.
            if y % 2 == 0 { continue 'inner; } // Continues the loop over y.
            println!("x: {}, y: {}", x, y);
        }
    }
}
