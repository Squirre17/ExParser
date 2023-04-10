pub mod elf;
use colored::Colorize;

use crate::binbuf::BinBuf;
use crate::parser::elf::elf_struct::Elf64Ehdr;
use crate::parser::elf::elf_struct::Elf64Phdr;
use crate::parser::elf::elf_struct::Elf64Shdr;
use std::hash::Hash;
use std::mem;
use std::collections::BTreeMap;
use std::collections::HashMap;

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
    #[test]
    fn test_print_shdr() {
        let parser = Parser::new("/bin/ls");
        parser.show_shdrs();
    }
}