use std::io::prelude::*;
use std::fs::File;

pub struct BinBuf {
    pub buf : Vec<u8>,
    pub cur : usize,
    filename : String,
}

impl BinBuf {
    // TODO: check it is elf (magic)
    pub fn new(filename : &str) -> BinBuf {
        
        let filename = String::from(filename);
        let mut file = match File::open(filename.as_str()) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Error opening file: {}", e);
                std::process::exit(1);
            }
        };
        
        let mut buf = Vec::new();
        if let Err(e) = file.read_to_end(&mut buf) {
            eprintln!("Error reading file: {}", e);
            std::process::exit(1);
        }

        BinBuf { 
            buf,
            cur : 0,
            filename
        }
    } 
    pub fn idx_to_string(&self, idx : usize) -> String {
        // find a Null-terminated string at specific index
        let mut s = String::new();
        
        for c in &self.buf[idx..] {
            if *c == 0 {
                break;
            }
            s.push(*c as char);
        }
        s
    }
}