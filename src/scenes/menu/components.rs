use bevy::prelude::Component;

use crate::constants::Game;

#[derive(Component)]
pub struct MenuComponent;

#[derive(Component)]
pub struct MenuGameItem {
    pub game: Game,
}