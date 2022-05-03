#![allow(dead_code)]
#![allow(unused)]
use nix::sys::ptrace;
use nix::unistd::Pid;

const BOTTOM_BYTE_MASK: u64 = 0xff;
const INT3: u64 = 0xcc;

pub struct Breakpoint {
    pid: Pid,
    addr: u64,
    enabled: bool,
    saved_data: u8,
}

impl Breakpoint {
    pub fn new(pid: Pid, addr: u64) -> Self {
        Self {
            pid,
            addr,
            enabled: false,
            saved_data: 0,
        }
    }

    pub fn enable(&mut self) {
        let mut data: u64 = ptrace::read(self.pid, self.addr as ptrace::AddressType)
            .expect("Could not read memory")
            .try_into()
            .expect("Data read from memory does not fit into u64");
        self.saved_data = bottom_byte(data);
        let data_with_int3: u64 = set_int3_at_end_of_data(data);

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
        let data: u64 = ptrace::read(self.pid, self.addr as ptrace::AddressType)
            .expect("Failed to read memory")
            .try_into()
            .expect("Data read from memory does not fit into u64");

        let restored_data = restore_data_from_int3(data, self.saved_data);

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

    pub fn get_address(&self) -> u64 {
        self.addr
    }
}

/// Retrieves the bottom byte of a u64
fn bottom_byte(orig_data: u64) -> u8 {
    (orig_data & BOTTOM_BYTE_MASK) as u8
}

/// Sets the int3 (0xcc) value at the bottom byte of a u64.
/// This is used to set a breakpoint at a memory address of
/// a traced program.
fn set_int3_at_end_of_data(orig_data: u64) -> u64 {
    (orig_data & !0xff) | INT3
}

/// Restores the original data from a data with a int3 bottom byte set.
/// This is done by AND the data with the 0xffffffffffffff00 mask to
/// nil out the bottom byte, and then OR the data with the saved data
/// from that before the breakpoint was set.
fn restore_data_from_int3(data_with_int3: u64, saved_data: u8) -> u64 {
    // Uses the !0xff (0xffffffffffffff00) mask to ignore
    // the original bottom byte and then OR it with the saved data
    (data_with_int3 & !0xff) as u64 | saved_data as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn save_bottom_byte() {
        let orig_data: u64 = 0x4800000e6d358d48;
        let bottom_byte = bottom_byte(orig_data);
        let expected: u8 = 0x48;
        assert_eq!(expected, bottom_byte);
    }

    #[test]
    fn set_int3_at_end_of_data_test() {
        let orig_data: u64 = 0x4800000e6d358d48;
        let orig_data_with_int3 = set_int3_at_end_of_data(orig_data);
        let expected: u64 = 0x4800000e6d358dcc;
        assert_eq!(expected, orig_data_with_int3);
    }

    #[test]
    fn restore_data_from_int3_test() {
        let data_with_int3: u64 = 0x4800000e6d358dcc;
        let saved_data: u8 = 0x48;
        let restored_data = restore_data_from_int3(data_with_int3, saved_data);
        let expected: u64 = 0x4800000e6d358d48;
        assert_eq!(expected, restored_data);
    }
}
