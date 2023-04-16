use bevy::app::App;
use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::common::{AppState, ViewportSize};
use crate::scenes::asteroid::components::{Asteroid, LaserBeam, Ship};
use crate::scenes::asteroid::constants::{
    ASTEROID_STARTING_COUNT_MAX, ASTEROID_STARTING_COUNT_MIN, SHIP_SHOOT_COOLDOWN,
};
use crate::scenes::asteroid::events::{FireLaserEvent, SpawnAsteroidsEvent, StartGameEvent};
use crate::scenes::asteroid::resources::SpaceShooterSpriteSheet;
use crate::scenes::asteroid::utils::{get_random_meteor_sprite, FrameSet};

pub struct EventHandlersPlugin;

impl Plugin for EventHandlersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (
                start_game_event_handler,
                spawn_asteroids_event_handler.after(start_game_event_handler),
                fire_laser_event_handler,
            )
                .in_set(OnUpdate(AppState::Asteroid))
                .in_set(FrameSet::EventHandling),
        );
    }
}

fn start_game_event_handler(
    mut commands: Commands,
    mut start_events: EventReader<StartGameEvent>,
    mut asteroids_spawn_event: EventWriter<SpawnAsteroidsEvent>,
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

    asteroids_spawn_event.send(
        SpawnAsteroidsEvent::from_count(
            thread_rng().gen_range(ASTEROID_STARTING_COUNT_MIN..=ASTEROID_STARTING_COUNT_MAX),
        )
        .with_spawn_area_ratio(1.0),
    );
}

fn spawn_asteroids_event_handler(
    mut commands: Commands,
    mut spawn_event: EventReader<SpawnAsteroidsEvent>,
    space_shooter_sprite_sheet: Res<SpaceShooterSpriteSheet>,
    viewport_size: Res<ViewportSize>,
    ship_q: Query<&Transform, With<Ship>>,
) {
    let width_half = viewport_size.width / 2.0;
    let height_half = viewport_size.height / 2.0;
    let Ok(ship_p) = ship_q.get_single()
        .map(|transform| Vec2::new(transform.translation.x, transform.translation.y))
        else { return; };
    let atlas_handle = space_shooter_sprite_sheet.atlas_handle().unwrap();

    for event in spawn_event.iter() {
        let viewport = Rect::new(
            ship_p.x - width_half * event.spawn_area_ratio,
            ship_p.y - height_half * event.spawn_area_ratio,
            ship_p.x + width_half * event.spawn_area_ratio,
            ship_p.y + height_half * event.spawn_area_ratio,
        );

        let random_point_in_viewport = || {
            Vec2::new(
                thread_rng().gen_range(viewport.min.x..viewport.max.x),
                thread_rng().gen_range(viewport.min.y..viewport.max.y),
            )
        };

        for _ in 0..event.count {
            let mut pos = random_point_in_viewport();
            while pos.distance_squared(ship_p) < event.safe_radius * event.safe_radius {
                pos = random_point_in_viewport();
            }

            let dir = ((ship_p - pos).normalize()
                + Vec2::new(
                    thread_rng().gen_range(-event.miss_factor..event.miss_factor),
                    thread_rng().gen_range(-event.miss_factor..event.miss_factor),
                ))
            .normalize();
            let speed = thread_rng().gen_range(event.speed_range.clone());
            let rotation_speed =
                thread_rng().gen_range(-event.max_rotation_speed..event.max_rotation_speed);

            let (size_class, sprite_name) = get_random_meteor_sprite(event.big_ratio);

            commands.spawn((
                SpriteSheetBundle {
                    texture_atlas: atlas_handle.clone(),
                    sprite: TextureAtlasSprite::new(
                        space_shooter_sprite_sheet
                            .index_of(sprite_name.as_str())
                            .unwrap(),
                    ),
                    transform: Transform::from_translation(Vec3::new(pos.x, pos.y, 0.0)),
                    ..default()
                },
                Asteroid {
                    rotation_speed,
                    size_class,
                    speed: dir * speed,
                    ..default()
                },
            ));
        }
    }
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
