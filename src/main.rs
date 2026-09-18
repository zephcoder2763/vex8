use std::io;
use std::fs;


fn main() {
    let mut registers = [0u8; 8];
    let mut memory = [0u8; 255];
    println!("Please Enter the File you would like to run on the Vex8 Virtual Emulator: ");
    let mut file_name = String::from("");
    io::stdin().read_line(&mut file_name).expect("Failed to read File name, please try again.");
    let trimmed_file_name: String = String::from(file_name.trim());
    let program= match fs::read_to_string(trimmed_file_name) {
        Ok(v) => {v},
        Err(_e) => return,
    };

    let mut program_memory: Vec<u8> = Vec::new();

    //can easily be implemented with .map but im lowk lazy and dont wanna rewrite it rn :/
    for byte in program.split_whitespace() {
        let number = u8::from_str_radix(byte, 16).expect("invalid hex byte");
        program_memory.push(number);
    }


    let mut pc = 0x00;
    let mut oldpc = pc;
    let mut opcode: u8;

    while pc < program_memory.len() {
        opcode = program_memory[pc];
        oldpc = pc;
        match opcode {
            0x01 => {
                break;
            },
            0x02 => {
                registers[program_memory[pc+1] as usize] = registers[program_memory[pc+2] as usize] + registers[program_memory[pc+3] as usize];
            },
            0x03 => {
                registers[program_memory[pc+1] as usize] = registers[program_memory[pc+2] as usize] - registers[program_memory[pc+3] as usize];
            },
            0x04 => {
                registers[program_memory[pc+1] as usize] = program_memory[pc+2]
            },
            0x05 => {
                memory[program_memory[pc+1] as usize] = registers[program_memory[pc+2] as usize];
            },
            0x06 => {
                registers[program_memory[pc+1] as usize] = memory[program_memory[pc+2] as usize];
            },
            0x07 => {
                pc = program_memory[pc+1] as usize
            },
            _ => {println!("Invalid Instruction! 0x{:02X}", opcode);
                  return;
            }
        }
        if (pc == oldpc) {
            pc += 4;
        }
    }

    let mut reg = 0;
    for register in registers {
        println!("{}: {:02X}", {reg}, {register});
        reg += 1;
    }
    
} 