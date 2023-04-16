use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::common::{AppState, ViewportSize};
use crate::scenes::asteroid::components::{Asteroid, LaserBeam, Ship};
use crate::scenes::asteroid::constants::{LASER_BEAM_DESPAWN_SCALE, LASER_BEAM_SPEED};
use crate::scenes::asteroid::events::SpawnAsteroidsEvent;
use crate::scenes::asteroid::resources::GameState;
use crate::scenes::asteroid::utils::FrameSet;

pub struct GameLogicPlugin;

impl Plugin for GameLogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (
                move_ship_system,
                move_lasers_system,
                move_asteroids_system,
                asteroid_spawner_system,
            )
                .in_set(OnUpdate(AppState::Asteroid))
                .in_set(FrameSet::GameLogic),
        );
    }
}

fn move_ship_system(time: Res<Time>, mut ship_q: Query<(&mut Transform, &Ship)>) {
    for (mut transform, ship) in ship_q.iter_mut() {
        transform.translation.x += ship.speed.x * time.delta_seconds();
        transform.translation.y += ship.speed.y * time.delta_seconds();

        // TODO: handle warping back when leaving play area
    }
}

fn move_lasers_system(
    mut commands: Commands,
    time: Res<Time>,
    viewport_size: Res<ViewportSize>,
    mut laser_q: Query<(&mut Transform, &LaserBeam, Entity)>,
    ship_q: Query<&mut Transform, (With<Ship>, Without<LaserBeam>)>,
) {
    let no_despawn_are = ship_q.get_single().map(|ship_transform| {
        let despawn_width = viewport_size.width / 2.0 * LASER_BEAM_DESPAWN_SCALE;
        let despawn_height = viewport_size.height / 2.0 * LASER_BEAM_DESPAWN_SCALE;
        let bot_left = Vec2::new(
            ship_transform.translation.x - despawn_width,
            ship_transform.translation.y - despawn_height,
        );
        let top_right = Vec2::new(
            ship_transform.translation.x + despawn_width,
            ship_transform.translation.y + despawn_height,
        );
        Rect::from_corners(bot_left, top_right)
    });

    for (mut transform, laser, entity) in laser_q.iter_mut() {
        transform.translation.x += laser.dir.x * LASER_BEAM_SPEED * time.delta_seconds();
        transform.translation.y += laser.dir.y * LASER_BEAM_SPEED * time.delta_seconds();

        if let Ok(area) = no_despawn_are {
            if !area.contains(transform.translation.truncate()) {
                commands.entity(entity).despawn();
            }
        }
    }
}

fn move_asteroids_system(time: Res<Time>, mut asteroids_q: Query<(&mut Transform, &mut Asteroid)>) {
    for (mut transform, mut asteroid) in asteroids_q.iter_mut() {
        transform.translation.x += asteroid.speed.x * time.delta_seconds();
        transform.translation.y += asteroid.speed.y * time.delta_seconds();
        asteroid.rotation += asteroid.rotation_speed * time.delta_seconds();
    }
}

fn asteroid_spawner_system(
    time: Res<Time>,
    mut game_state: ResMut<GameState>,
    viewport_size: Res<ViewportSize>,
    mut spawn_asteroids_events: EventWriter<SpawnAsteroidsEvent>,
) {
    game_state.asteroid_spawn_timer.tick(time.delta());

    if game_state.asteroid_spawn_timer.just_finished()
        && thread_rng().gen_bool(game_state.asteroid_spawn_chance as f64)
    {
        let safe_radius = viewport_size.width / 2.0 + 200.0;
        spawn_asteroids_events
            .send(SpawnAsteroidsEvent::from_count(1).with_safe_radius(safe_radius))
    }
}
