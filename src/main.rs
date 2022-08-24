// std
use std::env;
use std::ffi::CString;
use std::path::Path;

// 3rd party
use nix::sys::personality::Persona;
use nix::sys::{personality, ptrace};
use nix::unistd::{execv, fork, ForkResult};

// own
mod breakpoint;

mod command;

mod debugger;
use crate::debugger::Debugger;

mod elf;

mod register;

fn execute_debugee(path: CString) {
    ptrace::traceme().unwrap();

    let args = vec![CString::new("").unwrap()];
    execv(&path, &args[..]).unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program_path_string = &args[1];

    let path = Path::new(&program_path_string);
    let c_str_path = CString::new(path.to_str().unwrap()).expect("CString::new failed");

    match unsafe { fork() } {
        Ok(ForkResult::Child) => {
            let _res = personality::set(Persona::ADDR_NO_RANDOMIZE);
            execute_debugee(c_str_path)
        }
        Ok(ForkResult::Parent { child }) => {
            let mut debugger = Debugger::new(path, child);
            debugger.run();
        }
        Err(err) => {
            panic!("[main] fork() failed: {}", err);
        }
    }
}
