## VEX8!!
### 1. Intro!
Vex8 is a virtual CPU emulator designed to be simple and east to learn to help people get into coding in ASM!
the emulator is designed to be simple, with only 255 bytes for program memory and 255 bytes of mutible memory!
there is no VRAM as of now, im working on it AND an assembler
### 2. Features im working on
1. VRAM
2. Assembler
3. JIT (Jmp if true, not just in time compiliation thats not hgow tjat works!!!!)
4. CMP (comparison of 2 registers!!)
### 3. Current instruction set!
0x01 - HLT (completely halts execution rn, will change soonish)
0x02 - ADD (adds two values, takes output register and 2 registers to add the values of)
0x03 - SUB (subtracts 2 vaues, same inputs as ADD)
0x04 - LOAD (loads a value (hex) into a register
0x05 - LOADM (loads a value into a defined memory address, takes 1 register and 1 memory location)
0x06 - READM ( reads a value into a register, takes 1 memory address and 1 register)
0x07 - JMP (Jump to a memory address in program memory, must be a multiple of 4 cause thats how it works uhh yeahhhhh)
