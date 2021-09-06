use std::io;


fn main() {
    let mut input = String::new();

    println!("What is your name?");

    io::stdin().read_line(&mut input).ok().expect("Couldn't read line");

    println!("Hello {}", input);
}
