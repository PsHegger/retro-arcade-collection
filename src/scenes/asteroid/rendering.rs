use bevy::prelude::*;

use crate::common::AppState;
use crate::scenes::asteroid::components::Ship;
use crate::scenes::asteroid::utils::FrameSet;

pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (ship_rotation_update_system, camera_follow_system)
                .in_set(OnUpdate(AppState::Asteroid))
                .in_set(FrameSet::Rendering),
        );
    }
}

fn ship_rotation_update_system(mut ship_q: Query<(&mut Transform, &Ship)>) {
    for (mut transform, ship) in ship_q.iter_mut() {
        transform.rotation = Quat::from_rotation_z(ship.rotation);
    }
}

fn camera_follow_system(
    ship_q: Query<&Transform, With<Ship>>,
    mut camera_q: Query<&mut Transform, (With<Camera>, Without<Ship>)>,
) {
    let Ok(ship_transform) = ship_q.get_single() else { return; };
    let Ok(mut camera_transform) = camera_q.get_single_mut() else { return; };
    camera_transform.translation = ship_transform.translation;
}