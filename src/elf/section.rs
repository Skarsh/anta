// TODO: remove
#![allow(dead_code)]

use bitflags::bitflags;

use super::types::*;

#[derive(Debug, Clone, PartialEq, Eq)]
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

bitflags! {
    pub struct Elf32SectionFlags: Elf32Word {
        const WRITE = 0x1;
        const ALLOC = 0x2;
        const EXECINSTR = 0x4;
        const MERGE = 0x10;
        const STRINGS = 0x20;
        const INFOLINK = 0x40;
        const LINKORDER = 0x80;
        const OSNONCONFORMING = 0x100;
        const GROUP = 0x200;
        const TLS = 0x400;
        const MASKOS = 0x0ff00000;
        const MASKPROC = 0xf0000000;
    }
}

bitflags! {
    pub struct Elf64SectionFlags: Elf64Xword {
        const WRITE = 0x1;
        const ALLOC = 0x2;
        const EXECINSTR = 0x4;
        const MERGE = 0x10;
        const STRINGS = 0x20;
        const INFOLINK = 0x40;
        const LINKORDER = 0x80;
        const OSNONCONFORMING = 0x100;
        const GROUP = 0x200;
        const TLS = 0x400;
        const MASKOS = 0x0ff00000;
        const MASKPROC = 0xf0000000;

    }
}

#[derive(Debug)]
#[repr(C)]
pub struct Elf32Shdr {
    pub name: Elf32Word,
    pub sh_type: ElfSectionType,
    pub flags: Elf32SectionFlags,
    pub addr: Elf32Addr,
    pub offset: Elf32Off,
    pub size: Elf32Word,
    pub link: Elf32Word,
    pub info: Elf32Word,
    pub addr_align: Elf32Word,
    pub ent_size: Elf32Word,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct Elf64Shdr {
    pub name: Elf64Word,
    pub sh_type: ElfSectionType,
    pub flags: Elf64SectionFlags,
    pub addr: Elf64Addr,
    pub offset: Elf64Off,
    pub size: Elf64Xword,
    pub link: Elf64Word,
    pub info: Elf64Word,
    pub addr_align: Elf64Xword,
    pub ent_size: Elf64Xword,
}

#[derive(Debug, Clone)]
pub enum ElfSectionHeader<'a> {
    Section32(&'a Elf32Shdr),
    Section64(&'a Elf64Shdr),
}

// TODO: Think more about this
#[derive(Debug)]
pub struct Section<'a> {
    pub name: &'a str,
    r#type: &'a ElfSectionType,
    bytes: &'a [u8],
    section_header: ElfSectionHeader<'a>,
}

impl<'a> Section<'a> {
    pub fn new(
        name: &'a str,
        r#type: &'a ElfSectionType,
        bytes: &'a [u8],
        section_header: ElfSectionHeader<'a>,
    ) -> Self {
        Self {
            name,
            r#type,
            bytes,
            section_header,
        }
    }
}
