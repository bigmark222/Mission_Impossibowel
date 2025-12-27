#[derive(Clone, Copy, Debug)]
pub enum WarehouseStore {
    Memory,
    Mmap,
    Stream,
}

impl WarehouseStore {
    pub fn as_str(&self) -> &'static str {
        match self {
            WarehouseStore::Memory => "memory",
            WarehouseStore::Mmap => "mmap",
            WarehouseStore::Stream => "stream",
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ModelKind {
    Tiny,
    Big,
}

impl ModelKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            ModelKind::Tiny => "tiny",
            ModelKind::Big => "big",
        }
    }
}

#[derive(Clone, Debug)]
pub struct CmdConfig<'a> {
    pub manifest: &'a str,
    pub store: WarehouseStore,
    pub prefetch: Option<usize>, // used only when store == Stream
    pub model: ModelKind,
    pub batch_size: usize,
    pub log_every: usize,
    pub extra_args: &'a str,
}

pub const DEFAULT_CONFIG: CmdConfig<'static> = CmdConfig {
    manifest: "artifacts/tensor_warehouse/v<version>/manifest.json",
    store: WarehouseStore::Stream,
    prefetch: Some(8),
    model: ModelKind::Big,
    batch_size: 32,
    log_every: 1,
    extra_args: "",
};

pub fn build_ps_command(cfg: &CmdConfig<'_>) -> String {
    let mut env_parts = Vec::new();
    env_parts.push(format!(
        "$env:TENSOR_WAREHOUSE_MANIFEST=\"{}\"",
        cfg.manifest
    ));
    env_parts.push(format!("$env:WAREHOUSE_STORE=\"{}\"", cfg.store.as_str()));
    if matches!(cfg.store, WarehouseStore::Stream) {
        let depth = cfg.prefetch.unwrap_or(2);
        env_parts.push(format!("$env:WAREHOUSE_PREFETCH=\"{}\"", depth));
    }
    env_parts.push("$env:WGPU_POWER_PREF=\"high-performance\"".into());
    env_parts.push("$env:RUST_LOG=\"info,wgpu_core=info\"".into());

    let mut cmd_parts = Vec::new();
    cmd_parts.push("cargo train_hp".to_string());
    cmd_parts.push(format!("--model {}", cfg.model.as_str()));
    cmd_parts.push(format!("--batch-size {}", cfg.batch_size));
    cmd_parts.push(format!("--log-every {}", cfg.log_every));
    if !cfg.extra_args.trim().is_empty() {
        cmd_parts.push(cfg.extra_args.trim().to_string());
    }

    format!("{}; {}", env_parts.join("; "), cmd_parts.join(" "))
}

pub fn build_bash_command(cfg: &CmdConfig<'_>) -> String {
    let mut env_parts = Vec::new();
    env_parts.push(format!(
        "TENSOR_WAREHOUSE_MANIFEST=\"{}\"",
        cfg.manifest
    ));
    env_parts.push(format!("WAREHOUSE_STORE=\"{}\"", cfg.store.as_str()));
    if matches!(cfg.store, WarehouseStore::Stream) {
        let depth = cfg.prefetch.unwrap_or(2);
        env_parts.push(format!("WAREHOUSE_PREFETCH=\"{}\"", depth));
    }
    env_parts.push("WGPU_POWER_PREF=\"high-performance\"".into());
    env_parts.push("RUST_LOG=\"info,wgpu_core=info\"".into());

    let mut cmd_parts = Vec::new();
    cmd_parts.push("cargo train_hp".to_string());
    cmd_parts.push(format!("--model {}", cfg.model.as_str()));
    cmd_parts.push(format!("--batch-size {}", cfg.batch_size));
    cmd_parts.push(format!("--log-every {}", cfg.log_every));
    if !cfg.extra_args.trim().is_empty() {
        cmd_parts.push(cfg.extra_args.trim().to_string());
    }

    format!("{} {}", env_parts.join(" "), cmd_parts.join(" "))
}
