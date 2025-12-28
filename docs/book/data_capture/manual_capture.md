# Manual capture & controls

Run the simulator interactively to collect runs and inspect frames/labels before ETL.

## Launch interactive capture
```bash
cargo run --bin sim_view
```
`sim_view` does not open an existing run_dir. It always creates a new `run_<timestamp>` under `--output-root` (default `assets/datasets/captures`) and writes `run_manifest.json`, `images/`, `labels/`, and `overlays/` there (see `init_run_dirs` in `vision.rs`). To choose where a new run is saved, set `--output-root <path>`; it will create the timestamped run inside that directory.

<details>
<summary>Flag reference</summary>

- Full CLI with overrides:
```bash
cargo run --bin sim_view -- \
  --mode sim \
  --output-root assets/datasets/captures \
  --max-frames <optional_cap> \
  --headless <true|false> \
  --seed <optional_seed> \
  --infer-obj-thresh <float> \
  --infer-iou-thresh <float> \
  --prune-empty <true|false> \
  --prune-output-root <path>
```
- `--mode sim` (optional, default `sim`): interactive simulator mode (vs `datagen`).
- `--output-root <path>` (optional, default `assets/datasets/captures`): directory where a new `run_<timestamp>` will be created.
- `--max-frames <N>` (optional): stop recording after N frames.
- `--headless <bool>` (optional, default `false`): hide the main window (offscreen run).
- `--seed <u64>` (optional): deterministic polyp layout/placement.
- `--infer-obj-thresh <float>` (optional, default `0.3`): runtime objectness threshold for burn inference.
- `--infer-iou-thresh <float>` (optional, default `0.5`): runtime IoU threshold for NMS.
- `--prune-empty <bool>` (optional, default `false`): prune empty-label frames after datagen; writes a filtered copy.
- `--prune-output-root <path>` (optional): target for the pruned copy; defaults to `<output_root>_filtered` when `--prune-empty` is set.
</details>

Note: live detector bounding boxes now live in `inference_view`; `sim_view` is focused on sim/datagen.

## Common controls
- Use the on-screen HUD/help overlay in `sim_view`—it lists the active keybindings for capture start/stop, pause/resume, camera switches, overlays/boxes/HUD toggles, stepping frames, snapshots, and quit.
- Keybindings can change between builds; prefer the HUD over stale docs. Capture a screenshot of the HUD if you need to share controls with teammates.
- If you customize bindings, note them here for your team.

## Probe control adjustments (from `src/controls.rs`)
- Tension: `[` / `]` to decrease/increase.
- Stiffness: `;` / `'` to decrease/increase.
- Damping: `,` / `.` to decrease/increase.
- Thrust: `1` / `2` to decrease/increase.
- Target speed: `3` / `4` to decrease/increase.
- Linear damping: `5` / `6` to decrease/increase.
- Friction: `7` / `8` to decrease/increase.
- These update spring motors and damping/friction on probe segments live. Use the HUD to confirm any additional bindings.

## HUD readout (what you see in `sim_view`)
- Displays live probe params: tension (TNS), stiffness (STF), damping (DMP), thrust (THR), target speed (SPD), linear damping (LIN), friction (FRI) with their hotkeys.
- Shows tip pressure (R/U/F), steer vector/strength, polyp counts and nearest distance.
- Vision status: camera on/off, detector kind (BURN/HEUR), confidence/boxes/latency, consensus state, recording status, removal progress.

## Notes on labels
- Keep overlays and boxes visible while capturing to spot misalignments early.
- If you see label jitter or offsets, pause and capture a snapshot, then record the run_dir and steps to reproduce.

## Workflow tips
- Keep notes on what changed during a run (settings, POV).
- After a manual session, prune/organize the run_dir before ETL.
- If you find label/render issues, capture a minimal repro and file it with run_dir and steps.
