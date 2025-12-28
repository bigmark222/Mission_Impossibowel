# Code pointers and doc update rules

## Main modules (illustrative; adjust to actual paths)
- ETL: `tools/src/bin/warehouse_etl.rs` (pipeline, transforms, manifest writing).
- Warehouse commands: `tools/src/bin/warehouse_cmd.rs` + `tools/src/warehouse_commands/` (one-liner builders, CLI).
- Training loaders: `src/` (tensor warehouse reader, store modes).
- Data schemas: `docs/src/data_schema.md` (legacy) and new reference tables in this book.

## When interfaces change
- Update CLI reference and env var index in the Reference section.
- Update warehouse layout/versioning docs if shard/manifest formats change.
- Update training docs when flags/envs change or new store modes/backends appear.
- Add/refresh diagrams tied to changed flows (see `media/diagrams_checklist.md`).

## Process
- Include doc updates in the same PR as code changes when possible.
- Link to relevant code paths from commands/examples to keep readers oriented.
- If adding a new tool/bin, add its CLI reference and a short recipe.
