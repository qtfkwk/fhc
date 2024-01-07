use anyhow::Result;
use clap::Parser;
use fhc::*;
use std::path::PathBuf;

#[derive(Clone, Debug, clap::ValueEnum)]
enum ProcessOption {
    Sequential,
    Threading,
    Messaging,
}

use ProcessOption::*;

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
        Sequential
    } else {
        cli.process.clone()
    };

    match process {
        Sequential => {
            for file in &cli.files {
                match process_file(file) {
                    Ok(r) => println!("{}", r),
                    Err(e) => {
                        eprintln!("ERROR: {}", e);
                    }
                }
            }
        }
        Threading => {
            let mut handles = vec![];
            for file in cli.files.iter().cloned() {
                handles.push(std::thread::spawn(move || process_file(file)));
            }
            for handle in handles {
                match handle.join() {
                    Ok(t) => match t {
                        Ok(r) => println!("{}", r),
                        Err(e) => {
                            eprintln!("ERROR: {}", e);
                        }
                    },
                    Err(e) => {
                        eprintln!("ERROR: {:?}", e);
                    }
                }
            }
        }
        Messaging => {
            let mut rxs = vec![];
            for file in cli.files.iter().cloned() {
                let (tx, rx) = std::sync::mpsc::channel();
                rxs.push(rx);
                std::thread::spawn(move || tx.send(process_file(file)).unwrap());
            }
            for rx in rxs {
                match rx.recv() {
                    Ok(t) => match t {
                        Ok(r) => println!("{}", r),
                        Err(e) => {
                            eprintln!("ERROR: {}", e);
                        }
                    },
                    Err(e) => {
                        eprintln!("ERROR: {:?}", e);
                    }
                }
            }
        }
    }

    Ok(())
}
