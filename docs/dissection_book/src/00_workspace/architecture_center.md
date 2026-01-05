# Architecture Center

## Core crates (high centrality)
1) sim_core — foundation for runtime wiring and recorder hooks; depended on by vision_runtime and inference.
2) vision_core — shared vision data model/traits; consumed by vision_runtime, capture_utils, inference.
3) vision_runtime — core runtime plugins; ties sim_core + vision_core + inference.
4) data_contracts — schemas underpin capture/tools/ETL/training.
5) models — model definitions used by training/inference.

## Mid-layer
- inference — detector factory bridging models into runtime/tools.
- training — consumes data_contracts/models to produce checkpoints; feeds inference.
- capture_utils — recorder sinks/helpers used by sim_core/tools.
- burn_dataset — data loader for Burn, used in training/tools.
- cli_support — shared CLI parsing for tools; lower centrality but reused.

## Leaves / tooling
- colon_sim_tools — tooling wrapper; depends on many, few depend on it; app-specific bins planned to move.
- cortenforge (umbrella) — re-export facade; aggregates others.

## Rationale
- Core crates sit on the critical path of runtime (sim_core/vision_core/vision_runtime) and data contracts/models that feed training/inference.
- Mid-layer crates adapt core capabilities to specific tasks (detector factory, training, recorder sinks, CLI parsing).
- Tools/umbrella are consumers/facades with fewer inward dependencies.
