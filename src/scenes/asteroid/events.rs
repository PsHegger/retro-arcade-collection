use std::ops::Range;

use bevy::app::App;
use bevy::prelude::Plugin;
use bevy::utils::default;

use crate::scenes::asteroid::constants::{
    ASTEROID_DEFAULT_BIG_RATIO, ASTEROID_DEFAULT_MISS_FACTOR, ASTEROID_MAX_ROTATION_SPEED,
    ASTEROID_SPAWN_AREA_RATIO, ASTEROID_SPAWN_SAFE_RADIUS, ASTEROID_STARTING_SPEED_MAX,
    ASTEROID_STARTING_SPEED_MIN,
};

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<StartGameEvent>()
            .add_event::<FireLaserEvent>()
            .add_event::<SpawnAsteroidsEvent>();
    }
}

#[derive(Default)]
pub struct StartGameEvent;

#[derive(Default)]
pub struct FireLaserEvent;

pub struct SpawnAsteroidsEvent {
    pub count: i32,
    pub safe_radius: f32,
    pub big_ratio: f32,
    pub miss_factor: f32,
    pub speed_range: Range<f32>,
    pub max_rotation_speed: f32,
    pub spawn_area_ratio: f32,
}

#[allow(dead_code)]
impl SpawnAsteroidsEvent {
    pub fn from_count(count: i32) -> Self {
        SpawnAsteroidsEvent { count, ..default() }
    }

    pub fn with_safe_radius(mut self, safe_radius: f32) -> Self {
        self.safe_radius = safe_radius;
        self
    }

    pub fn with_big_ratio(mut self, big_ratio: f32) -> Self {
        self.big_ratio = big_ratio;
        self
    }

    pub fn with_miss_factor(mut self, miss_factor: f32) -> Self {
        self.miss_factor = miss_factor;
        self
    }

    pub fn with_speed_range(mut self, min: f32, max: f32) -> Self {
        self.speed_range = min..max;
        self
    }

    pub fn with_max_rotation_speed(mut self, max_rotation_speed: f32) -> Self {
        self.max_rotation_speed = max_rotation_speed;
        self
    }

    pub fn with_spawn_area_ratio(mut self, spawn_area_ratio: f32) -> Self {
        self.spawn_area_ratio = spawn_area_ratio;
        self
    }
}

impl Default for SpawnAsteroidsEvent {
    fn default() -> Self {
        SpawnAsteroidsEvent {
            count: 1,
            safe_radius: ASTEROID_SPAWN_SAFE_RADIUS,
            big_ratio: ASTEROID_DEFAULT_BIG_RATIO,
            miss_factor: ASTEROID_DEFAULT_MISS_FACTOR,
            speed_range: ASTEROID_STARTING_SPEED_MIN..ASTEROID_STARTING_SPEED_MAX,
            max_rotation_speed: ASTEROID_MAX_ROTATION_SPEED,
            spawn_area_ratio: ASTEROID_SPAWN_AREA_RATIO,
        }
    }
}
