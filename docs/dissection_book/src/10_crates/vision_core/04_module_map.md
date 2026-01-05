# vision_core: Module Map

- `capture`: Capture-related resources/types (CaptureLimit, FrontCamera markers, FrontCaptureTarget/Readback).
- `interfaces`: Core vision types and traits (Frame, FrameRecord, DetectionResult, Label, Detector, Recorder, FrameSource, BurnDetectorFactory).
- `overlay`: Overlay helpers (normalize_box, draw_rect) and related utilities.
- `prelude`: Convenience re-exports for downstream users.

Cross-module dependencies: interfaces define the shared contracts; capture provides resources used by runtime; overlay operates on data from interfaces; prelude re-exports common items.
