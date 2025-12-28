# Offline overlays

Generate overlay images with bounding boxes for a recorded run.

```bash
cargo run --bin overlay_labels -- \
  --run-dir assets/datasets/captures/<run_dir> \
  --output-dir assets/datasets/captures/<run_dir>/overlays
```

<details>
<summary>Notes</summary>

- Uses recorded `labels/` to draw boxes onto images; good for QA after a run.
- If you want to re-run inference on recorded frames with a new model, add that flow here (e.g., a script/binary that loads frames, runs the detector, and writes new overlays).
- Ensure the run has `run_manifest.json`, `images/`, and `labels/` in place.
- Adjust paths if you store runs elsewhere; outputs default to the run’s `overlays/` dir above.
</details>
