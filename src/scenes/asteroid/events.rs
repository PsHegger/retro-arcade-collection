use bevy::app::App;
use bevy::prelude::Plugin;

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<StartGameEvent>()
            .add_event::<FireLaserEvent>();
    }
}

#[derive(Default)]
pub struct StartGameEvent;

#[derive(Default)]
pub struct FireLaserEvent;
