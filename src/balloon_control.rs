use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::probe::ProbeHead;

#[derive(Resource)]
pub struct BalloonControl {
    pub max_offset: f32,
    pub move_speed: f32,
    pub inflated: bool,
    pub deflated_radius: f32,
    pub inflated_radius: f32,
    pub half_length: f32,
    pub position: Vec3,
    pub initialized: bool,
}

impl Default for BalloonControl {
    fn default() -> Self {
        Self {
            max_offset: 8.0,
            move_speed: 3.0,
            inflated: false,
            deflated_radius: 0.3,
            inflated_radius: 1.6,
            half_length: 0.8,
            position: Vec3::ZERO,
            initialized: false,
        }
    }
}

/// Simple input for a virtual balloon separate from the probe tip.
/// B toggles inflate/deflate; V moves it forward and C moves it back along the tunnel (global +Z).
pub fn balloon_control_input(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut balloon: ResMut<BalloonControl>,
    tip_q: Query<&GlobalTransform, With<ProbeHead>>,
) {
    let forward = Vec3::Z;

    let tip_z = tip_q.single().map(|t| t.translation().z).unwrap_or(0.0);

    if !balloon.initialized {
        balloon.position = Vec3::new(0.0, 0.0, tip_z + 5.0);
        balloon.initialized = true;
    }

    if keys.just_pressed(KeyCode::KeyB) {
        balloon.inflated = !balloon.inflated;
    }

    let step = balloon.move_speed * time.delta_secs();
    if keys.pressed(KeyCode::KeyV) {
        balloon.position += forward * step;
    }
    if keys.pressed(KeyCode::KeyC) {
        balloon.position -= forward * step;
    }

    // Clamp balloon within reachable distance from probe tip based on front stop offset.
    let front_offset = balloon.half_length * 5.0;
    let dist = balloon.position.z - tip_z;
    if dist > front_offset {
        balloon.position.z = tip_z + front_offset;
    }
}

#[derive(Component)]
pub struct BalloonMarker;

pub fn spawn_balloon_marker(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Mesh::from(bevy::math::primitives::Sphere {
        radius: 1.0,
    }));
    let mat = materials.add(StandardMaterial {
        base_color: Color::srgba(0.55, 1.0, 0.55, 0.15),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });

    commands.spawn((
        BalloonMarker,
        Mesh3d(mesh),
        MeshMaterial3d(mat),
        Transform::default(),
        GlobalTransform::default(),
        Visibility::default(),
    ));
}

pub fn balloon_marker_update(
    balloon: Res<BalloonControl>,
    mut marker_q: Query<&mut Transform, With<BalloonMarker>>,
) {
    let Ok(mut tf) = marker_q.single_mut() else {
        return;
    };

    let radius = if balloon.inflated {
        balloon.inflated_radius
    } else {
        balloon.deflated_radius
    };

    tf.translation = balloon.position;
    tf.scale = Vec3::splat(radius);
}

#[derive(Component)]
pub struct BalloonBody;
#[derive(Component)]
pub struct BalloonWall;

pub fn spawn_balloon_body(mut commands: Commands) {
    commands.spawn((
        BalloonBody,
        Transform::default(),
        GlobalTransform::default(),
        RigidBody::KinematicPositionBased,
        Collider::capsule_z(0.5, 0.3),
        Sensor,
        CollisionGroups::new(Group::GROUP_2, Group::ALL ^ (Group::GROUP_1 | Group::GROUP_3)),
    ));

    commands.spawn((
        BalloonWall,
        Transform::default(),
        GlobalTransform::default(),
        RigidBody::KinematicPositionBased,
        Collider::ball(0.4),
        CollisionGroups::new(Group::GROUP_4, Group::GROUP_3),
    ));
}

pub fn balloon_body_update(
    balloon: Res<BalloonControl>,
    mut parts: ParamSet<(
        Query<(&mut Transform, &mut Collider), With<BalloonBody>>,
        Query<&mut Transform, With<BalloonWall>>,
    )>,
) {
    let mut body_q = parts.p0();
    let Ok((mut tf, mut collider)) = body_q.single_mut() else {
        return;
    };

    // Shift collider backward so its front tip aligns near the visual marker at balloon.position.
    tf.translation = balloon.position - Vec3::new(0.0, 0.0, balloon.half_length);
    tf.rotation = Quat::IDENTITY;

    let radius = if balloon.inflated {
        balloon.inflated_radius
    } else {
        balloon.deflated_radius
    };
    *collider = Collider::capsule_z(balloon.half_length, radius);

    let mut wall_q = parts.p1();
    if let Ok(mut wall_tf) = wall_q.single_mut() {
        // Place stop further ahead of the balloon tip to align with the expanded tunnel bulge.
        let front_offset = balloon.half_length * 5.0;
        wall_tf.translation = balloon.position + Vec3::new(0.0, 0.0, front_offset);
        wall_tf.rotation = Quat::IDENTITY;
    }
}
