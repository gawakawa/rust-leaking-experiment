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

## Code Architecture

Each module demonstrates a different memory safety issue caused by `mem::forget`:

- **drain** (`src/drain.rs`): Shows use-after-free when `Drain` iterator is forgotten mid-iteration, leaving dangling pointers to deallocated `Box` data
- **rc** (`src/rc.rs`): Custom `Rc<T>` implementation with intentionally small `u8` ref_count to demonstrate:
  - Memory leak: forgotten clones inflate ref_count, preventing deallocation
  - Use-after-free: 255 forgotten clones overflow ref_count back to 0, causing premature deallocation
