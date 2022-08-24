use super::{section::ElfSectionHeader, symbol::ElfSym};

#[derive(Debug)]
#[allow(dead_code)]
pub struct ElfFile<'a> {
    pub section_headers: Vec<ElfSectionHeader<'a>>,
    pub symbols: Vec<ElfSym<'a>>,
}

impl<'a> ElfFile<'a> {
    pub fn new(section_headers: Vec<ElfSectionHeader<'a>>, symbols: Vec<ElfSym<'a>>) -> Self {
        ElfFile {
            section_headers,
            symbols,
        }
    }
}
