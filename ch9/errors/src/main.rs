use std::fs::File;
use std::io::{self, Read, read_to_string};
use std::net::IpAddr;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hellonew.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn home() {
    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Yes");
}

fn main() {
    match read_username_from_file() {
        Ok(string) => read_username_from_file(),
        Err(err) => panic!("Error!")
    };

    home();
}
