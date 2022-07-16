// TODO: Remove
#![allow(dead_code)]
#![allow(unused_imports)]

use super::header::*;
use super::section::{Elf32Shdr, Elf64SectionFlags, Elf64Shdr, ElfSectionHeader, ElfSectionType};
use super::types::Elf32Section;
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

    pub fn parse_header(&self) -> ElfHeader {
        let class_byte = self.file_bytes[EI_CLASS_IDX];
        let class = Class::try_from(class_byte).unwrap();

        let data_byte = self.file_bytes[EI_DATA_IDX];
        let _data = Data::try_from(data_byte).unwrap();

        // Need to know whether the Elf file 64-bit or 32-bit
        // and which endianness we its represented in before parsing
        match class {
            Class::ElfClassNone => panic!(),
            Class::ElfClass32 => {
                let (head, body, _tail) = unsafe { self.file_bytes.align_to::<Elf32Ehdr>() };
                assert!(head.is_empty(), "Data was not aligned");
                let elf_32_ehdr = &body[0];
                assert!(validate_elf_ident(&elf_32_ehdr.ident));
                ElfHeader::Elf32(elf_32_ehdr)
            }
            Class::ElfClass64 => {
                let (head, body, _tail) = unsafe { self.file_bytes.align_to::<Elf64Ehdr>() };
                assert!(head.is_empty(), "Data was not aligned");
                let elf_64_ehdr = &body[0];
                assert!(validate_elf_ident(&elf_64_ehdr.ident));
                ElfHeader::Elf64(elf_64_ehdr)
            }
        }
    }

    pub fn parse_section_headers(&self, elf_header: ElfHeader) -> Vec<ElfSectionHeader> {
        let mut section_header_entries = Vec::new();
        match elf_header {
            ElfHeader::Elf32(header) => {
                let mut entry_offset: usize = header.sh_off.try_into().unwrap();
                for _entry in 1..=header.sh_num {
                    let entry_slice =
                        &self.file_bytes[entry_offset..entry_offset + header.sh_ent_size as usize];
                    // TODO: Error handling, what if data is not aligned
                    let (_head, body, _tail) = unsafe { entry_slice.align_to::<Elf32Shdr>() };
                    let section_header = &body[0];
                    section_header_entries
                        .push(ElfSectionHeader::ElfSectionHeader32(section_header));

                    entry_offset += header.sh_ent_size as usize;
                }
            }
            ElfHeader::Elf64(header) => {
                let mut entry_offset: usize = header.sh_off.try_into().unwrap();
                for _entry in 1..=header.sh_num {
                    let entry_slice =
                        &self.file_bytes[entry_offset..entry_offset + header.sh_ent_size as usize];
                    // TODO: Error handling, what if data is not aligned
                    let (_head, body, _tail) = unsafe { entry_slice.align_to::<Elf64Shdr>() };
                    let section_header = &body[0];
                    section_header_entries
                        .push(ElfSectionHeader::ElfSectionHeader64(section_header));

                    entry_offset += header.sh_ent_size as usize;
                }
            }
        }
        section_header_entries
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

    #[test]
    fn test_parse_elf64_header() {
        let mut parser = ElfParser::new(Path::new("samples/bin/hello"));
        parser.read_elf_file_into_buffer();
        // These asertions are of course very thightly linked to the test file
        if let ElfHeader::Elf64(header) = parser.parse_header() {
            assert_eq!(header.elf_type, ElfType::Exec);
            assert_eq!(header.machine, Machine::X86_64);
            assert_eq!(header.version, Version::Current);
            assert_eq!(header.entry, 0x401000);
            assert_eq!(header.ph_off, 64);
            assert_eq!(header.sh_off, 8528);
            assert_eq!(header.flags, 0x0);
            assert_eq!(header.eh_size, 64);
            assert_eq!(header.ph_ent_size, 56);
            assert_eq!(header.ph_num, 3);
            assert_eq!(header.sh_ent_size, 64);
            assert_eq!(header.sh_num, 6);
            assert_eq!(header.sh_str_ndx, 5);
        }
    }

    #[test]
    fn test_parse_section_headers() {
        let mut parser = ElfParser::new(Path::new("samples/bin/hello"));
        parser.read_elf_file_into_buffer();
        let elf_header = parser.parse_header();
        let section_headers = parser.parse_section_headers(elf_header);
        assert_eq!(section_headers.len(), 6);
        for section_header in &section_headers {
            println!("{:?}", section_header)
        }

        // First entry
        if let ElfSectionHeader::ElfSectionHeader64(section_header) = &section_headers[0] {
            assert_eq!(section_header.sh_type, ElfSectionType::Null);
            assert_eq!(section_header.flags, Elf64SectionFlags::empty());
            assert_eq!(section_header.addr, 0x0);
            assert_eq!(section_header.offset, 0x0);
            assert_eq!(section_header.size, 0x0);
            assert_eq!(section_header.link, 0);
            assert_eq!(section_header.info, 0);
            assert_eq!(section_header.addr_align, 0);
            assert_eq!(section_header.ent_size, 0x0);
        }

        // Second entry
        if let ElfSectionHeader::ElfSectionHeader64(section_header) = &section_headers[1] {
            assert_eq!(section_header.sh_type, ElfSectionType::ProgBits);
            assert_eq!(
                section_header.flags,
                Elf64SectionFlags::ALLOC | Elf64SectionFlags::EXECINSTR
            );
            assert_eq!(section_header.addr, 0x0000000000401000);
            assert_eq!(section_header.offset, 0x00001000);
            assert_eq!(section_header.size, 0x0000000000000025);
            assert_eq!(section_header.link, 0);
            assert_eq!(section_header.info, 0);
            assert_eq!(section_header.addr_align, 16);
            assert_eq!(section_header.ent_size, 0x0000000000000000);
        }

        // Third entry
        if let ElfSectionHeader::ElfSectionHeader64(section_header) = &section_headers[2] {
            assert_eq!(section_header.sh_type, ElfSectionType::ProgBits);
            assert_eq!(
                section_header.flags,
                Elf64SectionFlags::WRITE | Elf64SectionFlags::ALLOC
            );
            assert_eq!(section_header.addr, 0x0000000000402000);
            assert_eq!(section_header.offset, 0x00002000);
            assert_eq!(section_header.size, 0x0000000000000009);
            assert_eq!(section_header.link, 0);
            assert_eq!(section_header.info, 0);
            assert_eq!(section_header.addr_align, 4);
            assert_eq!(section_header.ent_size, 0x0000000000000000);
        }

        // Fourth entry
        if let ElfSectionHeader::ElfSectionHeader64(section_header) = &section_headers[3] {
            assert_eq!(section_header.sh_type, ElfSectionType::SymTab);
            assert_eq!(section_header.flags, Elf64SectionFlags::empty());
            assert_eq!(section_header.addr, 0x0000000000000000);
            assert_eq!(section_header.offset, 0x00002010);
            assert_eq!(section_header.size, 0x00000000000000d8);
            assert_eq!(section_header.link, 4);
            assert_eq!(section_header.info, 5);
            assert_eq!(section_header.addr_align, 8);
            assert_eq!(section_header.ent_size, 0x0000000000000018);
        }

        // Fifth
        if let ElfSectionHeader::ElfSectionHeader64(section_header) = &section_headers[4] {
            assert_eq!(section_header.sh_type, ElfSectionType::StrTab);
            assert_eq!(section_header.flags, Elf64SectionFlags::empty());
            assert_eq!(section_header.addr, 0x0000000000000000);
            assert_eq!(section_header.offset, 0x000020e8);
            assert_eq!(section_header.size, 0x000000000000003e);
            assert_eq!(section_header.link, 0);
            assert_eq!(section_header.info, 0);
            assert_eq!(section_header.addr_align, 1);
            assert_eq!(section_header.ent_size, 0x0000000000000000);
        }

        // Sixth
        if let ElfSectionHeader::ElfSectionHeader64(section_header) = &section_headers[5] {
            assert_eq!(section_header.sh_type, ElfSectionType::StrTab);
            assert_eq!(section_header.flags, Elf64SectionFlags::empty());
            assert_eq!(section_header.addr, 0x0000000000000000);
            assert_eq!(section_header.offset, 0x00002126);
            assert_eq!(section_header.size, 0x0000000000000027);
            assert_eq!(section_header.link, 0);
            assert_eq!(section_header.info, 0);
            assert_eq!(section_header.addr_align, 1);
            assert_eq!(section_header.ent_size, 0x0000000000000000);
        }
    }
}
