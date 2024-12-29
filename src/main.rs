use bevy::{
    prelude::*,
    window::{WindowMode, PresentMode},
    app::AppExit,
    input::keyboard::KeyCode,
};
use std::collections::VecDeque;

mod snake;
use snake::{components::*, resources::*};

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 800.0;
const GRID_SIZE: u32 = 20;

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
        .insert_state(GameState::Playing)
        .add_systems(PreStartup, setup)
        .add_systems(Startup, spawn_snake)
        .add_systems(Update, check_for_exit)
        .run();
}

// Handle ESC key to exit the game
fn check_for_exit(
    keyboard_input: ResMut<ButtonInput<KeyCode>>,
    mut exit: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
    }
}

fn setup(mut commands: Commands) {
    // Spawn camera
    commands.spawn(Camera2d::default());
}

fn spawn_snake(mut commands: Commands, mut segments: ResMut<SnakeSegments>) {
    *segments = SnakeSegments(VecDeque::new());

    // Spawn snake head
    segments.0.push_front(
        commands
            .spawn((
                Sprite {
                    color: Color::rgb(0.7, 0.7, 0.7),
                    custom_size: Some(Vec2::new(0.8 * WINDOW_WIDTH / GRID_SIZE as f32,
                                              0.8 * WINDOW_HEIGHT / GRID_SIZE as f32)),
                    ..default()
                },
                Transform::from_xyz(0.0, 0.0, 1.0),
                SnakeHead {
                    direction: Direction::Up,
                },
                SnakeSegment,
                Position { x: 3, y: 3 },
                Name::new("Snake Head"),
            ))
            .id(),
    );

    // Spawn initial snake segment
    segments.0.push_back(
        commands
            .spawn((
                Sprite {
                    color: Color::rgb(0.3, 0.3, 0.3),
                    custom_size: Some(Vec2::new(0.8 * WINDOW_WIDTH / GRID_SIZE as f32,
                                              0.8 * WINDOW_HEIGHT / GRID_SIZE as f32)),
                    ..default()
                },
                Transform::from_xyz(0.0, -1.0, 0.0),
                SnakeSegment,
                Position { x: 3, y: 2 },
                Name::new("Snake Segment"),
            ))
            .id(),
    );
}
