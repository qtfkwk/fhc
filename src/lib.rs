use anyhow::Result;
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
