use super::types::*;

#[derive(Debug)]
#[allow(dead_code)]
pub enum ElfProgramheader<'a> {
    ProgramHeader32(&'a Elf32Phdr),
    ProgramHeader64(&'a Elf64Phdr),
}

#[derive(Debug)]
#[repr(C)]
pub struct Elf32Phdr {
    r#type: Elf32Word,
    offset: Elf32Off,
    vaddr: Elf32Addr,
    paddr: Elf32Addr,
    filesz: Elf32Word,
    memsz: Elf32Word,
    flags: Elf32Word,
    align: Elf32Word,
}

#[derive(Debug)]
#[repr(C)]
pub struct Elf64Phdr {
    r#type: Elf64Word,
    flags: Elf64Word,
    offset: Elf64Off,
    vaddr: Elf64Addr,
    paddr: Elf64Addr,
    filesz: Elf64Xword,
    memsz: Elf64Xword,
    align: Elf64Xword,
}
