# build

* clippy
* `README.md`
* test

```
cargo build --release
```

# `README.md`

* `t/README.md`
* `Cargo.toml`
* `CHANGELOG.md`
* `src/**/*.rs`

```
cargo build --release
kapow {0} >{target}
```

# clippy

```
cargo clippy -- -D clippy::all
```

# test

```
cargo test -- --nocapture --test-threads=1
```

# bench

```
cargo bench
fd '.*\.b3' |xargs -rP0 rm
fd '.*\.sha256' |xargs -rP0 rm
cp target/criterion/SingleFile/report/violin.svg t/violin1.svg
cp target/criterion/ProcessOption/report/violin.svg t/violin2.svg
```

# check

```
cargo outdated --exit-code 1
cargo audit
```

# update

```
cargo upgrade --incompatible
cargo update
```

# install

* `README.md`

```
cargo install --path .
```

# uninstall

```
cargo uninstall $(toml get -r Cargo.toml package.name)
```

# install-deps

```
cargo install cargo-audit cargo-edit cargo-outdated cocomo dtg fd-find kapow tokei toml-cli
```

# scaffold

```bash -eo pipefail
if ! toml get -r Cargo.toml package.description >/dev/null; then
toml set Cargo.toml package.description "Insert a description here" >Cargo.toml.new
mv Cargo.toml.new Cargo.toml
echo Edit package description in Cargo.toml, then rerun \`mkrs scaffold\`.
exit 0
fi
mkdir -p t
if [ ! -e t/README.md ]; then
NAME=$(toml get -r Cargo.toml package.name)
ABOUT=$(toml get -r Cargo.toml package.description)
cat <<EOF >t/README.md
# About

$ABOUT

# Usage

~~~text
\$ $NAME -V
!run:../target/release/$NAME -V 2>&1
~~~

~~~text
\$ $NAME -h
!run:../target/release/$NAME -h 2>&1
~~~

!inc:../CHANGELOG.md

EOF
fi
if [ ! -e CHANGELOG.md ]; then
VERSION=$(toml get -r Cargo.toml package.version)
TODAY=$(dtg -n %Y-%m-%d)
cat <<EOF >CHANGELOG.md
# Changelog

* $VERSION ($TODAY): Initial release

EOF
fi
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

# full

* update
* check
* build
* install

