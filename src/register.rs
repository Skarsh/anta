use nix::{unistd::Pid, sys::ptrace};


//pub const REGISTERS: [RegisterKind; 27] = [
//    RegisterKind::R15(15, "r15"),
//    RegisterKind::R14(14, "r14"),
//    RegisterKind::R13(13, "r13"),
//    RegisterKind::R12(12, "r12"),
//    RegisterKind::Rbp(6, "rbp"),
//    RegisterKind::Rbx(3, "rbx"),
//    RegisterKind::R11(11, "r11"),
//    RegisterKind::R10(10, "r10"),
//    RegisterKind::R9(9, "r9"),
//    RegisterKind::R8(8, "r8"),
//    RegisterKind::Rax(0, "rax"),
//    RegisterKind::Rcx(2, "rcx"),
//    RegisterKind::Rdx(1, "rdx"),
//    RegisterKind::Rsi(4, "rsi"),
//    RegisterKind::Rdi(5, "rdi"),
//    RegisterKind::OrigRax(-1, "orig_rax"),
//    RegisterKind::Rip(-1, "rip"),
//    RegisterKind::Cs(51, "cs"),
//    RegisterKind::Rflags(49, "eflags"),
//    RegisterKind::Rsp(7, "rsp"),
//    RegisterKind::Ss(52, "ss"),
//    RegisterKind::FsBase(58, "fs_base"),
//    RegisterKind::GsBase(59, "gs_base"),
//    RegisterKind::Ds(53, "ds"),
//    RegisterKind::Es(50, "es"),
//    RegisterKind::Fs(54, "fs"),
//    RegisterKind::Gs(55, "gs")
//];

/// Register dwarf numbering
/// rip: -1 </br>
/// orig_rax: -1 </br>
/// rax: 0 </br>
/// rdx: 1 </br>
/// rcx: 2 </br>
/// rbx: 3 </br>
/// rsi: 4 </br>
/// rdi: 5 </br>
/// rbp: 6 </br>
/// rsp: 7 </br>
/// r8: 8 </br>
/// r9: 9 </br>
/// r10: 10 </br>
/// r11: 11 </br>
/// r12: 12 </br>
/// r13: 13 </br>
/// r14: 14 </br>
/// r15: 15 </br>
/// eflags: 49 </br>
/// es: 50 </br>
/// cs: 51 </br>
/// ss: 52 </br>
/// ds: 53 </br>
/// fs: 54 </br>
/// gs: 55 </br>
/// fs_base: 58 </br>
/// gs_base: 59 </br>


pub const REGISTERS: [Register; 27] = [
    Register{reg_kind: RegisterKind::R15, dwarf_reg: 15, name: "r15"}, // 0
    Register{reg_kind: RegisterKind::R14, dwarf_reg: 14, name: "r14"}, // 1
    Register{reg_kind: RegisterKind::R13, dwarf_reg: 13, name: "r13"}, // 2
    Register{reg_kind: RegisterKind::R12, dwarf_reg: 12, name: "r12"}, // 3
    Register{reg_kind: RegisterKind::Rbp, dwarf_reg: 6, name: "rbp"}, // 4
    Register{reg_kind: RegisterKind::Rbx, dwarf_reg: 3, name: "rbx"}, // 5
    Register{reg_kind: RegisterKind::R11, dwarf_reg: 11, name: "r11"}, // 6
    Register{reg_kind: RegisterKind::R10, dwarf_reg: 10, name: "r10"}, // 7
    Register{reg_kind: RegisterKind::R9, dwarf_reg: 9, name: "r9"}, // 8
    Register{reg_kind: RegisterKind::R8, dwarf_reg: 8, name: "r8"}, // 9
    Register{reg_kind: RegisterKind::Rax, dwarf_reg: 0, name: "rax"}, // 10
    Register{reg_kind: RegisterKind::Rcx, dwarf_reg: 2, name: "rcx"}, // 11
    Register{reg_kind: RegisterKind::Rdx, dwarf_reg: 1, name: "rdx"}, // 12
    Register{reg_kind: RegisterKind::Rsi, dwarf_reg: 4, name: "rsi"}, // 13
    Register{reg_kind: RegisterKind::Rdi, dwarf_reg: 5, name: "rdi"}, // 14
    Register{reg_kind: RegisterKind::OrigRax, dwarf_reg: -1, name: "orig_rax"}, // 15
    Register{reg_kind: RegisterKind::Rip, dwarf_reg: -1, name: "rip"}, // 16
    Register{reg_kind: RegisterKind::Cs, dwarf_reg: 51, name: "cs"}, // 17
    Register{reg_kind: RegisterKind::Rflags, dwarf_reg: 49, name: "eflags"}, // 18
    Register{reg_kind: RegisterKind::Rsp, dwarf_reg: 7, name: "rsp"}, // 19
    Register{reg_kind: RegisterKind::Ss, dwarf_reg: 52, name: "ss"}, // 20
    Register{reg_kind: RegisterKind::FsBase, dwarf_reg: 58, name: "fs_base"}, // 21
    Register{reg_kind: RegisterKind::GsBase, dwarf_reg: 59, name: "gs_base"}, // 22
    Register{reg_kind: RegisterKind::Ds, dwarf_reg: 53, name: "ds"}, // 23 
    Register{reg_kind: RegisterKind::Es, dwarf_reg: 50, name: "es"}, // 24
    Register{reg_kind: RegisterKind::Fs, dwarf_reg: 54, name: "fs"}, // 25
    Register{reg_kind: RegisterKind::Gs, dwarf_reg: 55, name: "gs"} // 26
];

pub struct Register<'a> {
    reg_kind: RegisterKind,
    dwarf_reg: i32,
    name: &'a str
}

impl<'a> Default for Register<'a> {
    fn default() -> Self {
        Self {
            reg_kind: RegisterKind::Rax,
            dwarf_reg: 0,
            name: "rax"
        }
    }
}

/// Represents a register
/// 
/// The 0th field represents a dwarf register by reg number </br>
/// The 1th field represents the name of the register
pub enum RegisterKind {
    Rax,
    Rbx,
    Rcx,
    Rdx,
    Rdi,
    Rsi,
    Rbp,
    Rsp,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
    Rip,
    Rflags,
    Cs,
    OrigRax,
    FsBase,
    GsBase,
    Fs,
    Gs,
    Ss,
    Ds,
    Es,
}

fn get_register_value(pid: Pid, reg: &Register) -> u64 {
    let regs = ptrace::getregs(pid).expect("Faied to getregs");

    match reg.reg_kind {
        RegisterKind::Rax => regs.rax,
        RegisterKind::Rbx => regs.rbx,
        RegisterKind::Rcx => regs.rcx,
        RegisterKind::Rdx => regs.rdx,
        RegisterKind::Rdi => regs.rdi,
        RegisterKind::Rsi => regs.rsi,
        RegisterKind::Rbp => regs.rbp,
        RegisterKind::Rsp => regs.rsp,
        RegisterKind::R8 => regs.r8,
        RegisterKind::R9 => regs.r9,
        RegisterKind::R10 => regs.r10,
        RegisterKind::R11 => regs.r11,
        RegisterKind::R12 => regs.r12,
        RegisterKind::R13 => regs.r13,
        RegisterKind::R14 => regs.r14,
        RegisterKind::R15 => regs.r15,
        RegisterKind::Rip => regs.rip,
        RegisterKind::Rflags => regs.eflags,
        RegisterKind::Cs => regs.cs,
        RegisterKind::OrigRax => regs.orig_rax,
        RegisterKind::FsBase => regs.fs_base,
        RegisterKind::GsBase => regs.gs_base,
        RegisterKind::Fs => regs.fs,
        RegisterKind::Gs => regs.gs,
        RegisterKind::Ss => regs.ss,
        RegisterKind::Ds => regs.ds,
        RegisterKind::Es => regs.es,
    }
}

// TODO: Should this return a Result?
fn set_register_value(pid: Pid, reg: RegisterKind, value: u64) {
    let mut regs = ptrace::getregs(pid).expect("Faied to getregs");

    match reg {
        RegisterKind::Rax => regs.rax = value,
        RegisterKind::Rbx => regs.rbx = value,
        RegisterKind::Rcx => regs.rcx = value,
        RegisterKind::Rdx => regs.rdx = value,
        RegisterKind::Rdi => regs.rdi = value,
        RegisterKind::Rsi => regs.rsi = value,
        RegisterKind::Rbp => regs.rbp = value,
        RegisterKind::Rsp => regs.rsp = value,
        RegisterKind::R8 => regs.r8 = value,
        RegisterKind::R9 => regs.r9 = value,
        RegisterKind::R10 => regs.r10 = value,
        RegisterKind::R11 => regs.r11 = value,
        RegisterKind::R12 => regs.r12 = value,
        RegisterKind::R13 => regs.r13 = value,
        RegisterKind::R14 => regs.r14 = value,
        RegisterKind::R15 => regs.r15 = value,
        RegisterKind::Rip => regs.rip = value,
        RegisterKind::Rflags => regs.eflags = value,
        RegisterKind::Cs => regs.cs = value,
        RegisterKind::OrigRax => regs.orig_rax = value,
        RegisterKind::FsBase => regs.fs_base = value,
        RegisterKind::GsBase => regs.gs_base = value,
        RegisterKind::Fs => regs.fs = value,
        RegisterKind::Gs => regs.gs = value,
        RegisterKind::Ss => regs.ss = value,
        RegisterKind::Ds => regs.ds = value,
        RegisterKind::Es => regs.es = value,
    }

    ptrace::setregs(pid, regs).expect("Failed to setregs");
}

fn get_register_value_from_dwarf_register(pid: Pid, reg_num: i32) -> u64 {
    
    let mut reg_value = 0;
    let mut found_reg = false;
    for reg in REGISTERS {
        if reg.dwarf_reg == reg_num {
            reg_value = get_register_value(pid, &reg);
            found_reg = true;
        }
    };
    
    // The register has to be found during the for loop.
    // If not an illegal reg num has been given, and we need to error
    // TODO: Proper error handling instead of just panicing like this will
    assert!(found_reg);
    reg_value
}

fn get_register_name<'a>(reg: &'a Register) -> &'a str {
    reg.name
}

fn get_register_from_name<'a>(name: &'a str) -> Register<'a> {
    let mut ret_reg = Register::default();
    let mut found_reg = false;
    for reg in REGISTERS {
        if reg.name == name {
            ret_reg = reg;
            found_reg = true
        }
    };

    assert!(found_reg);

    ret_reg
}
