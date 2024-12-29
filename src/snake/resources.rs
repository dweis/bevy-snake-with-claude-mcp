use bevy::prelude::*;
use std::collections::VecDeque;
use crate::snake::components::Position;

#[derive(Resource)]
pub struct SnakeSegments(pub VecDeque<Entity>);

#[derive(Resource)]
pub struct GridSize {
    pub x: u32,
    pub y: u32,
}

#[derive(Resource)]
pub struct Score {
    pub value: u32,
}

#[derive(Resource, Default)]
pub struct LastTailPosition(pub Option<Position>);

#[derive(Resource)]
pub struct FoodSpawnTimer(pub Timer);

impl Default for FoodSpawnTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(3.0, TimerMode::Repeating))
    }
}

impl Default for SnakeSegments {
    fn default() -> Self {
        Self(VecDeque::new())
    }
}

impl Default for GridSize {
    fn default() -> Self {
        Self {
            x: crate::GRID_SIZE,
            y: crate::GRID_SIZE,
        }
    }
}

impl Default for Score {
    fn default() -> Self {
        Self { value: 0 }
    }
}