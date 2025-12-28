# Single image inference

Run the detector on one image and emit a boxed PNG.

```bash
cargo run --bin single_infer -- \
  --image path/to/frame.png \
  --out path/to/frame_boxed.png \
  --infer-obj-thresh 0.3 \
  --infer-iou-thresh 0.5
```

Smoke test (with Burn features enabled, assuming weights in `checkpoints/tinydet.bin`):
```bash
cargo run --features "burn_runtime_wgpu" --bin single_infer -- \
  --image assets/datasets/captures/sample.png
```

<details>
<summary>Flags</summary>

- `--image <path>` (required): input image.
- `--out <path>` (optional): output boxed image. Defaults to `<stem>_boxed.png` next to input.
- `--infer-obj-thresh` (default `0.3`): objectness threshold.
- `--infer-iou-thresh` (default `0.5`): IoU threshold for NMS.
- WGPU/envs: set `WGPU_BACKEND`, `WGPU_ADAPTER_NAME`, `WGPU_POWER_PREF` if needed for your GPU; set `RUST_LOG` for logging.
</details>

<details>
<summary>Notes</summary>

- Requires Burn features/weights to use the trained model; falls back to heuristic if weights are missing.
- Writes a boxed PNG even if no boxes are found (logs a “no detections” note).
- Uses the same detector path as `inference_view`; thresholds map to the same logic.
</details>
