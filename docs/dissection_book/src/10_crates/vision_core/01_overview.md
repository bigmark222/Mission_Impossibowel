# vision_core: Overview

## Problem statement
Provide a detector/capture data model and overlay math that are engine-agnostic, forming the core vision interfaces for the stack.

## Scope
- Data types: frames, frame records, labels, detection results.
- Traits/interfaces: Detector/Recorder abstractions.
- Overlay utilities (e.g., draw_rect) and capture limits.
- Pure Rust; no Bevy/runtime dependencies.

## Non-goals
- No Bevy plugins or runtime scheduling (handled by vision_runtime).
- No detector implementations or model loading (handled by inference/models).
- No recorder sinks (handled by capture_utils/apps).

## Who should use it
- Runtime/plugins (vision_runtime) needing the shared vision interfaces and overlay math.
- Tools and sinks that operate on frame/label data structures.
- Contributors defining detectors/recorders or working on overlay logic.
