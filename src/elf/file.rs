use super::{section::Section, symbol::Symbol};

#[derive(Debug)]
pub struct ElfFile<'a> {
    pub sections: Vec<Section<'a>>,
    pub symbols: Vec<Symbol<'a>>,
}

impl<'a> ElfFile<'a> {
    pub fn new(sections: Vec<Section<'a>>, symbols: Vec<Symbol<'a>>) -> Self {
        ElfFile { sections, symbols }
    }

    #[allow(dead_code, unused_variables)]
    pub fn get_section_by_name(&self, name: &str) -> Option<&Section> {
        for section in &self.sections {
            if name == section.name {
                return Some(section);
            }
        }
        None
    }
}
