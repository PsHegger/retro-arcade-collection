use bevy::app::App;
use bevy::prelude::*;

use crate::scenes::breakout::event_handlers::EventHandlerPlugin;
use crate::scenes::breakout::events::{EventsPlugin, RestartGameEvent};
use crate::scenes::breakout::input::InputPlugin;
use crate::scenes::breakout::logic::LogicPlugin;
use crate::scenes::breakout::rendering::RenderPlugin;
use crate::scenes::breakout::resources::ResourcesPlugin;

pub struct BreakoutScenePlugin;

impl Plugin for BreakoutScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ResourcesPlugin)
            .add_plugin(EventsPlugin)
            .add_plugin(InputPlugin)
            .add_plugin(RenderPlugin)
            .add_plugin(EventHandlerPlugin)
            .add_plugin(LogicPlugin)
            .add_startup_system(setup_scene);
    }
}

fn setup_scene(mut restart_events: EventWriter<RestartGameEvent>) {
    restart_events.send_default();
}
