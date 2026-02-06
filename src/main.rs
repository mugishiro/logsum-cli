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

fn summarize(content: &str, by: &str) -> (std::collections::HashMap<String, u32>, u32) {
    let mut skipped = 0;
    let mut counts: std::collections::HashMap<String, u32> = std::collections::HashMap::new();

    for line in content.lines() {
        if let Some(key) = parse_line(line, by) {
            *counts.entry(key).or_insert(0) += 1;
        } else {
            skipped += 1;
        }
    }

    (counts, skipped)
}

fn sort_items(counts: std::collections::HashMap<String, u32>) -> Vec<(String, u32)> {
    let mut items: Vec<(String, u32)> = counts.into_iter().collect();
    items.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    items
}

fn print_report(items: &[(String, u32)], skipped: u32) {
    for (key, count) in items {
        println!("{} {}", key, count);
    }
    println!("skipped {}", skipped);
}

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let (path, by) = parse_args(&args).map_err(|e| e.to_string())?;
    let content = fs::read_to_string(path)
        .map_err(|e| format!("error: could not read file {}: {}", path, e))?;

    let (counts, skipped) = summarize(&content, by);
    let items = sort_items(counts);
    print_report(&items, skipped);

    Ok(())
}

fn main() {
    if let Err(msg) = run() {
        eprintln!("{}", msg);
    }
}
