use std::io::prelude::*;
use std::fs::File;
use std::process::id;

pub struct BinBuf {
    pub buf  : Vec<u8>,
    pub size : u32,
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
            size : buf.len() as u32,
            buf,
            filename
        }
    } 
    pub fn idx_to_string(&self, idx : usize) -> String {
        // find a Null-terminated string at specific index in binbuf
        let mut s = String::new();
        
        for c in &self.buf[idx..] {
            if *c == 0 {
                break;
            }
            s.push(*c as char);
        }
        s
    }
    pub fn get_content(&self, idx : usize, sz : usize) -> Vec<u8> {
        self.buf[idx..idx+sz].to_vec()
    }
}