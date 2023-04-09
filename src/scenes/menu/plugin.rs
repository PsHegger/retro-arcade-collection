use bevy::asset::AssetServer;
use bevy::prelude::{App, Color, Commands, Plugin, Res, TextAlignment, Transform};
use bevy::sprite::Anchor;
use bevy::text::{Text, Text2dBundle, TextStyle};
use bevy::utils::default;

use crate::constants::WINDOW_HEIGHT;
use crate::scenes::menu::components::MenuComponent;

pub struct MenuScenePlugin;

impl Plugin for MenuScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_menu);
    }
}

fn setup_menu(mut commands: Commands, assets: Res<AssetServer>) {
    let font = assets.load("RobotoMono.ttf");
    let title_style = TextStyle {
        font: font.clone(),
        font_size: 60.0,
        color: Color::WHITE,
    };

    commands.spawn((
        Text2dBundle {
            text: Text::from_section("Retro Arcade\nCollection", title_style.clone())
                .with_alignment(TextAlignment::Center),
            transform: Transform::from_xyz(0., WINDOW_HEIGHT / 2.0, 0.0),
            text_anchor: Anchor::TopCenter,
            ..default()
        },
        MenuComponent
    ));
}