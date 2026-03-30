use std::io::{self, Read};

use set_lexer::lexer;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    println!("{:?}", lexer(&buffer));
}
