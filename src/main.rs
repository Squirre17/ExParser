mod parser;
mod binbuf;

use parser::Parser;
use std::env;
use clap::{arg, command, value_parser, ArgAction, Command, Arg};

fn main() {
    let matches = Command::new("ExParser")
        .version("1.0")
        .author("Squ17. <ler2sq@gmail.com>")
        .about("Executable Parser")
        // .arg(arg!(-b --bin <VALUE> "executable path").required(true))
        // // .arg(arg!(-o --out <VALUE> "modified file writeback to ram").required(false))
        // .arg(arg!(-h --file-header "Display the ELF file header").required(false)).t
        // .arg(arg!(-l --program-headers "Display the program headers").required(false))
        // .arg(arg!(-S --section-headers "Display the sections' header").required(false))
        .arg(Arg::new("bin")
            .short('b')
            .long("bin")
            .value_name("path")
            .help("executable path")
            .required(true))
        .arg(Arg::new("elf-header")
            .short('e')
            .long("file-header")
            .action(ArgAction::SetTrue)
            .required(false)
            .help("Display the ELF file header"))
        .arg(Arg::new("program-headers")
            .short('p')
            .long("program-headers")
            .action(ArgAction::SetTrue)
            .required(false)
            .help("Display the program headers"))
        .arg(Arg::new("section-headers")
            .short('s')
            .long("section-headers")
            .action(ArgAction::SetTrue)
            .required(false)
            .help("Display the sections' header"))
        .arg(Arg::new("section-layout")
            .short('l')
            .long("section-layout")
            .action(ArgAction::SetTrue)
            .required(false)
            .help("Display the section header's layout"))
        .get_matches();


    let path = matches.get_one::<String>("bin").expect("required");
    
    let parser = Parser::new(&path);
    
    // if let Some(_) = matches. {

    // }
    if matches.get_flag("elf-header") {
        parser.show_header();
    }
    if matches.get_flag("program-headers") {
        parser.show_segments();
    }
    if matches.get_flag("section-headers") {
        parser.show_sections();
    }
    if matches.get_flag("section-layout") {
        parser.show_layout();
    }
    // parser.show_segments().show_sections().show_layout();

    // if let Some(out) = matches.get_one::<String>("out"){
    //     parser.writeback(out);
    // };

    // if let Some(fh) = matches.

    // println!("Hello, world!");
}
