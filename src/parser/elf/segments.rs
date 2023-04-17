use crate::parser::elf::elf_struct::Elf64Phdr;
use colored::Colorize;

mod segtype {

    // TODO: try struct enum???
    #[allow(non_camel_case_types)]
    #[derive(Debug)]
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
                0          => SegmentType::PT_NULL,
                1          => SegmentType::PT_LOAD,
                2          => SegmentType::PT_DYNAMIC,
                3          => SegmentType::PT_INTERP,
                4          => SegmentType::PT_NOTE,
                5          => SegmentType::PT_SHLIB,
                6          => SegmentType::PT_PHDR,
                7          => SegmentType::PT_TLS,
                8          => SegmentType::PT_NUM,
                0x60000000 => SegmentType::PT_LOOS,
                0x6474e550 => SegmentType::PT_GNU_EH_FRAME,
                0x6474e551 => SegmentType::PT_GNU_STACK,
                0x6474e552 => SegmentType::PT_GNU_RELRO,
                _ => SegmentType::PT_UNKNOWN,
            }
        }
    }

    pub fn get_seg_type_str(p_type: u32) -> &'static str {
        // get segment type str
        
        match SegmentType::from(p_type) {
            SegmentType::PT_NULL          => "NULL",
            SegmentType::PT_LOAD          => "LOAD",
            SegmentType::PT_DYNAMIC       => "DYNAMIC",
            SegmentType::PT_INTERP        => "INTEPR",
            SegmentType::PT_NOTE          => "NOTE",
            SegmentType::PT_SHLIB         => "SHLIB",
            SegmentType::PT_PHDR          => "PHDR",
            SegmentType::PT_TLS           => "TLS",
            SegmentType::PT_NUM           => "NUM",
            SegmentType::PT_LOOS          => "LOOS",
            SegmentType::PT_GNU_EH_FRAME  => "GNU_EH_FRAME",
            SegmentType::PT_GNU_STACK     => "GNU_STACK",
            SegmentType::PT_GNU_RELRO     => "GNU_RELRO",
            SegmentType::PT_UNKNOWN       => "UNKNOWN",
            _ => panic!("unknown type"),
        }
    }

}
pub struct Segments {
    // manage all segments
    pub segs  : Vec<Segment>,
    pub pos   : usize,
}
#[derive(Debug)]
pub struct Segment {
    // just a wrap for Elf64Phdr
    pub phdr : Elf64Phdr,
    pub name : String,
}

impl Segments {
    pub fn new(phdrs : Vec<Elf64Phdr>) -> Self{

        let mut segs = vec![];
        
        for phdr in phdrs {
            segs.push(Segment{
                name : segtype::get_seg_type_str(phdr.p_type).to_string(),
                phdr,
            });
        }

        Segments { 
            segs ,
            pos : 0,
        }
    }

    pub fn show_phdrs(&self) -> &Self {

        print!("{:>18}", "Type".red());
        print!("{:>19}", "Offset".blue());
        print!("{:>19}", "VirtAddr".green());
        print!("{:>19}", "FileSize".yellow());
        print!("{:>19}", "MemSize".cyan());
        println!();

        // TODO: add sections here
        for (_, seg) in self.segs.iter().enumerate() {
            let mut fields = Vec::new();
            fields.push(format!("{:<018}", segtype::get_seg_type_str(seg.phdr.p_type)));
            fields.push(format!("{:<018x}", seg.phdr.p_offset));
            fields.push(format!("{:<018x}", seg.phdr.p_vaddr));
            fields.push(format!("{:<018x}", seg.phdr.p_filesz));
            fields.push(format!("{:<018x}", seg.phdr.p_memsz));

            println!("{}", fields.join(" "));
        }
        self
    }
}

impl std::ops::Index<usize> for Segments {

    type Output = Segment;

    fn index(&self, index: usize) -> &Self::Output {
        &self.segs[index]
    }
}

impl<'a> Iterator for &'a Segments {
    type Item = &'a Segment;

    fn next(&mut self) -> Option<Self::Item> {
        self.segs.iter().next()
    }
}