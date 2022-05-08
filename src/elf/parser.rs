use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Type {
    None = 0x0,
    Rel = 0x1,
    Exec = 0x2,
    Dyn = 0x3,
    Core = 0x4,
}

impl Type {
    pub fn from_u16(r#type: u16) -> Option<Self> {
        match r#type {
            0 => Some(Self::None),
            1 => Some(Self::Rel),
            2 => Some(Self::Exec),
            3 => Some(Self::Dyn),
            4 => Some(Self::Core),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Machine {
    X86 = 0x03,
    X86_64 = 0x3e,
}

impl Machine {
    pub fn from_u16(machine: u16) -> Option<Self> {
        match machine {
            0x03 => Some(Self::X86),
            0x3e => Some(Self::X86_64),
            _ => None,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct ElfFile {
    r#type: Option<Type>,
    machine: Option<Machine>,
}

#[derive(Debug)]
pub enum ElfParseError {
    InvalidMagicBytes,
}

#[allow(dead_code)]
impl ElfFile {
    const MAGIC: &'static [u8] = &[0x7f, 0x45, 0x4c, 0x46];
    const TYPE_OFFSET: usize = 16;
    const MACHINE_OFFSET: usize = 18;

    pub fn new(r#type: Option<Type>, machine: Option<Machine>) -> Self {
        Self { r#type, machine }
    }

    pub fn read_file(file_path: &Path) -> io::Result<Vec<u8>> {
        let mut f = File::open(file_path).unwrap();
        let mut buffer = Vec::new();

        f.read_to_end(&mut buffer)?;
        Ok(buffer)
    }

    pub fn parse(buffer: Vec<u8>) -> Result<ElfFile, ElfParseError> {
        let magic_bytes = &buffer[..ElfFile::MAGIC.len()];
        let valid_magic_bytes = ElfFile::validate_magic(magic_bytes);
        if !valid_magic_bytes {
            return Err(ElfParseError::InvalidMagicBytes);
        }

        // TODO: Check endianness before parsing
        let type_bytes = &buffer[ElfFile::TYPE_OFFSET..ElfFile::MACHINE_OFFSET];
        // TODO: Error handling here
        let r#type = Type::from_u16(u16::from_le_bytes(type_bytes.try_into().unwrap()));

        let machine_bytes = &buffer[ElfFile::MACHINE_OFFSET..ElfFile::MACHINE_OFFSET + 2];
        // TODO: Error handling here
        let machine = Machine::from_u16(u16::from_le_bytes(machine_bytes.try_into().unwrap()));

        Ok(ElfFile::new(r#type, machine))
    }

    pub fn validate_magic(magic_bytes: &[u8]) -> bool {
        magic_bytes == ElfFile::MAGIC
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_magic_bytes() {
        let file_path = Path::new("./samples/bin/hello");
        let buffer = ElfFile::read_file(file_path).unwrap();
        let magic_bytes = &buffer[..ElfFile::MAGIC.len()];
        let valid_magic_bytes = ElfFile::validate_magic(magic_bytes);
        assert!(valid_magic_bytes);
    }

    #[test]
    fn type_from_u16() {
        assert_eq!(Type::from_u16(0x0), Some(Type::None));
        assert_eq!(Type::from_u16(0x1), Some(Type::Rel));
        assert_eq!(Type::from_u16(0x2), Some(Type::Exec));
        assert_eq!(Type::from_u16(0x3), Some(Type::Dyn));
        assert_eq!(Type::from_u16(0x4), Some(Type::Core));
        assert_eq!(Type::from_u16(0xdead), None);
    }

    #[test]
    fn parse_elf_file() {
        let file_path = Path::new("./samples/bin/hello");
        let buffer = ElfFile::read_file(file_path).unwrap();
        let elf_file = ElfFile::parse(buffer).unwrap();

        assert_eq!(elf_file.r#type.unwrap(), Type::Exec);
        assert_eq!(elf_file.machine.unwrap(), Machine::X86_64);
    }
}
