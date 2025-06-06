use bevy::color::palettes::css::{GOLD, GREEN};
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::text::TextSpanAccess;
use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use bevy_svg::prelude::*;

/// Provides some common functionallity for all examples.
/// Like toggling visibility and through origin.
pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin::default())
            .add_systems(
                Startup,
                (setup_legend, setup_fps_counter, setup_origin_text),
            )
            .add_systems(
                Update,
                (
                    fps_text_update_system,
                    camera_zoom_system,
                    camera_pan_system,
                ),
            );
    }
}

fn setup_legend(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_bold = asset_server.load("fonts/FiraSans-Bold.ttf");
    let font_medium = asset_server.load("fonts/FiraMono-Medium.ttf");

    commands
        .spawn((
            Text::default(),
            TextColor::WHITE,
            TextFont::from_font(font_medium).with_font_size(20.0),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(5.0),
                right: Val::Px(15.0),
                ..default()
            },
        ))
        .with_children(|commands| {
            commands.spawn((
                TextSpan::new("Key Info"),
                TextFont::from_font(font_bold.clone()).with_font_size(30.0),
            ));
            commands.spawn((TextSpan::new("\nF"), TextFont::from_font(font_bold.clone())));
            commands.spawn(TextSpan::new(" - Toggle Frame Diagnostics"));
            commands.spawn((TextSpan::new("\nO"), TextFont::from_font(font_bold.clone())));
            commands.spawn(TextSpan::new(" - Cycle through Origins"));
            commands.spawn((TextSpan::new("\nV"), TextFont::from_font(font_bold.clone())));
            commands.spawn(TextSpan::new(" - Toggle visibility"));
        });
}

#[derive(Component)]
pub struct DontChange;

#[derive(Component)]
struct FpsText;

#[derive(Component)]
struct FpsMinText;

#[derive(Component)]
struct FpsMaxText;

#[derive(Component)]
struct FrameTimeText;

#[derive(Component)]
struct FpsTextRoot;

#[derive(Resource)]
struct FpsValues {
    min: f64,
    max: f64,
}

impl Default for FpsValues {
    fn default() -> Self {
        Self {
            min: 10000.0,
            max: 0.0,
        }
    }
}

fn setup_fps_counter(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_bold = asset_server.load("fonts/FiraSans-Bold.ttf");
    let font_medium = asset_server.load("fonts/FiraMono-Medium.ttf");

    commands
        .spawn((
            Text::default(),
            TextColor::WHITE,
            TextFont::from_font(font_medium).with_font_size(20.0),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(5.0),
                left: Val::Px(15.0),
                ..default()
            },
            FpsTextRoot,
        ))
        .with_children(|commands| {
            commands.spawn((
                TextSpan::new("FPS: "),
                TextFont::from_font(font_bold.clone()).with_font_size(30.0),
            ));
            commands.spawn((
                TextSpan::default(),
                TextFont::from_font_size(30.0),
                TextColor::from(GOLD),
                FpsText,
            ));
            commands.spawn((
                TextSpan::new("\n(min: "),
                TextFont::from_font(font_bold.clone()),
            ));
            commands.spawn((TextSpan::default(), TextColor::from(GOLD), FpsMinText));
            commands.spawn((
                TextSpan::new(" - max: "),
                TextFont::from_font(font_bold.clone()),
            ));
            commands.spawn((TextSpan::default(), TextColor::from(GOLD), FpsMaxText));
            commands.spawn((TextSpan::new(")"), TextFont::from_font(font_bold.clone())));
            commands.spawn((
                TextSpan::new("\nms/frame: "),
                TextFont::from_font(font_bold.clone()).with_font_size(30.0),
            ));
            commands.spawn((
                TextSpan::default(),
                TextFont::from_font_size(30.0),
                TextColor::from(GREEN),
                FrameTimeText,
            ));
        });
}

fn fps_text_update_system(
    diagnostics: Res<DiagnosticsStore>,
    mut fps_values: Local<FpsValues>,
    mut query: ParamSet<(
        Query<&mut TextSpan, With<FpsText>>,
        Query<&mut TextSpan, With<FpsMinText>>,
        Query<&mut TextSpan, With<FpsMaxText>>,
        Query<&mut TextSpan, With<FrameTimeText>>,
    )>,
) {
    if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(fps_smoothed) = fps.smoothed() {
            if let Ok(mut text) = query.p0().single_mut() {
                *text.write_span() = format!("{fps_smoothed:.2}");
            }
            fps_values.min = fps_values.min.min(fps_smoothed);
            if let Ok(mut text) = query.p1().single_mut() {
                *text.write_span() = format!("{:.2}", fps_values.min);
            }
            fps_values.max = fps_values.max.max(fps_smoothed);
            if let Ok(mut text) = query.p2().single_mut() {
                *text.write_span() = format!("{:.2}", fps_values.max);
            }
        }
    }
    if let Some(frame_time) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FRAME_TIME) {
        if let Some(frame_time_smoothed) = frame_time.smoothed() {
            if let Ok(mut text) = query.p3().single_mut() {
                *text.write_span() = format!("{frame_time_smoothed:.2}");
            }
        }
    }
}

#[derive(Component)]
struct OriginText;

#[derive(Component)]
struct OriginTextRoot;

fn setup_origin_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_bold = asset_server.load("fonts/FiraSans-Bold.ttf");
    let font_medium = asset_server.load("fonts/FiraMono-Medium.ttf");

    commands
        .spawn((
            Text::default(),
            TextColor::WHITE,
            TextFont::from_font(font_medium).with_font_size(20.0),
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(5.0),
                left: Val::Px(15.0),
                ..default()
            },
            OriginTextRoot,
        ))
        .with_children(|commands| {
            commands.spawn((TextSpan::new("Origin: "), TextFont::from_font(font_bold)));
            commands.spawn((TextSpan::default(), TextColor::from(GOLD), OriginText));
        });
}

pub fn camera_zoom_system(
    mut evr_scroll: EventReader<MouseWheel>,
    mut camera: Query<(Option<Mut<Projection>>, Mut<Transform>), With<Camera>>,
) {
    for ev in evr_scroll.read() {
        for (projection, mut transform) in camera.iter_mut() {
            let amount = match ev.unit {
                MouseScrollUnit::Line => ev.y,
                MouseScrollUnit::Pixel => ev.y,
            };
            if let Some(mut projection) = projection {
                if let Projection::Orthographic(ref mut projection) = *projection {
                    projection.scale -= if projection.scale <= 1.0 {
                        amount * 0.05
                    } else {
                        amount
                    };
                    projection.scale = projection.scale.clamp(0.01, 10.0);
                }
            } else {
                transform.translation.z -= amount;
            }
        }
    }
}

pub fn camera_pan_system(
    input: Res<ButtonInput<KeyCode>>,
    mut camera: Query<Mut<Transform>, With<Camera>>,
) {
    for mut transform in camera.iter_mut() {
        if input.pressed(KeyCode::KeyW) {
            transform.translation.y += 1.0;
        }
        if input.pressed(KeyCode::KeyS) {
            transform.translation.y -= 1.0;
        }
        if input.pressed(KeyCode::KeyA) {
            transform.translation.x -= 1.0;
        }
        if input.pressed(KeyCode::KeyD) {
            transform.translation.x += 1.0;
        }
    }
}
