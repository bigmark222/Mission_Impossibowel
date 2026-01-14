# Test Suite Baseline Report

**Date:** 2026-01-13
**Branch:** hacky-crates-publishing-fix
**Rust Version:** 1.91.0

## Summary

**Total Tests Passed:** 30
**Total Tests Failed:** 0
**Test Suites:** 51 test binaries/modules

All tests passing across the entire workspace.

## Command Run

```bash
cargo test --workspace --locked
```

## Result

```
Finished `test` profile [unoptimized + debuginfo] target(s) in 0.23s
```

## Test Coverage by Crate

| Crate | Unit Tests | Integration Tests | Total |
|-------|-----------|------------------|-------|
| cortenforge (umbrella) | 0 | 0 | 0 |
| burn_dataset | 1 | 0 | 1 |
| capture_utils | 2 | 1 | 3 |
| cli_support | 0 | 0 | 0 |
| data_contracts | 0 | 4 | 4 |
| inference | 1 | 2 | 3 |
| models | 0 | 0 | 0 |
| sim_core | 0 | 1 | 1 |
| cortenforge-tools | 3 | 7 | 10 |
| training | 0 | 7 | 7 |
| vision_core | 0 | 1 | 1 |
| vision_runtime | 0 | 1 | 1 |

## Analysis

**Strengths:**
- All existing tests pass reliably
- Good integration test coverage for tools crate (warehouse commands, config loading)
- Training crate has solid smoke tests for both TinyDet and BigDet models
- Data contracts have validation tests for schemas

**Gaps (to be addressed in H1/H2 priority tasks):**
- Many crates have zero unit tests (sim_core, models, cli_support, vision_core, vision_runtime)
- No integration tests for the full sim-to-real pipeline
- No integration tests for capture → ETL → warehouse → training flow
- Missing tests for error handling and edge cases
- Low coverage of burn_dataset module (only 1 test for 3,082 lines of code)

## Next Steps

This baseline establishes the starting point before the refactor roadmap tasks:
- **H1**: Add integration tests for end-to-end workflows
- **H2**: Add unit tests targeting 60%+ coverage
- **C2**: burn_dataset refactor will enable better test isolation

Current test infrastructure is solid but needs expansion to support production confidence.
