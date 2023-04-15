use bevy::app::App;
use bevy::prelude::*;

use crate::common::AppState;
use crate::scenes::breakout::components::BreakoutEntity;
use crate::scenes::breakout::event_handlers::EventHandlerPlugin;
use crate::scenes::breakout::events::{EventsPlugin, RestartGameEvent};
use crate::scenes::breakout::input::InputPlugin;
use crate::scenes::breakout::logic::LogicPlugin;
use crate::scenes::breakout::resources::ResourcesPlugin;

pub struct BreakoutScenePlugin;

impl Plugin for BreakoutScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ResourcesPlugin)
            .add_plugin(EventsPlugin)
            .add_plugin(InputPlugin)
            .add_plugin(EventHandlerPlugin)
            .add_plugin(LogicPlugin)
            .add_system(setup_scene.in_schedule(OnEnter(AppState::Breakout)))
            .add_system(despawn_game.in_schedule(OnExit(AppState::Breakout)));
    }
}

fn setup_scene(mut restart_events: EventWriter<RestartGameEvent>) {
    restart_events.send_default();
}

fn despawn_game(mut commands: Commands, entities: Query<Entity, With<BreakoutEntity>>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn();
    }
}
