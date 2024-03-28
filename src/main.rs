use clap::Parser;
use std::fs::File;
use std::io::{Read, Write};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // Number of UUIDs to generate
    #[arg(short, long, default_value_t = 1, help = "Specifies the number of UUIDs to generate.")]
    count: u32,

    // Format of the UUIDs
    #[arg(short, long, default_value = "u", help = "Sets the format of the UUIDs. ('u' for bare, 'ul' for bare w. comma, 'q' for quoted, 'ql' for quoted w. comma, 'qlb' for [] brackets, 'qlbl' for {} brackets)")]
    format: String,

    // Output file
    #[arg(short, long, help = "Specifies an output file. Prints to stdout if not set.")]
    output: Option<String>,

    // Check for duplicates
    #[arg(long, help = "Goes over initial output to check for duplicates. Fixes silently.")]
    check: bool,

    // Verbose output
    #[arg(long, help = "Displays benchmarking info, also displays check results if applicable.")]
    verbose: bool,
}

fn main() {
    println!("Hello, world!");
}
