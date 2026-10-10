use std::io;
use std::fs;


fn main() {
    let mut registers = [0u8; 9];
    let mut memory = [[0u8; 255]; 16];
    let mut vmemory = [0u8; 255];
    let mut palette = [0u32; 5];
    palette[0] = 0x000000;
    palette[1] = 0xFF0000;
    palette[2] = 0x00FF00;
    palette[3] = 0x0000FF;
    palette[4] = 0xFFFFFF;

    println!("Please Enter the File you would like to run on the Vex8 Virtual Emulator: ");
    let mut file_name = String::from("");
    io::stdin().read_line(&mut file_name).expect("Failed to read File name, please try again.");
    let trimmed_file_name: String = String::from(file_name.trim());
    let program= match fs::read(trimmed_file_name) {
        Ok(v) => {v},
        Err(_e) => return,
    };

    let mut program_memory: Vec<u8> = Vec::new();

    //can easily be implemented with .map but im lowk lazy and dont wanna rewrite it rn :/
    for byte in program {
        let number = u8::from_str_radix(&format!("{:02X}", byte), 16).expect("invalid hex byte");
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
                registers[program_memory[pc+1] as usize] = program_memory[pc+2];
            },
            0x05 => {
                memory[registers[8] as usize][program_memory[pc+1] as usize] = registers[program_memory[pc+2] as usize];
            },
            0x06 => {
                registers[program_memory[pc+1] as usize] = memory[registers[8] as usize][program_memory[pc+2] as usize];
            },
            0x07 => {
                pc = program_memory[pc+1] as usize
            },
            0x08 => { //CMP
                if (registers[program_memory[pc+1] as usize] == registers[program_memory[pc+2] as usize]) {
                    registers[7] = 1;
                } else {
                    registers[7] = 0;
                }
            },
            0x09 => { // JE
                if (registers[7] == 1) {
                    pc = program_memory[pc+1] as usize
                }
            },
            0x0A => { // INT
                if (program_memory[pc+1] as usize == 0x10) {
                    println!("Printing!");
                }
            },
            0x0B => { //SPI
                registers[8] = program_memory[pc+1];
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

fn conversion(palette: &[u32; 5], vmem: &[u8; 255]) -> [u32; 255] {
    let mut pixel_buffer: [u32; 255] = [0u32; 255];
    for (i, &idx) in vmem.iter().enumerate() {
        let color = palette.get(idx as usize).copied().unwrap_or(0x000000);
        // 0x00RRGGBB -> 0xFFBBGGRR (RGBA byte order, alpha = 1)
        let r = (color >> 16) & 0xFF;
        let g = (color >> 8) & 0xFF;
        let b = color & 0xFF;
        pixel_buffer[i] = 0xFF000000 | (b << 16) | (g << 8) | r;
    }
    pixel_buffer
}
