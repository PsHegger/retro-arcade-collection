use bevy::math::Vec2;
use bevy::prelude::Component;

#[derive(Component, Debug)]
pub struct Renderable {
    pub pos: Vec2,
    pub size: Vec2,
    pub scale_x: bool,
    pub scale_y: bool,
}

#[derive(Component)]
pub struct Paddle {
    pub speed: f32,
}

#[derive(Component)]
pub struct Ball {
    pub is_attached: bool
}

#[derive(Component)]
pub struct Block {
    pub score: i32
}
