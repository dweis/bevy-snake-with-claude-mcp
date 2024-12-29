use bevy::prelude::*;
use rand::Rng;
use crate::snake::{components::*, resources::*};
use crate::{WINDOW_WIDTH, WINDOW_HEIGHT, GRID_SIZE, GameState};

pub const SNAKE_MOVEMENT_INTERVAL: f32 = 0.15;

#[derive(Resource)]
pub struct MovementTimer(pub Timer);

impl Default for MovementTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(SNAKE_MOVEMENT_INTERVAL, TimerMode::Repeating))
    }
}

pub fn snake_movement_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut SnakeHead>,
) {
    if let Ok(mut head) = query.get_single_mut() {
        let dir: Direction = if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
            Direction::Left
        } else if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
            Direction::Right
        } else if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
            Direction::Down
        } else if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
            Direction::Up
        } else {
            head.direction
        };

        // Prevent 180-degree turns
        if dir != head.direction.opposite() {
            head.direction = dir;
        }
    }
}

pub fn snake_movement(
    mut timer: ResMut<MovementTimer>,
    snake_positions: Res<SnakeSegments>,
    mut last_tail_position: ResMut<LastTailPosition>,
    mut game_state: ResMut<NextState<GameState>>,
    mut snake_query: Query<(&mut Position, &mut Transform, &SnakeHead)>,
    mut segments_query: Query<(&mut Position, &mut Transform), (With<SnakeSegment>, Without<SnakeHead>)>,
    time: Res<Time>,
) {
    timer.0.tick(time.delta());

    if !timer.0.just_finished() {
        return;
    }

    if let Ok((mut head_pos, mut head_transform, head)) = snake_query.get_single_mut() {
        // Get current positions before updating
        let segment_positions: Vec<_> = segments_query
            .iter()
            .map(|(pos, _)| *pos)
            .collect();

        let new_head_pos = match head.direction {
            Direction::Left => Position {
                x: head_pos.x - 1,
                y: head_pos.y
            },
            Direction::Right => Position {
                x: head_pos.x + 1,
                y: head_pos.y
            },
            Direction::Up => Position {
                x: head_pos.x,
                y: head_pos.y + 1
            },
            Direction::Down => Position {
                x: head_pos.x,
                y: head_pos.y - 1
            },
        };

        // Check for self-collision
        if segment_positions.contains(&new_head_pos) {
            game_state.set(GameState::GameOver);
            return;
        }

        // Check for wall collision
        if new_head_pos.x < 0 || new_head_pos.x >= GRID_SIZE as i32 ||
           new_head_pos.y < 0 || new_head_pos.y >= GRID_SIZE as i32 {
            game_state.set(GameState::GameOver);
            return;
        }

        // Calculate scaled grid coordinates
        let grid_x = new_head_pos.x as f32 * (WINDOW_WIDTH / GRID_SIZE as f32);
        let grid_y = new_head_pos.y as f32 * (WINDOW_HEIGHT / GRID_SIZE as f32);
        let grid_center_x = grid_x - WINDOW_WIDTH / 2.0 + (WINDOW_WIDTH / GRID_SIZE as f32) / 2.0;
        let grid_center_y = grid_y - WINDOW_HEIGHT / 2.0 + (WINDOW_HEIGHT / GRID_SIZE as f32) / 2.0;

        let mut new_head_transform = Transform::from_xyz(
            grid_center_x,
            grid_center_y,
            2.0, // Keep head at z=2
        );
        new_head_transform.scale = head_transform.scale;

        // Update head position
        *head_pos = new_head_pos;
        *head_transform = new_head_transform;

        // Store positions for updating segments
        let mut positions = vec![new_head_pos];
        positions.extend(segment_positions.iter().take(snake_positions.0.len() - 1));

        // Update the last tail position for potential growth
        if let Some(last_pos) = segment_positions.last() {
            *last_tail_position = LastTailPosition(Some(*last_pos));
        }

        // Update segment positions
        for (i, entity) in snake_positions.0.iter().skip(1).enumerate() {
            if let Ok((mut pos, mut transform)) = segments_query.get_mut(*entity) {
                *pos = positions[i];
                let grid_x = pos.x as f32 * (WINDOW_WIDTH / GRID_SIZE as f32);
                let grid_y = pos.y as f32 * (WINDOW_HEIGHT / GRID_SIZE as f32);
                let grid_center_x = grid_x - WINDOW_WIDTH / 2.0 + (WINDOW_WIDTH / GRID_SIZE as f32) / 2.0;
                let grid_center_y = grid_y - WINDOW_HEIGHT / 2.0 + (WINDOW_HEIGHT / GRID_SIZE as f32) / 2.0;

                transform.translation = Vec3::new(
                    grid_center_x,
                    grid_center_y,
                    1.0, // Keep segments at z=1
                );
            }
        }
    }
}

pub fn food_spawning(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<FoodSpawnTimer>,
    food_query: Query<&Position, With<Food>>,
    snake_query: Query<&Position, Or<(With<SnakeHead>, With<SnakeSegment>)>>,
    grid_size: Res<GridSize>,
) {
    timer.0.tick(time.delta());

    if !timer.0.just_finished() {
        return;
    }

    // Get all occupied positions
    let snake_positions: Vec<Position> = snake_query.iter().copied().collect();
    let food_positions: Vec<Position> = food_query.iter().copied().collect();

    // Only spawn food if there isn't any
    if food_positions.is_empty() {
        let mut rng = rand::thread_rng();
        let mut new_pos = Position {
            x: rng.gen_range(0..grid_size.x as i32),
            y: rng.gen_range(0..grid_size.y as i32),
        };

        // Keep generating new positions until we find an unoccupied one
        while snake_positions.contains(&new_pos) {
            new_pos = Position {
                x: rng.gen_range(0..grid_size.x as i32),
                y: rng.gen_range(0..grid_size.y as i32),
            };
        }

        // Calculate grid-centered position
        let grid_x = new_pos.x as f32 * (WINDOW_WIDTH / GRID_SIZE as f32);
        let grid_y = new_pos.y as f32 * (WINDOW_HEIGHT / GRID_SIZE as f32);
        let grid_center_x = grid_x - WINDOW_WIDTH / 2.0 + (WINDOW_WIDTH / GRID_SIZE as f32) / 2.0;
        let grid_center_y = grid_y - WINDOW_HEIGHT / 2.0 + (WINDOW_HEIGHT / GRID_SIZE as f32) / 2.0;

        commands.spawn((
            Food,
            new_pos,
            Sprite {
                color: Color::srgb(1.0, 0.0, 0.0), // Red color
                custom_size: Some(Vec2::new(
                    0.8 * WINDOW_WIDTH / GRID_SIZE as f32,
                    0.8 * WINDOW_HEIGHT / GRID_SIZE as f32,
                )),
                ..default()
            },
            Transform::from_xyz(
                grid_center_x,
                grid_center_y,
                0.0, // Food at z=0, below segments and head
            ),
            Name::new("Food"),
        ));
    }

    // Reset the timer
    timer.0.reset();
}

pub fn food_collection(
    mut commands: Commands,
    mut snake_segments: ResMut<SnakeSegments>,
    mut score: ResMut<Score>,
    food_query: Query<(Entity, &Position), With<Food>>,
    head_query: Query<&Position, With<SnakeHead>>,
    last_tail_pos: Res<LastTailPosition>,
) {
    if let Ok(head_pos) = head_query.get_single() {
        for (food_entity, food_pos) in food_query.iter() {
            if head_pos == food_pos {
                // Remove the food
                commands.entity(food_entity).despawn();

                // Increase score
                score.value += 1;

                // Spawn new segment
                if let Some(last_pos) = last_tail_pos.0 {
                    // Calculate grid-centered position for new segment
                    let grid_x = last_pos.x as f32 * (WINDOW_WIDTH / GRID_SIZE as f32);
                    let grid_y = last_pos.y as f32 * (WINDOW_HEIGHT / GRID_SIZE as f32);
                    let grid_center_x = grid_x - WINDOW_WIDTH / 2.0 + (WINDOW_WIDTH / GRID_SIZE as f32) / 2.0;
                    let grid_center_y = grid_y - WINDOW_HEIGHT / 2.0 + (WINDOW_HEIGHT / GRID_SIZE as f32) / 2.0;

                    snake_segments.0.push_back(
                        commands
                            .spawn((
                                Sprite {
                                    color: Color::srgb(0.3, 0.3, 0.3),
                                    custom_size: Some(Vec2::new(
                                        0.8 * WINDOW_WIDTH / GRID_SIZE as f32,
                                        0.8 * WINDOW_HEIGHT / GRID_SIZE as f32,
                                    )),
                                    ..default()
                                },
                                Transform::from_xyz(
                                    grid_center_x,
                                    grid_center_y,
                                    1.0,
                                ),
                                SnakeSegment,
                                last_pos,
                                Name::new("Snake Segment"),
                            ))
                            .id(),
                    );
                }
            }
        }
    }
}