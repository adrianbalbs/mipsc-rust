use std::env;
use std::process;
use std::fs;
use std::u32;

const MAX_REGISTERS: usize = 32;
const MASK_D: u32 = 0xF800;
const MASK_T: u32 = 0x1F0000;
const MASK_S: u32 = 0x3E00000;
const MASK_IMM: u32 = 0xFFFF;
const MASK_HI_BITS: u32 = 0xFC000000;
const MASK_LO_BITS: u32 = 0x7FF;
const MASK_HI_REG: u64 = 0xFFFFFFFF00000000;
const MASK_LO_REG: u32 = 0xFFFFFFFF;

const SHIFT_D: u32 = 11;
const SHIFT_T: u32 = 16;
const SHIFT_S: u32 = 21;
const SHIFT_HI_BITS: u32 = 26;
const SHIFT_HI_REG: u32 = 32;
const SHIFT_LUI: u32 = 16;

const ADDI: u32 = 0b001000;
const BNE: u32 = 0b000101;
const BEQ: u32 = 0b000100;

struct Hilo {
    hi: i32,
    lo: i32,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let args: (String, bool) = process_args(&args);

    let (filename, trace_mode) = args;
    let instructions: Vec<u32> =  read_instructions(filename);

    execute_instructions(instructions, trace_mode);
}

// simulate execution of  instruction codes in  instructions array
// output from syscall instruction & any error messages are printed
//
// if trace_mode != 0:
//     information is printed about each instruction as it executed
//
// execution stops if it reaches the end of the array
fn execute_instructions(instructions: Vec<u32>, trace_mode: bool) {
    // Values in registers are stored in an array
    let mut registers: [i32; MAX_REGISTERS] = [0; 32];

    let mut hi_lo_reg = Hilo {
        hi: 0,
        lo: 0,
    };

    // Jump is set to 0 until BNE and BEQ instructions are true
    let mut jump = 0; 

    let mut pc: i32 = 0;
    while pc < instructions.len() as i32 {
        check_registers(&instructions[pc as usize], &mut registers, 
                        &mut hi_lo_reg, &trace_mode, &mut pc, &mut jump);

        if registers[0] != 0 {
            registers[0] = 0;
        }

        if jump != 0 {
            pc += jump;
            if pc > instructions.len() as i32 || pc < 0 {
                eprintln!("Illegal branch to non-instruction: PC = {}", pc);
                process::exit(1);
            }
        } else {
            pc += 1;
        }
    }
}

// check_arguments is given command-line arguments
// it sets *trace_mode to 0 if -r is specified
//         *trace_mode is set to 1 otherwise
// the filename specified in command-line arguments is returned
fn process_args(args: &Vec<String>) -> (String, bool) {
    if args.len() < 2 || args.len() > 3 ||
        (args.len() == 2 && args[1] == "-r".to_owned()) ||
        (args.len() == 3 && args[1] != "-r".to_owned()) {
            eprintln!("Usage {} [-r] <file>\n", args[0]);
            process::exit(1);
    }
    let trace_mode: bool = match args.len() {
        2 => true,
        _ => false,
    };
    
    let rtrn_arg = String::from(args[args.len() - 1].clone());

    (rtrn_arg, trace_mode)
}

// read hexadecimal numbers from filename one per line and convert into a vector
fn read_instructions(filename: String) -> Vec<u32> {
    let file = match fs::read_to_string(filename) {
        Ok(file) => file,
        Err(filename) => {
            eprintln!("{}", filename);
            process::exit(1);
        }
    };
    let mut instructions: Vec<u32> = Vec::new();
    for instruction in file.split("\n") {
        if instruction != "" {
            instructions.push(u32::from_str_radix(instruction, 16).unwrap());
        }
    }
    instructions
}

fn check_registers(instruction: &u32, registers: &mut [i32; MAX_REGISTERS], 
                    hi_lo_reg: &mut Hilo, trace_mode: &bool, pc: &mut i32, jump: &mut i32) {
    let hi_bits: u32 = (instruction & MASK_HI_BITS) >> SHIFT_HI_BITS;
    let lo_bits: u32 = instruction & MASK_LO_BITS;
    
    let d: i32 = ((instruction & MASK_D) >> SHIFT_D).try_into().unwrap();
    let t: i32 = ((instruction & MASK_T) >> SHIFT_T).try_into().unwrap();
    let s: i32 = ((instruction & MASK_S) >> SHIFT_S).try_into().unwrap();
    let imm: i16 = (instruction & MASK_IMM).try_into().unwrap();
    
    if hi_bits != 0 {
        match hi_bits {
            ADDI => addi(s, t, imm, registers, trace_mode, &pc, &instruction),
            BNE => bne(s, t, imm, registers, trace_mode, pc, instruction, jump),
            BEQ => beq(s, t, imm, registers, trace_mode, pc, instruction, jump),
            _=> unimplemented!("Not implemented!"),
        }
    }
}

fn addi(s: i32, t: i32, imm: i16, registers: &mut [i32; MAX_REGISTERS], 
        trace_mode: &bool, pc: &i32, instruction: &u32) {
    registers[t as usize] = registers[s as usize] + imm as i32;
    if *trace_mode {
        println!("{}: {:#08X} addi ${}, ${}, {}", pc, instruction, t, s, imm);
        println!(">>> ${} = {}", t, registers[t as usize]);
    }
} 

fn bne(s: i32, t: i32, imm: i16, registers: &mut [i32; MAX_REGISTERS], 
        trace_mode: &bool, pc: &i32, instruction: &u32, jump: &mut i32) {
    if registers[s as usize] != registers[t as usize] {
        *jump = imm as i32;
    }
    if *trace_mode {
        println!("{}: {:#08X} bne ${}, ${}, ${}", pc, instruction, s, t, imm);
        if *jump != 0 {
            println!(">>> branch taken to PC = {}", pc + *jump);
        } else {
            println!(">>> branch not taken");
        }
    }
}

fn beq(s: i32, t: i32, imm: i16, registers: &mut [i32; MAX_REGISTERS], 
        trace_mode: &bool, pc: &i32, instruction: &u32, jump: &mut i32) {
    if registers[s as usize] == registers[t as usize] {
        *jump = imm as i32;
    }
    if *trace_mode {
        println!("{}: {:#08X} beq ${}, ${}, ${}", pc, instruction, s, t, imm);
        if *jump != 0 {
            println!(">>> branch taken to PC = {}", pc + *jump);
        } else {
            println!(">>> branch not taken");
        }
    }
}
