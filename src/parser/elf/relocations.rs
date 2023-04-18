use crate::parser::Sections;
use crate::parser::Section;
#[derive(Debug)]
pub struct Relocations {
    relocs : Vec<Relocation>,
}

impl Relocations {
    pub fn new(relocs : Vec<Relocation>) -> Self {

        Relocations {
            relocs
        }
    }
}

#[derive(Debug)]
pub struct Relocation {
    addr : u64,     // The content of GOT //don't considering 32-bit
    offset : usize, // offset of file
}

impl Relocation {
    pub fn new(addr : u64, offset : usize) -> Self {
        Relocation { 
            addr, 
            offset
        }
    }
}

