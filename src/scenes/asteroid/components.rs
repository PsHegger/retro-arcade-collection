use bevy::math::Vec2;
use bevy::prelude::Component;

#[derive(Copy, Clone, Debug, Component, Default)]
pub struct Ship {
    pub speed: Vec2,
    pub rotation: f32,
}

#[derive(Copy, Clone, Debug, Component, Default)]
pub struct Asteroid;
