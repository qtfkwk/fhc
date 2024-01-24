use anyhow::{anyhow, Result};
use rayon::prelude::*;
use sha2::{Digest, Sha256};
use std::{
    fs::File,
    io::{copy, BufRead, BufReader, Write},
    path::Path,
};

#[cfg(test)]
mod tests;

/**
Calculate the SHA256 hash for a file
*/
pub fn sha256<P: AsRef<Path>>(file: P) -> Result<String> {
    let mut file = File::open(file)?;
    let mut hasher = Sha256::new();
    copy(&mut file, &mut hasher)?;
    Ok(format!("{:x}", hasher.finalize()))
}

/**
Process a file
*/
pub fn process_file<P: AsRef<Path>>(file: P) -> Result<String> {
    let file = file.as_ref();

    // Calculate the SHA256 hash
    let hash = sha256(file)?;

    let ckfile = format!("{}.sha256", file.display());
    Ok(if Path::new(&ckfile).exists() {
        // The `.sha256` file exists, so open it, read the expected hash,
        // compare to the calculated hash, and return the result.
        let ckfile = File::open(&ckfile).expect("Could not open file");
        let mut reader = BufReader::new(ckfile);
        let mut expected = String::new();
        reader.read_line(&mut expected)?;
        expected = expected.lines().next().unwrap().to_string();
        expected.truncate(64);
        format!(
            "{}: {}",
            file.display(),
            if hash == expected { "OK" } else { "FAILED" }
        )
    } else {
        // The `.sha256` file does not exist, so save the hash to a new
        // `.sha256` file and return it.
        let mut ckfile = File::create(&ckfile)?;
        let filename = Path::new(&file).file_name().unwrap().to_str().unwrap();
        let content = format!("{}  {}\n", hash, filename);
        ckfile.write_all(content.as_bytes())?;
        format!("{}  {}", hash, file.display())
    })
}

#[derive(Clone, Debug, clap::ValueEnum)]
pub enum ProcessOption {
    SequentialForLoop,
    SequentialIter,
    Threading,
    Messaging,
    RayonParIter,
}

pub use ProcessOption::*;

impl ProcessOption {
    pub fn run<P: AsRef<Path> + Clone + Send + Sync + 'static>(
        &self,
        files: &[P],
    ) -> Vec<Result<String>> {
        match self {
            SequentialForLoop => seq_for_loop(files),
            SequentialIter => seq_iter(files),
            Threading => threading(files),
            Messaging => messaging(files),
            RayonParIter => rayon_par_iter(files),
        }
    }
}

pub fn seq_for_loop<P: AsRef<Path> + Clone + Send + Sync + 'static>(
    files: &[P],
) -> Vec<Result<String>> {
    let mut r = vec![];
    for file in files {
        r.push(process_file(file));
    }
    r
}

pub fn seq_iter<P: AsRef<Path> + Clone + Send + Sync + 'static>(
    files: &[P],
) -> Vec<Result<String>> {
    files.iter().map(process_file).collect()
}

pub fn threading<P: AsRef<Path> + Clone + Send + Sync + 'static>(
    files: &[P],
) -> Vec<Result<String>> {
    let mut r = vec![];
    let mut handles = vec![];
    for file in files.iter().cloned() {
        handles.push(std::thread::spawn(move || process_file(file)));
    }
    for handle in handles {
        match handle.join() {
            Ok(t) => {
                r.push(t);
            }
            Err(e) => {
                r.push(Err(anyhow!(format!("{e:?}"))));
            }
        }
    }
    r
}

pub fn messaging<P: AsRef<Path> + Clone + Send + Sync + 'static>(
    files: &[P],
) -> Vec<Result<String>> {
    let mut r = vec![];
    let mut rxs = vec![];
    for file in files.iter().cloned() {
        let (tx, rx) = std::sync::mpsc::channel();
        rxs.push(rx);
        std::thread::spawn(move || tx.send(process_file(file)).unwrap());
    }
    for rx in rxs {
        match rx.recv() {
            Ok(t) => {
                r.push(t);
            }
            Err(e) => {
                r.push(Err(anyhow!(format!("{e:?}"))));
            }
        }
    }
    r
}

pub fn rayon_par_iter<P: AsRef<Path> + Clone + Send + Sync + 'static>(
    files: &[P],
) -> Vec<Result<String>> {
    files.par_iter().map(process_file).collect()
}
