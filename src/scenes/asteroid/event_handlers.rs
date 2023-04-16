use bevy::app::App;
use bevy::prelude::*;

use crate::common::AppState;
use crate::scenes::asteroid::components::{Asteroid, Ship};
use crate::scenes::asteroid::events::StartGameEvent;
use crate::scenes::asteroid::resources::SpaceShooterSpriteSheet;

pub struct EventHandlersPlugin;

impl Plugin for EventHandlersPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(start_game_event_handler.in_set(OnUpdate(AppState::Asteroid)));
    }
}

fn start_game_event_handler(
    mut commands: Commands,
    mut start_events: EventReader<StartGameEvent>,
    space_shooter_sprite_sheet: Res<SpaceShooterSpriteSheet>,
) {
    if start_events.is_empty() {
        return;
    }
    start_events.clear();

    let atlas_handle = space_shooter_sprite_sheet.atlas_handle().unwrap();

    // sprite size: 99x75
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: atlas_handle.clone(),
            sprite: TextureAtlasSprite::new(
                space_shooter_sprite_sheet
                    .index_of("playerShip2_orange.png")
                    .unwrap(),
            ),
            transform: Transform::from_scale(Vec3::new(2.0 / 3.0, 2.0 / 3.0, 0.0)),
            ..default()
        },
        Ship::default(),
    ));

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: atlas_handle.clone(),
            sprite: TextureAtlasSprite::new(
                space_shooter_sprite_sheet
                    .index_of("Meteors/meteorBrown_big1.png")
                    .unwrap(),
            ),
            transform: Transform::from_translation(Vec3::new(-100.0, 100.0, 0.0)),
            ..default()
        },
        Asteroid::default(),
    ));
}
