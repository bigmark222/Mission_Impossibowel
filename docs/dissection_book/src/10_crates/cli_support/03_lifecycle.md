# cli_support: Lifecycle

## Typical usage
- Define CLI structs in tools/apps using provided opts:
  ```rust,ignore
  #[derive(clap::Parser)]
  struct Args {
      #[clap(flatten)]
      capture: CaptureOutputOpts,
      #[clap(flatten)]
      warehouse: WarehouseOutputOpts,
      #[clap(flatten)]
      weights: WeightsOpts,
      #[clap(flatten)]
      thresholds: ThresholdOpts,
  }
  let args = Args::parse();
  ```
- Resolve seeds:
  ```rust,ignore
  let seed = resolve_seed(args.seed);
  ```
- Pass parsed options into tooling/runtime setup.

## Execution flow
- CLI derives parse args; `flatten`ed structs supply common options.
- Optional Bevy integration if `bevy-resource` feature is used to insert resources.
- Caller uses parsed values to configure capture/warehouse/weights/thresholds.

## Notes
- Stateless; lifecycle ends after parsing/using args. Features enable Bevy resource adapters if needed.
