# Resolved issues

Historical issues that are no longer active, kept for context.

## burn-core publish break (fixed in 0.14.1)
- **Symptom**: `cargo publish --dry-run -p burn-core` (or dependents) failed with `decode_borrowed_from_slice` missing in `bincode::serde`.
- **Cause**: `burn-core 0.14.0` resolved to `bincode 2.0.1` on crates.io; API change removed the function.
- **Resolution**: `burn-core 0.14.1` fixes the issue; no vendor patch is required.
- **Repro (historical)**:
  ```bash
  git clone https://github.com/tracel-ai/burn
  cd burn
  cargo publish --dry-run -p burn-core
  ```
- **Notes**: If this reappears, re-check burn-core/bincode compatibility and refresh the lockfile.
