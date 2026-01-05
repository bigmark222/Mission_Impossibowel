# training: Module Map

- `dataset`: DatasetConfig, SampleIndex, DatasetSample, collate/transform pipeline, summaries/validation, iterators for batches/train/val loaders.
- `util`: TrainArgs, ModelKind/BackendKind, run_train entrypoint, checkpoint loaders (TinyDet/BigDet), target building helpers.
- `lib.rs`: Re-exports dataset/util, aliases backend/model types, pulls TinyDet/BigDet from models.

Cross-module dependencies: dataset feeds util/run_train; util constructs models from `models` and uses dataset loaders. Consumers are training bin and external callers.
