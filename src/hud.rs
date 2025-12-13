use bevy::prelude::*;
use bevy::ui::{
    AlignItems, BackgroundColor, BorderColor, BorderRadius, Display, FlexDirection, JustifyContent,
    PositionType, UiRect, Val,
};

use crate::controls::ControlParams;
use crate::probe::TipSense;

#[derive(Component)]
pub struct ControlText;

pub fn spawn_controls_ui(mut commands: Commands) {
    let bg = Color::srgba(0.04, 0.08, 0.14, 0.82);
    let border = Color::srgba(0.0, 0.8, 0.75, 0.85);
    let accent = Color::srgba(0.28, 0.9, 1.0, 0.95);
    let line = Color::srgba(0.75, 0.88, 1.0, 0.9);
    let soft = Color::srgba(0.5, 0.74, 1.0, 0.85);

    commands.spawn((
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::FlexStart,
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            padding: UiRect::axes(Val::Px(14.0), Val::Px(12.0)),
            border: UiRect::all(Val::Px(1.5)),
            width: Val::Px(360.0),
            ..default()
        },
        BackgroundColor(bg),
        BorderColor::all(border),
        BorderRadius::all(Val::Px(10.0)),
        Text::new("AUX // PROBE HUD\n───────────────\n"),
        TextFont {
            font_size: 17.0,
            ..default()
        },
        TextColor(accent),
        ControlText,
        children![
            (
                TextSpan::from("TNS :: 0.50 [ [ ] ]\n"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(line),
            ),
            (
                TextSpan::from("STF :: 500 [ ; ' ]\n"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(line),
            ),
            (
                TextSpan::from("DMP :: 20 [ , . ]\n"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(soft),
            ),
            (
                TextSpan::from("THR :: 40 [ 1 2 ]\n"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(line),
            ),
            (
                TextSpan::from("SPD :: 1.20 [ 3 4 ]\n"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(soft),
            ),
            (
                TextSpan::from("LIN :: 0.20 [ 5 6 ]\n"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(line),
            ),
            (
                TextSpan::from("FRI :: 1.20 [ 7 8 ]\n"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(line),
            ),
            (
                TextSpan::from("TIP PRESS R/U/F: 0.0 / 0.0 / 0.0\n"),
                TextFont {
                    font_size: 15.0,
                    ..default()
                },
                TextColor(soft),
            ),
            (
                TextSpan::from("TIP STEER >> 0.0 0.0 0.0 (0.0)"),
                TextFont {
                    font_size: 15.0,
                    ..default()
                },
                TextColor(accent),
            ),
        ],
    ));
}

pub fn update_controls_ui(
    control: Res<ControlParams>,
    sense: Res<TipSense>,
    ui: Single<Entity, (With<ControlText>, With<Text>)>,
    mut writer: TextUiWriter,
) {
    if control.is_changed() || sense.is_changed() {
        *writer.text(*ui, 1) = format!("TNS :: {:.2} [ [ ] ]\n", control.tension);
        *writer.text(*ui, 2) = format!("STF :: {:.0} [ ; ' ]\n", control.stiffness);
        *writer.text(*ui, 3) = format!("DMP :: {:.1} [ , . ]\n", control.damping);
        *writer.text(*ui, 4) = format!("THR :: {:.1} [ 1 2 ]\n", control.thrust);
        *writer.text(*ui, 5) = format!("SPD :: {:.2} [ 3 4 ]\n", control.target_speed);
        *writer.text(*ui, 6) = format!("LIN :: {:.2} [ 5 6 ]\n", control.linear_damping);
        *writer.text(*ui, 7) = format!("FRI :: {:.2} [ 7 8 ]\n", control.friction);

        // Local pressure components: +X right, +Y up, +Z forward along the tip.
        let px = sense.pressure_local.x;
        let py = sense.pressure_local.y;
        let pz = sense.pressure_local.z;
        *writer.text(*ui, 8) = format!("TIP PRESS R/U/F: {:.2} / {:.2} / {:.2}\n", px, py, pz);
        *writer.text(*ui, 9) = format!(
            "TIP STEER >> {:.2} {:.2} {:.2} ({:.2})",
            sense.steer_dir.x, sense.steer_dir.y, sense.steer_dir.z, sense.steer_strength
        );
    }
}
