pub mod components;
pub mod resources;
pub mod systems;

pub use components::{Direction, Position, SnakeHead, SnakeSegment};
pub use resources::{FoodSpawnTimer, GridSize, LastTailPosition, Score, SnakeSegments};
pub use systems::{food_collection, food_spawning, snake_movement, snake_movement_input, MovementTimer};