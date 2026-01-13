use burn::tensor::backend::Backend;
use burn_dataset::{
    CacheableTransformConfig, DatasetSummary, Endianness, ResizeMode, ShardDType, ShardMetadata,
    ValidationThresholds, WarehouseLoaders, WarehouseManifest,
};
use std::path::{Path, PathBuf};

#[cfg(feature = "backend-ndarray")]
type SmokeBackend = burn_ndarray::NdArray<f32>;

fn write_test_shard(root: &Path) -> std::io::Result<(PathBuf, ShardMetadata)> {
    let shard_path = root.join("shard.bin");
    let width = 2u32;
    let height = 1u32;
    let channels = 3u32;
    let max_boxes = 1usize;
    let samples = 2usize;
    let header_len = 64usize;
    let img_bytes = samples * width as usize * height as usize * channels as usize * 4;
    let box_bytes = samples * max_boxes * 4 * 4;
    let mask_bytes = samples * max_boxes * 4;
    let image_offset = header_len;
    let boxes_offset = image_offset + img_bytes;
    let mask_offset = boxes_offset + box_bytes;
    let mut data = vec![0u8; header_len + img_bytes + box_bytes + mask_bytes];
    data[0..4].copy_from_slice(b"TWH1");
    data[4..8].copy_from_slice(&(1u32).to_le_bytes()); // shard_version
    data[8..12].copy_from_slice(&(0u32).to_le_bytes()); // dtype f32
    data[12..16].copy_from_slice(&(0u32).to_le_bytes()); // endianness little
    data[16..20].copy_from_slice(&width.to_le_bytes());
    data[20..24].copy_from_slice(&height.to_le_bytes());
    data[24..28].copy_from_slice(&channels.to_le_bytes());
    data[28..32].copy_from_slice(&(max_boxes as u32).to_le_bytes());
    data[32..40].copy_from_slice(&(samples as u64).to_le_bytes());
    data[40..48].copy_from_slice(&(image_offset as u64).to_le_bytes());
    data[48..56].copy_from_slice(&(boxes_offset as u64).to_le_bytes());
    data[56..64].copy_from_slice(&(mask_offset as u64).to_le_bytes());

    let mut cursor = image_offset;
    for sample in 0..samples {
        for _ in 0..(width * height * channels) {
            let val = (sample + 1) as f32;
            data[cursor..cursor + 4].copy_from_slice(&val.to_le_bytes());
            cursor += 4;
        }
    }
    let mut cursor = boxes_offset;
    let boxes = [[0.0f32, 0.0, 0.5, 0.5], [0.1, 0.2, 0.3, 0.4]];
    for b in &boxes {
        for v in b {
            data[cursor..cursor + 4].copy_from_slice(&v.to_le_bytes());
            cursor += 4;
        }
    }
    let mut cursor = mask_offset;
    for m in [1.0f32, 0.0f32] {
        data[cursor..cursor + 4].copy_from_slice(&m.to_le_bytes());
        cursor += 4;
    }

    std::fs::write(&shard_path, data)?;

    let meta = ShardMetadata {
        id: "test".into(),
        relative_path: "shard.bin".into(),
        shard_version: 1,
        samples,
        width,
        height,
        channels,
        max_boxes,
        checksum_sha256: None,
        dtype: ShardDType::F32,
        endianness: Endianness::Little,
    };

    Ok((shard_path, meta))
}

#[cfg(feature = "backend-ndarray")]
#[test]
fn warehouse_manifest_smoke() -> anyhow::Result<()> {
    let tmp = tempfile::tempdir()?;
    let root = tmp.path();
    let (_shard_path, meta) = write_test_shard(root)?;

    let transform = CacheableTransformConfig {
        target_size: None,
        resize_mode: ResizeMode::Force,
        max_boxes: meta.max_boxes,
    };
    let code_version = WarehouseManifest::default_code_version();
    let version = WarehouseManifest::compute_version(root, &transform, false, &code_version);
    let version_recipe =
        "sha256(dataset_root + transform + max_boxes + skip_empty + code_version)".to_string();

    let manifest = WarehouseManifest::new(
        root.to_path_buf(),
        transform,
        version,
        version_recipe,
        code_version,
        vec![meta],
        DatasetSummary::default(),
        ValidationThresholds::default(),
    );

    let manifest_path = root.join("manifest.json");
    manifest.save(&manifest_path)?;

    let loaders = WarehouseLoaders::from_manifest_path(&manifest_path, 0.0, Some(0), false)?;
    let mut iter = loaders.train_iter();
    let device = <SmokeBackend as Backend>::Device::default();
    let batch = iter
        .next_batch::<SmokeBackend>(1, &device)?
        .expect("expected one batch");
    let collated = training::collate_from_burn_batch::<SmokeBackend>(batch, 1)?;
    assert_eq!(collated.images.dims()[0], 1);

    Ok(())
}
