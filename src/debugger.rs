use std::collections::HashMap;
use std::i64;
use std::io;

use nix::sys::ptrace;
use nix::sys::wait::waitpid;
use nix::unistd::Pid;

use crate::breakpoint::Breakpoint;
use crate::register::REGISTERS;

pub struct Debugger {
    _prog_name: String,
    pid: Pid,
    running: bool,
    breakpoints: HashMap<isize, Breakpoint>,
}

impl Debugger {
    pub fn new(prog_name: String, pid: Pid) -> Self {
        Self {
            _prog_name: prog_name,
            pid,
            running: true,
            breakpoints: HashMap::new(),
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

    pub fn continue_execution(&self) {
        ptrace::cont(self.pid, None).expect("failed to continue execution");
        waitpid(self.pid, None).expect("failed to waitpid()");
    }

    pub fn set_breakpoint_at_address(&mut self, addr: isize) {
        println!("Set breakpoint at address 0x{:x}", addr);

        let mut breakpoint = Breakpoint::new(self.pid, addr);
        breakpoint.enable();
        self.breakpoints.insert(addr, breakpoint);
    }

    // TODO: Need proper formatting and printing here, in line with what is in the
    // tutorial series
    pub fn dump_registers() {
        for register in REGISTERS {
            println!("{:?}", register);
        }
    }

    fn handle_command(&mut self, line: String) {
        let mut args = line.split_whitespace();
        let command = args.next().expect("No command given");

        if command == "continue" {
            self.continue_execution();
        } else if command == "break" {
            let addr = args.next().expect("No addr argmument given");
            let addr = i64::from_str_radix(addr, 16)
                .expect("Failed to parse hexadecimal address for breakpoint");
            self.set_breakpoint_at_address(addr as isize);
        } else if command == "exit" {
            self.running = false;
        } else {
            eprintln!("Unknown command");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_breakpoint() {
        let mut debugger = Debugger::new(String::from("Hello"), Pid::this());
        debugger.set_breakpoint_at_address(0xff);
    }
}
