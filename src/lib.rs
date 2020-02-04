#[cfg(test)]
mod tests {
    use super::sha256;

    #[test]
    fn empty() -> Result<(), std::io::Error> {
        let r = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
        assert_eq!(sha256("tests/empty.txt")?, r);
        Ok(())
    }

    #[test]
    fn good() -> Result<(), std::io::Error> {
        let r = "32f5be18c7eba9db87f0138604f64fd541f29b6aa940fb6db0b3255e5071bdd5";
        assert_eq!(sha256("tests/lorem.txt")?, r);
        Ok(())
    }

    #[test]
    fn file_does_not_exist() {
        assert!(match sha256("tests/none.txt") {
            Err(ref e) => e.kind() == std::io::ErrorKind::NotFound,
            _ => false,
        });
    }
}

use sha2::Sha256;
use sha2::Digest;

fn sha256(path: &str) -> Result<String, std::io::Error> {
    let mut file = std::fs::File::open(&path)?;
    let mut hasher = Sha256::default();
    let _n = std::io::copy(&mut file, &mut hasher)?;
    Ok(format!("{:x}", hasher.result()))
}

