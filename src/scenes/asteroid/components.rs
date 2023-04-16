use bevy::math::Vec2;
use bevy::prelude::Component;
use bevy::time::Timer;

#[derive(Debug, Component, Default)]
pub struct Ship {
    pub speed: Vec2,
    pub rotation: f32,
    pub sprite_name: String,
    pub shoot_cooldown: Timer,
}

#[derive(Copy, Clone, Debug, Component, Default)]
pub struct Asteroid {
    pub size_class: usize,
    pub speed: Vec2,
    pub rotation_speed: f32,
    pub rotation: f32,
}

#[derive(Copy, Clone, Debug, Component, Default)]
pub struct LaserBeam {
    pub dir: Vec2,
}
