# vision_core: Public API

| Item | Kind | Purpose |
| ---- | ---- | ------- |
| Frame | struct | Represents a captured frame |
| FrameRecord<'a> | struct | Frame plus associated labels/metadata |
| DetectionResult | struct | Detector output (labels/confidence) |
| Label | struct | Bounding box/class metadata |
| FrameSource | trait | Source that can produce frames |
| Detector | trait | Detector interface consuming frames and producing results |
| Recorder | trait | Recorder interface for frames/labels |
| BurnDetectorFactory | trait | Factory for Burn-backed detectors |
| CaptureLimit | struct | Limits for capture (frame counts, etc.) |
| FrontCamera | struct | Marker for front camera |
| FrontCaptureCamera | struct | Marker for capture camera |
| FrontCaptureTarget | struct | Capture render target resource |
| FrontCaptureReadback | struct | Capture readback resource |
| normalize_box | fn | Normalize bbox coords to pixel space |
| draw_rect | fn | Draw rectangle onto RGBA image |
| Modules (pub mod) | module | capture, interfaces, overlay, prelude |
