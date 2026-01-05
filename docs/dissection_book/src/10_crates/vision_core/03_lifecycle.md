# vision_core: Lifecycle

## Typical usage
- Implement a detector:
  ```rust,ignore
  struct MyDet;
  impl Detector for MyDet {
      fn detect(&mut self, frame: &Frame) -> DetectionResult { /* ... */ }
  }
  ```
- Use recorder/detector traits in runtime/tools; construct `FrameRecord` and emit `Label`/`DetectionResult` as needed.
- Apply overlay helpers when rendering labels:
  ```rust,ignore
  if let Some(bbox_px) = normalize_box(bbox_norm, dims) {
      draw_rect(&mut image, bbox_px, color, thickness);
  }
  ```

## Execution flow
- Consumers (vision_runtime/tools) create `Frame`/`FrameRecord` instances.
- Detectors implementing `Detector` consume frames and produce `DetectionResult`.
- Recorders implementing `Recorder` ingest frames/labels per schema.
- Overlay helpers render bounding boxes to images.

## Notes
- Pure Rust types/traits; no Bevy lifecycle. Initialization/teardown is managed by consumers.
