use asset_loader::AssetLoaderPlugin;
use bevy::prelude::*;
mod asset_loader;
mod asteroids;
mod camera;
mod components;
mod debug;
mod movement;
mod spaceship;

use asteroids::AsteroidPlugin;
use camera::CameraPlugin;
use debug::DebugPlugin;
use movement::MovementPlugin;
use spaceship::SpaceshipPlugin;

/// main is the entry point of the game logic
fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.01, 0.00, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 100.0,
        })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (840.0, 480.0).into(),
                title: "Asteriods".to_string(),
                ..default()
            }),
            ..default()
        }))
        // User defined plugins
        .add_plugins(CameraPlugin)
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(SpaceshipPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(DebugPlugin)
        .add_plugins(AsteroidPlugin)
        .run();
}
