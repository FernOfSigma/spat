# Spat 𐃸

This is a command-line utility for shortening UNIX-like paths which also provides
the [`spat::shorten`](https://docs.rs/spat) function capable of doing the same.

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

Run this to use `spat::shorten` without installing the binary.
```console
$ cargo add spat --no-default-features
```

Alternatively, add this line to `[dependencies]` in your `Cargo.toml` file.
```toml
spat = { version = 0.2, default-features = false }
```
