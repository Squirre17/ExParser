mod parser;
mod binbuf;

use parser::Parser;
use std::env;

fn main() {
    let args : Vec<String> = env::args().collect();
    Parser::new(args.iter().nth(1).unwrap().as_str());
    println!("Hello, world!");
}
