use super::*;

#[test]
fn sha256_empty() {
    assert_eq!(
        file_sha256("tests/empty.txt").unwrap(),
        vec![(
            String::from("tests/empty.txt.sha256"),
            String::from("SHA256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"),
        )],
    );
}

#[test]
fn sha256_good() {
    assert_eq!(
        file_sha256("tests/lorem.txt").unwrap(),
        vec![(
            String::from("tests/lorem.txt.sha256"),
            String::from("SHA256:32f5be18c7eba9db87f0138604f64fd541f29b6aa940fb6db0b3255e5071bdd5"),
        )],
    );
}

#[test]
fn sha256_file_does_not_exist() {
    assert_eq!(
        file_sha256("tests/none.txt").unwrap_err().to_string(),
        "No such file or directory (os error 2)"
    );
}

#[test]
fn blake3_empty() {
    assert_eq!(
        file_blake3("tests/empty.txt").unwrap(),
        vec![(
            String::from("tests/empty.txt.b3"),
            String::from("BLAKE3:af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262"),
        )],
    );
}

#[test]
fn blake3_good() {
    assert_eq!(
        file_blake3("tests/lorem.txt").unwrap(),
        vec![(
            String::from("tests/lorem.txt.b3"),
            String::from("BLAKE3:726e07a80d19aa22a9cefe8e2aaf565d2aa906d915860fa51b0973f47ec347f6"),
        )],
    );
}

#[test]
fn blake3_file_does_not_exist() {
    assert_eq!(
        file_blake3("tests/none.txt").unwrap_err().to_string(),
        "No such file or directory (os error 2)"
    );
}

#[test]
fn blake3_sha256_empty() {
    assert_eq!(
        file_blake3_sha256("tests/empty.txt").unwrap(),
        vec![
            (
                String::from("tests/empty.txt.b3"),
                String::from(
                    "BLAKE3:af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262",
                ),
            ),
            (
                String::from("tests/empty.txt.sha256"),
                String::from(
                    "SHA256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
                ),
            ),
        ],
    );
}

#[test]
fn blake3_sha256_good() {
    assert_eq!(
        file_blake3_sha256("tests/lorem.txt").unwrap(),
        vec![
            (
                String::from("tests/lorem.txt.b3"),
                String::from(
                    "BLAKE3:726e07a80d19aa22a9cefe8e2aaf565d2aa906d915860fa51b0973f47ec347f6",
                ),
            ),
            (
                String::from("tests/lorem.txt.sha256"),
                String::from(
                    "SHA256:32f5be18c7eba9db87f0138604f64fd541f29b6aa940fb6db0b3255e5071bdd5",
                ),
            ),
        ],
    );
}

#[test]
fn blake3_sha256_file_does_not_exist() {
    assert_eq!(
        file_blake3_sha256("tests/none.txt")
            .unwrap_err()
            .to_string(),
        "No such file or directory (os error 2)"
    );
}

#[test]
fn all_empty() {
    assert_eq!(
        file_all("tests/empty.txt").unwrap(),
        vec![
            (
                String::from("tests/empty.txt.b3"),
                String::from(
                    "BLAKE3:af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262",
                ),
            ),
            (
                String::from("tests/empty.txt.sha256"),
                String::from(
                    "SHA256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
                ),
            ),
        ],
    );
}

#[test]
fn all_good() {
    assert_eq!(
        file_all("tests/lorem.txt").unwrap(),
        vec![
            (
                String::from("tests/lorem.txt.b3"),
                String::from(
                    "BLAKE3:726e07a80d19aa22a9cefe8e2aaf565d2aa906d915860fa51b0973f47ec347f6",
                ),
            ),
            (
                String::from("tests/lorem.txt.sha256"),
                String::from(
                    "SHA256:32f5be18c7eba9db87f0138604f64fd541f29b6aa940fb6db0b3255e5071bdd5",
                ),
            ),
        ],
    );
}

#[test]
fn all_file_does_not_exist() {
    assert_eq!(
        file_all("tests/none.txt").unwrap_err().to_string(),
        "No such file or directory (os error 2)"
    );
}
