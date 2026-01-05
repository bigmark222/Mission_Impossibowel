# vision_runtime: Lifecycle

## Typical usage
1) Prepare detector and thresholds:
   ```rust,ignore
   let detector = inference_factory.load_from_checkpoint(path).unwrap_or_else(|| inference_factory.heuristic());
   app.insert_resource(DetectorHandle { detector, kind: DetectorKind::Burn });
   app.insert_resource(InferenceThresholds { obj_thresh, iou_thresh });
   ```
2) Add plugins:
   ```rust,ignore
   app.add_plugins((CapturePlugin, InferencePlugin));
   ```
3) Optionally adjust capture target sizing or add observers.

## Execution flow
- Capture pipeline (CapturePlugin):
  - `setup_front_capture` configures render target/readback resources.
  - `track_front_camera_state` tracks camera info.
  - `capture_front_camera_frame` writes frames to buffer; `on_front_capture_readback` handles GPU readback.
- Inference pipeline (InferencePlugin):
  - Schedules detector tasks on captured frames (`schedule_burn_inference`).
  - Updates overlay state; handles threshold hotkeys (`threshold_hotkeys`).
  - `poll_inference_task` monitors async detector completion.
- Recorder interaction: `recorder_draw_rect` can draw detection overlays into recorder output.

## Notes
- Runs as Bevy plugins; depends on sim_core-built app context.
- Detectors are provided by the inference crate; this crate orchestrates capture/inference scheduling.
