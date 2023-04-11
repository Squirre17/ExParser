mod parser;
mod binbuf;

use parser::Parser;
use std::env;

fn main() {
    let args : Vec<String> = env::args().collect();
    let mut p = Parser::new(args.iter().nth(1).unwrap().as_str());
    p.show_phdrs();
    p.show_shdrs();
    p.parse_dynsymtab();
    println!("Hello, world!");
}
