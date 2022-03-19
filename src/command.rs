use crate::register::{self, RegisterKind};

#[allow(dead_code)]
#[derive(Clone)]
pub struct ReadContainer<T> {
    pub source: T,
}

#[derive(Clone)]
pub struct WriteContainer<T, U> {
    pub dest: T,
    pub value: U,
}

#[derive(Clone)]
pub enum CommandKind {
    Break(u64),
    Continue,
    Exit,
    Memory(MemoryCommandKind),
    Register(RegisterCommandKind),
    Unknown,
}

#[derive(Clone)]
pub enum RegisterCommandKind {
    Dump,
    Read(ReadContainer<String>),
    Write(WriteContainer<RegisterKind, u64>),
}

#[derive(Clone)]
pub enum MemoryCommandKind {
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
pub fn parse_command<'a>(line: String) -> CommandKind {
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
            CommandKind::Break(break_point_addr)
        }
        "continue" => CommandKind::Continue,
        "exit" => CommandKind::Exit,
        "memory" => {
            let command_arg = args.next().expect("No memory command argument given");
            match command_arg {
                "read" => {
                    let source =
                        u64::from_str_radix(args.next().expect("No read memory address given"), 16)
                            .expect("Failed to parse read memory address to hexadecimal value");
                    CommandKind::Memory(MemoryCommandKind::Read(ReadContainer { source }))
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
                    CommandKind::Memory(MemoryCommandKind::Write(WriteContainer { dest, value }))
                }
                _ => CommandKind::Unknown,
            }
        }
        "register" => {
            let command_arg = args.next().expect("No register command argument given");
            match command_arg {
                "dump" => CommandKind::Register(RegisterCommandKind::Dump),
                "read" => {
                    let source = args.next().expect("No read register name given");
                    CommandKind::Register(RegisterCommandKind::Read(ReadContainer {
                        source: source.to_string(),
                    }))
                }
                "write" => {
                    let reg_name = args.next().expect("No write register name given");
                    let value =
                        u64::from_str_radix(args.next().expect("No write memory value given"), 16)
                            .expect("Failed to parse write memory value to hexadecimal value");

                    let reg = register::get_register_from_name(reg_name.to_string().clone());

                    CommandKind::Register(RegisterCommandKind::Write(WriteContainer {
                        dest: reg.expect("The register enum was None"),
                        value,
                    }))
                }
                _ => CommandKind::Unknown,
            }
        }
        _ => CommandKind::Unknown,
    }
}
