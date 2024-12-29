use bevy::prelude::*;
use crate::snake::resources::Score;
use super::components::*;

pub fn spawn_score_ui(mut commands: Commands) {
    // In Bevy 0.15.0, we need to create a simple UI text
    commands.spawn((
        // Text component
        Text::new("Score: 0".to_string()),
        // Position in top-left corner using Transform
        Transform::from_xyz(-380.0, 360.0, 10.0),
        GlobalTransform::default(),
        Visibility::default(),
        ScoreText,
    ));
}

pub fn update_score_text(
    score: Res<Score>,
    mut query: Query<&mut Text, With<ScoreText>>,
) {
    if score.is_changed() {
        if let Ok(mut text) = query.get_single_mut() {
            *text = Text::new(format!("Score: {}", score.value));
        }
    }
}