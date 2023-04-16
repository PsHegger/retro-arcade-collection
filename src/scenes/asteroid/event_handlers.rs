use bevy::app::App;
use bevy::prelude::*;

use crate::common::AppState;
use crate::scenes::asteroid::components::{Asteroid, Ship};
use crate::scenes::asteroid::events::StartGameEvent;

pub struct EventHandlersPlugin;

impl Plugin for EventHandlersPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(start_game_event_handler.in_set(OnUpdate(AppState::Asteroid)));
    }
}

fn start_game_event_handler(mut commands: Commands, mut start_events: EventReader<StartGameEvent>) {
    if start_events.is_empty() {
        return;
    }
    start_events.clear();

    // sprite size: 99x75
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(66.0, 50.0)),
                color: Color::GREEN,
                ..default()
            },
            ..default()
        },
        Ship::default(),
    ));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(100.0, 80.0)),
                color: Color::GRAY,
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(-100.0, 100.0, 0.0)),
            ..default()
        },
        Asteroid::default(),
    ));
}
