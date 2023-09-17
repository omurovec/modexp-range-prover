## Quick Start

First, make sure [rustup](https://rustup.rs) is installed. This project uses a [nightly](https://doc.rust-lang.org/book/appendix-07-nightly-rust.html) version of [Rust](https://doc.rust-lang.org/book/ch01-01-installation.html). The [`rust-toolchain`](rust-toolchain) file will be used by `cargo` to automatically install the correct version.

To install the risczero toolchain, run:

```sh
cargo install cargo-risczero
cargo risczero install
```

To build all methods and execute the method within the zkVM, run the following command:

```sh
cargo run
```

To perform tests, run:

```sh
cargo test
```
