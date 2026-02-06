use std::io::{self, Write};

pub mod prelude;
mod profiler;
mod registry;
mod solutions;
mod utils;

use registry::{SOLUTIONS, run_entry, select_solution};

#[global_allocator]
static GLOBAL: profiler::CountingAllocator = profiler::CountingAllocator::new();

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if SOLUTIONS.is_empty() {
        eprintln!("No solutions found. Add files to src/solutions (e.g. src/solutions/c0003.rs).");
        std::process::exit(1);
    }

    // Commands
    #[allow(clippy::single_match)]
    match args.first().map(|s| s.as_str()) {
        Some("copy") => {
            let sel = args
                .get(1)
                .ok_or_else(|| "Usage: copy <idx|name>".to_string())
                .unwrap();
            let entry = select_solution(sel).unwrap();
            let code = utils::paste_ready_code(entry.file).unwrap();

            utils::copy_to_clipboard(&code);

            println!("Copied `{}` ({}) to clipboard.", entry.name, entry.file);
            return;
        }
        _ => (),
    }

    // If an arg is passed, skip selector:
    if !args.is_empty() {
        let (sel, rest) = (&args[0], &args[1..]);
        match select_solution(sel) {
            Ok(entry) => {
                run_entry(entry, rest);
            }
            Err(e) => {
                eprintln!("{e}");
                print_help();
                std::process::exit(2);
            }
        }
        return;
    }

    // Interactive selector
    println!("Discovered solutions:");
    for (i, e) in SOLUTIONS.iter().enumerate() {
        println!("  [{i}] {} ({})", e.name, e.file);
    }
    print!("Select index: ");
    io::stdout().flush().ok();

    let mut line = String::new();
    if io::stdin().read_line(&mut line).is_err() {
        eprintln!("Failed to read selection.");
        std::process::exit(2);
    }
    let line = line.trim();
    if line.is_empty() {
        return;
    }

    let entry = match select_solution(line) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(2);
        }
    };

    println!("Enter args (optional).");
    println!(r#"  - You can type one JSON array: ["abcabcbb"]"#);
    println!(r#"  - Or type space-separated args: abcabcbb"#);
    print!("Args: ");
    io::stdout().flush().ok();

    let mut args_line = String::new();
    io::stdin().read_line(&mut args_line).ok();
    let args_line = args_line.trim();

    let run_args: Vec<String> = if args_line.is_empty() {
        vec![]
    } else if args_line.trim_start().starts_with('[') {
        vec![args_line.to_string()]
    } else {
        args_line
            .split_whitespace()
            .map(|s| s.to_string())
            .collect()
    };

    run_entry(entry, &run_args);
}

fn print_help() {
    eprintln!("\nUsage:");
    eprintln!("  cargo run                 # interactive selector");
    eprintln!("  cargo run -- <idx|name>   # run solution by index or name");
    eprintln!("\nArgs format:");
    eprintln!(r#"  cargo run -- 0 '["au"]'          # single JSON array"#);
    eprintln!(r#"  cargo run -- 0 '"au"'            # separate JSON value(s)"#);
}
