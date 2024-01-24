# About

File hash checker represents a minimal solution meeting the following
requirements:

* Calculate the SHA256 hashes of one or more files in sequence (for loop,
  iterator) or parallel (threading, messaging, [`rayon`] parallel iterator)
* Provide library API
* Benchmarks via [`criterion`]
* Minimal dependencies: [`sha2`], [`anyhow`], [`clap`], [`rayon`]
* Provide a CLI utiility:
    * Simpler and more straightforward usage than `sha256sum`
    * Save the SHA256 hash to an adjacent `.sha256` file (if it doesn't already
      exist)
    * Compare the current SHA256 hash to an adjacent `.sha256` file (if it
      exists)

[`anyhow`]: https://crates.io/crates/anyhow
[`clap`]: https://crates.io/crates/clap
[`criterion`]: https://crates.io/crates/criterion
[`sha2`]: https://crates.io/crates/sha2
[`rayon`]: https://crates.io/crates/rayon

# Usage

```
$ fhc -h
!run:../target/release/fhc -h
```

# Example

1. Run `fhc` against one or more files to calculate and print the SHA256 hash
   for each file to stdout and save in adjacent `.sha256` files.
2. At any later point, run `fhc` against one or more files with adjacent
   `.sha256` files to re-calculate the SHA256 hash for each file and report `OK`
   if the hashes match and `FAILED` if the hashes do not match.

# Benchmark

![](violin1.svg)

![](violin2.svg)

!inc:../CHANGELOG.md

