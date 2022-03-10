use std::io;

use nix::sys::ptrace;
use nix::sys::wait::waitpid;
use nix::unistd::Pid;

pub struct Debugger {
    prog_name: String,
    pid: Pid,
    running: bool,
}

impl Debugger {
    pub fn new(prog_name: String, pid: Pid) -> Self {
        Self {
            prog_name,
            pid,
            running: true,
        }
    }

    pub fn run(&self) {
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
        waitpid(self.pid, None).expect("failed to waidpid");
    }

    fn handle_command(&self, line: String) {
        let mut args = line.split_whitespace();
        let command = args.next().unwrap();

        if command == "continue" {
            self.continue_execution();
        } else {
            eprintln!("Unknown command");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_command() {
        let dbg = Debugger::new(String::from(""), Pid::this());
        let line = String::from("continue execution");
        dbg.handle_command(line);

        assert!(true);
    }
}
