use bevy::prelude::*;
use bevy::window::{close_on_esc, PresentMode, Window, WindowPlugin};

use crate::constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::scenes::MenuScenePlugin;

mod scenes;
mod constants;

const CLEAR_COLOR: ClearColor = ClearColor(Color::BLACK);

fn main() {
    App::new()
        .insert_resource(CLEAR_COLOR)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Retro Arcade Collection".into(),
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                resizable: false,
                present_mode: PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_system(close_on_esc)
        .add_startup_system(setup_camera)
        .add_plugin(MenuScenePlugin)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}