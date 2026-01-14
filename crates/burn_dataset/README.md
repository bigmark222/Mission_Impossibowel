# burn_dataset

[![crates.io](https://img.shields.io/crates/v/cortenforge-burn-dataset.svg)](https://crates.io/crates/cortenforge-burn-dataset) [![docs.rs](https://docs.rs/cortenforge-burn-dataset/badge.svg)](https://docs.rs/cortenforge-burn-dataset) [![MSRV](https://img.shields.io/badge/rustc-1.75+-orange.svg)](#)

Burn dataset loading, validation, splitting, and batching utilities used by the CortenForge stack.

## Features
- `burn-runtime` (off by default): enables Burn-backed batching, mmap/crossbeam/rayon helpers.
- Without `burn-runtime`, the crate still provides JSON label parsing, splitting, and filtering utilities.

## License
Apache-2.0 (see `LICENSE` in the repo root).
