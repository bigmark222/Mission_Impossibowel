# Dependency Graph

```mermaid
flowchart LR
    CLI[cli_support]
    BD[burn_dataset]
    TOOLS[colon_sim_tools]
    CU[capture_utils]
    SC[sim_core]
    VC[vision_core]
    VR[vision_runtime]
    DC[data_contracts]
    TR[training]
    INF[inference]
    M[models]
    CF[cortenforge]

    TOOLS --> CLI
    TOOLS --> BD
    TOOLS --> CU
    TOOLS --> DC
    TOOLS --> INF
    TOOLS --> M

    BD --> DC
    CU --> DC
    VR --> SC
    VR --> VC
    VR --> INF
    INF --> SC
    INF --> VC
    INF --> M
    TR --> DC
    TR --> M
    CF --> SC
    CF --> VC
    CF --> VR
    CF --> DC
    CF --> CU
    CF --> M
    CF --> TR
    CF --> INF
    CF --> CLI
    CF --> BD
    CF --> TOOLS
```

## Interpretation
- Core runtime path: `sim_core` + `vision_core` + `vision_runtime` form the runtime/capture/inference stack; `inference` wires detectors; `models` provides TinyDet/BigDet.
- Data path: `data_contracts` defines schemas; `capture_utils` and tools use them; `burn_dataset` consumes schemas for Burn loaders.
- Training path: `training` depends on `models` and `data_contracts` to produce checkpoints; `inference` consumes them.
- Tooling: `colon_sim_tools` wraps CLI helpers (`cli_support`), recorder/capture (`capture_utils`), schemas (`data_contracts`), dataset (`burn_dataset`), and inference/models.
- Umbrella: `cortenforge` re-exports the stack with feature wiring.
