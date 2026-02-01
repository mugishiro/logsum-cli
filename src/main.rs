use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: logsum <path> --by status|path");
        return;
    }

    let path = &args[1];
    match fs::read_to_string(path) {
        Ok(_) => println!("ok: loaded file {}", path),
        Err(e) => eprintln!("error: could not read file {}: {}", path, e),
    }
}
