use std::env;
use std::fs;

const USAGE: &str = "Usage: logsum <path> [status|path]";

fn parse_args(args: &[String]) -> Result<(&str, &str), &'static str> {
    if args.len() != 2 && args.len() != 3 {
        return Err(USAGE);
    }

    let path = args[1].as_str();
    let by = if args.len() == 3 {
        args[2].as_str()
    } else {
        "status"
    };

    if by != "status" && by != "path" {
        return Err(USAGE);
    }

    Ok((path, by))
}

fn parse_line(line: &str, by: &str) -> Option<String> {
    let parts: Vec<&str> = line.split_whitespace().collect();

    if parts.len() != 4 {
        return None;
    }
    if !parts[3].chars().all(|c| c.is_ascii_digit()) {
        return None;
    }

    let key = if by == "status" { parts[3] } else { parts[2] };

    Some(key.to_string())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let (path, by) = match parse_args(&args) {
        Ok(v) => v,
        Err(msg) => {
            eprintln!("{}", msg);
            return;
        }
    };
    let mut counts: std::collections::HashMap<String, u32> = std::collections::HashMap::new();
    let mut skipped = 0;

    match fs::read_to_string(path) {
        Ok(content) => {
            for line in content.lines() {
                if let Some(key) = parse_line(line, by) {
                    *counts.entry(key).or_insert(0) += 1;
                } else {
                    skipped += 1;
                }
            }

            let mut items: Vec<(String, u32)> = counts.into_iter().collect();
            items.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

            for (key, count) in items {
                println!("{} {}", key, count);
            }
            println!("skipped {}", skipped);
        }
        Err(e) => eprintln!("error: could not read file {}: {}", path, e),
    }
}
