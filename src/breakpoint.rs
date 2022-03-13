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
        let mut data = ptrace::read(self.pid, self.addr as ptrace::AddressType)
            .expect("Could not read memory");
        self.saved_data = (data & 0xff) as u8; // save bottom byte
        let int3 = 0xcc;
        let data_with_int3 = ((data & 0xff) | int3);

        // SAFETY: This is needed to be able to change the instruction at the place we want to set a breakpoint
        unsafe {
            ptrace::write(
                self.pid,
                self.addr as ptrace::AddressType,
                data_with_int3 as ptrace::AddressType,
            )
            .expect("Failed to write memory");
        }

        self.enabled = true;
    }

    pub fn disable(&mut self) {
        let data = ptrace::read(self.pid, self.addr as ptrace::AddressType)
            .expect("Failed to read memory");
        let restored_data = ((data & 0xff) | self.saved_data as i64);

        // SAFETY: This is needed to be able to restore the instruction at the location where we set a breakpoint
        unsafe {
            ptrace::write(
                self.pid,
                self.addr as ptrace::AddressType,
                restored_data as ptrace::AddressType,
            )
            .expect("Failed to write memory")
        }

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
