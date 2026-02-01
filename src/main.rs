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

    let mut counts: std::collections::HashMap<String, u32> = std::collections::HashMap::new();
    let by = &args[3];

    match fs::read_to_string(path) {
        Ok(content) => {
            for line in content.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() != 4 {
                    continue;
                }

                let key = if by == "status" {
                    parts[3].to_string()
                } else {
                    parts[2].to_string()
                };

                *counts.entry(key).or_insert(0) += 1;
            }

            for (key, count) in counts {
                println!("{}: {}", key, count);
            }
        }
        Err(e) => eprintln!("error: could not read file {}: {}", path, e),
    }

}
