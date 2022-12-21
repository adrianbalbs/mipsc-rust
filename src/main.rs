use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args: (String, bool) = process_args(&args);
}

fn process_args(args: &Vec<String>) -> (String, bool) {
    if args.len() < 2 || args.len() > 3 ||
        (args.len() == 2 && args[1] == "-r".to_owned()) ||
        (args.len() == 3 && args[1] != "-r".to_owned()) {
            eprintln!("Usage {} [-r] <file>\n", args[0]);
            process::exit(1);
    }
    let trace_mode: bool = match args.len() {
        2 => false,
        _ => true,
    };

    let rtrn_arg = String::from(args[args.len() - 1].clone());

    (rtrn_arg, trace_mode)
}

