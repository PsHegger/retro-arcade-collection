use bevy::prelude::Component;

use crate::common::Game;

#[derive(Component)]
pub struct MenuComponent;

#[derive(Component, Debug)]
pub struct MenuGameItem {
    pub game: Game,
}
