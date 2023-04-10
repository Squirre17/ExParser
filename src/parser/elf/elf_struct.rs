use std::any::Any;
use std::mem;
use std::ptr;
use std::fmt;
/*
+--------------------------+
|     ELF Header           | : Ehdr
+--------------------------+
|     Program Header Table | : Phdr
+--------------------------+
|    Section Header Table  | : Shdr
+--------------------------+
|         Sections         |
+--------------------------+

*/
const EI_NIDENT : usize = 0x10;// TEMP:

type Elf64Half  = u16;
type Elf64Word  = u32;
type Elf64Addr  = u64;
type Elf64Off   = u64;
type Elf64Xword = u64;

/* Elf Header */
#[repr(C)]   /* for forbid rearrange */
pub struct Elf64Ehdr {
        e_ident     : [u8; EI_NIDENT], /* Magic number and other info */
        e_type      : Elf64Half,       /* Object file type */
        e_machine   : Elf64Half,       /* Architecture */
        e_version   : Elf64Word,       /* Object file version */
    pub e_entry     : Elf64Addr,       /* Entry point virtual address */
    pub e_phoff     : Elf64Off,        /* Program header table file offset */
    pub e_shoff     : Elf64Off,        /* Section header table file offset */
        e_flags     : Elf64Word,       /* Processor-specific flags */
        e_ehsize    : Elf64Half,       /* ELF header size in bytes */
        e_phentsize : Elf64Half,       /* Program header table entry size */
    pub e_phnum     : Elf64Half,       /* Program header table entry count */
        e_shentsize : Elf64Half,       /* Section header table entry size */
    pub e_shnum     : Elf64Half,       /* Section header table entry count */
    pub e_shstrndx  : Elf64Half,       /* Section header string table index */
}

impl fmt::Debug for Elf64Ehdr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Format the e_ident field in hexadecimal format
        let ident = self.e_ident.iter().map(|b| format!("{:02X}", b)).collect::<Vec<String>>().join(" ");
        
        // Use format! macro to print the struct fields
        write!(f, "Elf64Ehdr {{\n")?;
        write!(f, "    e_ident     : {},\n", ident)?;
        write!(f, "    e_type      : 0x{:x},\n", self.e_type)?;
        write!(f, "    e_machine   : 0x{:x},\n", self.e_machine)?;
        write!(f, "    e_version   : 0x{:x},\n", self.e_version)?;
        write!(f, "    e_entry     : 0x{:x},\n", self.e_entry)?;
        write!(f, "    e_phoff     : 0x{:x},\n", self.e_phoff)?;
        write!(f, "    e_shoff     : 0x{:x},\n", self.e_shoff)?;
        write!(f, "    e_flags     : 0x{:x},\n", self.e_flags)?;
        write!(f, "    e_ehsize    : 0x{:x},\n", self.e_ehsize)?;
        write!(f, "    e_phentsize : 0x{:x},\n", self.e_phentsize)?;
        write!(f, "    e_phnum     : 0x{:x},\n", self.e_phnum)?;
        write!(f, "    e_shentsize : 0x{:x},\n", self.e_shentsize)?;
        write!(f, "    e_shnum     : 0x{:x},\n", self.e_shnum)?;
        write!(f, "    e_shstrndx  : 0x{:x},\n", self.e_shstrndx)?;
        write!(f, "}}")
    }
}

impl Elf64Ehdr {
    pub fn new(buf : &[u8]) -> Self {

        let mut header = Elf64Ehdr {
            e_ident     : [0; EI_NIDENT],
            e_type      : 0,
            e_machine   : 0,
            e_version   : 0,
            e_entry     : 0,
            e_phoff     : 0,
            e_shoff     : 0,
            e_flags     : 0,
            e_ehsize    : 0,
            e_phentsize : 0,
            e_phnum     : 0,
            e_shentsize : 0,
            e_shnum     : 0,
            e_shstrndx  : 0,
        };

        
        let sz = mem::size_of::<Elf64Ehdr>();
        assert!(buf.len() >= sz);
        
        let (l, _) = buf.split_at(sz);
        unsafe {
            // &mut header as *mut _ make header as rawptr
            ptr::copy_nonoverlapping(l.as_ptr(), &mut header as *mut _ as *mut u8, sz);
        }

        dbg!(&header);
        header
    }
}

#[repr(C)]
pub struct EIdent {
    ei_mag0 : u8,
    ei_mag1 : u8,
    ei_mag2 : u8,
    ei_mag3 : u8,

    ei_class : u8,
    ei_data : u8,
    ei_version : u8,
    ei_pad : [u8; 9],
}
  

  
/* Elf Program Header */
#[repr(C)]
pub struct Elf64Phdr
{
        p_type   : Elf64Word,			/* Segment type */
        p_flags  : Elf64Word,			/* Segment flags */
    pub p_offset : Elf64Off,		    /* Segment file offset */
    pub p_vaddr  : Elf64Addr,		    /* Segment virtual address */
  	pub p_paddr  : Elf64Addr,		    /* Segment physical address */
  	    p_filesz : Elf64Xword,		    /* Segment size in file */
  	    p_memsz  : Elf64Xword,		    /* Segment size in memory */
  	    p_align  : Elf64Xword,		    /* Segment alignment */
}

impl Elf64Phdr {
    pub fn new(buf : &[u8]) -> Self {

        let mut header = Elf64Phdr {
            p_type   : 0,
            p_flags  : 0,
            p_offset : 0,
            p_vaddr  : 0,
            p_paddr  : 0,
            p_filesz : 0,
            p_memsz  : 0,
            p_align  : 0,
        };

        let sz = mem::size_of::<Elf64Phdr>();
        assert!(buf.len() >= sz);

        let (l, _) = buf.split_at(sz);
        unsafe {
            // &mut header as *mut _ make header as rawptr
            ptr::copy_nonoverlapping(l.as_ptr(), &mut header as *mut _ as *mut u8, l.len());
        }

        header
    }
}

/* Elf Section Header */
#[repr(C)]
#[derive(Clone)]
pub struct Elf64Shdr
{
    pub sh_name      : Elf64Word,		/* Section name (string tbl index) */
        sh_type      : Elf64Word,		/* Section type */
        sh_flags     : Elf64Xword,		/* Section flags */
    pub sh_addr      : Elf64Addr,		/* Section virtual addr at execution */
    pub sh_offset    : Elf64Off,		/* Section file offset */
    pub sh_size      : Elf64Xword,		/* Section size in bytes */
        sh_link      : Elf64Word,		/* Link to another section */
        sh_info      : Elf64Word,		/* Additional section information */
        sh_addralign : Elf64Xword,		/* Section alignment */
        sh_entsize   : Elf64Xword,		/* Entry size if section holds table */
}
// TEMP:
const SHT_SYMTAB : Elf64Word = 3;
const SHT_DYNSYM : Elf64Word = 11;
#[deprecated]
pub fn find_symtab_in_shdrs(shdrs : &Vec<Elf64Shdr>) -> Elf64Shdr {
    for shdr in shdrs {
        if shdr.sh_type == SHT_SYMTAB {
            return shdr.clone();
        }
    }
    unreachable!()
}
impl Elf64Shdr {
    pub fn new(buf : &[u8]) -> Self {

        let mut header = Elf64Shdr {
            sh_name      : 0,
            sh_type      : 0,
            sh_flags     : 0,
            sh_addr      : 0,
            sh_offset    : 0,
            sh_size      : 0,
            sh_link      : 0,
            sh_info      : 0,
            sh_addralign : 0,
            sh_entsize   : 0,            
        };

        let sz = mem::size_of::<Elf64Shdr>();
        assert!(buf.len() >= sz);

        unsafe {
            // &mut header as *mut _ make header as rawptr
            ptr::copy_nonoverlapping(buf.as_ptr(), &mut header as *mut _ as *mut u8, sz);
        }
        // dbg!(header.clone());
        // println!("sh_size at 0x{:x}", header.sh_size);
        // println!("sh_addr at 0x{:x}", header.sh_addr);
        header
    }
}

impl fmt::Debug for Elf64Shdr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        write!(f, "Section Headers\n")?;
        write!(f, "sh_name: {:#x},\n", self.sh_name)?;
        write!(f, "sh_type: {:#x},\n", self.sh_type)?;
        write!(f, "sh_flags: {:#x},\n", self.sh_flags)?;
        write!(f, "sh_addr: {:#x},\n", self.sh_addr)?;
        write!(f, "sh_offset: {:#x},\n", self.sh_offset)?;
        write!(f, "sh_size: {:#x},\n", self.sh_size)?;
        write!(f, "sh_link: {:#x},\n", self.sh_link)?;
        write!(f, "sh_info: {:#x},\n", self.sh_info)?;
        write!(f, "sh_addralign: {:#x},\n", self.sh_addralign)?;
        write!(f, "sh_entsize: {:#x},\n", self.sh_entsize)?;
        
        Ok(())
    }
}