# Overview

Welcome to the contributor guide for CortenForge. This book explains how the substrate crates fit together, what design choices were made, and how to extend or modify the system without getting lost.

## What this is
- A modular simulation substrate (CortenForge) distributed as shared crates for runtime orchestration, capture/inference, ETL, training, and tooling.
- Apps (including the `colon_sim` reference and other demos) now live in their own repositories; this repo is library-only.
- A map for contributors: where code lives, how pieces talk, and how to add or change behavior safely.

## Crates/versions/features (crates.io)
- Version `0.1.0` on crates.io: `cortenforge-sim-core`, `cortenforge-vision-core`, `cortenforge-vision-runtime`, `cortenforge-data-contracts`, `cortenforge-capture-utils`, `cortenforge-models`, `cortenforge-training`, `cortenforge-inference`, `cortenforge-cli-support`, `cortenforge-burn-dataset`.
- Feature flags:
  - Training/inference: `backend-wgpu` for GPU; defaults to NdArray.
  - Inference: `tinydet`/`bigdet`.
  - Tools: `scheduler`, `tui`, `gpu_nvidia` (tools crate not published by default).
- MSRV: Rust 1.75+. `burn-core` is temporarily patched to a vendored 0.14.0; drop the patch when upstream releases a fixed version.

## Who should read this
- New contributors ramping up on architecture and conventions.
- Engineers adding features (runtime hooks, vision/capture, recorder sinks, tools).
- Domain authors building a new app on the substrate (app-side docs live in the app repos).

## How to use this book
- Start with **Introduction** for scope and expectations.
- Read **Architecture** for the substrate vs. app split and the runtime/data flow.
- Jump to the chapter that matches your work:
  - **Core crates** if you’re inside shared runtime/vision/recorder code.
  - **Hooks / extension points** when wiring new behavior into the sim loop or recorder.
  - **Tools crate** for CLI utilities and adding new commands.
  - **Testing** / **CI** for validation and pipelines.
  - **Roadmap** for upcoming changes and migration notes.

## Scope
- In scope: architecture, crate responsibilities, extension points, app wiring, tools, testing/CI, and migration guidance.
- Out of scope: end-user gameplay instructions (see user book), hardware/patent licensing specifics (see `COMMERCIAL_LICENSE.md`), and exhaustive API docs (read the code; this book points you there).

## Repo map (at a glance)
- `sim_core/`, `vision_core/`, `vision_runtime/`, `data_contracts/`, `capture_utils/`, `models/`, `training/`, `inference/`, `tools/`: substrate crates.
- `crates/*`: supporting libraries (e.g., CLI support, dataset helpers).
- App crates live elsewhere; pull them from the app repos when needed.
- `docs/user_book/`, `docs/contributor_book/`: mdBooks (run `mdbook build docs/contributor_book`).

## Conventions
- Keep core crates domain-agnostic and detector-free; apps supply domain systems and sinks.
- Favor small, composable surfaces (SimHooks, recorder meta/world state, vision hooks).
- Prefer defaults and clear wiring over deep abstraction; gate heavy deps behind features.
