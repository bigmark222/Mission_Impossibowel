use std::fs;
use std::path::PathBuf;

use data_contracts::capture::{CaptureMetadata, PolypLabel};
use training::{DatasetConfig, collate};

#[test]
fn load_and_collate_synthetic() {
    let temp = tempfile::tempdir().unwrap();
    let labels_dir = temp.path().join("labels");
    fs::create_dir_all(&labels_dir).unwrap();
    let meta = CaptureMetadata {
        frame_id: 1,
        sim_time: 0.0,
        unix_time: 0.0,
        image: "frame_00001.png".into(),
        image_present: true,
        camera_active: true,
        polyp_seed: 42,
        polyp_labels: vec![PolypLabel {
            center_world: [0.0, 0.0, 0.0],
            bbox_px: Some([0.0, 0.0, 10.0, 10.0]),
            bbox_norm: Some([0.1, 0.1, 0.2, 0.2]),
        }],
    };
    let json = serde_json::to_vec(&meta).unwrap();
    fs::write(labels_dir.join("frame_00001.json"), json).unwrap();

    let cfg = DatasetConfig {
        root: PathBuf::from(temp.path()),
        labels_subdir: "labels".into(),
        images_subdir: ".".into(),
    };
    let samples = cfg.load().unwrap();
    assert_eq!(samples.len(), 1);

    // Collate with NdArray backend
    let batch = collate::<burn_ndarray::NdArray<f32>>(&samples).unwrap();
    assert_eq!(batch.images.dims(), [1, 3, 1, 1]);
    assert_eq!(batch.targets.dims(), [1, 1]);
}
