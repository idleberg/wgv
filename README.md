# wgv

![Crates.io License](https://img.shields.io/crates/l/wgv?style=for-the-badge)
[![Crates.io Version](https://img.shields.io/crates/v/wgv?style=for-the-badge)](https://crates.io/crates/wgv)
[![CI](https://img.shields.io/github/actions/workflow/status/idleberg/wgv/ci.yml?style=for-the-badge)](https://github.com/idleberg/wgv/actions)

> Cross-platform winget manifest validator.

## Description

The official Winget [CLI](https://github.com/microsoft/winget-cli/) provides a command to validate manifests. This is quite useful when you're on Windows. `wgv` is an implementation of the validator written in Rust and available for Linux and macOS. It supports manifests version `1.0.0` up to `1.28.0` and was extensively tested against the Winget [package repository](https://github.com/microsoft/winget-pkgs).

## Installation

### Cargo

```shell
cargo install wgv
```

### Homebrew

```shell
brew install idleberg/asahi/wgv
```

### Nix

```shell
nix profile install github:idleberg/wgv
```

### Source

```shell
git clone https://github.com/idleberg/wgv.git
cd wgv
cargo build --release
```

The binary is at `target/release/wgv`.

## Usage

```
Cross-platform winget manifest validator

Usage: wgv [OPTIONS] <MANIFESTS>...

Arguments:
  <MANIFESTS>...  Paths or glob patterns to manifest files or directories

Options:
      --log-level <LOG_LEVEL>  Set log level [possible values: error, warn, info] [default: info]
  -q, --quiet                  Only show errors (shorthand for --log-level error)
  -h, --help                   Print help
  -V, --version                Print version
```

## License

This work is licensed under [The MIT License](LICENSE).
