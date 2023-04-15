use bevy::prelude::*;
use bevy::window::WindowResized;

use crate::common::{Renderable, ViewportSize};

pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(resize_notificator)
            .add_system(scale_handler.in_base_set(CoreSet::PostUpdate))
            .add_system(renderable_transform_handler.in_base_set(CoreSet::PostUpdate));
    }
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

fn scale_handler(
    viewport_size: Res<ViewportSize>,
    mut query: Query<(&mut Transform, &Renderable)>,
) {
    if !viewport_size.is_changed() {
        return;
    }

    for (mut transform, renderable) in query.iter_mut() {
        let scale = viewport_size.height / renderable.target_resolution.y;
        let scale_x = if renderable.scale_x { scale } else { 1.0 };
        let scale_y = if renderable.scale_y { scale } else { 1.0 };
        transform.scale = Vec3::new(scale_x, scale_y, 1.0);
        // update position
        let scaled_pos_x = if renderable.translate_x {
            renderable.pos.x * scale
        } else {
            renderable.pos.x
        };
        let scaled_pos_y = if renderable.translate_y {
            renderable.pos.y * scale
        } else {
            renderable.pos.y
        };
        transform.translation = Vec3::new(scaled_pos_x, scaled_pos_y, transform.translation.z);
    }
}

pub fn renderable_transform_handler(
    viewport_size: Res<ViewportSize>,
    mut query: Query<(&mut Transform, &Renderable), Changed<Renderable>>,
) {
    for (mut transform, renderable) in query.iter_mut() {
        let scale = viewport_size.height / renderable.target_resolution.y;
        let scaled_pos_x = renderable.pos.x * scale;
        let scaled_pos_y = renderable.pos.y * scale;
        transform.translation = Vec3::new(scaled_pos_x, scaled_pos_y, transform.translation.z);
    }
}
