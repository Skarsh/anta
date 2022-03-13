#![allow(dead_code)]
#![allow(unused)]
use nix::sys::ptrace;
use nix::unistd::Pid;

pub struct Breakpoint {
    pid: Pid,
    addr: isize,
    enabled: bool,
    saved_data: u8,
}

impl Breakpoint {
    pub fn new(pid: Pid, addr: isize) -> Self {
        Self {
            pid,
            addr,
            enabled: false,
            saved_data: 0,
        }
    }

    pub fn enable(&mut self) {
        // TODO: Continue here
        //let data = ptrace::read(self.pid, self.addr);

        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn get_address(&self) -> isize {
        self.addr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enable_after_new_expect_enabled() {
        let mut breakpoint = Breakpoint::new(Pid::this(), 0);
        assert_eq!(breakpoint.is_enabled(), false);
        breakpoint.enable();
        assert_eq!(breakpoint.is_enabled(), true);
    }

    #[test]
    fn disable_after_new_expect_disabled() {
        let mut breakpoint = Breakpoint::new(Pid::this(), 0);
        assert_eq!(breakpoint.is_enabled(), false);
        breakpoint.disable();
        assert_eq!(breakpoint.is_enabled(), false);
    }
}
