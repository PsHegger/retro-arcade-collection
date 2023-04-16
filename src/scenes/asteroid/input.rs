use bevy::prelude::*;

use crate::common::AppState;
use crate::scenes::asteroid::components::Ship;
use crate::scenes::asteroid::constants::{SHIP_ACCELERATION, SHIP_MAX_SPEED, SHIP_ROTATION_SPEED};
use crate::scenes::asteroid::utils::FrameSet;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            ship_keyboard_input_system
                .in_set(OnUpdate(AppState::Asteroid))
                .in_set(FrameSet::Input),
        );
    }
}

fn ship_keyboard_input_system(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut ship_q: Query<&mut Ship>,
) {
    let Ok(mut ship) = ship_q.get_single_mut() else { return; };

    if keys.any_pressed([KeyCode::A, KeyCode::Left]) {
        ship.rotation += time.delta_seconds() * SHIP_ROTATION_SPEED;
    } else if keys.any_pressed([KeyCode::D, KeyCode::Right]) {
        ship.rotation -= time.delta_seconds() * SHIP_ROTATION_SPEED;
    }

    if keys.any_pressed([KeyCode::W, KeyCode::Up]) {
        let speed_change_x = SHIP_ACCELERATION * time.delta_seconds() * -ship.rotation.sin();
        let speed_change_y = SHIP_ACCELERATION * time.delta_seconds() * ship.rotation.cos();
        let mut new_speed = Vec2::new(ship.speed.x + speed_change_x, ship.speed.y + speed_change_y);
        if new_speed.length_squared() > SHIP_MAX_SPEED * SHIP_MAX_SPEED {
            new_speed = new_speed.normalize() * SHIP_MAX_SPEED;
        }
        ship.speed = new_speed;
    }
}
