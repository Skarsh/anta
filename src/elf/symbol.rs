use super::types::*;

#[derive(Debug)]
#[allow(dead_code)]
pub enum ElfSym<'a> {
    Sym32(&'a Elf32Sym),
    Sym64(&'a Elf64Sym),
}

#[derive(Debug)]
#[repr(C)]
pub struct Elf32Sym {
    name: Elf32Word,
    value: Elf32Addr,
    size: Elf32Word,
    info: u8,
    other: u8,
    shndx: Elf32Half,
}

#[derive(Debug)]
#[repr(C)]
pub struct Elf64Sym {
    name: Elf64Word,
    info: u8,
    other: u8,
    shndx: Elf64Half,
    value: Elf64Addr,
    size: Elf64Xword,
}

#[derive(Debug)]
#[repr(u8)]
#[allow(dead_code)]
pub enum SymBinding {
    Local = 0,
    Global = 1,
    Weak = 2,
    LoOs = 10,
    HiOs = 12,
    LoProc = 13,
    HiProc = 15,
}

#[derive(Debug)]
#[repr(u8)]
#[allow(dead_code)]
pub enum SymType {
    NoType = 0,
    Object = 1,
    Func = 2,
    Section = 3,
    File = 4,
    Common = 5,
    Tls = 6,
    LoOs = 10,
    HiOs = 12,
    LoProc = 13,
    HiProc = 15,
}
