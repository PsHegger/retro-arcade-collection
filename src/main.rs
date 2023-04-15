use bevy::prelude::*;
use bevy::window::{close_on_esc, PresentMode, Window, WindowPlugin, WindowResized};

use crate::common::{AppState, ViewportSize};
use crate::constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::debug_plugin::DebugPlugin;
use crate::scenes::{BreakoutScenePlugin, MenuScenePlugin};

mod common;
mod constants;
mod debug_plugin;
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
        .add_system(close_on_esc.run_if(in_menu))
        .add_system(resize_notificator)
        .add_startup_system(setup_camera)
        .add_plugin(BreakoutScenePlugin)
        .add_plugin(MenuScenePlugin)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn resize_notificator(
    mut resize_event: EventReader<WindowResized>,
    mut viewport_size: ResMut<ViewportSize>,
) {
    for e in resize_event.iter() {
        if e.width != viewport_size.width || e.height != viewport_size.height {
            viewport_size.width = e.width;
            viewport_size.height = e.height;
        }
    }
}

fn in_menu(app_state: Res<State<AppState>>) -> bool {
    app_state.0 == AppState::Menu
}
