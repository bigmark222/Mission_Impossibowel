# Migration

Guidance for moving code/features into the current layout and branding.

## Refactor snapshot
- This repository is library-only: shared crates (sim_core, vision_core/runtime, data_contracts, capture_utils, models, training, inference, colon_sim_tools) intended for crates.io.
- Apps (including the `colon_sim` reference and `hello_substrate` demo) live in their own repository; pull them from the app repo when you need binaries.
- Crates.io: version `0.1.0` published for the shared crates. App repo: https://github.com/via-balaena/Deep-Poo.
- Tools live in `colon_sim_tools`; bins reuse shared helpers via `cortenforge-cli-support` and `colon_sim_tools::services`.
- Recorder defaults to `JsonRecorder`; apps supply metadata/world-state hooks and can inject sinks.
- Branding: substrate is “CortenForge”; app crates consume it.
- See `MIGRATION.md` at repo root for detailed steps and notes.

## Porting a feature to the new layout
1) Decide if it belongs in substrate (generic) or app (domain-specific).
2) If generic, add hooks/helpers to core crates; gate heavy deps with features.
3) If app-only, implement it in the app repository and wire it through the app hooks/plugins.
4) Update docs (user/contributor) and add a smoke test (NdArray) if applicable.

## PR checklist
- Docs updated (user and/or contributor book).
- Defaults and CLI examples verified.
- Tests: `cargo check --workspace`; add feature-gated tests if new features introduced.

## Adding a new app
- Use the app repository template (or clone the reference app repo) as a starting point; wire hooks, bins, and tests there.

## Extending tools
- Put helpers in `tools/src/services.rs` or `tools/src/warehouse_commands/`; keep bins thin; gate heavy deps with features.
