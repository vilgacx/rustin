use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    print!("> ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Error");
    println!("{}",input.trim());
}
