# all

* clippy
* test
* build
* doc

# check

* outdated
* audit

# update

* update-toml
* update-lock

# run

* `target/release/{dirname}`

```
target/release/{dirname}
```

# clippy

* `Cargo.lock`
* `Cargo.toml`
* `**/*.rs`

```
cargo clippy -- -D clippy::all
```

# test

* `Cargo.lock`
* `Cargo.toml`
* `**/*.rs`

```
cargo test
```

# bench

```
cargo bench
fd '.*\.b3' |xargs -rP0 rm
fd '.*\.sha256' |xargs -rP0 rm
cp target/criterion/SingleFile/report/violin.svg t/violin1.svg
cp target/criterion/ProcessOption/report/violin.svg t/violin2.svg
```

# build

* `target/release/{dirname}`

# `target/release/{dirname}`

* `Cargo.lock`
* `Cargo.toml`
* `**/*.rs`
* `README.md`

```
cargo build --release
```

# `README.md`

* `t/README.md`
* `Cargo.toml`
* `CHANGELOG.md`
* `**/*.rs`

```
cargo build --release
kapow {0} >{target}
```

# doc

```
cargo doc
```

# outdated

```
cargo outdated --exit-code=1
```

# audit

```
cargo audit
```

# update-toml

```
cargo upgrade -i
```

# update-lock

```
cargo update
```

# install

* `README.md`

```
cargo install --path .
```

# uninstall

```
cargo uninstall {dirname}
```

# install-deps

```
cargo install cargo-audit cargo-edit cargo-outdated cocomo dtg kapow tokei toml-cli
```

# clean

```
cargo clean
```

# cocomo

```bash -eo pipefail
tokei; echo
cocomo -o sloccount
cocomo
```

# commit

```bash
set -xeo pipefail
V=$(toml get -r Cargo.toml package.version)
git commit -m "$V"
git tag -a "$V" -m "$V"
```

# publish

```
cargo publish
git push
git push --tags
```

# full

* update
* check
* all
* install

