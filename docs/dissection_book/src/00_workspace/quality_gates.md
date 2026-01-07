# Quality Gates

Rules to keep the book concise, useful, and consistent.

## Per-page rules
- **State the problem/purpose** up front (why this page exists).
- **Key types/actors**: name the important structs/traits/resources involved.
- **Invariants**: list constraints/assumptions; note feature gates if relevant.
- **Example**: include at least one runnable/minimal snippet or command.
- **Gotchas**: pitfalls, edge cases, feature flags, perf risks.
- **Links**: source links (`crate/path.rs:L123`) and/or docs.rs links for referenced items.
- **Scope**: avoid duplicating exhaustive API docs; defer to docs.rs for signatures.

## Crate chapter checklist
For each crate (`sim_core`, `vision_core`, `vision_runtime`, `data_contracts`, `capture_utils`, `models`, `training`, `inference`, `cli_support`, `burn_dataset`, `colon_sim_tools`, `cortenforge`):
- [ ] `01_overview.md`: purpose/scope/non-goals clear; links to docs.rs/source.
- [ ] `02_public_api.md`: table of pub items present; aligns with current code.
- [ ] `03_lifecycle.md`: construction/usage narrative accurate.
- [ ] `04_module_map.md`: modules listed with responsibilities.
- [ ] `05_traits_and_generics.md`: extensibility/constraints captured.
- [ ] `06_error_model.md`: error surfaces and handling noted.
- [ ] `07_ownership_and_concurrency.md`: Send/Sync/async/borrowing captured.
- [ ] `08_performance_notes.md`: hot paths/alloc/cloning highlighted.
- [ ] `09_examples.md`: 2–3 minimal examples compile in principle.
- [ ] `10_design_review.md`: strengths/risks/refactors noted.

## Progress (publish order)
### data_contracts
- [x] `01_overview.md`: purpose/scope/non-goals clear; links to docs.rs/source.
- [x] `02_public_api.md`: table of pub items present; aligns with current code.
- [x] `03_lifecycle.md`: construction/usage narrative accurate.
- [x] `04_module_map.md`: modules listed with responsibilities.
- [x] `05_traits_and_generics.md`: extensibility/constraints captured.
- [x] `06_error_model.md`: error surfaces and handling noted.
- [x] `07_ownership_and_concurrency.md`: Send/Sync/async/borrowing captured.
- [x] `08_performance_notes.md`: hot paths/alloc/cloning highlighted.
- [x] `09_examples.md`: 2–3 minimal examples compile in principle.
- [x] `10_design_review.md`: strengths/risks/refactors noted.

### models
- [x] `01_overview.md`: purpose/scope/non-goals clear; links to docs.rs/source.
- [x] `02_public_api.md`: table of pub items present; aligns with current code.
- [x] `03_lifecycle.md`: construction/usage narrative accurate.
- [x] `04_module_map.md`: modules listed with responsibilities.
- [x] `05_traits_and_generics.md`: extensibility/constraints captured.
- [x] `06_error_model.md`: error surfaces and handling noted.
- [x] `07_ownership_and_concurrency.md`: Send/Sync/async/borrowing captured.
- [x] `08_performance_notes.md`: hot paths/alloc/cloning highlighted.
- [x] `09_examples.md`: 2–3 minimal examples compile in principle.
- [x] `10_design_review.md`: strengths/risks/refactors noted.

### burn_dataset
- [x] `01_overview.md`: purpose/scope/non-goals clear; links to docs.rs/source.
- [x] `02_public_api.md`: table of pub items present; aligns with current code.
- [x] `03_lifecycle.md`: construction/usage narrative accurate.
- [x] `04_module_map.md`: modules listed with responsibilities.
- [x] `05_traits_and_generics.md`: extensibility/constraints captured.
- [x] `06_error_model.md`: error surfaces and handling noted.
- [x] `07_ownership_and_concurrency.md`: Send/Sync/async/borrowing captured.
- [x] `08_performance_notes.md`: hot paths/alloc/cloning highlighted.
- [x] `09_examples.md`: 2–3 minimal examples compile in principle.
- [x] `10_design_review.md`: strengths/risks/refactors noted.

### cli_support
- [x] `01_overview.md`: purpose/scope/non-goals clear; links to docs.rs/source.
- [x] `02_public_api.md`: table of pub items present; aligns with current code.
- [x] `03_lifecycle.md`: construction/usage narrative accurate.
- [x] `04_module_map.md`: modules listed with responsibilities.
- [x] `05_traits_and_generics.md`: extensibility/constraints captured.
- [x] `06_error_model.md`: error surfaces and handling noted.
- [x] `07_ownership_and_concurrency.md`: Send/Sync/async/borrowing captured.
- [x] `08_performance_notes.md`: hot paths/alloc/cloning highlighted.
- [x] `09_examples.md`: 2–3 minimal examples compile in principle.
- [x] `10_design_review.md`: strengths/risks/refactors noted.

### vision_core
- [x] `01_overview.md`: purpose/scope/non-goals clear; links to docs.rs/source.
- [x] `02_public_api.md`: table of pub items present; aligns with current code.
- [x] `03_lifecycle.md`: construction/usage narrative accurate.
- [x] `04_module_map.md`: modules listed with responsibilities.
- [x] `05_traits_and_generics.md`: extensibility/constraints captured.
- [x] `06_error_model.md`: error surfaces and handling noted.
- [x] `07_ownership_and_concurrency.md`: Send/Sync/async/borrowing captured.
- [x] `08_performance_notes.md`: hot paths/alloc/cloning highlighted.
- [x] `09_examples.md`: 2–3 minimal examples compile in principle.
- [x] `10_design_review.md`: strengths/risks/refactors noted.

### sim_core
- [x] `01_overview.md`: purpose/scope/non-goals clear; links to docs.rs/source.
- [x] `02_public_api.md`: table of pub items present; aligns with current code.
- [x] `03_lifecycle.md`: construction/usage narrative accurate.
- [x] `04_module_map.md`: modules listed with responsibilities.
- [x] `05_traits_and_generics.md`: extensibility/constraints captured.
- [x] `06_error_model.md`: error surfaces and handling noted.
- [x] `07_ownership_and_concurrency.md`: Send/Sync/async/borrowing captured.
- [x] `08_performance_notes.md`: hot paths/alloc/cloning highlighted.
- [x] `09_examples.md`: 2–3 minimal examples compile in principle.
- [x] `10_design_review.md`: strengths/risks/refactors noted.

### capture_utils
- [x] `01_overview.md`: purpose/scope/non-goals clear; links to docs.rs/source.
- [x] `02_public_api.md`: table of pub items present; aligns with current code.
- [x] `03_lifecycle.md`: construction/usage narrative accurate.
- [x] `04_module_map.md`: modules listed with responsibilities.
- [x] `05_traits_and_generics.md`: extensibility/constraints captured.
- [x] `06_error_model.md`: error surfaces and handling noted.
- [x] `07_ownership_and_concurrency.md`: Send/Sync/async/borrowing captured.
- [x] `08_performance_notes.md`: hot paths/alloc/cloning highlighted.
- [x] `09_examples.md`: 2–3 minimal examples compile in principle.
- [x] `10_design_review.md`: strengths/risks/refactors noted.

### inference
- [x] `01_overview.md`: purpose/scope/non-goals clear; links to docs.rs/source.
- [x] `02_public_api.md`: table of pub items present; aligns with current code.
- [x] `03_lifecycle.md`: construction/usage narrative accurate.
- [x] `04_module_map.md`: modules listed with responsibilities.
- [x] `05_traits_and_generics.md`: extensibility/constraints captured.
- [x] `06_error_model.md`: error surfaces and handling noted.
- [x] `07_ownership_and_concurrency.md`: Send/Sync/async/borrowing captured.
- [x] `08_performance_notes.md`: hot paths/alloc/cloning highlighted.
- [x] `09_examples.md`: 2–3 minimal examples compile in principle.
- [x] `10_design_review.md`: strengths/risks/refactors noted.

### vision_runtime
- [x] `01_overview.md`: purpose/scope/non-goals clear; links to docs.rs/source.
- [x] `02_public_api.md`: table of pub items present; aligns with current code.
- [x] `03_lifecycle.md`: construction/usage narrative accurate.
- [x] `04_module_map.md`: modules listed with responsibilities.
- [x] `05_traits_and_generics.md`: extensibility/constraints captured.
- [x] `06_error_model.md`: error surfaces and handling noted.
- [x] `07_ownership_and_concurrency.md`: Send/Sync/async/borrowing captured.
- [x] `08_performance_notes.md`: hot paths/alloc/cloning highlighted.
- [x] `09_examples.md`: 2–3 minimal examples compile in principle.
- [x] `10_design_review.md`: strengths/risks/refactors noted.

### training
- [x] `01_overview.md`: purpose/scope/non-goals clear; links to docs.rs/source.
- [x] `02_public_api.md`: table of pub items present; aligns with current code.
- [x] `03_lifecycle.md`: construction/usage narrative accurate.
- [x] `04_module_map.md`: modules listed with responsibilities.
- [x] `05_traits_and_generics.md`: extensibility/constraints captured.
- [x] `06_error_model.md`: error surfaces and handling noted.
- [x] `07_ownership_and_concurrency.md`: Send/Sync/async/borrowing captured.
- [x] `08_performance_notes.md`: hot paths/alloc/cloning highlighted.
- [x] `09_examples.md`: 2–3 minimal examples compile in principle.
- [x] `10_design_review.md`: strengths/risks/refactors noted.

### cortenforge
- [x] `01_overview.md`: purpose/scope/non-goals clear; links to docs.rs/source.
- [x] `02_public_api.md`: table of pub items present; aligns with current code.
- [x] `03_lifecycle.md`: construction/usage narrative accurate.
- [x] `04_module_map.md`: modules listed with responsibilities.
- [x] `05_traits_and_generics.md`: extensibility/constraints captured.
- [x] `06_error_model.md`: error surfaces and handling noted.
- [x] `07_ownership_and_concurrency.md`: Send/Sync/async/borrowing captured.
- [x] `08_performance_notes.md`: hot paths/alloc/cloning highlighted.
- [x] `09_examples.md`: 2–3 minimal examples compile in principle.
- [x] `10_design_review.md`: strengths/risks/refactors noted.

## Cross-workspace pages
- `canonical_flows.md`: flows stay current with crate changes; diagrams updated.
- `integration_contracts.md`: assumptions updated when interfaces/schemas change.
- `docsrs_alignment.md` & `linking_style.md`: reflect current linking conventions.
- `how_to_feed_codex.md`: stays aligned with best practice inputs for Codex.

## Update cadence
- On each release/tag: run through the crate checklist and flows; update line links if code moved.
- When adding a crate: scaffold full set of pages + add to `SUMMARY.md` and checklists.
- When removing/migrating a crate (e.g., `colon_sim_tools` split): update flows, integration contracts, and remove chapters as appropriate.
