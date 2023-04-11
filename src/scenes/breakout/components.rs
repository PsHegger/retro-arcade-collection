use bevy::math::Vec2;
use bevy::prelude::Component;

#[derive(Component, Debug)]
pub struct Renderable {
    pub pos: Vec2,
    pub size: Vec2,
    pub scale_x: bool,
    pub scale_y: bool,
}
