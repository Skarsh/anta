# Tamandua

![Tamandua](../assets/tamandua_grey.jpg?raw=true)

[![Build Status](https://github.com/skarsh/debug-rs/actions/workflows/build.yml/badge.svg)](https://github.com/Skarsh/debug-rs/actions/workflows/build.yml)
[![Security audit](https://github.com/Skarsh/debug-rs/actions/workflows/audit.yml/badge.svg?branch=main)](https://github.com/Skarsh/debug-rs/actions/workflows/audit.yml)

## Introduction
Tamandua is a debugger from scratch project. It is initially intended as project to learn more about binaries,
how they work, and understand the magic that are modern day debuggers.
Currently most of the code is a Rust implementation of the [TartanLlama's debugger series] (https://blog.tartanllama.xyz/writing-a-linux-debugger-setup/)
(highly recommended) to get off the ground. But the vision goes beyond that.

## Disclaimer
This project is under heavy development and is not yet intended to be used in any serious context.
It is still primarily a project meant for learning purposes.

## Building
Building the debugger requires `Cargo`, the Rust build tool. It can be built running `cargo build --release` command.

## Usage
Tamandua needs the path to the executable to be debugged, e.g. `tamandua some/path/to/executable`

There are four possible commands that can be given to the debugger
1. `break address(hex)`: Sets a breakpoint at the given address
2. `continue`: Continues the execution of the program to the next breakpoint
3. `register`: 
    - `read reg_name`: Reads the value from the specified register by name
    - `write reg_name value(hex)`: Write the specified hexadecimal value to the register by name
    - `dump`: Dumps the values of all the registers
4. `memory` 
    - `read address(hex)`: Read memory from a specific address location
    - `write address(hex) value(hex)`: Write 'value' to memory location at 'address'

## Roadmap 
This section outlines the major milestones needed to reach a 1.0 version implementation
- [ ] Elf Parser Library
    - [x] Elf Header
    - [x] Elf Sections
    - [x] Elf SectionHeader String Table
    - [x] Elf Symbols
    - [ ] Program Header
    - [ ] Relocations
    - [ ] Split parsing functionality into own library crate
- [ ] Dwarf Debug Symbols Parser Library
- [ ] Debug backend 
- [ ] Debug frontend

