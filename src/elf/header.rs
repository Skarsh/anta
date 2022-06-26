// TODO Remove deacitvated lints
#![allow(unused_variables)]
#![allow(dead_code)]

type ElfByte = u8;
type Elf32Addr = u32;
type Elf32Off = u32;
type Elf32Section = u32;
type Elf32Versym = u32;
type Elf32Half = u16;
type Elf32Sword = u32;
type Elf32Word = u32;
type Elf32Sxword = i64;
type Elf32Xword = u64;

type Elf64Addr = u64;
type Elf64Off = u64;
type Elf64Section = u32;
type Elf64Versym = u32;
type Elf64Half = u16;
type Elf64Sword = u32;
type Elf64Word = u32;
type Elf64Sxword = i64;
type Elf64Xword = u64;

const ELFMAG0: u8 = 0x7f;
const ELFMAG1: u8 = 0x45;
const ELFMAG2: u8 = 0x4c;
const ELFMAG3: u8 = 0x46;

const EI_MAG0: usize = 0;
const EI_MAG1: usize = 1;
const EI_MAG2: usize = 2;
const EI_MAG3: usize = 3;
const EI_CLASS: usize = 4;
const EI_DATA: usize = 5;
const EI_VERSION: usize = 6;
const EI_OSABI: usize = 7;
const EI_ABIVERSION: usize = 8;
const EI_PAD: usize = 9;
const EI_NIDENT: usize = 16;

const ELF_IDENT_PAD_SIZE: usize = 7;

#[derive(Debug)]
#[repr(u8)]
enum EIClass {
    ElfClassNone = 0,
    ElfClass32 = 1,
    ElfClass64 = 2,
}

#[derive(Debug)]
#[repr(u8)]
enum EIData {
    ElfDataNone = 0,
    ElfData2LSB = 1,
    ElfData2MSB = 2,
}

/// The version number of the ELF specification
/// Currently this must be EVCurrent
#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
enum EIVersion {
    EvNone,
    EvCurrent,
}

#[derive(Debug)]
enum EIOSABI {
    /// No extension or unspecified
    ElfOSABINone = 0,
    /// Hewlett-Packard HP-UX
    ElfOSABIHpux = 1,
    /// NetBSD
    ElfOSABINetBSD = 2,
    /// Linux
    ElfOSABILinux = 3,
    /// Sun Solaris
    ElfOSABISolaris = 6,
    /// AIX
    ElfOSABIAix = 7,
    /// IRIX
    ElfOSABIIrix = 8,
    /// FreeBSD
    ElfOSABIFreeBSD = 9,
    /// Compaq Tru64 UNIX
    ElfOSABITru64 = 10,
    /// Novell Modesto
    ElfOSABIModesto = 11,
    /// OpenBSD
    ElfOSABIOpenBSD = 12,
    /// OpenVMS
    ElfOSABIOpenVMS = 13,
    /// Hewlett-Packard Non-Stop Kernel
    ElfOSABINsk = 14,
}

#[derive(Debug)]
#[repr(C)]
pub struct ElfIdent {
    ei_mag0: ElfByte,
    ei_mag1: ElfByte,
    ei_mag2: ElfByte,
    ei_mag3: ElfByte,
    ei_class: EIClass,
    ei_data: EIData,
    ei_version: EIVersion,
    ei_osabi: EIOSABI,
    ei_abi_version: ElfByte,
    ei_pad: [u8; ELF_IDENT_PAD_SIZE],
}

#[repr(C)]
pub struct Elf32Ehdr {
    e_ident: ElfIdent,
    e_type: Elf32Half,
    e_machine: Elf32Half,
    e_version: Elf32Word,
    e_entry: Elf32Addr,
    e_phoff: Elf32Off,
    e_shoff: Elf32Off,
    e_flags: Elf32Word,
    e_ehsize: Elf32Half,
    e_phentsize: Elf32Half,
    e_phnum: Elf32Half,
    e_shentsize: Elf32Half,
    e_shnum: Elf32Half,
    e_shstrndx: Elf32Half,
}

pub struct Elf64Ehdr {
    e_ident: ElfIdent,
    e_type: Elf64Half,
    e_machine: Elf64Half,
    e_version: Elf64Word,
    e_entry: Elf64Addr,
    e_phoff: Elf64Off,
    e_shoff: Elf64Off,
    e_flags: Elf64Word,
    e_ehsize: Elf64Half,
    e_phentsize: Elf64Half,
    e_phnum: Elf64Half,
    e_shentsize: Elf64Half,
    e_shnum: Elf64Half,
    e_shstrndx: Elf64Half,
}

pub enum ElfHeader {
    Elf32(Elf32Ehdr),
    Elf64(Elf64Ehdr),
}

#[allow(clippy::upper_case_acronyms)]
pub enum ElfType {
    ETNone = 0,
    ETRel = 1,
    ETExec = 2,
    ETDyn = 3,
    ETCore = 4,
    ETLOOS = 0xfee,
    ETHIOS = 0xfeff,
    ETLOProc = 0xff00,
    ETHIProc = 0xffff,
}

#[allow(clippy::upper_case_acronyms)]
pub enum ElfMachine {
    /// No machine
    EMNONE = 0,
    /// AT&T WE 32100
    EMM32 = 1,
    /// SPARC
    EMSPARC = 2,
    /// Intel 80386
    EM386 = 3,
    /// Motorola 68000
    EM68K = 4,
    /// Motorola 88000
    EM88K = 5,
    /// Intel 80860
    EM860 = 7,
    /// MIPS I Architecture
    EMMIPS = 8,
    /// IBM System/370 Processor
    EMS370 = 9,
    /// MIPS RS3000 Little-endian
    EMMIPSRS3LE = 10,
    /// Hewlett-Packard PA-RISC
    EMPARISC = 15,
    /// Fujitsu VPP500
    EMVPP500 = 17,
    /// Enhanced instruction set SPARC
    EMSPARC32PLUS = 18,
    /// Intel 80960
    EM960 = 19,
    /// PowerPC
    EMPPC = 20,
    /// 64-bit PowerPC
    EMPPC64 = 21,
    /// IBM System/390 Processor
    EMS390 = 22,
    /// NEC V800
    EMV800 = 36,
    /// Fujitsu FR20
    EMFR20 = 37,
    /// TRW RH-32
    EMRH32 = 38,
    /// Motorola RCE
    EMRCE = 39,
    /// Advanced RISC Machines ARM
    EMARM = 40,
    /// Digital Alpha
    EMALPHA = 41,
    /// Hitachi SH
    EMSH = 42,
    /// SPARC Version 9
    EMSPARCV9 = 43,
    /// Siemens TriCore embedded processor
    EMTRICORE = 44,
    /// Argonaut RISC Core, Argonaut Technologies Inc.
    EMARC = 45,
    /// Hitachi H8/300
    EMH8_300 = 46,
    /// Hitachi H8/300H
    EMH8_300H = 47,
    /// Hitachi H8S
    EMH8S = 48,
    /// Hitachi H8/500
    EMH8_500 = 49,
    /// Intel IA-64 processor architecture
    EMIA64 = 50,
    /// Stanford MIPS-X
    EMMIPSX = 51,
    /// Motorola ColdFire
    EMCOLDFIRE = 52,
    /// Motorola M68HC12
    EM68HC12 = 53,
    /// Fujitsu MMA Multimedia Accelerator
    EMMMA = 54,
    /// Siemens PCP
    EMPCP = 55,
    /// Sony nCPU embedded RISC processor
    EMNCPU = 56,
    /// Denso NDR1 microprocessor
    EMNDR1 = 57,
    /// Motorola Star*Core processor
    EMSTARCORE = 58,
    /// Toyota ME16 processor
    EMME16 = 59,
    /// STMicroelectronics ST100 processor
    EMST100 = 60,
    /// Advanced Logic Corp. TinyJ embedded processor family
    EMTINYJ = 61,
    /// AMD x86-64 architecture
    EMX86_64 = 62,
    /// Sony DSP Processor
    EMPDSP = 63,
    /// Digital Equipment Corp. PDP-10
    EMPDP10 = 64,
    /// Digital Equipment Corp. PDP-11
    EMPDP11 = 65,
    /// Siemens FX66 microcontroller
    EMFX66 = 66,
    /// STMicroelectronics ST9+ 8/16 bit microcontroller
    EMST9PLUS = 67,
    /// STMicroelectronics ST7 8-bit microcontroller
    EMST7 = 68,
    /// Motorola MC68HC16 Microcontroller
    EM68HC16 = 69,
    /// Motorola MC68HC11 Microcontroller
    EM68HC11 = 70,
    /// Motorola MC68HC08 Microcontroller
    EM68HC08 = 71,
    /// Motorola MC68HC05 Microcontroller
    EM68HC05 = 72,
    /// Silicon Graphics SVx
    EMSVX = 73,
    /// STMicroelectronics ST19 8-bit microcontroller
    EMST19 = 74,
    /// Digital VAX
    EMVAX = 75,
    /// Axis Communications 32-bit embedded processor
    EMCRIS = 76,
    /// Infineon Technologies 32-bit embedded processor
    EMJAVELIN = 77,
    /// Element 14 64-bit DSP Processor
    EMFIREPATH = 78,
    /// LSI Logic 16-bit DSP Processor
    EMZSP = 79,
    /// Donald Knuth's educational 64-bit processor
    EMMMIX = 80,
    /// Harvard University machine-independent object files
    EMHUANY = 81,
    /// SiTera Prism
    EMPRISM = 82,
    /// Atmel AVR 8-bit microcontroller
    EMAVR = 83,
    /// Fujitsu FR30
    EMFR30 = 84,
    /// Mitsubishi D10V
    EMD10V = 85,
    /// Mitsubishi D30V
    EMD30V = 86,
    /// NEC v850
    EMV850 = 87,
    /// Mitsubishi M32R
    EMM32R = 88,
    /// Matsushita MN10300
    EMMN10300 = 89,
    /// Matsushita MN10200
    EMMN10200 = 90,
    /// picoJava
    EMPJ = 91,
    /// OpenRISC 32-bit embedded processor
    EMOPENRISC = 92,
    /// ARC Cores Tangent-A5
    EMARCA5 = 93,
    /// Tensilica Xtensa Architecture
    EMXTENSA = 94,
    /// Alphamosaic VideoCore processor
    EMVIDEOCORE = 95,
    /// Thompson Multimedia General Purpose Processor
    EMTMMGPP = 96,
    /// National Semiconductor 32000 series
    EMNS32K = 97,
    /// Tenor Network TPC processor
    EMTPC = 98,
    /// Trebia SNP 1000 processor
    EMSNP1K = 99,
    /// STMicroelectronics ST200 microcontroller
    EMST200 = 100,
}

/// Checks whether the EIDENT bytes has valid values
/// for its size, magic bytes and elf specification version
pub fn validate_elf_ident(elf_ident: &ElfIdent) -> bool {
    let valid_size = std::mem::size_of_val(elf_ident) == EI_NIDENT;

    let valid_magic = elf_ident.ei_mag0 == ELFMAG0
        && elf_ident.ei_mag1 == ELFMAG1
        && elf_ident.ei_mag2 == ELFMAG2
        && elf_ident.ei_mag3 == ELFMAG3;

    let valid_version = elf_ident.ei_version == EIVersion::EvCurrent;

    valid_size && valid_magic && valid_version
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn check_size_of_ident_struct() {
        let ident = ElfIdent {
            ei_mag0: 0x7f,
            ei_mag1: 0x45,
            ei_mag2: 0x4c,
            ei_mag3: 0x46,
            ei_class: EIClass::ElfClass64,
            ei_data: EIData::ElfData2LSB,
            ei_version: EIVersion::EvCurrent,
            ei_osabi: EIOSABI::ElfOSABILinux,
            ei_abi_version: 1,
            ei_pad: [0; ELF_IDENT_PAD_SIZE],
        };

        assert_eq!(std::mem::size_of::<ElfIdent>(), EI_NIDENT);
        assert_eq!(std::mem::size_of_val(&ident), EI_NIDENT);
    }

    #[test]
    fn parse_ident_from_elf_file() {
        let mut f = File::open("samples/bin/hello").unwrap();
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer).unwrap();

        let (head, body, tail) = unsafe { buffer.align_to::<ElfIdent>() };
        assert!(head.is_empty(), "Data was not aligned");
        let elf_ident = &body[0];

        assert!(validate_elf_ident(elf_ident));
    }
}
