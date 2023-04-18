use crate::parser::elf::elf_struct::Elf64Sym;


#[derive(Debug)]
pub struct DynSymTab {
    // wrapper of dynsym and dynstr
    pub sym : Elf64Sym,
    pub str : String,
}

impl DynSymTab {
    pub fn new(sym : Elf64Sym, str : String) -> Self {
        DynSymTab { sym, str }
    }
}

pub struct DynSymTables {
    pub tables : Vec<DynSymTab>,
}

impl DynSymTables {
    pub fn new(tables : Vec<DynSymTab>) -> Self{
        DynSymTables {  
            tables,
        }
    }
}
