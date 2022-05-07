use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::{io, io::prelude::*, io::BufReader};

use gimli::{Dwarf, EndianSlice, RunTimeEndian};
use nix::sys::ptrace;
use nix::sys::ptrace::AddressType;
use nix::sys::wait::waitpid;
use nix::unistd::Pid;

use crate::breakpoint::Breakpoint;
use crate::command::{parse_command, Command, MemoryCommand, RegisterCommand};
use crate::register;
use crate::register::{RegisterKind, REGISTERS};

// TODO: Remove allow macro
#[allow(dead_code)]
pub struct Debugger<'a> {
    path: &'a Path,
    pid: Pid,
    running: bool,
    breakpoints: HashMap<u64, Breakpoint>,
    elf: object::File<'a>,
    dwarf: Dwarf<EndianSlice<'a, RunTimeEndian>>,
}

impl<'a> Debugger<'a> {
    pub fn new(path: &'a Path, pid: Pid, object: object::File<'a>) -> Self {
        Self {
            path,
            pid,
            running: true,
            breakpoints: HashMap::new(),
            elf: object,
            dwarf: Dwarf::default(),
        }
    }

    pub fn run(&mut self) {
        println!("Started debugging process {}", self.pid);

        waitpid(self.pid, None).unwrap();

        while self.running {
            let mut line = String::new();
            io::stdin()
                .read_line(&mut line)
                .expect("Failed to read line");

            self.handle_command(line);
        }
    }

    pub fn continue_execution(&mut self) {
        self.step_over_breakpoint();
        ptrace::cont(self.pid, None).expect("failed to continue execution");
        wait_for_signal(self.pid);
    }

    pub fn set_breakpoint_at_address(&mut self, addr: u64) {
        println!("Set breakpoint at address 0x{:016x}", addr);
        let mut breakpoint = Breakpoint::new(self.pid, addr);
        breakpoint.enable();
        self.breakpoints.insert(addr, breakpoint);
    }

    // TODO: Need proper formatting and printing here, in line with what is in the
    // tutorial series
    pub fn dump_registers(&self) {
        for reg in REGISTERS {
            println!(
                "{}: 0x{:016x}",
                reg.name,
                register::get_register_value(self.pid, reg.reg_kind)
            );
        }
    }

    fn read_memory(&self, address: u64) -> u64 {
        ptrace::read(self.pid, address as AddressType)
            .expect("Failed to read memory")
            .try_into()
            .expect("The i64 address does not fit into u64")
    }

    // Safety: We're relying on ptrace to ensure safety here.
    fn write_memory(&self, address: u64, value: u64) {
        unsafe {
            ptrace::write(self.pid, address as AddressType, value as AddressType)
                .expect("Failed to write memory");
        }
    }

    fn step_over_breakpoint(&mut self) {
        // -1 because execution will go past the breakpoint
        let possible_breakpoint_location = get_pc(self.pid) - 1;

        if self.breakpoints.contains_key(&possible_breakpoint_location) {
            let bp = self
                .breakpoints
                .borrow_mut()
                .get_mut(&possible_breakpoint_location)
                .unwrap();

            if bp.is_enabled() {
                let previous_instruction_address = possible_breakpoint_location;
                set_pc(self.pid, previous_instruction_address);

                bp.disable();
                ptrace::step(self.pid, None).expect("Failed to single step");
                wait_for_signal(self.pid);
                bp.enable();
            }
        }
    }

    fn handle_command(&mut self, line: String) {
        let command = parse_command(line);

        match command {
            Command::Continue => self.continue_execution(),
            Command::Break(addr) => self.set_breakpoint_at_address(addr),
            Command::Exit => self.running = false,
            Command::Memory(memory_kind) => match memory_kind {
                MemoryCommand::Read(read_container) => {
                    println!("0x{:016x}", self.read_memory(read_container.source))
                }
                MemoryCommand::Write(write_container) => {
                    self.write_memory(write_container.dest, write_container.value);
                }
            },
            Command::Register(register_command_kind) => match register_command_kind {
                RegisterCommand::Dump => self.dump_registers(),
                RegisterCommand::Read(read_container) => {
                    println!(
                        "0x{:016x}",
                        register::get_register_value(
                            self.pid,
                            register::get_register_from_name(read_container.source)
                                .expect("The reg enum was None")
                        )
                    );
                }
                RegisterCommand::Write(write_container) => register::set_register_value(
                    self.pid,
                    write_container.dest,
                    write_container.value,
                ),
            },
            Command::Unknown => eprintln!("Unknown command"),
        }
    }

    // TODO: Remove allow macro
    #[allow(dead_code)]
    fn get_function_from_pc(_pc: u64) {}
}

fn get_pc(pid: Pid) -> u64 {
    register::get_register_value(pid, RegisterKind::Rip)
}

fn set_pc(pid: Pid, pc: u64) {
    register::set_register_value(pid, RegisterKind::Rip, pc);
}

fn wait_for_signal(pid: Pid) {
    waitpid(pid, None).expect("Failed to waitpid()");
}

// TODO: Remove allowing of dead code when this is used.
#[allow(dead_code)]
fn print_source(file_path: &Path, line: usize, n_lines_context: usize) {
    let start_line = if line <= n_lines_context {
        1
    } else {
        line - n_lines_context
    };

    let line_diff = if line < n_lines_context {
        n_lines_context - line
    } else {
        0
    };

    let end_line = line + n_lines_context + line_diff + 1;

    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);

    for (current_line, src_line) in reader.lines().enumerate() {
        if current_line >= start_line && current_line <= end_line {
            if current_line == line {
                println!("> {} {}", current_line, src_line.unwrap());
            } else {
                println!("  {} {}", current_line, src_line.unwrap());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_source_test() {
        let path = Path::new("./src/main.rs");
        print_source(path, 10, 6);
    }
}
