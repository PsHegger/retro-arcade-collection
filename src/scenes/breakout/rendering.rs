use bevy::prelude::*;

use crate::common::ViewportSize;
use crate::scenes::breakout::components::Renderable;
use crate::scenes::breakout::constants::PLAY_AREA_HEIGHT;
use crate::scenes::breakout::resources::ViewportScale;

pub fn scale_handler(
    viewport_size: Res<ViewportSize>,
    mut viewport_scale: ResMut<ViewportScale>,
    mut query: Query<(&mut Sprite, &mut Transform, &Renderable)>,
) {
    if !viewport_size.is_changed() { return; }
    let scale = viewport_size.height / PLAY_AREA_HEIGHT;

    viewport_scale.scale = scale;

    for (mut sprite, mut transform, renderable) in query.iter_mut() {
        let scaled_width = if renderable.scale_x { renderable.size.x * scale } else { renderable.size.x };
        let scaled_height = if renderable.scale_y { renderable.size.y * scale } else { renderable.size.y };
        let scaled_pos_x = renderable.pos.x * scale;
        let scaled_pos_y = renderable.pos.y * scale;
        sprite.custom_size = Some(Vec2::new(scaled_width, scaled_height));
        transform.translation = Vec3::new(scaled_pos_x, scaled_pos_y, 0.0);
    }
}

pub fn renderable_transform_handler(
    viewport_scale: Res<ViewportScale>,
    mut query: Query<(&mut Transform, &Renderable), Changed<Renderable>>,
) {
    for (mut transform, renderable) in query.iter_mut() {
        let scaled_pos_x = renderable.pos.x * viewport_scale.scale;
        let scaled_pos_y = renderable.pos.y * viewport_scale.scale;
        transform.translation = Vec3::new(scaled_pos_x, scaled_pos_y, 0.0);
    }
}
