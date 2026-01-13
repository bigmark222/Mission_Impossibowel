use data_contracts::RunManifest;

#[test]
fn run_manifest_json_smoke() {
    let json = r#"
{
  "schema_version": "V1",
  "seed": 42,
  "output_root": "artifacts/warehouse",
  "run_dir": "runs/2026-01-12",
  "started_at_unix": 0.0,
  "max_frames": 100
}
"#;

    let manifest: RunManifest = serde_json::from_str(json).expect("valid run manifest JSON");
    manifest.validate().expect("manifest validation");
}
