pub mod elf;
use colored::Colorize;

use crate::binbuf::BinBuf;
use crate::parser::elf::elf_struct::Elf64Ehdr;
use crate::parser::elf::elf_struct::Elf64Phdr;
use crate::parser::elf::elf_struct::Elf64Shdr;
use std::hash::Hash;
use std::mem;
use num_derive::FromPrimitive;
use std::convert::TryFrom;
use std::collections::BTreeMap;
use std::collections::HashMap;

/* Legal values for p_type (segment type).  */
#[allow(non_camel_case_types)]
#[derive(FromPrimitive, Debug)]
enum SegmentType {
    PT_NULL          = 0,           // Program header table entry unused
    PT_LOAD          = 1,           // Loadable program segment
    PT_DYNAMIC       = 2,           // Dynamic linking information
    PT_INTERP        = 3,           // Program interpreter
    PT_NOTE          = 4,           // Auxiliary information
    PT_SHLIB         = 5,           // Reserved
    PT_PHDR          = 6,           // Entry for header table itself
    PT_TLS           = 7,           // Thread-local storage segment
    PT_NUM           = 8,           // Number of defined types
    PT_LOOS          = 0x60000000,  // Start of OS-specific
    PT_GNU_EH_FRAME  = 0x6474e550,  // GCC .eh_frame_hdr segment
    PT_GNU_STACK     = 0x6474e551,  // Indicates stack executability
    PT_GNU_RELRO     = 0x6474e552,  // Read-only after relocation
    // PT_LOSUNW        = 0x6ffffffa,
    // PT_SUNWBSS       = 0x6ffffffa,  // Sun Specific segment
    // PT_SUNWSTACK     = 0x6ffffffb,  // Stack segment
    // PT_HISUNW        = 0x6fffffff,
    // PT_HIOS          = 0x6fffffff,  // End of OS-specific
    // PT_LOPROC        = 0x70000000,  // Start of processor-specific
    // PT_HIPROC        = 0x7fffffff,  // End of processor-specific
    PT_UNKNOWN      = 0xffffffff,   // self-added
}
    
impl From<u32> for SegmentType {
    fn from(p_type: u32) -> Self {
        match p_type {
            0 => SegmentType::PT_NULL,
            1 => SegmentType::PT_LOAD,
            2 => SegmentType::PT_DYNAMIC,
            3 => SegmentType::PT_INTERP,
            4 => SegmentType::PT_NOTE,
            5 => SegmentType::PT_SHLIB,
            6 => SegmentType::PT_PHDR,
            7 => SegmentType::PT_TLS,
            8 => SegmentType::PT_NUM,
            0x60000000 => SegmentType::PT_LOOS,
            0x6474e550 => SegmentType::PT_GNU_EH_FRAME,
            0x6474e551 => SegmentType::PT_GNU_STACK,
            0x6474e552 => SegmentType::PT_GNU_RELRO,
            _ => SegmentType::PT_UNKNOWN,
        }
    }
}

pub struct Parser {
    binbuf : BinBuf,
    ehdr   : Elf64Ehdr,
    phdrs  : Vec<Elf64Phdr>,
    shdrs  : Vec<Elf64Shdr>,
}
impl Parser {
    pub fn get_sec_name(&self, offset : u32) -> String {
        // get section name by sh_name in shdr
        let shstrtab = &self.shdrs[self.ehdr.e_shstrndx as usize];

        // println!("[DBG] shstrtab at 0x{:x}", shstrtab.sh_offset);
        let offset = offset as usize + shstrtab.sh_offset as usize;
        let mut s = String::new();

        for c in &self.binbuf.buf[offset..] {
            if *c == '\x00' as u8 {
                break;
            }
            s.push(c.clone() as char);
        }

        s
    }
    pub fn show_shdrs(&self) {

        // TODO: clearify sh_addr
        
        print!("{:>4}", "Nr".red());
        print!("{:>19}", "Name".blue());
        print!("{:>19}", "Addr".green());
        print!("{:>19}", "Offset".yellow());
        print!("{:>19}", "Size".cyan());
        println!();

        for (i, shdr) in self.shdrs.iter().enumerate() {
            let sec_name = self.get_sec_name(shdr.sh_name);

            let mut fields = Vec::new();
            fields.push(format!("[{:<2}]", i));
            fields.push(format!("{:<018}", sec_name.blue()));
            fields.push(format!("{:<018x}", shdr.sh_addr));
            fields.push(format!("{:<018x}", shdr.sh_offset));
            fields.push(format!("{:<018x}", shdr.sh_size));

            println!("{}", fields.join(" "));

        }
    }
    fn get_seg_type_str(&self, p_type: u32) -> &'static str {
        // get segment type str

        match SegmentType::from(p_type) {
            SegmentType::PT_NULL  => "NULL",
            SegmentType::PT_LOAD  => "LOAD",
            SegmentType::PT_DYNAMIC  => "DYNAMIC",
            SegmentType::PT_INTERP  => "INTEPR",
            SegmentType::PT_NOTE  => "NOTE",
            SegmentType::PT_SHLIB  => "SHLIB",
            SegmentType::PT_PHDR  => "PHDR",
            SegmentType::PT_TLS  => "TLS",
            SegmentType::PT_NUM  => "NUM",
            SegmentType::PT_LOOS  => "LOOS",
            SegmentType::PT_GNU_EH_FRAME  => "GNU_EH_FRAME",
            SegmentType::PT_GNU_STACK  => "GNU_STACK",
            SegmentType::PT_GNU_RELRO  => "GNU_RELRO",
            SegmentType::PT_UNKNOWN  => "UNKNOWN",
            _ => panic!("unknown type"),
        }
    }
    pub fn show_phdrs(&self) {

        print!("{:>18}", "Type".red());
        print!("{:>19}", "Offset".blue());
        print!("{:>19}", "VirtAddr".green());
        print!("{:>19}", "FileSize".yellow());
        print!("{:>19}", "MemSize".cyan());
        println!();

        // TODO: add sections here
        for (_, phdr) in self.phdrs.iter().enumerate() {
            let mut fields = Vec::new();
            fields.push(format!("{:<018}", self.get_seg_type_str(phdr.p_type)));
            fields.push(format!("{:<018x}", phdr.p_offset));
            fields.push(format!("{:<018x}", phdr.p_vaddr));
            fields.push(format!("{:<018x}", phdr.p_filesz));
            fields.push(format!("{:<018x}", phdr.p_memsz));

            println!("{}", fields.join(" "));
        }
    }
}
/*
遍历ELF文件中所有的PHDR，找到类型为PT_LOAD（表示可加载的段）的PHDR。
对于每个类型为PT_LOAD的PHDR，记录下其虚拟地址（p_vaddr）、文件偏移量（p_offset）、内存大小（p_memsz）和文件大小（p_filesz）等信息。这些信息可以用来确定该段在文件中的位置以及在内存中的位置。
遍历ELF文件中所有的SHDR，找到与当前PT_LOAD段相关的SHDR。具体判断方法是：如果一个SHDR的sh_addr字段值在当前PT_LOAD段的虚拟地址范围内，则认为该SHDR与当前PT_LOAD段相关。
找到与当前PT_LOAD段相关的SHDR后，就可以获取该段在文件中的位置和大小等信息，从而可以将该段从文件中读取出来或者映射到内存中去。
 */
#[deprecated]
fn print_null_terminated_string(data: &[u8]) {
    // 从头开始迭代字节序列
    for &byte in data.iter() {
        // 如果遇到了\x00字节，则停止迭代
        if byte == 0 {
            break;
        }
        // 将字节转换为对应的字符并打印
        print!("{}", byte as char);
    }
    println!(); // 打印一个换行符以便于显示下一行输出
}

impl Parser {
    pub fn new(filename : &str) -> Parser {
        let mut binbuf = BinBuf::new(filename);
        for i in 0..0x10 {
            print!("{:02x} ", binbuf.buf[i]);
        }
        println!("");
        // skip magic field(this alreadly in Elf64Ehdr)
        // binbuf.cur += 0x10;
        
        let ehdr = Elf64Ehdr::new(binbuf.buf[binbuf.cur..].as_ref());
        binbuf.cur = ehdr.e_phoff as usize;

        let mut phdrs = vec![];
        for _ in 0..ehdr.e_phnum {
            let phdr = Elf64Phdr::new(binbuf.buf[binbuf.cur..].as_ref());
            binbuf.cur += mem::size_of::<Elf64Phdr>();
            phdrs.push(phdr);
        }

        binbuf.cur = ehdr.e_shoff as usize;

        println!("------------------------");
        let mut shdrs = vec![];
        binbuf.cur = ehdr.e_shoff as usize;

        for _ in 0..ehdr.e_shnum {
            let shdr = Elf64Shdr::new(binbuf.buf[binbuf.cur..].as_ref());
            binbuf.cur += mem::size_of::<Elf64Shdr>();
            shdrs.push(shdr);
        }
        // let symtab_shdr = crate::parser::elf::elf_struct::find_symtab_in_shdrs(&shdrs);
        // dbg!(&symtab_shdr);
        let shstrtab = &shdrs[ehdr.e_shstrndx as usize];
        dbg!(shstrtab);
        binbuf.cur = shstrtab.sh_offset as usize;
        binbuf.cur += shstrtab.sh_name as usize;

        print_null_terminated_string(&binbuf.buf[binbuf.cur..]);

        Parser {  
            binbuf,
            ehdr,
            phdrs,
            shdrs,
        }
    }
    #[deprecated]
    pub fn read_e_ident(&mut self) -> &[u8] {
        self.binbuf.cur += 0x10;
        return self.binbuf.buf[0..0x10].as_ref();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn test_print_shdr() {
        let parser = Parser::new("/bin/ls");
        parser.show_shdrs();
    }
    #[test]
    fn test_print_phdr () {
        let parser = Parser::new("/bin/ls");
        parser.show_phdrs();
    }
}