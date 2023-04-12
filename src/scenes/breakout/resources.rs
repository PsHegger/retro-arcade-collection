use bevy::prelude::Resource;

#[derive(Resource)]
pub struct ViewportScale {
    pub scale: f32,
}

impl Default for ViewportScale {
    fn default() -> Self {
        ViewportScale { scale: 1.0 }
    }
}

#[derive(Resource)]
pub struct PlayerScore(pub i32);
