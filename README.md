# Google Authenticator in Rust 
[![Build Status](https://travis-ci.org/WANG-lp/gauth-rust.svg?branch=master)](https://travis-ci.org/WANG-lp/gauth-rust)

This repo implementes a basic Google authenticator in Rust programming language.

NOTICE: this repo is heavily influenced by the [gauth](https://github.com/pcarrier/gauth) project.


## Build:

You need the latest Rust toolchain: https://www.rust-lang.org/learn/get-started

```bash
$ git clone git@github.com:WANG-lp/gauth-rust.git
$ cd gauth-rust
$ cargo build --release
$ cp gauth.csv ~/.config/gauth.csv
$ vim ~/.config/gauth.csv #modify and add your own secrets
$ ./target/release/gauth-rust
```

## Usage:

- Get your secrets of your services, for example, `234567qrstuvwxyz` for Github.

- Store one secret per line in `~/.config/gauth.csv`, in the format name:secret. For example:

```
Github:234567qrstuvwxyz
Test:ABCDEFGHIJKLMNOPQRSTUVWXYZ234567ABCDEFGHIJKLMNOPQRSTUVWXYZ234567
```

- Run `gauth-rust`:

```
Github: 905867
Test: 813833
```

