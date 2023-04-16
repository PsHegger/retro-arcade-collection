use bevy::prelude::*;
use bevy::window::close_on_esc;

use crate::common::AppState;
use crate::scenes::asteroid::event_handlers::EventHandlersPlugin;
use crate::scenes::asteroid::events::{EventsPlugin, StartGameEvent};
use crate::scenes::asteroid::game_logic::GameLogicPlugin;
use crate::scenes::asteroid::input::InputPlugin;
use crate::scenes::asteroid::rendering::RenderingPlugin;
use crate::scenes::asteroid::utils::FrameSet;

pub struct AsteroidScenePlugin;

impl Plugin for AsteroidScenePlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(FrameSet::GameLogic.after(FrameSet::Input))
            .configure_set(FrameSet::Rendering.after(FrameSet::GameLogic))
            .add_plugin(EventsPlugin)
            .add_plugin(EventHandlersPlugin)
            .add_plugin(GameLogicPlugin)
            .add_plugin(InputPlugin)
            .add_plugin(RenderingPlugin)
            .add_system(close_on_esc)
            .add_system(setup_scene.in_schedule(OnEnter(AppState::Asteroid)));
    }
}

fn setup_scene(mut start_events: EventWriter<StartGameEvent>) {
    start_events.send_default();
}
