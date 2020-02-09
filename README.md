# About

File hash checker ("fhc") represents a minimal solution meeting the following
requirements:

* Calculate the SHA256 hash of one or more files in parallel
* Minimal external dependencies:
    * [sha2](https://crates.io/crates/sha2)
* Provide a library function than returns the SHA256 hash as a hex string for a
  string path
* Provide a CLI utiility:
    * Simpler and more straightforward usage than `sha256sum`
    * Save the SHA256 hash to an adjacent `.sha256` file (if it doesn't already
      exist)
    * Compare the current SHA256 hash to an adjacent `.sha256` file (if it
      exists)

# Installation

## User

```bash
cargo install fhc
```

## Developer

```bash
git clone https://github.com/qtfkwk/fhc.git
cd fhc
cargo install --path=.
```

# Usage

```
$ fhc -h
# fhc (file hash checker), 0.2.0, 2020-02-09

Usage: `fhc [-h|--help] [--version] [-p|--process OPTION] [file]`

Option/argument       | Description
----------------------|--------------------------------------------------
`-h|--help`           | show help
`--version`           | show version/date
`-p|--process OPTION` | process option: messaging*, threading, sequential
`file`                | one or more file paths

```

# Example

1. Run `fhc` against one or more files to calculate and print the SHA256 hash
   for each file to stdout and save in adjacent `.sha256` files.
2. At any later point, run `fhc` against one or more files with adjacent
   `.sha256` files to re-calculate the SHA256 hash for each file and report `OK`
   if the hashes match and `FAILED` if the hashes do not match.

# Tests

The [test.log](test.log) file saves the output of
`cargo test 2>&1 |tee test.log` at each commit.

# Changelog

* 0.1.0 (2020-02-03): initial public release
* 0.2.0 (2020-02-09): produce `.sha256` files compatible w/ `sha256sum -c`

