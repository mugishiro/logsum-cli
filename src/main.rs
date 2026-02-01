use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: logsum <path> --by status|path");
        return;
    }

    let path = &args[1];
    if args[2] != "--by" || (args[3] != "status" && args[3] != "path") {
        eprintln!("Usage: logsum <path> --by status|path");
        return;
    }
    match fs::read_to_string(path) {
        Ok(content) => {
            for line in content.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() != 4 {
                    continue;
                }
            }
        }
        Err(e) => eprintln!("error: could not read file {}: {}", path, e),
    }
}
