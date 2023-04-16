use bevy::app::App;
use bevy::prelude::Plugin;

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<StartGameEvent>();
    }
}

#[derive(Default)]
pub struct StartGameEvent;
