# App patterns

How apps sit on the substrate. Apps are hosted in their own repository (reference app: https://github.com/via-balaena/Deep-Poo); this chapter describes patterns when building against these crates.

## Reference app (app repo)
- Domain systems: world/entities, HUD, controls/autopilot, recorder world-state updates.
- Bins: `sim_view`, `inference_view` live in the app repo and exercise the substrate crates.
- Uses `SimHooks` to register controls/autopilot; updates `RecorderWorldState` and meta; relies on default JSON sink.
- Good for: seeing a full integration of capture + inference + recorder + UI.

## Minimal demo (app repo)
- Tiny plugin that adds a system to the substrate without domain code.
- Good for: a clean starter layout and minimal bin wiring.

## Build your own app
- In the app repo, create an app crate with:
  - `src/lib.rs`: plugins + systems.
  - `src/prelude.rs`: re-exports for bins/tests.
  - Binaries (e.g., `sim_view`, `inference_view`) that parse CLI args and build the app via your orchestrator that wraps `sim_core::build_app`.
- Wiring steps:
  1) Start from the minimal demo in the app repo as a template.
  2) Add your world/entities and systems; register controls/autopilot via `SimHooks`.
  3) Add a system to update `RecorderWorldState`; provide `RecorderMetaProvider` if you need custom metadata.
  4) Optionally insert custom recorder sinks (keep schemas compatible with data_contracts).
  5) Include capture (`vision_runtime::CapturePlugin`) and inference plugins as needed.
  6) Run `cargo check --workspace`; add a README describing systems/controls.
  7) Add a smoke test or CLI example to ensure the app builds after changes.
- Principles:
  - Keep domain logic in the app crate; core crates stay detector- and domain-agnostic.
  - Use `ModeSet`/`SimRunMode` to gate systems (e.g., only inference in inference mode).
