# data_contracts: Lifecycle

## Typical usage
- Define/load manifests and labels:
  ```rust,ignore
  let manifest: RunManifest = serde_json::from_str(json_str)?;
  manifest.validate()?;
  let label = PolypLabel { /* fields */ };
  ```
- Validate captures/warehouse artifacts against schemas before ETL/training.

## Execution flow
- Producers (recorder/tools) construct `RunManifest` and `PolypLabel` per capture.
- Consumers (ETL/training/tools) deserialize manifests/labels and optionally call validation helpers.
- Schema versioning via `RunManifestSchemaVersion` allows compatibility checks.

## Notes
- Pure data types/validation; no runtime lifecycle. Initialization/teardown managed by callers.
