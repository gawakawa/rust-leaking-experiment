# Rust Leaking Experiment

An experimental repository for exploring memory leaks in Rust, based on the [Rustonomicon's Leaking chapter](https://doc.rust-lang.org/nomicon/leaking.html).

## Features

- **drain**: `mem::forget` on `Drain` iterator causing use-after-free
- **Rc memory leak**: Normal `drop` vs `mem::forget` on custom `Rc`
- **Rc use after free**: ref_count overflow (u8) leading to premature deallocation

## Usage

```bash
nix run
```
