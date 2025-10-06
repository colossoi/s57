use clap::Parser;
use s57::S57File;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "s57")]
#[command(about = "Parse and inspect S-57 Electronic Navigational Chart files", long_about = None)]
struct Cli {
    /// S-57 file to parse
    #[arg(value_name = "FILE")]
    file: PathBuf,

    /// Show verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Show detailed information about each record
    #[arg(short, long)]
    details: bool,
}

fn main() {
    let cli = Cli::parse();

    // Initialize logger
    if cli.verbose {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
    } else {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    }

    // Read the file
    let data = match std::fs::read(&cli.file) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            std::process::exit(1);
        }
    };

    println!("Parsing file: {}", cli.file.display());
    println!("File size: {} bytes", data.len());

    // Parse the S-57 file
    match S57File::from_bytes(&data) {
        Ok(file) => {
            let records = file.records();
            println!("Successfully parsed {} records", records.len());

            if cli.details {
                println!("\nRecord details:");
                for (i, record) in records.iter().enumerate() {
                    println!("\nRecord {}:", i);
                    println!("  Type: {}", if record.leader.is_ddr() { "DDR" } else { "DR" });
                    println!("  Length: {} bytes", record.leader.record_length);
                    println!("  Fields: {}", record.fields.len());

                    for field in &record.fields {
                        println!("    - {} ({} bytes)", field.tag, field.data.len());
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error parsing file: {}", e);
            std::process::exit(1);
        }
    }
}
