# capture_utils: Lifecycle

## Typical usage
- Use default recorder sink:
  ```rust,ignore
  let recorder = JsonRecorder::new(run_dir)?;
  recorder.write_label(&label)?;
  ```
- Overlay/prune helpers in tools:
  ```rust,ignore
  generate_overlays(run_dir)?;
  let (_kept, _dropped) = prune_run(input_run, output_root)?;
  ```

## Execution flow
- Recorder sinks consume labels/frames produced by runtime/tools and write JSON manifests.
- Overlay/prune functions operate on capture directories (frames/labels) to produce overlays or filtered runs.

## Notes
- Stateless helpers; lifecycle driven by callers (runtime/tools).
