use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

// TODO: Remove allow
#[allow(dead_code)]
#[derive(Debug)]
enum Type {
    None,
    Rel,
    Exec,
    Dyn,
    Core,
}

// TODO: Remove allow
#[allow(dead_code)]
#[derive(Debug)]
enum Machine {
    X86 = 0x03,
    X86_64 = 0x3e,
}

#[derive(Debug)]
pub struct ElfFile {
    // Placeholder
}

// TODO: Remove allow
#[allow(dead_code)]
impl ElfFile {
    const MAGIC: &'static [u8] = &[0x7f, 0x45, 0x4c, 0x46];

    pub fn new() -> Self {
        Self {}
    }

    pub fn read_file(&self, file_path: &Path) -> io::Result<Vec<u8>> {
        let mut f = File::open(file_path).unwrap();
        let mut buffer = Vec::new();

        f.read_to_end(&mut buffer)?;
        Ok(buffer)
    }

    pub fn validate_magic(&self, buffer: Vec<u8>) -> bool {
        let elf_magic_bytes = &buffer[..ElfFile::MAGIC.len()];
        elf_magic_bytes == ElfFile::MAGIC
    }
}

impl Default for ElfFile {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_magic() {
        let file_path = Path::new("./samples/bin/hello");
        let elf_file = ElfFile::default();
        let buffer = elf_file.read_file(file_path).unwrap();
        let valid = elf_file.validate_magic(buffer);
        assert!(valid);
    }
}
