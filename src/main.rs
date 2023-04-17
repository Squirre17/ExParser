mod parser;
mod binbuf;

use parser::Parser;
use std::env;

fn main() {
    let args : Vec<String> = env::args().collect();
    let p = Parser::new(args.iter().nth(1).unwrap().as_str());
    p.show_segments().show_sections();
    // println!("shstrtab at offset of {:x}", )

    println!("Hello, world!");
}
