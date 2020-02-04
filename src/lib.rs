/*

Copyright 2020 qtfkwk <qtfkwk+fhc@gmail.com>

Permission is hereby granted, free of charge, to any person obtaining a copy of 
this software and associated documentation files (the "Software"), to deal in 
the Software without restriction, including without limitation the rights to 
use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies 
of the Software, and to permit persons to whom the Software is furnished to do 
so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all 
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR 
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, 
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE 
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER 
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, 
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE 
SOFTWARE.

*/

#[cfg(test)]
mod tests {
    use super::sha256;
    use std::io::Error;
    use std::io::ErrorKind;

    #[test]
    fn empty() -> Result<(), Error> {
        let r = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
        assert_eq!(sha256("tests/empty.txt")?, r);
        Ok(())
    }

    #[test]
    fn good() -> Result<(), Error> {
        let r = "32f5be18c7eba9db87f0138604f64fd541f29b6aa940fb6db0b3255e5071bdd5";
        assert_eq!(sha256("tests/lorem.txt")?, r);
        Ok(())
    }

    #[test]
    fn file_does_not_exist() {
        assert!(match sha256("tests/none.txt") {
            Err(ref e) => e.kind() == ErrorKind::NotFound,
            _ => false,
        });
    }
}

use sha2::Sha256;
use sha2::Digest;

use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::copy;
use std::io::Error;
use std::io::Write;
use std::iter::FromIterator;
use std::path::Path;
use std::process::exit;
use std::sync::mpsc;
use std::thread;

fn sha256(path: &str) -> Result<String, Error> {
    let mut file = File::open(&path)?;
    let mut hasher = Sha256::default();
    let _n = copy(&mut file, &mut hasher)?;
    Ok(format!("{:x}", hasher.result()))
}

fn banner() {
    println!("fhc (file hash checker), 0.1.0, 2020-02-04");
}

fn usage() {
    banner();
    println!("");
    println!("Usage: `fhc [-h|--help] [--version] [-p|--process OPTION] [file]`");
    println!("");
    println!("Option/argument       | Description");
    println!("----------------------|--------------------------------------------------");
    println!("`-h|--help`           | show help");
    println!("`--version`           | show version/date");
    println!("`-p|--process OPTION` | process option: messaging*, threading, sequential");
    println!("`file`                | one or more file paths");
    println!("");
}

pub fn cli() -> Result<(), Error> {
    let mut paths = Vec::new();
    let mut process = "messaging".to_string();
    let process_options: HashSet<String> = HashSet::from_iter(vec![
        "messaging".to_string(),
        "sequential".to_string(),
        "threading".to_string(),
    ]);
    let mut p = false;
    let mut errors = 0;

    for arg in env::args().skip(1) {
        if p {
            if process_options.contains(&arg) {
                process = arg;
                p = false;
            } else {
                eprintln!("ERROR: Invalid process option: `{}`!", arg);
                exit(1);
            }
        } else if arg == "-h" || arg == "--help" {
            usage();
            exit(0);
        } else if arg == "--version" {
            banner();
            exit(0);
        } else if arg == "-p" || arg == "--process" {
            p = true
        } else if arg.starts_with("-") {
            if Path::new(&arg).exists() {
                paths.push(arg);
            } else {
                eprintln!("ERROR: Invalid option: `{}`", arg);
                exit(1);
            }
        } else {
            paths.push(arg);
        }
    }

    let n_paths = paths.len();

    if n_paths < 1 {
        usage();
        exit(1);
    } else if n_paths == 1 || process == "sequential" {
        for path in paths {
            match process_file(&path) {
                Ok(r) => println!("{}", r),
                Err(e) => {
                    eprintln!("ERROR: {}", e);
                    errors += 1;
                },
            }
        }
    } else if process == "messaging" {
        let mut rxs = vec![];
        for path in paths {
            let (tx, rx) = mpsc::channel();
            rxs.push(rx);
            thread::spawn(move || {
                tx.send(process_file(&path)).unwrap()
            });
        }
        for rx in rxs {
            match rx.recv() {
                Ok(t) => match t {
                    Ok(r) => println!("{}", r),
                    Err(e) => {
                        eprintln!("ERROR: {}", e);
                        errors += 1;
                    },
                },
                Err(e) => {
                    eprintln!("ERROR: {:?}", e);
                    errors += 1;
                },
            }
        }
    } else if process == "threading" {
        let mut handles = vec![];
        for path in paths {
            handles.push(thread::spawn(move || {
                process_file(&path)
            }));
        }
        for handle in handles {
            match handle.join() {
                Ok(t) => match t {
                    Ok(r) => println!("{}", r),
                    Err(e) => {
                        eprintln!("ERROR: {}", e);
                        errors += 1;
                    },
                },
                Err(e) => {
                    eprintln!("ERROR: {:?}", e);
                    errors += 1;
                },
            }
        }
    } else {
        eprintln!("ERROR: Invalid process option: `{}`!", process);
        exit(1);
    }

    exit(errors);
}

fn err(e: Error, path: &str) -> Error {
    Error::new(e.kind(), format!("{} `{}`", e, path))
}

fn process_file(path: &str) -> Result<String, Error> {

    // Calculate the hash
    let hash = match sha256(&path) {
        Err(e) => return Err(err(e, path)),
        Ok(h) => h,
    };

    // Does the `.fhc` file exist?
    let result;
    let fhc = format!("{}.fhc", path);
    if Path::new(&fhc).exists() {
        // Yes, the `.fhc` file exists... so open it, read the expected hash,
        // and compare to the calculated hash.
        let file = File::open(&fhc).expect("Could not open file");
        let mut reader = BufReader::new(file);
        let mut expect = String::new();
        if let Err(e) = reader.read_line(&mut expect) {
            return Err(err(e, path));
        }
        if hash == expect.lines().next().unwrap() {
            result = format!("{}: OK", path);
        } else {
            result = format!("{}: FAILED", path);
        }
    } else {
        // No, the `.fhc` file does not exist... so save the hash to a new
        // `.fhc` file and print it to stdout.
        let mut file = match File::create(&fhc) {
            Err(e) => return Err(err(e, path)),
            Ok(h) => h,
        };
        if let Err(e) = file.write_all(format!("{}\n", hash).as_bytes()) {
            return Err(err(e, path));
        }
        result = format!("{}  {}", hash, path);
    }

    Ok(result)
}

