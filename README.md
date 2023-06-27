# Spat 𐃸

[<img alt="crates.io" src="https://img.shields.io/crates/v/spat?style=for-the-badge">](https://crates.io/crates/spat)
[<img alt="docs.rs" src="https://img.shields.io/docsrs/spat?style=for-the-badge">](https://docs.rs/spat/latest/spat)


This is a command-line utility for shortening UNIX-like paths which also provides
the [`spat::shorten`](https://docs.rs/spat/latest/spat/fn.shorten.html) function capable of doing the same.

## Install

`spat` can be built and installed using `cargo`.
```console
$ cargo install spat
```

## Usage

Run `spat --help` to view usage information. Basic usage is shown below.
```console
$ spat ~/.local/share/cargo/bin
~/.l/s/c/bin
```

## Library

```toml
[dependencies]
spat = { version = 0.2, default-features = false }
```
