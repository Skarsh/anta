use super::{section::ElfSectionHeader, symbol::ElfSym};

#[derive(Debug)]
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

    #[allow(dead_code, unused_variables)]
    pub fn get_section_by_name(&self, name: &str) -> &ElfSectionHeader {
        for section_header in &self.section_headers {
            if let ElfSectionHeader::Section64(header) = section_header {
                //if name == header.name {}
            }
        }
        // TODO: Remove, this is just a placeholder
        &self.section_headers[0]
    }
}
