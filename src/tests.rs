use super::*;

#[test]
fn empty() {
    assert_eq!(
        sha256("tests/empty.txt").unwrap(),
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
    );
}

#[test]
fn good() {
    assert_eq!(
        sha256("tests/lorem.txt").unwrap(),
        "32f5be18c7eba9db87f0138604f64fd541f29b6aa940fb6db0b3255e5071bdd5",
    );
}

#[test]
fn file_does_not_exist() {
    assert_eq!(
        sha256("tests/none.txt").unwrap_err().to_string(),
        "No such file or directory (os error 2)"
    );
}
