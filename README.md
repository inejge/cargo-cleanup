# Cached package cleanup for Cargo

[Cargo](https://doc.rust-lang.org/cargo/) keeps a local cache of downloaded
dependencies. Over time, as dependencies are updated, this cache can grow
pretty large, and full of outdated package versions which are unlikely to be
used ever again. This tool can help you discover which packages are no longer
needed. In the present, proof-of-concept form, only packages published on
crates.io and present in the default registry are considered, and no automatic
deletion is supported.

## Installation

```sh
$ cargo install cargo-cleanup
```

## Usage

Run

```sh
$ cargo cleanup
```

in the top-level directory of some crate. This will automatically try to use
`Cargo.lock` from that directory. (Alternatively, you can specify the path of
`Cargo.lock` as an argument to the command.) The program will read the names
and versions of all dependencies, then scan the Cargo unpacked crate cache
(`$HOME/.cargo/registry/src/*/...`) and list the packages whose names are
in the list, but versions aren't. For example, if your crate uses `rand-0.3.22`
and `rand-0.4.3`, and the cache contains those two plus `rand-0.5.5`, only
`rand-0.5.5` will be listed.

## Caveat

This is a proof of concept. There is no automatic deletion. Packages retrieved
directly from their repositories are not considered. Neither are alternative
registries.

## License

Licensed under either of:

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE)), or
 * MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.
