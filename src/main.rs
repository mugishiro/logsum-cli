use std::env;
use std::fs;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    if args.len() != 2 && args.len() != 3 {
        eprintln!("Usage: logsum <path> [status|path]");
        return;
    }
    if args.len() == 2 {
        args.push("status".to_string());
    }
    if args[2] != "status" && args[2] != "path" {
        eprintln!("Usage: logsum <path> [status|path]");
        return;
    }

    let path = &args[1];
    let by = &args[2];

    let mut counts: std::collections::HashMap<String, u32> = std::collections::HashMap::new();
    let mut skipped = 0;

    match fs::read_to_string(path) {
        Ok(content) => {
            for line in content.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();

                if parts.len() != 4 {
                    skipped += 1;
                    continue;
                }
                if !parts[3].chars().all(|c| c.is_ascii_digit()) {
                    skipped += 1;
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
            println!("skipped: {}", skipped);
        }
        Err(e) => eprintln!("error: could not read file {}: {}", path, e),
    }
}
