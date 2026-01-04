# Happy path (defaults only)

Use these minimal commands to exercise the full pipeline with defaults. App binaries (`sim_view`, `inference_view`) live in the app repository; run those commands there (colon_sim: https://github.com/via-balaena/Deep-Poo). Tools/training crates live here (or consume them via crates.io).

1) Capture (interactive, from app repo):
```bash
cargo run --bin sim_view
```
<details>
<summary>Release (faster) & how it works</summary>

```bash
cargo run --release --bin sim_view
```
- Optimized binary for smoother playback.
- Same args apply (`--headless` if you need it).
</details>

   Headless wrapper (requires `sim_view` built in the app repo and on PATH/alongside):
```bash
cargo run -p colon_sim_tools --bin datagen
```
<details>
<summary>Release (faster) & how it works</summary>

```bash
cargo run -p colon_sim_tools --release --bin datagen
```
- `datagen` invokes `sim_view` and passes the appropriate flags automatically.
- Ensure the `sim_view` binary from the app repo is on PATH or in the same profile dir (`target/release/sim_view` when using `--release`).
- If `sim_view` is missing there, build it once in the app repo: `cargo build --release --bin sim_view`.
- You can pass extra args: `--max-frames 100 --headless` etc.
</details>

2) ETL (build warehouse):
```bash
cargo run -p colon_sim_tools --bin warehouse_etl
```
<details>
<summary>Release (faster) & notes</summary>

```bash
cargo run -p colon_sim_tools --release --bin warehouse_etl
```
- Optimized binary; same flags as debug.
- Lives in `target/release/warehouse_etl` when built with `--release`.
</details>

3) Train (NdArray backend unless you enable WGPU):
```bash
cargo run -p training --features burn_runtime --bin train -- \
  --manifest artifacts/tensor_warehouse/v<version>/manifest.json
```
<details>
<summary>Release (faster) & notes</summary>

```bash
cargo run -p training --release --features burn_runtime --bin train -- \
  --manifest artifacts/tensor_warehouse/v<version>/manifest.json
```
- Uses optimized kernels; same CLI.
- Binary path when built: `target/release/train`.
</details>

4) Inference (real-time, from app repo):
```bash
cargo run --bin inference_view
```
<details>
<summary>Release (faster) & notes</summary>

```bash
cargo run --release --bin inference_view
```
- Optimized viewer for smoother overlay; pass your usual flags if any.
- Binary path when built: `target/release/inference_view`.
</details>
   Single image:
```bash
cargo run -p colon_sim_tools --bin single_infer -- --image path/to/image.png
```

Expected artifacts by the end:
- `assets/datasets/captures/run_<ts>/` (raw capture) → optionally `captures_filtered/` after prune.
- `artifacts/tensor_warehouse/v<ts>/manifest.json` + shards (ETL).
- `checkpoints/tinydet.bin` or `checkpoints/bigdet.bin` (training).
- Overlays or boxed PNGs from inference.

Quick defaults table (app binaries run from the app repo):
| Stage     | Command (default)                                                                  | Command (release)                                                      | Output                                      |
|-----------|------------------------------------------------------------------------------------|------------------------------------------------------------------------|---------------------------------------------|
| Capture   | `cargo run --bin sim_view` (app repo)                                              | `cargo run --release --bin sim_view` (app repo)                        | `assets/datasets/captures/run_<ts>/`        |
| Headless  | `cargo run -p colon_sim_tools --bin datagen`                                       | `cargo run -p colon_sim_tools --release --bin datagen`                 | `assets/datasets/captures/run_<ts>/`        |
| ETL       | `cargo run -p colon_sim_tools --bin warehouse_etl`                                 | `cargo run -p colon_sim_tools --release --bin warehouse_etl`           | `artifacts/tensor_warehouse/v<ts>/manifest` |
| Train     | `cargo run -p training --features burn_runtime --bin train -- --manifest ...`      | `cargo run -p training --release --features burn_runtime --bin train -- --manifest ...` | `checkpoints/tinydet.bin` (or bigdet)       |
| Infer RT  | `cargo run --bin inference_view` (app repo)                                        | `cargo run --release --bin inference_view` (app repo)                  | Live overlay; optional run dir if set       |
| Infer Img | `cargo run -p colon_sim_tools --bin single_infer -- --image img.png`               | (usually debug is fine)                                                | `img_boxed.png`                             |
