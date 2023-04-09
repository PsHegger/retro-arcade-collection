use std::slice::Iter;

use bevy::prelude::Color;

use crate::constants::Game::{Asteroid, Bomberman, Breakout, PacMan, Sokoban, SpaceInvaders, Tetris, Tron};

pub const WINDOW_WIDTH: f32 = 1280.0;
pub const WINDOW_HEIGHT: f32 = 800.0;

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

impl Game {
    pub fn supported_games() -> Iter<'static, Game> {
        static SUPPORTED_GAMES: [Game; 8] = [Asteroid, Bomberman, Breakout, PacMan, Sokoban, SpaceInvaders, Tetris, Tron];
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
            Tron => Color::GREEN
        }
    }
}