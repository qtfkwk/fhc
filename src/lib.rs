use {
    anyhow::{Result, anyhow},
    clap::ValueEnum,
    rayon::prelude::*,
    sha2::{Digest, Sha256, Sha512},
    std::{
        fs::File,
        io::{BufRead, BufReader, Read, Write, copy},
        path::Path,
    },
};

#[cfg(test)]
mod tests;

/// Hash algorithm
#[derive(Clone, Copy, Debug, clap::ValueEnum)]
pub enum Hash {
    Blake3,
    Sha256,
    Sha512,
    Blake3Sha256,
    Blake3Sha512,
    Sha256Sha512,
    All,
}

impl Hash {
    /**
    Hash a file and return the hash(es) as `(ckfile, hash)` tuples

    # Errors

    Returns an error if not able to read the given file
    */
    pub fn hash_file<P: AsRef<Path>>(&self, file: P) -> Result<Vec<(String, String)>> {
        match self {
            Hash::Blake3 => file_blake3(file),
            Hash::Sha256 => file_sha256(file),
            Hash::Sha512 => file_sha512(file),
            Hash::Blake3Sha256 => file_blake3_sha256(file),
            Hash::Blake3Sha512 => file_blake3_sha512(file),
            Hash::Sha256Sha512 => file_sha256_sha512(file),
            Hash::All => file_all(file),
        }
    }

    /**
    Process a file

    If the hash file exists, hash the file, compare hashes, and return the result.

    If the hash file does not exist, hash the file, save the hash file, and return the result.

    # Errors

    Returns an error if not able to process the given file
    */
    #[allow(clippy::missing_panics_doc)]
    pub fn process_file<P: AsRef<Path>>(&self, file: P) -> Result<String> {
        let file = file.as_ref();

        // Calculate the hashes
        let hashes = self.hash_file(file)?;

        Ok(if let Ok(expected) = self.expected(file) {
            // The hash file(s) exist, so verify them and return the result.
            format!(
                "{}: {}",
                file.display(),
                if hashes == expected { "OK" } else { "FAILED" },
            )
        } else {
            // The hash file(s) do not exist, so save the hash(es) to new hash file(s), and return it.
            let mut r = vec![];
            for (ckfile, hash) in &hashes {
                let mut ckfile = File::create(ckfile)?;
                let filename = file.file_name().unwrap().to_str().unwrap();
                let content = format!("{hash}  {filename}\n");
                ckfile.write_all(content.as_bytes())?;
                r.push(format!("{hash}  {}", file.display()));
            }
            r.join("\n")
        })
    }

    /**
    Get the expected hash(es) from hash file(s)

    # Panics

    Panics if not able to get the expected hash from the hash file(s)

    # Errors

    Returns an error if not able to get the expected hash from the hash file(s)
    */
    pub fn expected<P: AsRef<Path>>(&self, file: P) -> Result<Vec<(String, String)>> {
        let file = file.as_ref();

        let mut r = vec![];

        let ckfiles = match self {
            Hash::Blake3 => vec![format!("{}.b3", file.display())],
            Hash::Sha256 => vec![format!("{}.sha256", file.display())],
            Hash::Sha512 => vec![format!("{}.sha512", file.display())],
            Hash::Blake3Sha256 => vec![
                format!("{}.b3", file.display()),
                format!("{}.sha256", file.display()),
            ],
            Hash::Blake3Sha512 => vec![
                format!("{}.b3", file.display()),
                format!("{}.sha512", file.display()),
            ],
            Hash::Sha256Sha512 => vec![
                format!("{}.sha256", file.display()),
                format!("{}.sha512", file.display()),
            ],
            Hash::All => vec![
                format!("{}.b3", file.display()),
                format!("{}.sha256", file.display()),
                format!("{}.sha512", file.display()),
            ],
        };

        for ckfile in ckfiles {
            let mut reader = BufReader::new(File::open(&ckfile)?);
            let mut expected = String::new();
            reader.read_line(&mut expected)?;
            expected = expected.lines().next().unwrap().to_string();
            expected.truncate(expected.find(' ').unwrap_or(expected.len()));
            r.push((ckfile, expected));
        }

        Ok(r)
    }
}

/**
Calculate the SHA256 hash for a file

# Errors

Returns an error if not able to read the given file
*/
pub fn file_sha256<P: AsRef<Path>>(file: P) -> Result<Vec<(String, String)>> {
    let file = file.as_ref();
    let mut f = File::open(file)?;
    let mut hasher = Sha256::new();
    copy(&mut f, &mut hasher)?;
    Ok(vec![(
        format!("{}.sha256", file.display()),
        format!("SHA256:{:x}", hasher.finalize()),
    )])
}

/**
Calculate the SHA512 hash for a file

# Errors

Returns an error if not able to read the given file
*/
pub fn file_sha512<P: AsRef<Path>>(file: P) -> Result<Vec<(String, String)>> {
    let file = file.as_ref();
    let mut f = File::open(file)?;
    let mut hasher = Sha512::new();
    copy(&mut f, &mut hasher)?;
    Ok(vec![(
        format!("{}.sha512", file.display()),
        format!("SHA512:{:x}", hasher.finalize()),
    )])
}

/**
Calculate the BLAKE3 hash for a file

# Errors

Returns an error if not able to read the given file
*/
pub fn file_blake3<P: AsRef<Path>>(file: P) -> Result<Vec<(String, String)>> {
    let file = file.as_ref();
    let mut f = File::open(file)?;
    let mut hasher = blake3::Hasher::new();
    copy(&mut f, &mut hasher)?;
    Ok(vec![(
        format!("{}.b3", file.display()),
        format!("BLAKE3:{}", hasher.finalize()),
    )])
}

/**
Calculate the BLAKE3 and SHA256 hashes for a file

# Errors

Returns an error if not able to read the given file
*/
pub fn file_blake3_sha256<P: AsRef<Path>>(file: P) -> Result<Vec<(String, String)>> {
    let file = file.as_ref();
    let mut f = File::open(file)?;
    let mut buf = vec![];
    f.read_to_end(&mut buf)?;

    let mut hasher_b3 = blake3::Hasher::new();
    hasher_b3.update(&buf);

    let mut hasher_sha256 = Sha256::new();
    hasher_sha256.update(&buf);

    Ok(vec![
        (
            format!("{}.b3", file.display()),
            format!("BLAKE3:{}", hasher_b3.finalize()),
        ),
        (
            format!("{}.sha256", file.display()),
            format!("SHA256:{:x}", hasher_sha256.finalize()),
        ),
    ])
}

/**
Calculate the BLAKE3 and SHA512 hashes for a file

# Errors

Returns an error if not able to read the given file
*/
pub fn file_blake3_sha512<P: AsRef<Path>>(file: P) -> Result<Vec<(String, String)>> {
    let file = file.as_ref();
    let mut f = File::open(file)?;
    let mut buf = vec![];
    f.read_to_end(&mut buf)?;

    let mut hasher_b3 = blake3::Hasher::new();
    hasher_b3.update(&buf);

    let mut hasher_sha512 = Sha512::new();
    hasher_sha512.update(&buf);

    Ok(vec![
        (
            format!("{}.b3", file.display()),
            format!("BLAKE3:{}", hasher_b3.finalize()),
        ),
        (
            format!("{}.sha512", file.display()),
            format!("SHA512:{:x}", hasher_sha512.finalize()),
        ),
    ])
}

/**
Calculate the SHA256 and SHA512 hashes for a file

# Errors

Returns an error if not able to read the given file
*/
pub fn file_sha256_sha512<P: AsRef<Path>>(file: P) -> Result<Vec<(String, String)>> {
    let file = file.as_ref();
    let mut f = File::open(file)?;
    let mut buf = vec![];
    f.read_to_end(&mut buf)?;

    let mut hasher_sha256 = Sha256::new();
    hasher_sha256.update(&buf);

    let mut hasher_sha512 = Sha512::new();
    hasher_sha512.update(&buf);

    Ok(vec![
        (
            format!("{}.sha256", file.display()),
            format!("SHA256:{:x}", hasher_sha256.finalize()),
        ),
        (
            format!("{}.sha512", file.display()),
            format!("SHA512:{:x}", hasher_sha512.finalize()),
        ),
    ])
}

/**
Calculate all hashes for a file

# Errors

Returns an error if not able to read the given file
*/
pub fn file_all<P: AsRef<Path>>(file: P) -> Result<Vec<(String, String)>> {
    let file = file.as_ref();
    let mut f = File::open(file)?;
    let mut buf = vec![];
    f.read_to_end(&mut buf)?;

    let mut hasher_b3 = blake3::Hasher::new();
    hasher_b3.update(&buf);

    let mut hasher_sha256 = Sha256::new();
    hasher_sha256.update(&buf);

    let mut hasher_sha512 = Sha512::new();
    hasher_sha512.update(&buf);

    Ok(vec![
        (
            format!("{}.b3", file.display()),
            format!("BLAKE3:{}", hasher_b3.finalize()),
        ),
        (
            format!("{}.sha256", file.display()),
            format!("SHA256:{:x}", hasher_sha256.finalize()),
        ),
        (
            format!("{}.sha512", file.display()),
            format!("SHA512:{:x}", hasher_sha512.finalize()),
        ),
    ])
}

/// Approaches for processing multiple files
#[derive(Clone, Debug, ValueEnum)]
pub enum ProcessOption {
    RayonParIter,
    SequentialForLoop,
    SequentialIter,
    Threading,
    Messaging,
}

impl ProcessOption {
    /// Process files with the given hash
    pub fn run<P: AsRef<Path> + Clone + Send + Sync + 'static>(
        &self,
        files: &[P],
        hash: Hash,
    ) -> Vec<Result<String>> {
        match self {
            ProcessOption::SequentialForLoop => seq_for_loop(files, hash),
            ProcessOption::SequentialIter => seq_iter(files, hash),
            ProcessOption::Threading => threading(files, hash),
            ProcessOption::Messaging => messaging(files, hash),
            ProcessOption::RayonParIter => rayon_par_iter(files, hash),
        }
    }
}

/// Process files with the given hash algorithm via seqential for loop
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

/// Process files with the given hash algorithm via seqential iterator
pub fn seq_iter<P: AsRef<Path> + Clone + Send + Sync + 'static>(
    files: &[P],
    hash: Hash,
) -> Vec<Result<String>> {
    files.iter().map(|file| hash.process_file(file)).collect()
}

/// Process files with the given hash algorithm via threading
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

# Panics

Panics if not able to spawn a thread
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

/// Process files with the given hash algorithm via Rayon parallel iterator
pub fn rayon_par_iter<P: AsRef<Path> + Clone + Send + Sync + 'static>(
    files: &[P],
    hash: Hash,
) -> Vec<Result<String>> {
    files
        .par_iter()
        .map(|file| hash.process_file(file))
        .collect()
}
