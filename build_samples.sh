#!/bin/bash


mkdir -p -- "./samples/bin" && nasm -f elf64 ./samples/src/assembly/hello.asm -o ./samples/bin/hello.o && ld ./samples/bin/hello.o -o ./samples/bin/hello
