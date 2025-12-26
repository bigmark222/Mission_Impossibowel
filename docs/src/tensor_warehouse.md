# Tensor Warehouse (Proposal)

Make transforms and training fully decoupled by producing a versioned “tensor warehouse” artifact that training consumes directly—no on-the-fly decode/resize/augments.

## Shape
- Dedicated ETL job/binary:
  - Validates labels (schema, bboxes, image presence).
  - Applies deterministic transforms only (decode + resize/letterbox + normalize boxes).
  - Emits sharded tensor files (CHW tensors + padded boxes/masks) and a manifest with metadata/checksums.
- Training path:
  - Reads manifest + shards via a `--tensor-warehouse <path>` flag.
  - Refuses to decode/augment; falls back to live loader only if explicitly requested for dev.

## Artifacts & layout
- Versioned root: e.g., `artifacts/tensor_warehouse/vYYYYMMDD-hash/`
- Manifest (JSON/JSONL):
  - Dataset root, transform config, cache/transform version hash.
  - Shard list with counts, offsets, dims, max_boxes, checksums.
  - Validation summary (total/non-empty/empty/missing/invalid) and skip thresholds.
- Shards (binary):
  - Image tensors in CHW (f32 or fp16) at fixed target_size.
  - Boxes/masks padded to max_boxes.
  - Optional per-sample metadata (frame_id, run_id).

## Keys & versioning
- Key includes: source dataset root + transform config (target_size, resize_mode, max_boxes, skip_empty) + version hash.
- Any change → new warehouse version. No implicit reuse.
- Invalidation: reject if source mtime > manifest mtime or checksum mismatch; support `WAREHOUSE_CLEAR=1` to rebuild.

## Validation & QA
- Strict schema validation; block or warn if invalid/missing exceeds thresholds.
- Per-run and aggregate summaries stored with the manifest.
- Optionally fail the ETL build on excessive invalid/empty.

## Performance & format
- Parallel decode/resize via thread pool; batch writes to shards sized for fast NVMe.
- Consider fp16 for image tensors to reduce size/IO.
- Deterministic transforms only; augments remain runtime if needed (but not cached).

## Observability
- ETL job emits per-phase timings (decode, resize, write), sample rates, skip reasons.
- Training can log shard load times to catch IO regressions when reading the warehouse.

## Training contract
- Training prefers `--tensor-warehouse <path>`; live loader is dev-only fallback.
- Requires manifest + shards; refuses to proceed if validation thresholds are violated unless overridden.

## Data flow
1) Run the ETL job with a pinned transform config and warehouse version.
2) Produce manifest + shards under the versioned warehouse root.
3) Training reads the manifest, mmaps/streams shards into tensors; no decode/resize.
4) Rebuild the warehouse when inputs or transforms change; keep summaries for drift detection.

## Phased implementation (bite-sized)
1) Manifest/schema design
   - ✅ Define manifest fields (source root, transform config, version hash, shards, checksums, summaries).
   - ✅ Pick shard format (header + CHW/boxes/masks blobs) and size targets.
2) Validation harness
   - ✅ Reuse strict label checks; add thresholds and per-run/aggregate summaries to manifest (helpers: `summarize_with_thresholds`, `summarize_root_with_thresholds`, manifest fields for summary/thresholds).
   - Decide fail/warn thresholds for invalid/missing/empty.
3) ETL writer binary
   - Parse config, validate, decode/resize deterministically, write shards + manifest.
   - Parallelize decode/resize; compute checksums; emit ETL metrics/trace.
4) Training reader path
   - Add `--tensor-warehouse` flag; implement manifest/shard reader that mmaps/streams tensors.
   - Enforce validation thresholds; optional fallback to live loader for dev.
5) Versioning & invalidation
   - Compute version key from source + transform config; store under versioned root.
   - Implement rebuild/clear controls (e.g., `WAREHOUSE_CLEAR`).
6) Observability
   - Log ETL timings/skips; log shard load times in training.
   - Save summaries alongside manifest; optionally add drift detection hooks.
7) (Optional) Analytics layer
   - Export per-run/aggregate summaries to Parquet for ad-hoc analysis with Polars/DuckDB; keep shards/manifest as the source of truth.

## Manifest schema (JSON)
- Top-level fields: `dataset_root` (string, UTF-8), `transform` (cacheable transform config), `version` (sha256 hex), `version_recipe` (string), `code_version` (crate/git), `default_dtype` (`F32`/`F16`), `default_shard_version` (u32), `created_at_ms` (epoch ms), `thresholds` (validation limits), `summary` (per-run/totals), `shards` (array).
- `shards` entries: `id` (string), `relative_path` (string, relative to warehouse root), `shard_version` (u32), `samples` (usize), `width`/`height`/`channels` (u32), `max_boxes` (usize), `dtype` (`F32`/`F16`), `endianness` (`Little`/`Big`), `checksum_sha256` (hex string, optional until populated).

### Shard binary format
- Header (little-endian): magic `TWH1` (4 bytes), `shard_version` (u32), `dtype` (u32; 0=f32, 1=f16), `endianness` (u32; 0=little, 1=big), `width` (u32), `height` (u32), `channels` (u32; typically 3), `max_boxes` (u32), `samples` (u64), `image_offset` (u64), `boxes_offset` (u64), `mask_offset` (u64), `meta_offset` (u64 reserved), `checksum_offset` (u64 reserved).
- Payload (default little-endian): contiguous CHW tensors (`samples * 3 * H * W * dtype`), then padded boxes (`samples * max_boxes * 4 * dtype`), then `box_mask` (`samples * max_boxes * dtype`).
- Size targets: aim for ~128–256 MB per shard on NVMe; cap samples per shard based on dtype + `target_size` + `max_boxes`.

### Prep refactors (before Phase 1)
- ✅ Separate deterministic/cacheable transform config from runtime augments for a clear warehouse key (cacheable config + builder are in place).
- ✅ Factor validation/reporting into reusable helpers with thresholds and structured summaries (dataset summary helpers + env-driven thresholds, see burn_dataset.md).
- ✅ Define manifest/shard metadata structs (schema) up front; wire callers to use them (see `WarehouseManifest`/`ShardMetadata` in `src/tools/burn_dataset.rs`).
- ✅ Add a stub `--tensor-warehouse` flag/config branch in training (CLI accepts `--tensor-warehouse <path>` and currently falls back to the live loader with a notice).
- ✅ Decide serialization/path shape: JSON/JSONL manifest with UTF-8, root-relative paths (manifest uses string paths, no PathBuf).
- ✅ Choose hash/checksum formats: SHA256 hex for warehouse version key and per-shard checksums (see `checksum_sha256` / `version` fields).
- ✅ Lock shard file basics: header (magic/version, shape, max_boxes, dtype), endianness, f32 vs fp16 decision (see `ShardMetadata` fields: shard_version, dtype, endianness).
- ✅ Define version-key recipe: SHA256 of (source root + cacheable transform config + max_boxes + skip_empty + code/version string) stored in manifest.
- ✅ Add helpers: compute canonical warehouse version + save/load manifest JSON (see `WarehouseManifest::compute_version/save/load`).
