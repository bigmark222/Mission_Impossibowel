# models: Lifecycle

## Typical usage
- Construct model from config and backend:
  ```rust,ignore
  let cfg = TinyDetConfig::default();
  let model = cfg.build::<burn::backend::ndarray::NdArrayBackend<f32>>();
  ```
- Swap to BigDet or different backend based on features/config.

## Execution flow
- Consumer (training/inference) selects config (TinyDet/BigDet) and backend (NdArray default, WGPU opt-in).
- Build model instance; training/inference crates handle forward/backward/checkpointing.

## Notes
- Stateless definitions; lifecycle controlled by training/inference code.
