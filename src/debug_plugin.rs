use bevy::app::App;
use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::common::ViewportSize;
use crate::constants::{FONT_FILE, WINDOW_HEIGHT, WINDOW_WIDTH};

const FPS_MARGIN_LEFT: f32 = 5.0;
const FPS_MARGIN_TOP: f32 = 0.0;

#[derive(Component)]
struct FpsText;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        init_plugin(app);
    }
}

#[cfg(debug_assertions)]
fn init_plugin(app: &mut App) {
    app.add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup)
        .add_system(update_scale)
        .add_system(update_fps);
}

#[cfg(not(debug_assertions))]
fn init_plugin(_app: &mut App) {}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load(FONT_FILE.to_string());
    let text_style = TextStyle {
        font,
        font_size: 30.0,
        color: Color::WHITE,
    };
    commands.spawn((
        Text2dBundle {
            text: Text::from_section("0", text_style.clone()),
            transform: Transform::from_xyz(
                -WINDOW_WIDTH / 2.0 + FPS_MARGIN_LEFT,
                WINDOW_HEIGHT / 2.0 - FPS_MARGIN_TOP,
                100.0,
            ),
            text_anchor: Anchor::TopLeft,
            ..default()
        },
        FpsText,
    ));
}

fn update_scale(viewport_size: Res<ViewportSize>, mut query: Query<&mut Transform, With<FpsText>>) {
    if !viewport_size.is_changed() {
        return;
    }

    for mut transform in query.iter_mut() {
        transform.translation = Vec3::new(
            -viewport_size.width / 2.0 + FPS_MARGIN_LEFT,
            viewport_size.height / 2.0 - FPS_MARGIN_TOP,
            100.0,
        )
    }
}

fn update_fps(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(value) = fps.smoothed() {
            for mut label in query.iter_mut() {
                label.sections[0].value = format!("{value:.0}");
                label.sections[0].style.color = if value >= 59.5 {
                    Color::GREEN
                } else if value >= 29.5 {
                    Color::GOLD
                } else {
                    Color::RED
                };
            }
        }
    }
}
