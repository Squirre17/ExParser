mod parser;
mod binbuf;

use parser::Parser;
use std::env;
use clap::{arg, command, value_parser, ArgAction, Command};

fn main() {
    let matches = Command::new("MyApp")
        .version("1.0")
        .author("Squ17. <ler2sq@gmail.com>")
        .about("Executable Parser")
        .arg(arg!(-b --bin <VALUE> "executable path").required(true))
        .arg(arg!(-o --out <VALUE> "modified file writeback to ram").required(false))
        .get_matches();


    let path = matches.get_one::<String>("bin").expect("required");
    
    let parser = Parser::new(&path);
    parser.show_segments().show_sections().show_layout();

    if let Some(out) = matches.get_one::<String>("out"){
        parser.writeback(out);
    };

    // println!("Hello, world!");
}
