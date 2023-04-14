use bevy::app::{App, Plugin};
use bevy::prelude::Resource;

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ViewportScale::default())
            .insert_resource(GameState {
                score: 0,
                has_game_ended: false,
            });
    }
}

#[derive(Resource)]
pub struct ViewportScale(pub f32);

impl Default for ViewportScale {
    fn default() -> Self {
        ViewportScale(1.0)
    }
}

#[derive(Resource)]
pub struct GameState {
    pub score: i32,
    pub has_game_ended: bool,
}

impl GameState {
    pub fn reset(&mut self) {
        self.score = 0;
        self.has_game_ended = false;
    }
}
