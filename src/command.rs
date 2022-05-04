use crate::register::{self, RegisterKind};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadContainer<T> {
    pub source: T,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WriteContainer<T, U> {
    pub dest: T,
    pub value: U,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Break(u64),
    Continue,
    Exit,
    Memory(MemoryCommand),
    Register(RegisterCommand),
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegisterCommand {
    Dump,
    Read(ReadContainer<String>),
    Write(WriteContainer<RegisterKind, u64>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MemoryCommand {
    Read(ReadContainer<u64>),
    Write(WriteContainer<u64, u64>),
}

/// Takes in a string, depending on the first argument in the string several types of commands are possible
/// 1. 'continue': Continues the execution of the program to the next breakpoint or until the end
/// 2. 'break' 'address: hex': Sets a breakpoint at the given address
/// 3. 'register'
///      - 'read' 'reg_name: &str':               Reads the value from the specified register by name
///      - 'write' 'reg_name: &str' 'value: hex': Write the speicified hexadecimal value to the register by name.
///      - 'dump':                                Dumps the values of all the registers specified in breakpoint::REGISTERS
/// 4. 'memory'
///      - 'read' 'address: hex':                 Read memory from a specific address location
///      - 'write' 'addres: hex' 'value: hex':    Write 'value' to memory location at 'address'
pub fn parse_command(line: String) -> Command {
    // TODO: Lots of repeating here, should be possible to
    // move some of it out into private functions
    let mut args = line.split_whitespace();
    let command = args.next().expect("No command given!");

    match command {
        "break" => {
            let break_point_addr = u64::from_str_radix(
                args.next().expect("No breakpoint address argument given"),
                16,
            )
            .expect("Failed to parse breakpoint address to hexadecimal value");
            Command::Break(break_point_addr)
        }
        "continue" => Command::Continue,
        "exit" => Command::Exit,
        "memory" => {
            let command_arg = args.next().expect("No memory command argument given");
            match command_arg {
                "read" => {
                    let source =
                        u64::from_str_radix(args.next().expect("No read memory address given"), 16)
                            .expect("Failed to parse read memory address to hexadecimal value");
                    Command::Memory(MemoryCommand::Read(ReadContainer { source }))
                }
                "write" => {
                    let dest = u64::from_str_radix(
                        args.next().expect("No write memory address given"),
                        16,
                    )
                    .expect("Failed to parse write memory address to hexadecimal value");
                    let value =
                        u64::from_str_radix(args.next().expect("No write memory value given"), 16)
                            .expect("Failed to parse write memeory value to hexadecimal value");
                    Command::Memory(MemoryCommand::Write(WriteContainer { dest, value }))
                }
                _ => Command::Unknown,
            }
        }
        "register" => {
            let command_arg = args.next().expect("No register command argument given");
            match command_arg {
                "dump" => Command::Register(RegisterCommand::Dump),
                "read" => {
                    let source = args.next().expect("No read register name given");
                    Command::Register(RegisterCommand::Read(ReadContainer {
                        source: source.to_string(),
                    }))
                }
                "write" => {
                    let reg_name = args.next().expect("No write register name given");
                    let value =
                        u64::from_str_radix(args.next().expect("No write memory value given"), 16)
                            .expect("Failed to parse write memory value to hexadecimal value");

                    let reg = register::get_register_from_name(reg_name.to_string());

                    Command::Register(RegisterCommand::Write(WriteContainer {
                        dest: reg.expect("The register enum was None"),
                        value,
                    }))
                }
                _ => Command::Unknown,
            }
        }
        _ => Command::Unknown,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const HEX_BASE: u32 = 16;

    #[test]
    fn test_continue_command() {
        let command = parse_command(String::from("continue"));
        assert_eq!(command, Command::Continue);
    }

    #[test]
    fn test_exit_command() {
        let command = parse_command(String::from("exit"));
        assert_eq!(command, Command::Exit);
    }

    #[test]
    fn test_unknown_command() {
        let command = parse_command(String::from("unknown"));
        assert_eq!(command, Command::Unknown);
    }

    #[test]
    fn test_memory_command_read_0xff() {
        let source_address_str = "ff";
        let source_address_hex = u64::from_str_radix(source_address_str, HEX_BASE)
            .expect("Failed to parse hex string into u64");
        let command = parse_command(format!("memory read {}", source_address_str));
        match command {
            Command::Memory(MemoryCommand::Read(read_container)) => {
                assert_eq!(read_container.source, source_address_hex)
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn test_memory_command_write_0x420ff_to_0xff_address() {
        let dest_address_str = "ff";
        let dest_address_hex = u64::from_str_radix(dest_address_str, HEX_BASE)
            .expect("Failed to parse hex string into u64");

        let value_str = "420ff";
        let value_hex =
            u64::from_str_radix(value_str, HEX_BASE).expect("Failed to parse hex string into u64");

        let command = parse_command(format!("memory write {} {}", dest_address_str, value_str));
        match command {
            Command::Memory(MemoryCommand::Write(write_container)) => {
                assert_eq!(write_container.dest, dest_address_hex);
                assert_eq!(write_container.value, value_hex);
            }
            _ => unreachable!(),
        }
    }
}
