use anyhow::Result;
use clap::{CommandFactory, Parser};
use fhc::*;
use std::path::PathBuf;

#[derive(Parser)]
#[command(about, version, max_term_width = 80)]
struct Cli {
    /// Algorithm
    #[arg(short, default_value = "blake3")]
    algorithm: Hash,

    /// Approach for processing multiple files
    #[arg(short, long, default_value = "rayon-par-iter")]
    process: ProcessOption,

    /// File(s)
    files: Vec<PathBuf>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Print help if no files or arguments
    if cli.files.is_empty() {
        let mut cmd = Cli::command();
        cmd.build();
        cmd.print_help().unwrap();
        return Ok(());
    }

    let process = if cli.files.len() == 1 {
        ProcessOption::SequentialForLoop
    } else {
        cli.process.clone()
    };

    for result in process.run(&cli.files, cli.algorithm) {
        match result {
            Ok(result) => println!("{result}"),
            Err(e) => eprintln!("ERROR: {e}"),
        }
    }

    Ok(())
}
