#!/bin/bash


mkdir -p -- "bin" && nasm -f elf64 ./src/assembly/hello.asm -o ./bin/hello.o && ld ./bin/hello.o -o ./bin/hello
