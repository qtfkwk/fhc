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

#[derive(Clone, Copy, Debug, clap::ValueEnum)]
pub enum Hash {
    Sha256,
    Blake3,
}

impl Hash {
    /*
    Hash a file and return the hash
    */
    pub fn hash_file<P: AsRef<Path>>(&self, file: P) -> Result<String> {
        match self {
            Self::Sha256 => file_sha256(file),
            Self::Blake3 => file_blake3(file),
        }
    }

    fn ext(&self) -> &str {
        match self {
            Self::Sha256 => "sha256",
            Self::Blake3 => "b3",
        }
    }

    /**
    Process a file

    If the hash file does not exist, hash the file, save the hash file, and return the result.

    If the hash file exists, hash the file, compare hashes, and return the result.
    */
    pub fn process_file<P: AsRef<Path>>(&self, file: P) -> Result<String> {
        let file = file.as_ref();

        // Calculate the hash
        let hash = self.hash_file(file)?;

        let ckfile = format!("{}.{}", file.display(), self.ext());
        Ok(if Path::new(&ckfile).exists() {
            // The hash file exists, so open it, read the expected hash, compare to the calculated
            // hash, and return the result.
            let ckfile = File::open(&ckfile).expect("Could not open file");
            let mut reader = BufReader::new(ckfile);
            let mut expected = String::new();
            reader.read_line(&mut expected)?;
            expected = expected.lines().next().unwrap().to_string();
            expected.truncate(expected.find(' ').unwrap_or(expected.len()));
            format!(
                "{}: {}",
                file.display(),
                if hash == expected { "OK" } else { "FAILED" }
            )
        } else {
            // The hash file does not exist, so save the hash to a new hash file and return it.
            let mut ckfile = File::create(&ckfile)?;
            let filename = Path::new(&file).file_name().unwrap().to_str().unwrap();
            let content = format!("{}  {}\n", hash, filename);
            ckfile.write_all(content.as_bytes())?;
            format!("{}  {}", hash, file.display())
        })
    }
}

/**
Calculate the SHA256 hash for a file
*/
pub fn file_sha256<P: AsRef<Path>>(file: P) -> Result<String> {
    let mut file = File::open(file)?;
    let mut hasher = Sha256::new();
    copy(&mut file, &mut hasher)?;
    Ok(format!("{:x}", hasher.finalize()))
}

/**
Calculate the BLAKE3 hash for a file
*/
pub fn file_blake3<P: AsRef<Path>>(file: P) -> Result<String> {
    let mut file = File::open(file)?;
    let mut hasher = blake3::Hasher::new();
    copy(&mut file, &mut hasher)?;
    Ok(format!("{}", hasher.finalize()))
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
        hash: Hash,
    ) -> Vec<Result<String>> {
        match self {
            SequentialForLoop => seq_for_loop(files, hash),
            SequentialIter => seq_iter(files, hash),
            Threading => threading(files, hash),
            Messaging => messaging(files, hash),
            RayonParIter => rayon_par_iter(files, hash),
        }
    }
}

pub fn seq_for_loop<P: AsRef<Path> + Clone + Send + Sync + 'static>(
    files: &[P],
    hash: Hash,
) -> Vec<Result<String>> {
    let mut r = vec![];
    for file in files {
        r.push(hash.process_file(file));
    }
    r
}

pub fn seq_iter<P: AsRef<Path> + Clone + Send + Sync + 'static>(
    files: &[P],
    hash: Hash,
) -> Vec<Result<String>> {
    files.iter().map(|file| hash.process_file(file)).collect()
}

pub fn threading<P: AsRef<Path> + Clone + Send + Sync + 'static>(
    files: &[P],
    hash: Hash,
) -> Vec<Result<String>> {
    let mut r = vec![];
    let mut handles = vec![];
    for file in files.iter().cloned() {
        handles.push(std::thread::spawn(move || hash.process_file(file)));
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
    hash: Hash,
) -> Vec<Result<String>> {
    let mut r = vec![];
    let mut rxs = vec![];
    for file in files.iter().cloned() {
        let (tx, rx) = std::sync::mpsc::channel();
        rxs.push(rx);
        std::thread::spawn(move || tx.send(hash.process_file(file)).unwrap());
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
    hash: Hash,
) -> Vec<Result<String>> {
    files
        .par_iter()
        .map(|file| hash.process_file(file))
        .collect()
}
