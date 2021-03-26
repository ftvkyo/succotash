# succotash
Finding similar images efficiently and more.

# Ops

## Linting:
Clippy is already installed with the toolchain.
```console
$ cargo clippy
```

## Autoformatting:
```console
$ rustup toolchain install nightly
$ rustup component add rustfmt --toolchain nightly
```
From now on, use this to fix stuff:
```console
$ cargo +nightly fmt
```
