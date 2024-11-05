use super::*;

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
fn sha512_empty() {
    assert_eq!(
        file_sha512("tests/empty.txt").unwrap(),
        vec![(
            String::from("tests/empty.txt.sha512"),
            String::from("SHA512:cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e"),
        )],
    );
}

#[test]
fn sha512_good() {
    assert_eq!(
        file_sha512("tests/lorem.txt").unwrap(),
        vec![(
            String::from("tests/lorem.txt.sha512"),
            String::from("SHA512:2f8403b95fdb3ed42847fc6da7e3d7dfb7bbfb02a3d5fa04e2b1a585b31d4330dad3d425a3947757602b98246670ca44d8f7a8a62f97f7ce10fb6bf15ddf5b15"),
        )],
    );
}

#[test]
fn sha512_file_does_not_exist() {
    assert_eq!(
        file_sha512("tests/none.txt").unwrap_err().to_string(),
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
fn blake3_sha512_empty() {
    assert_eq!(
        file_blake3_sha512("tests/empty.txt").unwrap(),
        vec![
            (
                String::from("tests/empty.txt.b3"),
                String::from(
                    "BLAKE3:af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262",
                ),
            ),
            (
                String::from("tests/empty.txt.sha512"),
                String::from("SHA512:cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e",),
            ),
        ],
    );
}

#[test]
fn blake3_sha512_good() {
    assert_eq!(
        file_blake3_sha512("tests/lorem.txt").unwrap(),
        vec![
            (
                String::from("tests/lorem.txt.b3"),
                String::from(
                    "BLAKE3:726e07a80d19aa22a9cefe8e2aaf565d2aa906d915860fa51b0973f47ec347f6",
                ),
            ),
            (
                String::from("tests/lorem.txt.sha512"),
                String::from("SHA512:2f8403b95fdb3ed42847fc6da7e3d7dfb7bbfb02a3d5fa04e2b1a585b31d4330dad3d425a3947757602b98246670ca44d8f7a8a62f97f7ce10fb6bf15ddf5b15",),
            ),
        ],
    );
}

#[test]
fn blake3_sha512_file_does_not_exist() {
    assert_eq!(
        file_blake3_sha512("tests/none.txt")
            .unwrap_err()
            .to_string(),
        "No such file or directory (os error 2)"
    );
}

#[test]
fn sha256_sha512_empty() {
    assert_eq!(
        file_sha256_sha512("tests/empty.txt").unwrap(),
        vec![
            (
                String::from("tests/empty.txt.sha256"),
                String::from(
                    "SHA256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
                ),
            ),
            (
                String::from("tests/empty.txt.sha512"),
                String::from("SHA512:cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e",),
            ),
        ],
    );
}

#[test]
fn sha256_sha512_good() {
    assert_eq!(
        file_sha256_sha512("tests/lorem.txt").unwrap(),
        vec![
            (
                String::from("tests/lorem.txt.sha256"),
                String::from(
                    "SHA256:32f5be18c7eba9db87f0138604f64fd541f29b6aa940fb6db0b3255e5071bdd5",
                ),
            ),
            (
                String::from("tests/lorem.txt.sha512"),
                String::from("SHA512:2f8403b95fdb3ed42847fc6da7e3d7dfb7bbfb02a3d5fa04e2b1a585b31d4330dad3d425a3947757602b98246670ca44d8f7a8a62f97f7ce10fb6bf15ddf5b15",),
            ),
        ],
    );
}

#[test]
fn sha256_sha512_file_does_not_exist() {
    assert_eq!(
        file_sha256_sha512("tests/none.txt")
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
            (
                String::from("tests/empty.txt.sha512"),
                String::from("SHA512:cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e",),
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
            (
                String::from("tests/lorem.txt.sha512"),
                String::from("SHA512:2f8403b95fdb3ed42847fc6da7e3d7dfb7bbfb02a3d5fa04e2b1a585b31d4330dad3d425a3947757602b98246670ca44d8f7a8a62f97f7ce10fb6bf15ddf5b15",),
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
