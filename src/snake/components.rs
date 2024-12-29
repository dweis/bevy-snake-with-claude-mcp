use bevy::prelude::*;

#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

#[derive(Component)]
pub struct SnakeHead {
    pub direction: Direction,
}

#[derive(Component)]
pub struct SnakeSegment;

#[derive(Component)]
pub struct Food;

#[derive(PartialEq, Copy, Clone)]
pub enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }
}
