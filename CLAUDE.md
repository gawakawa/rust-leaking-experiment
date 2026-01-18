# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

Experimental repository for exploring memory leaks in Rust, based on the [Rustonomicon's Leaking chapter](https://doc.rust-lang.org/nomicon/leaking.html).

## Build Commands

```bash
nix build            # Build the project
nix run              # Run the binary
nix flake check      # Run all checks (clippy, tests, fmt, audit, deny)
nix fmt              # Format code (Rust and Nix files)
```

## Checks (via `nix flake check`)

- **my-crate**: Build the crate
- **my-crate-clippy**: Clippy with `--deny warnings`
- **my-crate-nextest**: Tests via cargo-nextest
- **my-crate-fmt**: Rust formatting
- **my-crate-toml-fmt**: TOML formatting via taplo
- **my-crate-audit**: Security audit against advisory-db
- **my-crate-deny**: License checking (MIT only)
- **my-crate-doc**: Documentation with `--deny warnings`
