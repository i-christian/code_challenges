use bevy::prelude::*;
mod components;
mod debug;
mod movement;
mod spaceship;

use debug::DebugPlugin;
use movement::MovementPlugin;
use spaceship::SpaceshipPlugin;

/// main is the entry point of the game logic
fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.04, 0.04, 0.04)))
        .add_plugins(SpaceshipPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(DebugPlugin)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (840.0, 480.0).into(),
                title: "Asteriods".to_string(),
                ..default()
            }),
            ..default()
        }))
        .run();
}
