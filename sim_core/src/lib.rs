use bevy::prelude::*;
use bevy_rapier3d::prelude::RapierPhysicsPlugin;

/// High-level run mode for the sim runtime (detector-free).
#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum SimRunMode {
    #[default]
    Sim,
    Datagen,
    Inference,
}

/// Common configuration for the sim runtime.
#[derive(Resource, Debug, Clone)]
pub struct SimConfig {
    pub mode: SimRunMode,
    pub headless: bool,
}

impl Default for SimConfig {
    fn default() -> Self {
        Self {
            mode: SimRunMode::Sim,
            headless: false,
        }
    }
}

/// System sets for the core sim scheduling.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ModeSet {
    Common,
    SimDatagen,
    Inference,
}

/// Core sim plugin: registers mode-based system sets and injects default config.
pub struct SimPlugin;

impl Plugin for SimPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SimConfig::default())
            .configure_sets(Update, (ModeSet::Common, ModeSet::SimDatagen, ModeSet::Inference));
    }
}

pub mod prelude {
    pub use super::{ModeSet, SimConfig, SimPlugin, SimRunMode};
    pub use crate::camera::{
        Flycam, PovState, ProbePovCamera, UiOverlayCamera, camera_controller, pov_toggle_system,
        setup_camera,
    };
    pub use crate::controls::{ControlParams, control_inputs_and_apply};
    pub use crate::probe_types::{ProbeSegment, SegmentSpring};
}

/// Build a base Bevy `App` with sim mode sets and config. Detector wiring is intentionally omitted.
pub fn build_app(sim_config: SimConfig) -> App {
    let mut app = App::new();
    app.insert_resource(sim_config)
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<()>::default())
        .configure_sets(Update, (ModeSet::Common, ModeSet::SimDatagen, ModeSet::Inference));
    app
}

pub mod camera;
pub mod controls;
pub mod probe_types;
