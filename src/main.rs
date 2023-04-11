use bevy::prelude::*;
use bevy::window::{close_on_esc, PresentMode, Window, WindowPlugin, WindowResized};

use crate::common::ViewportSize;
use crate::constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::scenes::BreakoutScenePlugin;

mod scenes;
mod constants;
mod common;

const CLEAR_COLOR: ClearColor = ClearColor(Color::BLACK);

fn main() {
    App::new()
        .insert_resource(CLEAR_COLOR)
        .insert_resource(ViewportSize::default())
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Retro Arcade Collection".into(),
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                present_mode: PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_system(close_on_esc)
        .add_system(resize_notificator)
        .add_startup_system(setup_camera)
        .add_plugin(BreakoutScenePlugin)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn resize_notificator(mut resize_event: EventReader<WindowResized>, mut viewport_size: ResMut<ViewportSize>) {
    for e in resize_event.iter() {
        if e.width != viewport_size.width || e.height != viewport_size.height {
            viewport_size.width = e.width;
            viewport_size.height = e.height;
        }
    }
}
