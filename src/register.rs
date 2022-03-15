use nix::{unistd::Pid, sys::ptrace};


pub const REGISTERS: [Register; 27] = [
    Register::R15(15, "r15"),
    Register::R14(14, "r14"),
    Register::R13(13, "r13"),
    Register::R12(12, "r12"),
    Register::Rbp(6, "rbp"),
    Register::Rbx(3, "rbx"),
    Register::R11(11, "r11"),
    Register::R10(10, "r10"),
    Register::R9(9, "r9"),
    Register::R8(8, "r8"),
    Register::Rax(0, "rax"),
    Register::Rcx(2, "rcx"),
    Register::Rdx(1, "rdx"),
    Register::Rsi(4, "rsi"),
    Register::Rdi(5, "rdi"),
    Register::OrigRax(-1, "orig_rax"),
    Register::Rip(-1, "rip"),
    Register::Cs(51, "cs"),
    Register::Rflags(49, "eflags"),
    Register::Rsp(7, "rsp"),
    Register::Ss(52, "ss"),
    Register::FsBase(58, "fs_base"),
    Register::GsBase(59, "gs_base"),
    Register::Ds(53, "ds"),
    Register::Es(50, "es"),
    Register::Fs(54, "fs"),
    Register::Gs(55, "gs")
];

pub enum Register<'a> {
    Rax(i32, &'a str),
    Rbx(i32, &'a str),
    Rcx(i32, &'a str),
    Rdx(i32, &'a str),
    Rdi(i32, &'a str),
    Rsi(i32, &'a str),
    Rbp(i32, &'a str),
    Rsp(i32, &'a str),
    R8(i32, &'a str),
    R9(i32, &'a str),
    R10(i32, &'a str),
    R11(i32, &'a str),
    R12(i32, &'a str),
    R13(i32, &'a str),
    R14(i32, &'a str),
    R15(i32, &'a str),
    Rip(i32, &'a str),
    Rflags(i32, &'a str),
    Cs(i32, &'a str),
    OrigRax(i32, &'a str),
    FsBase(i32, &'a str),
    GsBase(i32, &'a str),
    Fs(i32, &'a str),
    Gs(i32, &'a str),
    Ss(i32, &'a str),
    Ds(i32, &'a str),
    Es(i32, &'a str),
}

fn get_register_value(pid: Pid, reg: Register) -> u64 {
    let mut regs = ptrace::getregs(pid).expect("Faied to getregs");

    match reg {
        Register::Rax(_, _) => regs.rax,
        Register::Rbx(_, _) => regs.rbx,
        Register::Rcx(_, _) => regs.rcx,
        Register::Rdx(_, _) => regs.rdx,
        Register::Rdi(_, _) => regs.rdi,
        Register::Rsi(_, _) => regs.rsi,
        Register::Rbp(_, _) => regs.rbp,
        Register::Rsp(_, _) => regs.rsp,
        Register::R8(_, _) => regs.r8,
        Register::R9(_, _) => regs.r9,
        Register::R10(_, _) => regs.r10,
        Register::R11(_, _) => regs.r11,
        Register::R12(_, _) => regs.r12,
        Register::R13(_, _) => regs.r13,
        Register::R14(_, _) => regs.r14,
        Register::R15(_, _) => regs.r15,
        Register::Rip(_, _) => regs.rip,
        Register::Rflags(_, _) => regs.eflags,
        Register::Cs(_, _) => regs.cs,
        Register::OrigRax(_, _) => regs.orig_rax,
        Register::FsBase(_, _) => regs.fs_base,
        Register::GsBase(_, _) => regs.gs_base,
        Register::Fs(_, _) => regs.fs,
        Register::Gs(_, _) => regs.gs,
        Register::Ss(_, _) => regs.ss,
        Register::Ds(_, _) => regs.ds,
        Register::Es(_, _) => regs.es,
    }
}

// TODO: Should this return a Result?
fn set_register_value(pid: Pid, reg: Register, value: u64) {
    let mut regs = ptrace::getregs(pid).expect("Faied to getregs");

    match reg {
        Register::Rax(_, _) => regs.rax = value,
        Register::Rbx(_, _) => regs.rbx = value,
        Register::Rcx(_, _) => regs.rcx = value,
        Register::Rdx(_, _) => regs.rdx = value,
        Register::Rdi(_, _) => regs.rdi = value,
        Register::Rsi(_, _) => regs.rsi = value,
        Register::Rbp(_, _) => regs.rbp = value,
        Register::Rsp(_, _) => regs.rsp = value,
        Register::R8(_, _) => regs.r8 = value,
        Register::R9(_, _) => regs.r9 = value,
        Register::R10(_, _) => regs.r10 = value,
        Register::R11(_, _) => regs.r11 = value,
        Register::R12(_, _) => regs.r12 = value,
        Register::R13(_, _) => regs.r13 = value,
        Register::R14(_, _) => regs.r14 = value,
        Register::R15(_, _) => regs.r15 = value,
        Register::Rip(_, _) => regs.rip = value,
        Register::Rflags(_, _) => regs.eflags = value,
        Register::Cs(_, _) => regs.cs = value,
        Register::OrigRax(_, _) => regs.orig_rax = value,
        Register::FsBase(_, _) => regs.fs_base = value,
        Register::GsBase(_, _) => regs.gs_base = value,
        Register::Fs(_, _) => regs.fs = value,
        Register::Gs(_, _) => regs.gs = value,
        Register::Ss(_, _) => regs.ss = value,
        Register::Ds(_, _) => regs.ds = value,
        Register::Es(_, _) => regs.es = value,
    }

    ptrace::setregs(pid, regs).expect("Failed to setregs");
}

fn get_register_value_from_dwarf_register(pid: Pid, reg_num: i32) -> u64 {
    14
}