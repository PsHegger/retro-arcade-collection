use bevy::prelude::*;
use bevy::window::{close_on_esc, PresentMode, Window, WindowPlugin};

use crate::common::*;
use crate::constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::debug_plugin::DebugPlugin;
use crate::rendering_plugin::RenderingPlugin;
use crate::scenes::{BreakoutScenePlugin, MenuScenePlugin};

mod common;
mod constants;
mod debug_plugin;
mod rendering_plugin;
mod scenes;

const CLEAR_COLOR: ClearColor = ClearColor(Color::BLACK);

fn main() {
    App::new()
        .add_state::<AppState>()
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
        .add_plugin(DebugPlugin)
        .add_plugin(RenderingPlugin)
        .add_plugin(BreakoutScenePlugin)
        .add_plugin(MenuScenePlugin)
        .add_system(close_on_esc.run_if(is_in_menu))
        .add_startup_system(setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn is_in_menu(app_state: Res<State<AppState>>) -> bool {
    app_state.0 == AppState::Menu
}
