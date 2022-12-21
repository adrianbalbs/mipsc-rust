use std::env;
use std::process;
use std::fs;
use std::u32;

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
    let mut pc = 0;
    while pc < instructions.len() {
        if trace_mode {
            println!("{:#08X}", instructions[pc]);
        }
        pc += 1;
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


