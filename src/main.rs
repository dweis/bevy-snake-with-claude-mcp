use bevy::{
    prelude::*,
    window::{WindowMode, PresentMode},
    app::AppExit,
    input::keyboard::KeyCode,
};
use std::collections::VecDeque;

mod snake;
mod ui;
use snake::{
    Direction, Position, SnakeHead, SnakeSegment,
    FoodSpawnTimer, GridSize, LastTailPosition, Score, SnakeSegments, MovementTimer,
    food_collection, food_spawning, snake_movement, snake_movement_input
};
use ui::UiPlugin;

pub const WINDOW_WIDTH: f32 = 800.0;
pub const WINDOW_HEIGHT: f32 = 800.0;
pub const GRID_SIZE: u32 = 20;

// Game States
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Playing,
    GameOver,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Snake Game".to_string(),
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                present_mode: PresentMode::AutoVsync,
                mode: WindowMode::Windowed,
                resizable: false,
                transparent: false,
                window_level: bevy::window::WindowLevel::AlwaysOnTop,
                ..default()
            }),
            ..default()
        }))
        .init_resource::<SnakeSegments>()
        .init_resource::<LastTailPosition>()
        .init_resource::<Score>()
        .init_resource::<GridSize>()
        .init_resource::<MovementTimer>()
        .init_resource::<FoodSpawnTimer>()
        .insert_state(GameState::Playing)
        .add_plugins(UiPlugin)
        .add_systems(PreStartup, setup)
        .add_systems(Startup, spawn_snake)
        .add_systems(
            Update,
            (
                snake_movement_input.run_if(in_state(GameState::Playing)),
                snake_movement.run_if(in_state(GameState::Playing)),
                food_spawning.run_if(in_state(GameState::Playing)),
                food_collection.run_if(in_state(GameState::Playing)),
                check_for_exit,
            ),
        )
        .run();
}

fn check_for_exit(
    keyboard_input: ResMut<ButtonInput<KeyCode>>,
    mut exit: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
    }
}

fn setup(mut commands: Commands) {
    // Spawn camera with individual components
    commands.spawn((
        Camera2d,
        Transform::default(),
        GlobalTransform::default(),
        Visibility::default(),
    ));
}

fn spawn_snake(mut commands: Commands, mut segments: ResMut<SnakeSegments>) {
    *segments = SnakeSegments(VecDeque::new());

    // Spawn snake head at Z=2 to be above everything
    segments.0.push_front(
        commands
            .spawn((
                // Sprite component
                Sprite {
                    color: Color::srgb(0.7, 0.7, 0.7),
                    custom_size: Some(Vec2::new(0.8 * WINDOW_WIDTH / GRID_SIZE as f32,
                                              0.8 * WINDOW_HEIGHT / GRID_SIZE as f32)),
                    ..default()
                },
                // Transform and visibility components
                Transform::from_xyz(0.0, 0.0, 2.0),
                GlobalTransform::default(),
                Visibility::default(),
                // Game components
                SnakeHead {
                    direction: Direction::Up,
                },
                SnakeSegment,
                Position { x: 3, y: 3 },
                Name::new("Snake Head"),
            ))
            .id(),
    );

    // Spawn initial snake segment at Z=1 to be above food but below head
    segments.0.push_back(
        commands
            .spawn((
                // Sprite component
                Sprite {
                    color: Color::srgb(0.3, 0.3, 0.3),
                    custom_size: Some(Vec2::new(0.8 * WINDOW_WIDTH / GRID_SIZE as f32,
                                              0.8 * WINDOW_HEIGHT / GRID_SIZE as f32)),
                    ..default()
                },
                // Transform and visibility components
                Transform::from_xyz(0.0, -1.0, 1.0),
                GlobalTransform::default(),
                Visibility::default(),
                // Game components
                SnakeSegment,
                Position { x: 3, y: 2 },
                Name::new("Snake Segment"),
            ))
            .id(),
    );
}