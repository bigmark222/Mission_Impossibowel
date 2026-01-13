# training

## Overview
Burn-based training and evaluation for TinyDet and BigDet.

## Usage
Run `train`/`eval` bins or call `run_train`. Training defaults to warehouse manifests; use `--input-source capture-logs` only for legacy/dev workflows. docs.rs: https://docs.rs/cortenforge-training; source: https://github.com/via-balaena/CortenForge/tree/main/crates/training.

## Pitfalls
Backends/features must match inference; keep `max_boxes` aligned.
