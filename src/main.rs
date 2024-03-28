use clap::Parser;
use anyhow::{anyhow, Result};

use std::fs::{write, File};
use std::io::Read;
use std::str::FromStr;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // Filepath
    #[arg(help = "Path to the file to be modified.")]
    filepath: String,

    // Whether to increment or decrement
    #[arg(short, long, default_value = "i", help = "Determines whether the barony IDs will be incremented or decremented; i or d")]
    inc: String,

    // Range of baronies to be incremented/decremented. Must be formatted like: 200 300
    #[arg(short, long, default_value = "0 10000000", help = "Range of baronies to be inc/dec'd. Format like \"200 300\"")]
    range: String,

    // Number to increment/decrement by
    #[arg(short, long, default_value = "0", help = "How much to increment/decrement by.")]
    count: u32,
}

fn main() -> Result<()> {
    let start = std::time::Instant::now();
    let args: Args = Args::parse();

    let mut file = match File::open(&args.filepath) {
        Ok(fl) => fl,
        Err(e) => return Err(anyhow!("{} could not be read because: {:?}", &args.filepath, e)),
    };

    let mut content: String = String::new();
    match file.read_to_string(&mut content) {
        Ok(read_size) => {
            println!("{} bytes read from {}.", read_size, &args.filepath);
        }, 
        Err(e) => {
            println!("{} could not be read into internal String because: {:?}", &args.filepath, e);
        }
    }

    let modify_fn = if args.inc == "i" {
        u32::checked_add
    } else {
        u32::checked_sub
    };

    let mut current_token: String = String::new();
    let mut result: String = String::new();
    let mut in_brackets: bool = false;

    let (start_range, end_range) = match split_to_u32(args.range) {
        Ok(tup) => (tup.0, tup.1),
        Err(e) => {
            return Err(anyhow!(e.to_string()));
        }
    };

    for c in content.chars() {
        match c {
            '{' => {
                in_brackets = true;
                result.push_str("{");
            },
            '}' => {
                in_brackets = false;
                result.push_str("}")
            },
            _ if !in_brackets && c.is_whitespace() => {
                // Attempt to modify the number if it's outside brackets
                if let Ok(num) = u32::from_str(&current_token) {
                    if num >= start_range && num <= end_range {
                        let res = modify_fn(num, args.count).unwrap_or(num);
                        result.push_str(&res.to_string());
                    } else {
                        result.push_str(&current_token);
                    }
                } else {
                    result.push_str(&current_token);
                }
                result.push(c); // Preserve whitespace
                current_token.clear();
            },
            _ if !in_brackets => current_token.push(c),
            _ => result.push(c), // Inside brackets, just add character
        }
    }

    // Process any remaining token
    if !current_token.is_empty() {
        if let Ok(num) = u32::from_str(&current_token) {
            if num >= start_range && num <= end_range {
                let res = modify_fn(num, args.count).unwrap_or(num);
                result.push_str(&res.to_string());
    } else {
                result.push_str(&current_token);
            }
        } else {
            result.push_str(&current_token);
        }
    }

    match write(&args.filepath, result) {
        Ok(()) => (),
        Err(e) => {
            return Err(anyhow!("Could not modify {}: {:?}", &args.filepath, e))
        }
    };

    let elapsed_time = start.elapsed();
    println!("{} processed in {:?}", &args.filepath, elapsed_time);
    Ok(())
}

fn split_to_u32(input: String) -> Result<(u32, u32)> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() != 2 {
        return Err(anyhow!("Input {:#?} does not contain exactly two numbers in the unsigned 32-bit range.", &input));
    }

    let first_number = parts[0].parse::<u32>();
    let second_number = parts[1].parse::<u32>();

    match (first_number, second_number) {
        (Ok(first), Ok(second)) => Ok((first, second)),
        (Ok(_first), Err(e)) => Err(anyhow!("Failed to parse second number {}: {:?}", parts[1], e)),
        (Err(e), Ok(_second)) => Err(anyhow!("Failed to parse first number {}: {:?}", parts[0], e)),
        (Err(e1), Err(e2)) => Err(anyhow!("Failed to parse numbers:\n{:?}\n{:?}", e1, e2)),
    }
}