# Toy Clone Plan: vision_runtime

Goal: reimplement the essence of `vision_runtime` in ~10–20% of the code to clarify capture state, async inference scheduling, and detector swapping.

## Scope
- Minimal Bevy app integration (resources + systems).
- Frame capture stub (no real GPU readback; synthetic frames).
- Detector trait object with heuristic vs. mock “burn” detector.
- Async inference task with debounce and detector swap-in.
- Overlay state (boxes/scores/latency) for UI/logging.

## Non-goals
- Real GPU capture/readback, WGPU plumbing, or camera entities.
- Real model loading; use a mock detector with synthetic latency.
- Full feature flags or multiple modes; keep a single “Inference” mode.

## Work items
1) Define shared types/resources: `Frame`, `DetectorHandle`, `InferenceState` (pending task, last result), `OverlayState`, `Thresholds`.
2) Capture system: generate a dummy frame every tick with incrementing id and timestamp.
3) Inference scheduler: if no pending task, spawn an async task that consumes the current detector, runs `detect(frame)`, and returns detector + result + elapsed ms.
4) Poller: poll the task; when complete, restore detector to handle, update overlay state, record latency.
5) Hot swap: allow swapping detector kind via a simple command (e.g., keyboard flag or resource set at startup).
6) Keep everything in one crate (`toy_vision_runtime`) with 3 modules: `capture`, `inference`, `overlay` plus `lib.rs` wiring.

## Deliverables
- Crate layout + stub contents (see `toy_vision_runtime_layout.md`).
- Lessons learned after building (see `lessons.md`).
