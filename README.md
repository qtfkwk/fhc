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
File hash checker (SHA256, BLAKE3)

Usage: fhc [OPTIONS] [FILES]...

Arguments:
  [FILES]...  File(s)

Options:
  -a <ALGORITHM>           Algorithm [default: sha256] [possible values: sha256,
                           blake3]
  -p, --process <PROCESS>  Process option [default: messaging] [possible values:
                           sequential-for-loop, sequential-iter, threading,
                           messaging, rayon-par-iter]
  -h, --help               Print help
  -V, --version            Print version
```

# Example

1. Run `fhc` against one or more files to calculate and print the SHA256 hash
   for each file to stdout and save in adjacent `.sha256` files.
2. At any later point, run `fhc` against one or more files with adjacent
   `.sha256` files to re-calculate the SHA256 hash for each file and report `OK`
   if the hashes match and `FAILED` if the hashes do not match.

# Benchmark

![](t/violin1.svg)

![](t/violin2.svg)

# Changelog

* 0.1.0 (2020-02-03): Initial release
* 0.2.1 (2020-02-09): Produce `.sha256` files compatible w/ `sha256sum -c`
* 0.2.2 (2020-11-18): Update sha2 crate version
* 0.3.0 (2024-01-07): Modernize
* 0.4.0 (2024-01-24): Rename `sequential` process option to `sequentialforloop`;
  add `sequentialiter` and `rayonpariter` process options; move most of process
  option logic from CLI to library via process option functions and the process
  option `run` method; add [`criterion`] benchmarks; fix changelog; update
  dependencies
    * 0.4.1 (2024-01-24): Fix readme
    * 0.4.2 (2024-01-24): Fix changelog
* 0.5.0 (2024-01-24): Add [`blake3`] and `-a` option

[`criterion`]: https://crates.io/crates/criterion
[`blake3`]: https://crates.io/crates/blake3

