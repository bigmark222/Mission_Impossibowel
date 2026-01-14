//! Shared CLI argument structs and seed resolution helpers for CortenForge tooling.
//!
//! This crate provides reusable CLI configuration types (thresholds, weights, output options)
//! and deterministic seed resolution logic for tools and binaries. All types are plain data
//! structs with no framework dependencies by default.
//!
//! ## Optional Features
//! - `bevy-resource`: Derives `Resource` for `RunSeed` to enable Bevy ECS integration.
//!
//! ## Primary Use Case
//! Binaries and tools import these types to ensure consistent CLI parsing and reproducible
//! runs across the CortenForge stack.

pub mod common;
pub mod seed;
