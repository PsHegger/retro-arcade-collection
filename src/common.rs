use std::slice::Iter;

use bevy::math::Vec2;
use bevy::prelude::{Color, Component, Resource, States};

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

#[derive(Component)]
pub struct Renderable {
    pub pos: Vec2,
    pub size: Vec2,
    pub target_resolution: Vec2,
    pub scale_x: bool,
    pub scale_y: bool,
    pub translate_x: bool,
    pub translate_y: bool,
}

#[allow(dead_code)]
impl Renderable {
    pub fn new(pos: Vec2, target_resolution: Vec2) -> Self {
        Renderable {
            pos,
            size: Default::default(),
            target_resolution,
            scale_x: true,
            scale_y: true,
            translate_x: true,
            translate_y: true,
        }
    }

    pub fn with_size(mut self, size: Vec2) -> Self {
        self.size = size;
        self
    }

    pub fn with_scale(mut self, scale_x: bool, scale_y: bool) -> Self {
        self.scale_x = scale_x;
        self.scale_y = scale_y;
        self
    }

    pub fn with_translate(mut self, translate_x: bool, translate_y: bool) -> Self {
        self.translate_x = translate_x;
        self.translate_y = translate_y;
        self
    }

    pub fn left(&self) -> f32 {
        self.pos.x - self.size.x / 2.0
    }

    pub fn right(&self) -> f32 {
        self.pos.x + self.size.x / 2.0
    }

    pub fn top(&self) -> f32 {
        self.pos.y + self.size.y / 2.0
    }

    pub fn bottom(&self) -> f32 {
        self.pos.y - self.size.y / 2.0
    }

    pub fn min_x(&self) -> f32 {
        -(self.target_resolution.x - self.size.x) / 2.0
    }

    pub fn max_x(&self) -> f32 {
        (self.target_resolution.x - self.size.x) / 2.0
    }

    pub fn min_y(&self) -> f32 {
        -(self.target_resolution.y - self.size.y) / 2.0
    }

    pub fn max_y(&self) -> f32 {
        (self.target_resolution.y - self.size.y) / 2.0
    }
}
