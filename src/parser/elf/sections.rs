use crate::parser::elf::elf_struct::Elf64Shdr;
use colored::Colorize;
use std::{collections::HashMap, vec};

pub struct Sections {
    // manage all sections
    pub secs : Vec<Section>,
    shstrndx : usize,
    secnames : Vec<String>, /* store all section names */
    offset_to_secname : HashMap<usize, String>,
    pos : usize
}

pub struct Section {
    pub shdr : Elf64Shdr,
    pub name : String
}

impl Sections {
    // restrict caller to maintain scope of buf
    pub fn new(shdrs : Vec<Elf64Shdr>, shstrtab_buf : &[u8], shstrndx : usize) -> Self {

        let mut offset_to_secname = HashMap::new();
        // skip first null string(TODO: correct?)
        let mut prev = 0;
        // offset_to_sec_name.insert(0, String::from(""));

        // IMPORTANT : don't use \x00 to split buf cuz some string with overlap
        for shdr in &shdrs {

            prev = shdr.sh_name as usize;
            /*
                00 ff 00 aa bb 00
            1   p        n
            2            p     n
             */
            let next = match shstrtab_buf[prev..].iter().position(|&x| x == b'\x00') {
                Some(pos) => pos + 1,
                None      => break,
            };

            let s = match shstrtab_buf[prev] {
                b'\x00' => String::from("\x00"),
                _       => String::from_utf8(shstrtab_buf[prev..prev+next].to_vec()).unwrap()
                
            };

            offset_to_secname.insert(prev, s);
        }

        let mut v : Vec<(&usize, &String)>;

        v = offset_to_secname.iter().collect();
        v.sort_by(|a, b| a.0.cmp(&b.0));

        let secnames = v.into_iter()
                        .map(|(_, str)| str.clone())
                        .collect();

        let mut secs = vec![];
        
        for shdr in shdrs {
            secs.push(Section{
                name : offset_to_secname[&(shdr.sh_name as usize)].clone(),
                shdr, 
            });
        }
        
        Sections { 
            secs, 
            shstrndx,
            secnames ,
            offset_to_secname,
            pos : 0,
        }
    }
    
    fn get_sec_name(&self, offset : usize) -> String {
        // get section name by sh_name(offset in shstrtab) in shdr
        self.offset_to_secname[&offset].clone()
    }

    pub fn show_shdrs(&self) -> &Self {

        // TODO: clearify sh_addr
        
        print!("{:>4}", "Nr".red());
        print!("{:>22}", "Name".blue());
        print!("{:>19}", "Addr".green());
        print!("{:>19}", "Offset".yellow());
        print!("{:>19}", "Size".cyan());
        println!();

        for (i, sec) in self.secs.iter().enumerate() {
            
            let sec_name = self.get_sec_name(sec.shdr.sh_name as usize);

            let mut fields = Vec::new();
            fields.push(format!("[{:<2}]", i));
            fields.push(format!("{:<022}", sec_name.blue()));
            fields.push(format!("{:<018x}", sec.shdr.sh_addr));
            fields.push(format!("{:<018x}", sec.shdr.sh_offset));
            fields.push(format!("{:<018x}", sec.shdr.sh_size));

            println!("{}", fields.join(" "));

        }
        self
    }
}


impl std::ops::Index<usize> for Sections {

    type Output = Section;

    fn index(&self, index: usize) -> &Self::Output {
        &self.secs[index]
    }
}

impl<'a> Iterator for &'a Sections {
    type Item = &'a Section;

    fn next(&mut self) -> Option<Self::Item> {
        self.secs.iter().next()
    }
}