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

[`criterion`]: https://crates.io/crates/criterion
