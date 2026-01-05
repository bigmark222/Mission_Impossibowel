# Performance Notes (data_contracts)

## Hot paths
- None; crate is schema-only. Serialization/deserialization is the only work.

## Allocation patterns
- Owned vectors/paths in structs; serde allocates as needed. No pooling or reuse.

## Trait objects
- None.

## Assumptions
- Overhead is negligible relative to IO; performance driven by serde usage in callers.

## Improvements
- None required; if serde overhead becomes an issue, consider borrowing-friendly variants, but current usage is fine for run-level metadata.
