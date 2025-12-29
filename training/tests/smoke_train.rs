use training::{DatasetConfig, collate};

#[test]
fn collate_runs_with_empty_images() {
    let _cfg = DatasetConfig {
        root: "assets/datasets/captures_filtered".into(),
        labels_subdir: "labels".into(),
        images_subdir: ".".into(),
    };
    let samples = Vec::new();
    let batch = collate::<burn_ndarray::NdArray<f32>>(&samples).unwrap();
    assert_eq!(batch.images.dims(), [0, 3, 1, 1]);
    assert_eq!(batch.targets.dims(), [0, 1]);
}
