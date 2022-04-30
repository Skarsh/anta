#![allow(unused)]
// std
use std::env;
use std::ffi::CString;
use std::process::exit;

// 3rd party
use clap::Parser;
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
    let args = Args::parse();

    let program_name = args.binary;
    let path = format!("{}/{}", PREFIX_PATH, program_name);
    let path = CString::new(&*path).expect("CString::new failed");

    match unsafe { fork() } {
        Ok(ForkResult::Child) => {
            personality::set(Persona::ADDR_NO_RANDOMIZE);
            execute_debugee(path)
        }
        Ok(ForkResult::Parent { child }) => {
            let mut debugger = Debugger::new(program_name, child);
            debugger.run();
        }
        Err(err) => {
            panic!("[main] fork() failed: {}", err);
        }
    }
}

/// Simple debugger written in Rust
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    binary: String,
}
