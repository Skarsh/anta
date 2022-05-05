// std
use std::ffi::CString;
use std::path::Path;
use std::fs::File;

// 3rd party
use clap::Parser;
use nix::sys::personality::Persona;
use nix::sys::{personality, ptrace};
use nix::unistd::{execv, fork, ForkResult};

// own
mod debugger;
use crate::debugger::Debugger;

mod breakpoint;

mod command;

mod register;

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
    let path_string = format!("{}/{}", PREFIX_PATH, program_name);
    let path = Path::new(&path_string);
    let file = File::open(&path).unwrap();
    let mmap = unsafe { memmap::Mmap::map(&file).unwrap() };
    let object = object::File::parse(&*mmap).unwrap();

    let c_str_path = CString::new(&*path.to_str().unwrap()).expect("CString::new failed");


    match unsafe { fork() } {
        Ok(ForkResult::Child) => {
            let _res = personality::set(Persona::ADDR_NO_RANDOMIZE);
            execute_debugee(c_str_path)
        }
        Ok(ForkResult::Parent { child }) => {
            let mut debugger = Debugger::new(path, child, object);
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
