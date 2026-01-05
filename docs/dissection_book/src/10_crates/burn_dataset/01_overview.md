# burn_dataset: Overview

## Problem statement
Provide dataset loading, splitting, and Burn-compatible batching utilities for CortenForge, consuming capture/warehouse artifacts defined by data_contracts.

## Scope
- Dataset loader from capture/warehouse manifests to Burn tensors.
- Splitting, batching, and optional mmap/rayon support via features.
- Integrates with Burn backends (NdArray by default, WGPU optional).

## Non-goals
- No model definitions or training loop; feeds training crate.
- No capture/ETL generation; assumes artifacts exist.
- No app-specific dataset logic beyond generic loaders.

## Who should use it
- Training pipelines needing to load data into Burn.
- Tools that need to inspect or batch warehouse data.
- Contributors extending loader options or backend support.
