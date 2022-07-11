// TODO: remove
#![allow(dead_code)]

use super::types::*;

#[derive(Debug)]
#[repr(u32)]
pub enum ElfSectionType {
    Null = 0,
    ProgBits = 1,
    SymTab = 2,
    StrTab = 3,
    Rela = 4,
    Hash = 5,
    Dynamic = 6,
    Note = 7,
    NoBits = 8,
    Rel = 9,
    ShLib = 10,
    DynSym = 11,
    InitArray = 14,
    FiniArray = 15,
    PreInitArray = 16,
    Group = 17,
    SymTabShndx = 18,
    LoOs = 0x60000000,
    HiOs = 0x6fffffff,
    LoProc = 0x70000000,
    Hiproc = 0x7fffffff,
    LoUser = 0x80000000,
    HiUser = 0xffffffff,
}

#[derive(Debug)]
#[repr(u32)]
pub enum Elf32SectionFlag {
    Write = 0x1,
    Alloc = 0x2,
    Execinstr = 0x4,
    Merge = 0x10,
    Strings = 0x20,
    InfoLink = 0x40,
    LinkOrder = 0x80,
    OsNonconforming = 0x100,
    Group = 0x200,
    Tls = 0x400,
    MaskOs = 0x0ff00000,
    MaskProc = 0xf0000000,
}

#[derive(Debug)]
#[repr(u64)]
pub enum Elf64SectionFlag {
    Write = 0x1,
    Alloc = 0x2,
    Execinstr = 0x4,
    Merge = 0x10,
    Strings = 0x20,
    InfoLink = 0x40,
    LinkOrder = 0x80,
    OsNonconforming = 0x100,
    Group = 0x200,
    Tls = 0x400,
    MaskOs = 0x0ff00000,
    MaskProc = 0xf0000000,
}

#[derive(Debug)]
#[repr(C)]
pub struct Elf32Shdr {
    name: Elf32Word,
    sh_type: ElfSectionType,
    flags: Elf32SectionFlag,
    addr: Elf32Addr,
    offset: Elf32Off,
    size: Elf32Word,
    link: Elf32Word,
    info: Elf32Word,
    addr_align: Elf32Word,
    ent_size: Elf32Word,
}

#[derive(Debug)]
#[repr(C)]
pub struct Elf64Shdr {
    name: Elf64Word,
    sh_type: ElfSectionType,
    flags: Elf64SectionFlag,
    addr: Elf64Addr,
    offset: Elf64Off,
    size: Elf64Xword,
    link: Elf64Word,
    info: Elf64Word,
    addr_align: Elf64Xword,
    ent_size: Elf64Xword,
}

#[derive(Debug)]
pub enum ElfSection<'a> {
    ElfSection32(&'a Elf32Shdr),
    ElfSection64(&'a Elf64Shdr),
}

// ================================================== SYMBOLS ====================================================================== //

#[derive(Debug)]
pub struct Elf32Sym {
    name: Elf32Word,
    value: Elf32Addr,
    size: Elf32Word,
    info: u8,
    other: u8,
    shndx: Elf32Half,
}

#[derive(Debug)]
pub struct Elf64Sym {
    name: Elf64Word,
    info: u8,
    other: u8,
    shndx: Elf64Half,
    value: Elf64Addr,
    size: Elf64Xword,
}

#[cfg(test)]
mod tests {
    use super::super::header::*;
    use super::*;
    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn parse_section_headers() {
        let mut f = File::open("samples/bin/hello").unwrap();
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer).unwrap();

        let (head, body, _tail) = unsafe { buffer.align_to::<Elf64Ehdr>() };
        assert!(head.is_empty(), "Data was not aligned");
        let elf_64_ehdr = &body[0];
        assert!(validate_elf_ident(&elf_64_ehdr.ident));

        assert_eq!(elf_64_ehdr.elf_type, ElfType::Exec);
        assert_eq!(elf_64_ehdr.machine, Machine::X86_64);

        //println!("elf header: {:?}", elf_64_ehdr);
        println!(
            "offset to the section header table: {:x}",
            elf_64_ehdr.sh_off
        );
        println!(
            "number of entries in the section header table: {}",
            elf_64_ehdr.sh_num
        );
        println!("section header entry size: {}", elf_64_ehdr.sh_ent_size);

        let mut offset: usize = elf_64_ehdr.sh_off.try_into().unwrap();
        let mut section_header_entries = Vec::new();
        for _i in 1..=elf_64_ehdr.sh_num {
            let entry_slice = &buffer[offset..offset + elf_64_ehdr.sh_ent_size as usize];
            let (head, body, _tail) = unsafe { entry_slice.align_to::<Elf64Shdr>() };
            assert!(head.is_empty());
            let elf_64_shdr = &body[0];
            section_header_entries.push(elf_64_shdr);

            // Increase the offset for parsing the next entry
            // in the section header table
            offset += elf_64_ehdr.sh_ent_size as usize;
        }

        let string_table_section_header_entry =
            section_header_entries[elf_64_ehdr.sh_str_ndx as usize];

        let string_table_start_byte_ndx = string_table_section_header_entry.offset;
        let string_table_end_byte_ndx =
            string_table_start_byte_ndx + string_table_section_header_entry.size;
        let string_table_buffer_slice =
            &buffer[string_table_start_byte_ndx as usize..string_table_end_byte_ndx as usize];

        let first_string_table_byte = string_table_buffer_slice[0];
        println!("first string table byte {}", first_string_table_byte);
        let first_string_byte =
            string_table_buffer_slice[string_table_section_header_entry.name as usize];
        println!("first_string_byte {}", first_string_byte);

        let mut done = false;
        let mut current_index = string_table_section_header_entry.name as usize;

        while !done {
            let current_string_byte = string_table_buffer_slice[current_index];
            if current_string_byte == 0 {
                done = true;
                break;
            }
            current_index += 1;
        }

        // Prevent weird linting issued saying `done` is never used.
        let _ = done;

        let string_slice = &string_table_buffer_slice
            [string_table_section_header_entry.name as usize..current_index as usize];
        println!("string slice {:?}", string_slice);

        let string_slice_string = match std::str::from_utf8(string_slice) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence {}", e),
        };

        println!("result {}", string_slice_string);
        assert_eq!(string_slice_string, ".shstrtab");
    }
}
