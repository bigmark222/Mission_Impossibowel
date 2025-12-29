use bevy::prelude::*;
use bevy_rapier3d::prelude::PhysicsSet;

use sim_core::ModeSet;

use colon_sim_app::balloon_control::{
    balloon_body_update, balloon_control_input, balloon_marker_update, spawn_balloon_body,
    spawn_balloon_marker,
};
use colon_sim_app::hud::{update_controls_ui, spawn_controls_ui, spawn_detection_overlay};
use colon_sim_app::polyp::{apply_detection_votes, polyp_detection_system, polyp_removal_system, spawn_polyps};
use colon_sim_app::probe::{distributed_thrust, peristaltic_drive, spawn_probe};
use crate::sim::recorder::{
    auto_start_recording, auto_stop_recording_on_cecum, datagen_failsafe_recording,
    finalize_datagen_run, record_front_camera_metadata, recorder_toggle_hotkey,
};
use colon_sim_app::recorder::update_recorder_world_state;
use colon_sim_app::tunnel::{
    cecum_detection, setup_tunnel, start_detection, tunnel_expansion_system,
};
use crate::vision::CapturePlugin;
use sim_core::camera::{camera_controller, pov_toggle_system, setup_camera};
use sim_core::hooks::SimHooks;

/// Plugin that wires the existing autopilot/recorder and sim systems into ModeSet schedules.
pub struct SimSystemsPlugin;

impl Plugin for SimSystemsPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, (ModeSet::Common, ModeSet::SimDatagen, ModeSet::Inference))
            .add_systems(
                Startup,
                (
                    setup_camera,
                    setup_tunnel,
                    spawn_probe,
                    spawn_balloon_body,
                    spawn_balloon_marker,
                    spawn_polyps,
                    spawn_controls_ui,
                    spawn_detection_overlay,
                )
                    .chain(),
            )
            .add_systems(
                Update,
                (
                    balloon_control_input,
                    balloon_body_update,
                    camera_controller,
                    pov_toggle_system,
                    apply_detection_votes
                        .after(polyp_detection_system)
                        .after(crate::vision::schedule_burn_inference),
                )
                    .in_set(ModeSet::Common),
            )
            .add_systems(
                Update,
                (
                    balloon_marker_update,
                    update_recorder_world_state,
                    recorder_toggle_hotkey,
                    auto_start_recording,
                    auto_stop_recording_on_cecum,
                    finalize_datagen_run.after(auto_stop_recording_on_cecum),
                    datagen_failsafe_recording,
                    record_front_camera_metadata,
                )
                    .in_set(ModeSet::SimDatagen),
            )
            .add_systems(
                Update,
                (
                    update_controls_ui,
                    cecum_detection,
                    start_detection,
                    tunnel_expansion_system,
                    polyp_detection_system,
                    polyp_removal_system.after(polyp_detection_system),
                )
                    .in_set(ModeSet::SimDatagen),
            )
            .add_systems(
                FixedUpdate,
                (
                    peristaltic_drive,
                    distributed_thrust.before(PhysicsSet::SyncBackend),
                ),
            )
            .add_plugins(CapturePlugin);

        if let Some(hooks) = app.world_mut().remove_resource::<SimHooks>() {
            hooks.apply(app);
            app.insert_resource(hooks);
        }
    }
}
