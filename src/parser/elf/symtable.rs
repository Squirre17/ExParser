use crate::parser::elf::elf_struct::Elf64Sym;
// TODO: maybe can merge with dyn sym table

#[derive(Debug)]
pub struct Symbol {
    // wrapper of dynsym and dynstr
    pub sym : Elf64Sym,
    pub str : String,
}

impl Symbol {
    pub fn new(sym : Elf64Sym, str : String) -> Self {
        Symbol { sym, str }
    }
}

pub struct SymTables {
    pub syms : Vec<Symbol>,
}

impl SymTables {
    pub fn new(tables : Vec<Symbol>) -> Self{
        SymTables {  
            syms: tables,
        }
    }
}
