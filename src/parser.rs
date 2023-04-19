pub mod elf;
use colored::Colorize;

use crate::binbuf::BinBuf;
use crate::parser::elf::elf_struct::Elf64Ehdr;
use crate::parser::elf::elf_struct::Elf64Phdr;
use crate::parser::elf::elf_struct::Elf64Shdr;
use crate::parser::elf::elf_struct::Elf64Sym;
use crate::parser::elf::dyntable::DynSymTab;
use crate::parser::elf::dyntable::DynSymTables;
use crate::parser::elf::symtable::SymTables;
use crate::parser::elf::symtable::Symbol;
use crate::parser::elf::segments::Segments;
use crate::parser::elf::segments::Segment;
use crate::parser::elf::sections::Sections;
use crate::parser::elf::sections::Section;
use crate::parser::elf::relocations::Relocations;
use crate::parser::elf::relocations::Relocation;
use std::borrow::BorrowMut;
use std::mem;

use self::elf::dyntable;
use self::elf::symtable;

pub struct Parser {
    binbuf     : BinBuf,
    ehdr       : Elf64Ehdr,
    segments   : Segments,
    sections   : Sections,
    dynsymtabs : DynSymTables,
    symtables  : Option<SymTables>,
    relocs     : Option<Relocations>
}
impl Parser {


}
/*
遍历ELF文件中所有的PHDR，找到类型为PT_LOAD（表示可加载的段）的PHDR。
对于每个类型为PT_LOAD的PHDR，记录下其虚拟地址（p_vaddr）、文件偏移量（p_offset）、内存大小（p_memsz）和文件大小（p_filesz）等信息。这些信息可以用来确定该段在文件中的位置以及在内存中的位置。
遍历ELF文件中所有的SHDR，找到与当前PT_LOAD段相关的SHDR。具体判断方法是：如果一个SHDR的sh_addr字段值在当前PT_LOAD段的虚拟地址范围内，则认为该SHDR与当前PT_LOAD段相关。
找到与当前PT_LOAD段相关的SHDR后，就可以获取该段在文件中的位置和大小等信息，从而可以将该段从文件中读取出来或者映射到内存中去。
 */

impl Parser {

    pub fn show_sections(&self) -> &Self {
        self.sections.show_shdrs();
        self
    }

    pub fn show_segments(&self) -> &Self {
        self.segments.show_phdrs();
        self
    }

    // NOTE: debug info and symbol will not be loaded, i.e. not display in segment scope
    pub fn show_layout(&self) -> &Self {

        for seg in &self.segments.segs {

            let seg_start = seg.phdr.p_offset;
            let seg_end = seg.phdr.p_offset + seg.phdr.p_filesz;
            
            let start = format!("0x{:x}", seg_start);
            let end   = format!("0x{:x}", seg_end);

            println!("{:<022} {}", seg.name.red(), start.yellow());
            
            for sec in &self.sections.secs {

                let sec_start = sec.shdr.sh_offset;
                let sec_end = sec.shdr.sh_offset + sec.shdr.sh_size;

                // TODO: maybe unsound
                if sec_start >= seg_start && sec_end <= seg_end {
                    println!("\t{:<20} 0x{:x}-0x{:x}", sec.name.blue(), sec_start, sec_end );
                }
            }
            println!("{:<022} {}", "END".to_string().red(), end.yellow() );
            println!("-----------------------------------------------");
        }

        self
    }
    
    fn get_name(&self, shdr : &Elf64Shdr) -> String {
        // get name of given shdr
        let shstridx = self.ehdr.e_shstrndx as usize;
        let offset = shdr.sh_name;
        let i = self.sections[shstridx].shdr.sh_offset + offset as u64;

        return self.binbuf.idx_to_string(i as usize);
    }

    fn find_section(&self, sname : &str ) -> Option<&Section>{
        // find section by section name(e.g : .dynsym)

        // NOTE: consider remove shdrs??? replace it with section
        for section in &self.sections.secs {
            let s = self.get_name(&section.shdr);
            if s == sname {
                return Some(section);
            }
        }
        return None;
    }

    pub fn show_magic(&self) -> &Self {
        for i in 0..0x10 {
            print!("{:02x} ", self.binbuf.buf[i]);
        }
        println!("");
        self
    }

    pub fn new(filename : &str) -> Parser {
        // TODO: split it to diff parts

        let binbuf = BinBuf::new(filename);
        
        let idx = 0x0;
        let ehdr = Elf64Ehdr::new(binbuf.buf[idx..].as_ref());

        /* parse segments */
        let mut idx = ehdr.e_phoff as usize;
        let mut phdrs = vec![];

        for _ in 0..ehdr.e_phnum {
            let phdr = Elf64Phdr::new(binbuf.buf[idx..].as_ref());
            idx += mem::size_of::<Elf64Phdr>();
            phdrs.push(phdr);
        }

        /* parse sections */
        let mut idx = ehdr.e_shoff as usize;
        let mut shdrs = vec![];

        for _ in 0..ehdr.e_shnum {
            let shdr = Elf64Shdr::new(binbuf.buf[idx..].as_ref());
            idx += mem::size_of::<Elf64Shdr>();
            shdrs.push(shdr);
        }

        
        // 1. find section header string index 
        // 2. get offset of shstrtab in binary
        let shstridx = ehdr.e_shstrndx as usize;
        let offset = shdrs[shstridx].sh_offset as usize;
        let end = (shdrs[shstridx].sh_offset + shdrs[shstridx].sh_size) as usize;
        
        let segments = Segments::new(phdrs);
        let sections = Sections::new(shdrs, &binbuf.buf[offset..end], shstridx);
        
        /* parse .dynsym */
        let mut dynsyms = vec![];

        let dynsym_section = sections.get_section(".dynsym").unwrap();
        let mut offset = dynsym_section.shdr.sh_offset as usize;

        let dynsym_size : u64 = mem::size_of::<Elf64Sym>() as u64;
        let num_of_sym = dynsym_section.shdr.sh_size / dynsym_size;

        let sym_section = sections.get_section(".dynstr").unwrap();
        let sym_start_offset = sym_section.shdr.sh_offset as usize;

        for _ in 0..num_of_sym {
            let sym = Elf64Sym::new(binbuf.buf[offset..].as_ref());
            offset += mem::size_of::<Elf64Sym>();

            // NOTE: sym.st_name is offset in strtab here(maybe make it more human readable?)
            let string = binbuf.idx_to_string(sym.st_name as usize + sym_start_offset);
            
            dynsyms.push(DynSymTab{
                sym,
                str : string
            });
        }

        /* parse symbol (if exist) */
        let symtables;

        if let Some(sym_section) = sections.get_section(".symtab"){

            let mut symbols = vec![];
            
            let mut offset = sym_section.shdr.sh_offset as usize;

            let sym_size : u64 = mem::size_of::<Elf64Sym>() as u64;
            let num_of_sym = sym_section.shdr.sh_size / sym_size;
    
            let sym_str_section = sections.get_section(".strtab").unwrap();
            let sym_start_offset = sym_str_section.shdr.sh_offset as usize;

            for _ in 0..num_of_sym {
                let sym = Elf64Sym::new(binbuf.buf[offset..].as_ref());
                offset += mem::size_of::<Elf64Sym>();
    
                // NOTE: sym.st_name is offset in strtab here(maybe make it more human readable?)
                let string = binbuf.idx_to_string(sym.st_name as usize + sym_start_offset);
                
                symbols.push(Symbol::new(
                    sym,
                    string,
                ));
            }

            symtables = Some(SymTables::new(symbols));

        } else {
            symtables = None;
        }
        
        /* parse got(if dynamic) TODO: test statically */

        // TODO: maybe have .got(contain global variable)
        let relocs;

        if let Some(rel_section) = sections.get_section(".got.plt") {

            let mut _relocs = vec![];

            let mut start = rel_section.shdr.sh_offset as usize;
            let end = start + rel_section.shdr.sh_size as usize;
            let sz = std::mem::size_of::<usize>();

            while start < end {
                
                let addr : [u8; 8] = binbuf.get_content(start, sz)
                                           .try_into()
                                           .unwrap();
            
                let addr = u64::from_le_bytes(addr);

                _relocs.push(Relocation::new(
                    addr, start
                ));

                start += sz
            }

            relocs = Some(Relocations::new(_relocs));
        }else {
            relocs = None
        }


        Parser {  
            binbuf,
            ehdr,
            segments ,
            sections ,
            dynsymtabs : DynSymTables::new( dynsyms),
            symtables,
            relocs
        }
    }
    pub fn writeback(&self, path : &String) {

        unimplemented!()
    }
    pub fn add_new_section(&mut self, section : Section) -> &Self {
        /* 
         1. adjust ElfXX_Ehdr->e_shoff
         2. adjust all section beyond 0x1000
            - file offset
            - vir addr
         3. adjust all segments beyond 0x1000
            - file offset
            - vir addr
            - phy addr
         4. adjust dynsym
         5. adjust symbol
         6. adjust relocate
         */
         
        let shift = 0x1000;
        
        let phdr_start_offset = self.ehdr.e_phoff;
        let phdr_size = self.ehdr.e_phnum as u64;
        
        /*
        |------------------|
        |    ELF Header    |
        |------------------|
        | Program Header 1 |
        |------------------|
        | Program Header 2 |--------------| e.g. .intepret
        |------------------|              |
        |     ...          |              |
        |------------------|              |
        | Program Header n |              |
        |------------------| <<- from     |
        |                  |              |
        |      shift       | <------------|
        |                  |
        |------------------| <<- 0x1000 + from
        |     Section 1    |
        |------------------|
        |     Section 2    |
        |------------------|
        |        ...       |
        |------------------|
        |     Section m    |
        |------------------|
        | Section Header 1 |
        |------------------|
        | Section Header 2 |
        |------------------|
        |        ...       |
        |------------------|
        | Section Header k |
        |------------------|
         */
        let from = phdr_start_offset + phdr_size * self.segments.len() as u64;

        self.ehdr.e_shoff += shift;

        for seg in self.segments.borrow_mut() {

            if seg.phdr.p_offset >= from {

                // TODO: maybe some segments not need shift ?
                
                seg.phdr.p_filesz += shift;
                seg.phdr.p_vaddr  += shift;
                seg.phdr.p_vaddr  += shift;
                
            }
        }
        for sec in self.sections.borrow_mut() {
            // TODO: frame 
            if sec.shdr.sh_offset >= from {

                /* for something like following:

                                 Name               Addr             Offset               Size
                .shstrtab             000000000000000000 00000000000002229c 00000000000000011d
                 */
                if sec.shdr.sh_addr > 0 {
                    sec.shdr.sh_addr += shift;
                } 

                sec.shdr.sh_offset += shift;
            }
        }
        for dynsym in &mut self.dynsymtabs.tables {
            // st_value is an virtual address of value in memory
            if dynsym.sym.st_value >= from {
                dynsym.sym.st_value += shift;
            }
            // TODO: fini_array
        }

        // NOTE: need rewrite here
        if let Some(symtable) = &mut self.symtables {

            for sym in &mut symtable.syms {
                if sym.sym.st_value >= from {
                    sym.sym.st_value += shift;
                }
            }
        }

        self.ehdr.e_entry += shift;

        // TODO: GOT
        dbg!(self.segments.len());
        dbg!(self.sections.len());

        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_print_shdr() {
        let parser = Parser::new("/bin/ls");
        parser.sections.show_shdrs();
        let parser = Parser::new("test/test");
    }
    #[test]
    fn test_print_phdr () {
        let parser = Parser::new("/bin/ls");
        parser.segments.show_phdrs();
    }
    #[test]
    fn test_get_name() {
        /*
        [0 ]                    000000000000000000 000000000000000000 000000000000000000
        [1 ] .interp            000000000000400318 000000000000000318 00000000000000001c
        [2 ] .note.gnu.property 000000000000400338 000000000000000338 000000000000000020
        [3 ] .note.gnu.build-id 000000000000400358 000000000000000358 000000000000000024
         */
        let parser = Parser::new("/bin/ls");
        let s = parser.get_name(&parser.sections[1].shdr);
        assert_eq!(s, ".interp");

        let sec = parser.find_section(".note.gnu.property").unwrap();
        assert_eq!(parser.sections[2].shdr.sh_addr, sec.shdr.sh_addr);
        assert_eq!(parser.sections[2].shdr.sh_name, sec.shdr.sh_name);
        assert_eq!(parser.sections[2].shdr.sh_offset, sec.shdr.sh_offset);
    }
}