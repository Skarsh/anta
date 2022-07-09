// TODO: Remove
#![allow(dead_code)]
#![allow(unused_imports)]

use super::header::{self, validate_elf_ident, Class, Data, ElfHeader, EI_CLASS_IDX, EI_DATA_IDX};
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub struct ElfParser<'a> {
    file_path: &'a Path,
    file_bytes: Vec<u8>,
}

impl<'a> ElfParser<'a> {
    pub fn new(file_path: &'a Path) -> Self {
        ElfParser {
            file_path,
            file_bytes: Vec::new(),
        }
    }

    // TODO: This assumes that the whole file can be read
    // from start to end into a single Vec. If this is a very
    // big file, this should probably be done in a lazy manner instead
    pub fn read_elf_file_into_buffer(&mut self) {
        let mut file = File::open(self.file_path).unwrap();
        file.read_to_end(&mut self.file_bytes).unwrap();
    }

    pub fn parse_header(&mut self) -> ElfHeader {
        let class_byte = self.file_bytes[EI_CLASS_IDX];
        let class = Class::try_from(class_byte).unwrap();
        println!("{:?}", class);

        let data_byte = self.file_bytes[EI_DATA_IDX];
        let _data = Data::try_from(data_byte).unwrap();

        todo!();

    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::Path;

    #[test]
    fn new_parser() {
        let parser = ElfParser::new(Path::new("samples/bin/hello"));

        assert_eq!(parser.file_path.to_str().unwrap(), "samples/bin/hello");
        assert_eq!(parser.file_bytes.len(), 0);
    }

    #[test]
    fn test_read_elf_file_into_buffer() {
        let mut parser = ElfParser::new(Path::new("samples/bin/hello"));
        let expected_file_size = 8912;
        parser.read_elf_file_into_buffer();
        assert_eq!(parser.file_bytes.len(), expected_file_size);
    }

    #[should_panic]
    #[test]
    fn test_parse_elf_header() {
        let mut parser = ElfParser::new(Path::new("samples/bin/hello"));
        parser.read_elf_file_into_buffer();
        parser.parse_header();
    }
}
