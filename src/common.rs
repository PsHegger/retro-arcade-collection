use std::slice::Iter;

use bevy::prelude::{Color, Resource, States};

use crate::common::Game::*;
use crate::constants::{WINDOW_HEIGHT, WINDOW_WIDTH};

#[derive(Copy, Clone, Debug)]
pub enum Game {
    Asteroid,
    Bomberman,
    Breakout,
    PacMan,
    Sokoban,
    SpaceInvaders,
    Tetris,
    Tron,
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    Menu,
    Breakout,
}

impl Game {
    pub fn supported_games() -> Iter<'static, Game> {
        static SUPPORTED_GAMES: [Game; 8] = [
            Asteroid,
            Bomberman,
            Breakout,
            PacMan,
            Sokoban,
            SpaceInvaders,
            Tetris,
            Tron,
        ];
        SUPPORTED_GAMES.iter()
    }

    pub fn color(self) -> Color {
        match self {
            Asteroid => Color::MAROON,
            Bomberman => Color::RED,
            Breakout => Color::PINK,
            PacMan => Color::YELLOW,
            Sokoban => Color::BLUE,
            SpaceInvaders => Color::ORANGE,
            Tetris => Color::TEAL,
            Tron => Color::GREEN,
        }
    }
}

#[derive(Resource, Debug)]
pub struct ViewportSize {
    pub width: f32,
    pub height: f32,
}

impl Default for ViewportSize {
    fn default() -> Self {
        ViewportSize {
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
        }
    }
}
