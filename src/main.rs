#![allow(unused)]
// std
use std::env;
use std::ffi::CString;
use std::process::exit;

// 3rd party
use nix::sys::personality::Persona;
use nix::sys::{personality, ptrace};
use nix::unistd::{execv, fork, ForkResult};

// own
mod debugger;
use crate::debugger::Debugger;

mod breakpoint;
use crate::breakpoint::Breakpoint;

mod command;

mod register;
use crate::register::Register;

// constants
const PREFIX_PATH: &str = "target/debug/";

fn execute_debugee(path: CString) {
    ptrace::traceme().unwrap();

    let args = vec![CString::new("").unwrap()];
    execv(&path, &args[..]).unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Program name not specified");
        exit(-1);
    }

    let program_name = &args[1];
    let path = format!("{}/{}", PREFIX_PATH, program_name);
    let path = CString::new(&*path).expect("CString::new failed");

    match unsafe { fork() } {
        Ok(ForkResult::Child) => {
            println!("child");
            personality::set(Persona::ADDR_NO_RANDOMIZE);
            execute_debugee(path)
        }
        Ok(ForkResult::Parent { child }) => {
            println!("parent");
            let mut debugger = Debugger::new(program_name.to_owned(), child);
            debugger.run();
        }
        Err(err) => {
            panic!("[main] fork() failed: {}", err);
        }
    }
}
