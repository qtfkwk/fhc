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
    * 0.5.1 (2024-01-24): Fix readme
    * 0.5.2 (2024-01-25): Fix doc
    * 0.5.3 (2024-01-25): Fix readme
* 0.6.0 (2024-01-25): Change CLI defaults to `-a blake3` and `-p rayon-par-iter`
* 0.7.0 (2024-07-26): Print help if zero files; fix makefile; update dependencies

[`criterion`]: https://crates.io/crates/criterion
[`blake3`]: https://crates.io/crates/blake3

