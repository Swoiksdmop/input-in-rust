use std::io;

fn main() {
    let mut x = String::new();

    println!("Say something");

    io::stdin().read_line(&mut x).expect("Failed to read line");

    println!("You said: {x}");

    println!("Works!");
}
