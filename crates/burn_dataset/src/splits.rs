//! Train/validation splitting logic with stratification support.

use crate::types::{BurnDatasetError, DatasetResult, LabelEntry, SampleIndex};
use rand::{seq::SliceRandom, SeedableRng};
use serde_json;
use std::fs;
use std::path::PathBuf;

pub fn split_runs(
    indices: Vec<SampleIndex>,
    val_ratio: f32,
) -> (Vec<SampleIndex>, Vec<SampleIndex>) {
    let mut by_run: std::collections::BTreeMap<PathBuf, Vec<SampleIndex>> =
        std::collections::BTreeMap::new();
    for idx in indices {
        by_run.entry(idx.run_dir.clone()).or_default().push(idx);
    }
    let mut runs: Vec<_> = by_run.into_iter().collect();
    if val_ratio > 0.0 {
        let mut rng = rand::rng();
        runs.shuffle(&mut rng);
    }
    let total = runs.len().max(1);
    let val_count = ((val_ratio.clamp(0.0, 1.0) * total as f32).round() as usize).min(total);
    let (val_runs, train_runs) = runs.split_at(val_count);

    let mut train = Vec::new();
    let mut val = Vec::new();
    for (_, v) in train_runs {
        train.extend(v.clone());
    }
    for (_, v) in val_runs {
        val.extend(v.clone());
    }
    (train, val)
}

/// Stratified split by box count buckets (0,1,2+ boxes), seeded shuffle. Does not group by run.
pub fn split_runs_stratified(
    indices: Vec<SampleIndex>,
    val_ratio: f32,
    seed: Option<u64>,
) -> (Vec<SampleIndex>, Vec<SampleIndex>) {
    let mut buckets: [Vec<SampleIndex>; 3] = [Vec::new(), Vec::new(), Vec::new()];
    for idx in indices {
        let count = count_boxes(&idx).unwrap_or(0);
        let bucket_idx = if count == 0 {
            0
        } else if count == 1 {
            1
        } else {
            2
        };
        buckets[bucket_idx].push(idx);
    }
    let mut rng: Box<dyn rand::RngCore> = match seed {
        Some(s) => Box::new(rand::rngs::StdRng::seed_from_u64(s)),
        None => Box::new(rand::rng()),
    };
    let mut train = Vec::new();
    let mut val = Vec::new();
    for bucket in buckets.iter_mut() {
        bucket.shuffle(&mut rng);
        let total = bucket.len();
        if total == 0 {
            continue;
        }
        let val_count = ((val_ratio.clamp(0.0, 1.0) * total as f32).round() as usize).min(total);
        let (val_bucket, train_bucket) = bucket.split_at(val_count);
        val.extend_from_slice(val_bucket);
        train.extend_from_slice(train_bucket);
    }
    (train, val)
}

pub fn count_boxes(idx: &SampleIndex) -> DatasetResult<usize> {
    let raw = fs::read(&idx.label_path).map_err(|e| BurnDatasetError::Io {
        path: idx.label_path.clone(),
        source: e,
    })?;
    let meta: LabelEntry = serde_json::from_slice(&raw).map_err(|e| BurnDatasetError::Json {
        path: idx.label_path.clone(),
        source: e,
    })?;
    Ok(meta.polyp_labels.len())
}
