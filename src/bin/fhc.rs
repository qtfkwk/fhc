use anyhow::Result;
use clap::Parser;
use fhc::*;
use std::path::PathBuf;

#[derive(Parser)]
#[command(about, version, max_term_width = 80)]
struct Cli {
    /// Process option
    #[arg(short, long, default_value = "messaging")]
    process: ProcessOption,

    /// File(s)
    files: Vec<PathBuf>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let process = if cli.files.len() == 1 {
        SequentialForLoop
    } else {
        cli.process.clone()
    };

    for result in process.run(&cli.files) {
        match result {
            Ok(result) => println!("{result}"),
            Err(e) => eprintln!("ERROR: {e}"),
        }
    }

    Ok(())
}
