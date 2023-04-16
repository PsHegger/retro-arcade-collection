use bevy::app::App;
use bevy::prelude::*;

use crate::common::AppState;
use crate::scenes::asteroid::components::{Asteroid, LaserBeam, Ship};
use crate::scenes::asteroid::constants::SHIP_SHOOT_COOLDOWN;
use crate::scenes::asteroid::events::{FireLaserEvent, StartGameEvent};
use crate::scenes::asteroid::resources::SpaceShooterSpriteSheet;
use crate::scenes::asteroid::utils::FrameSet;

pub struct EventHandlersPlugin;

impl Plugin for EventHandlersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (start_game_event_handler, fire_laser_event_handler)
                .in_set(OnUpdate(AppState::Asteroid))
                .in_set(FrameSet::EventHandling),
        );
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
    let ship_sprite_name = "playerShip2_orange.png";

    // sprite size: 99x75
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: atlas_handle.clone(),
            sprite: TextureAtlasSprite::new(
                space_shooter_sprite_sheet
                    .index_of(ship_sprite_name)
                    .unwrap(),
            ),
            transform: Transform::from_scale(Vec3::new(2.0 / 3.0, 2.0 / 3.0, 0.0)),
            ..default()
        },
        Ship {
            sprite_name: ship_sprite_name.to_string(),
            shoot_cooldown: Timer::from_seconds(SHIP_SHOOT_COOLDOWN, TimerMode::Once),
            ..default()
        },
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

fn fire_laser_event_handler(
    mut commands: Commands,
    mut fire_events: EventReader<FireLaserEvent>,
    space_shooter_sprite_sheet: Res<SpaceShooterSpriteSheet>,
    ship_q: Query<(&Transform, &Ship)>,
) {
    if fire_events.is_empty() {
        return;
    }
    fire_events.clear();

    let Ok((ship_transform, ship)) = ship_q.get_single() else { return; };

    let atlas_handle = space_shooter_sprite_sheet.atlas_handle().unwrap();

    let laser_sprite_name = "Lasers/laserGreen05.png";
    let ship_sprite_bounds = space_shooter_sprite_sheet
        .bounds_of(ship.sprite_name.as_str())
        .unwrap();
    let ship_height = ship_sprite_bounds.height() * ship_transform.scale.y;
    let laser_sprite_bounds = space_shooter_sprite_sheet
        .bounds_of(laser_sprite_name)
        .unwrap();
    let required_translate = (ship_height + laser_sprite_bounds.height()) / 2.0;
    let trans_x = required_translate * -ship.rotation.sin();
    let trans_y = required_translate * ship.rotation.cos();

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: atlas_handle,
            sprite: TextureAtlasSprite::new(
                space_shooter_sprite_sheet
                    .index_of(laser_sprite_name)
                    .unwrap(),
            ),
            transform: Transform::from_translation(Vec3::new(
                ship_transform.translation.x + trans_x,
                ship_transform.translation.y + trans_y,
                0.0,
            ))
            .with_rotation(Quat::from_rotation_z(ship.rotation)),
            ..default()
        },
        LaserBeam {
            dir: Vec2::new(trans_x, trans_y).normalize(),
        },
    ));
}
