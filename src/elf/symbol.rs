use num_derive::FromPrimitive;

use super::types::*;

#[derive(Debug)]
#[allow(dead_code)]
pub enum ElfSym<'a> {
    Sym32(&'a Elf32Sym),
    Sym64(&'a Elf64Sym),
}

#[derive(Debug, FromPrimitive, PartialEq, Eq)]
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

#[derive(Debug, FromPrimitive, PartialEq, Eq)]
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

#[derive(Debug, FromPrimitive, PartialEq, Eq)]
#[repr(u8)]
#[allow(dead_code)]
pub enum SymVisibility {
    Default = 0,
    Internal = 1,
    Hidden = 2,
    Protected = 3,
}

pub trait Info {
    fn bind(&self) -> Option<SymBinding>;
    fn r#type(&self) -> Option<SymType>;
    fn info(bind: u8, r#type: u8) -> u8;
}

pub trait Visibility {
    fn visibility(&self) -> Option<SymVisibility>;
}

#[derive(Debug)]
#[repr(C)]
pub struct Elf32Sym {
    pub name: Elf32Word,
    pub value: Elf32Addr,
    pub size: Elf32Word,
    pub info: u8,
    pub other: u8,
    pub shndx: Elf32Half,
}

impl Info for Elf32Sym {
    fn bind(&self) -> Option<SymBinding> {
        let i = (self.info) >> 4;
        num::FromPrimitive::from_u8(i)
    }

    fn r#type(&self) -> Option<SymType> {
        let i = (self.info) & 0xf;
        num::FromPrimitive::from_u8(i)
    }

    fn info(bind: u8, r#type: u8) -> u8 {
        ((bind) << 4) + ((r#type) & 0xf)
    }
}

impl Visibility for Elf32Sym {
    fn visibility(&self) -> Option<SymVisibility> {
        let o = (self.other) & 0x3;
        num::FromPrimitive::from_u8(o)
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct Elf64Sym {
    pub name: Elf64Word,
    pub info: u8,
    pub other: u8,
    pub shndx: Elf64Half,
    pub value: Elf64Addr,
    pub size: Elf64Xword,
}

impl Info for Elf64Sym {
    fn bind(&self) -> Option<SymBinding> {
        let i = (self.info) >> 4;
        num::FromPrimitive::from_u8(i)
    }

    fn r#type(&self) -> Option<SymType> {
        let i = (self.info) & 0xf;
        num::FromPrimitive::from_u8(i)
    }

    fn info(bind: u8, r#type: u8) -> u8 {
        ((bind) << 4) + ((r#type) & 0xf)
    }
}

impl Visibility for Elf64Sym {
    fn visibility(&self) -> Option<SymVisibility> {
        let o = (self.other) & 0x3;
        num::FromPrimitive::from_u8(o)
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Symbol<'a> {
    pub name: &'a str,
    pub elf_sym: ElfSym<'a>,
}

impl<'a> Symbol<'a> {
    pub fn new(name: &'a str, elf_sym: ElfSym<'a>) -> Self {
        Self { name, elf_sym }
    }
}
