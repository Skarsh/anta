// TODO Remove deacitvated lints
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(clippy::upper_case_acronyms)]

use super::{error::ElfParseError, types::*};

pub const ELFMAG0: u8 = 0x7f;
pub const ELFMAG1: u8 = 0x45;
pub const ELFMAG2: u8 = 0x4c;
pub const ELFMAG3: u8 = 0x46;

pub const EI_MAG0_IDX: usize = 0;
pub const EI_MAG1_IDX: usize = 1;
pub const EI_MAG2_IDX: usize = 2;
pub const EI_MAG3_IDX: usize = 3;
pub const EI_CLASS_IDX: usize = 4;
pub const EI_DATA_IDX: usize = 5;
pub const EI_VERSION_IDX: usize = 6;
pub const EI_OSABI_IDX: usize = 7;
pub const EI_ABIVERSION_IDX: usize = 8;
pub const EI_PAD_IDX: usize = 9;
pub const EI_NIDENT: usize = 16;

pub const ELF_IDENT_PAD_SIZE: usize = 7;

#[derive(Debug)]
#[repr(u8)]
pub enum Class {
    ElfClassNone = 0,
    ElfClass32 = 1,
    ElfClass64 = 2,
}

impl TryFrom<u8> for Class {
    type Error = ElfParseError;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Class::ElfClassNone),
            1 => Ok(Class::ElfClass32),
            2 => Ok(Class::ElfClass64),
            _ => Err(ElfParseError::InvalidElfClass),
        }
    }
}

#[derive(Debug)]
#[repr(u8)]
pub enum Data {
    ElfDataNone = 0,
    ElfData2Lsb = 1,
    ElfData2Msb = 2,
}

impl TryFrom<u8> for Data {
    type Error = ElfParseError;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Data::ElfDataNone),
            1 => Ok(Data::ElfData2Lsb),
            2 => Ok(Data::ElfData2Msb),
            _ => Err(ElfParseError::InvalidElfData),
        }
    }
}

/// The version number of the ELF specification
/// Currently this must be EVCurrent
#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Version {
    None = 0,
    Current = 1,
}

#[derive(Debug)]
#[repr(u8)]
enum OsAbi {
    /// No extension or unspecified
    None = 0,
    /// Hewlett-Packard HP-UX
    HPUX = 1,
    /// NetBSD
    NetBSD = 2,
    /// Linux
    Linux = 3,
    /// Sun Solaris
    Solaris = 6,
    /// AIX
    AIX = 7,
    /// IRIX
    IRIX = 8,
    /// FreeBSD
    FreeBSD = 9,
    /// Compaq Tru64 UNIX
    Tru64 = 10,
    /// Novell Modesto
    Modesto = 11,
    /// OpenBSD
    OpenBSD = 12,
    /// OpenVMS
    OpenVMS = 13,
    /// Hewlett-Packard Non-Stop Kernel
    NSK = 14,
}

#[derive(Debug)]
#[repr(C)]
pub struct Ident {
    mag0: ElfByte,
    mag1: ElfByte,
    mag2: ElfByte,
    mag3: ElfByte,
    class: Class,
    data: Data,
    version: Version,
    osabi: OsAbi,
    abi_version: ElfByte,
    pad: [u8; ELF_IDENT_PAD_SIZE],
}

#[derive(Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum ElfType {
    None = 0,
    Rel = 1,
    Exec = 2,
    Dyn = 3,
    Core = 4,
    LoOs = 0xfee,
    HiOs = 0xfeff,
    LoProc = 0xff00,
    HiProc = 0xffff,
}

#[derive(Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Machine {
    /// No machine
    NONE = 0,
    /// AT&T WE 32100
    M32 = 1,
    /// SPARC
    SPARC = 2,
    /// Intel 80386
    EM386 = 3,
    /// Motorola 68000
    EM68K = 4,
    /// Motorola 88000
    EM88K = 5,
    /// Intel 80860
    EM860 = 7,
    /// MIPS I Architecture
    MIPS = 8,
    /// IBM System/370 Processor
    S370 = 9,
    /// MIPS RS3000 Little-endian
    MIPSRS3LE = 10,
    /// Hewlett-Packard PA-RISC
    PARISC = 15,
    /// Fujitsu VPP500
    VPP500 = 17,
    /// Enhanced instruction set SPARC
    SPARC32PLUS = 18,
    /// Intel 80960
    EM960 = 19,
    /// PowerPC
    PPC = 20,
    /// 64-bit PowerPC
    PPC64 = 21,
    /// IBM System/390 Processor
    S390 = 22,
    /// NEC V800
    V800 = 36,
    /// Fujitsu FR20
    FR20 = 37,
    /// TRW RH-32
    RH32 = 38,
    /// Motorola RCE
    RCE = 39,
    /// Advanced RISC Machines ARM
    ARM = 40,
    /// Digital Alpha
    ALPHA = 41,
    /// Hitachi SH
    SH = 42,
    /// SPARC Version 9
    SPARCV9 = 43,
    /// Siemens TriCore embedded processor
    TRICORE = 44,
    /// Argonaut RISC Core, Argonaut Technologies Inc.
    ARC = 45,
    /// Hitachi H8/300
    H8_300 = 46,
    /// Hitachi H8/300H
    H8_300H = 47,
    /// Hitachi H8S
    H8S = 48,
    /// Hitachi H8/500
    H8_500 = 49,
    /// Intel IA-64 processor architecture
    IA64 = 50,
    /// Stanford MIPS-X
    MIPSX = 51,
    /// Motorola ColdFire
    COLDFIRE = 52,
    /// Motorola M68HC12
    EM68HC12 = 53,
    /// Fujitsu MMA Multimedia Accelerator
    MMA = 54,
    /// Siemens PCP
    PCP = 55,
    /// Sony nCPU embedded RISC processor
    NCPU = 56,
    /// Denso NDR1 microprocessor
    NDR1 = 57,
    /// Motorola Star*Core processor
    STARCORE = 58,
    /// Toyota ME16 processor
    ME16 = 59,
    /// STMicroelectronics ST100 processor
    ST100 = 60,
    /// Advanced Logic Corp. TinyJ embedded processor family
    TINYJ = 61,
    /// AMD x86-64 architecture
    X86_64 = 62,
    /// Sony DSP Processor
    PDSP = 63,
    /// Digital Equipment Corp. PDP-10
    PDP10 = 64,
    /// Digital Equipment Corp. PDP-11
    PDP11 = 65,
    /// Siemens FX66 microcontroller
    FX66 = 66,
    /// STMicroelectronics ST9+ 8/16 bit microcontroller
    ST9PLUS = 67,
    /// STMicroelectronics ST7 8-bit microcontroller
    ST7 = 68,
    /// Motorola MC68HC16 Microcontroller
    EM68HC16 = 69,
    /// Motorola MC68HC11 Microcontroller
    EM68HC11 = 70,
    /// Motorola MC68HC08 Microcontroller
    EM68HC08 = 71,
    /// Motorola MC68HC05 Microcontroller
    EM68HC05 = 72,
    /// Silicon Graphics SVx
    SVX = 73,
    /// STMicroelectronics ST19 8-bit microcontroller
    ST19 = 74,
    /// Digital VAX
    VAX = 75,
    /// Axis Communications 32-bit embedded processor
    CRIS = 76,
    /// Infineon Technologies 32-bit embedded processor
    JAVELIN = 77,
    /// Element 14 64-bit DSP Processor
    FIREPATH = 78,
    /// LSI Logic 16-bit DSP Processor
    ZSP = 79,
    /// Donald Knuth's educational 64-bit processor
    MMIX = 80,
    /// Harvard University machine-independent object files
    HUANY = 81,
    /// SiTera Prism
    PRISM = 82,
    /// Atmel AVR 8-bit microcontroller
    AVR = 83,
    /// Fujitsu FR30
    FR30 = 84,
    /// Mitsubishi D10V
    D10V = 85,
    /// Mitsubishi D30V
    D30V = 86,
    /// NEC v850
    V850 = 87,
    /// Mitsubishi M32R
    M32R = 88,
    /// Matsushita MN10300
    MN10300 = 89,
    /// Matsushita MN10200
    MN10200 = 90,
    /// picoJava
    PJ = 91,
    /// OpenRISC 32-bit embedded processor
    OPENRISC = 92,
    /// ARC Cores Tangent-A5
    ARCA5 = 93,
    /// Tensilica Xtensa Architecture
    XTENSA = 94,
    /// Alphamosaic VideoCore processor
    VIDEOCORE = 95,
    /// Thompson Multimedia General Purpose Processor
    TMMGPP = 96,
    /// National Semiconductor 32000 series
    NS32K = 97,
    /// Tenor Network TPC processor
    TPC = 98,
    /// Trebia SNP 1000 processor
    SNP1K = 99,
    /// STMicroelectronics ST200 microcontroller
    ST200 = 100,
}

// TODO: pub or private access for fields?
#[derive(Debug)]
#[repr(C)]
pub struct Elf32Ehdr {
    pub ident: Ident,
    pub elf_type: ElfType,
    pub machine: Machine,
    pub version: Version,
    pub entry: Elf32Addr,
    pub ph_off: Elf32Off,
    pub sh_off: Elf32Off,
    pub flags: Elf32Word,
    pub eh_size: Elf32Half,
    pub ph_ent_size: Elf32Half,
    pub ph_num: Elf32Half,
    pub sh_ent_size: Elf32Half,
    pub sh_num: Elf32Half,
    pub sh_str_ndx: Elf32Half,
}

// TODO: pub or private access for fields?
#[derive(Debug)]
#[repr(C)]
pub struct Elf64Ehdr {
    pub ident: Ident,
    pub elf_type: ElfType,
    pub machine: Machine,
    pub version: Version,
    pub entry: Elf64Addr,
    pub ph_off: Elf64Off,
    pub sh_off: Elf64Off,
    pub flags: Elf64Word,
    pub eh_size: Elf64Half,
    pub ph_ent_size: Elf64Half,
    pub ph_num: Elf64Half,
    pub sh_ent_size: Elf64Half,
    pub sh_num: Elf64Half,
    pub sh_str_ndx: Elf64Half,
}

#[derive(Debug)]
pub enum ElfHeader<'a> {
    Elf32(&'a Elf32Ehdr),
    Elf64(&'a Elf64Ehdr),
}

/// Checks whether the EIDENT bytes has valid values
/// for its size, magic bytes and elf specification version
pub fn validate_elf_ident(elf_ident: &Ident) -> bool {
    let valid_size = std::mem::size_of_val(elf_ident) == EI_NIDENT;

    let valid_magic = elf_ident.mag0 == ELFMAG0
        && elf_ident.mag1 == ELFMAG1
        && elf_ident.mag2 == ELFMAG2
        && elf_ident.mag3 == ELFMAG3;

    let valid_version = elf_ident.version == Version::Current;

    valid_size && valid_magic && valid_version
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn check_size_of_ident_struct() {
        let ident = Ident {
            mag0: ELFMAG0,
            mag1: ELFMAG1,
            mag2: ELFMAG2,
            mag3: ELFMAG3,
            class: Class::ElfClass64,
            data: Data::ElfData2Lsb,
            version: Version::Current,
            osabi: OsAbi::Linux,
            abi_version: 1,
            pad: [0; ELF_IDENT_PAD_SIZE],
        };

        assert_eq!(std::mem::size_of::<Ident>(), EI_NIDENT);
        assert_eq!(std::mem::size_of_val(&ident), EI_NIDENT);
    }

    #[test]
    fn parse_ident_from_elf_file() {
        let mut f = File::open("samples/bin/hello").unwrap();
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer).unwrap();

        let (head, body, tail) = unsafe { buffer.align_to::<Ident>() };
        assert!(head.is_empty(), "Data was not aligned");
        let elf_ident = &body[0];

        assert!(validate_elf_ident(elf_ident));
    }

    #[test]
    fn parse_elf64_header() {
        let mut f = File::open("samples/bin/hello").unwrap();
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer).unwrap();

        let (head, body, tail) = unsafe { buffer.align_to::<Elf64Ehdr>() };
        assert!(head.is_empty(), "Data was not aligned");
        let elf_64_ehdr = &body[0];
        assert!(validate_elf_ident(&elf_64_ehdr.ident));

        assert_eq!(elf_64_ehdr.elf_type, ElfType::Exec);
        assert_eq!(elf_64_ehdr.machine, Machine::X86_64);

        println!("{:x?}", elf_64_ehdr);
    }
}
