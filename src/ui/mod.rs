use bevy::prelude::*;

mod components;
mod systems;

pub use systems::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_score_ui)
           .add_systems(Update, update_score_text);
    }
}