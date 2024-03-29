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
Hash algorithm
*/
#[derive(Clone, Copy, Debug, clap::ValueEnum)]
pub enum Hash {
    Blake3,
    Sha256,
}

impl Hash {
    /**
    Hash a file and return the hash
    */
    pub fn hash_file<P: AsRef<Path>>(&self, file: P) -> Result<String> {
        match self {
            Self::Sha256 => file_sha256(file),
            Self::Blake3 => file_blake3(file),
        }
    }

    /**
    Return the file extension for the hash file
    */
    fn ext(&self) -> &str {
        match self {
            Self::Sha256 => "sha256",
            Self::Blake3 => "b3",
        }
    }

    /**
    Process a file

    If the hash file exists, hash the file, compare hashes, and return the result.

    If the hash file does not exist, hash the file, save the hash file, and return the result.
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

/**
Approaches for processing multiple files
*/
#[derive(Clone, Debug, clap::ValueEnum)]
pub enum ProcessOption {
    RayonParIter,
    SequentialForLoop,
    SequentialIter,
    Threading,
    Messaging,
}

impl ProcessOption {
    /**
    Process files with the given hash
    */
    pub fn run<P: AsRef<Path> + Clone + Send + Sync + 'static>(
        &self,
        files: &[P],
        hash: Hash,
    ) -> Vec<Result<String>> {
        match self {
            Self::SequentialForLoop => seq_for_loop(files, hash),
            Self::SequentialIter => seq_iter(files, hash),
            Self::Threading => threading(files, hash),
            Self::Messaging => messaging(files, hash),
            Self::RayonParIter => rayon_par_iter(files, hash),
        }
    }
}

/**
Process files with the given hash algorithm via seqential for loop
*/
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

/**
Process files with the given hash algorithm via seqential iterator
*/
pub fn seq_iter<P: AsRef<Path> + Clone + Send + Sync + 'static>(
    files: &[P],
    hash: Hash,
) -> Vec<Result<String>> {
    files.iter().map(|file| hash.process_file(file)).collect()
}

/**
Process files with the given hash algorithm via threading
*/
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

/**
Process files with the given hash algorithm via messaging
*/
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

/**
Process files with the given hash algorithm via Rayon parallel iterator
*/
pub fn rayon_par_iter<P: AsRef<Path> + Clone + Send + Sync + 'static>(
    files: &[P],
    hash: Hash,
) -> Vec<Result<String>> {
    files
        .par_iter()
        .map(|file| hash.process_file(file))
        .collect()
}
