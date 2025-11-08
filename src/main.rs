mod cli;
mod reader;
mod formatter;
mod writer;
mod types;

use std::process;
use cli::Args;
use reader::FileCollector;
use formatter::Formatter;
use writer::OutputWriter;

fn main() {
    let args = match Args::parse() {
        Ok(args) => args,
        Err(e) => {
            eprintln!("Error: {}", e);
            eprintln!("\nUsage: krnch <path> <format>");
            eprintln!("  path   - Directory or file path to process");
            eprintln!("  format - Output format: json, text, xml, yaml, csv");
            process::exit(1);
        }
    };

    let collector = FileCollector::new();
    let files = match collector.collect(&args.path) {
        Ok(files) => files,
        Err(e) => {
            eprintln!("Error collecting files: {}", e);
            process::exit(1);
        }
    };

    if files.is_empty() {
        eprintln!("No files found at path: {}", args.path.display());
        process::exit(1);
    }

    let formatter = Formatter::new(args.format);
    let output = match formatter.format(&files) {
        Ok(output) => output,
        Err(e) => {
            eprintln!("Error formatting output: {}", e);
            process::exit(1);
        }
    };

    let writer = OutputWriter::new();
    match writer.write(&output, &args.format) {
        Ok(path) => println!("Output written to: {}", path),
        Err(e) => {
            eprintln!("Error writing output: {}", e);
            process::exit(1);
        }
    }
}

