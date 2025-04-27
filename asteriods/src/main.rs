use asset_loader::AssetLoaderPlugin;
use bevy::prelude::*;
mod asset_loader;
mod asteroids;
mod camera;
mod collision_detection;
mod components;
mod debug;
mod despawn;
mod health;
mod movement;
mod schedule;
mod spaceship;
mod state;

use asteroids::AsteroidPlugin;
use camera::CameraPlugin;
use collision_detection::CollisionDetectionPlugin;
use debug::DebugPlugin;
use despawn::DespawnPlugin;
use movement::MovementPlugin;
use schedule::SchedulePlugin;
use spaceship::SpaceshipPlugin;
use state::StatePlugin;

/// main is the entry point of the game logic
fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.01, 0.00, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 100.0,
            affects_lightmapped_meshes: true,
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
        .add_plugins(DespawnPlugin)
        .add_plugins(CollisionDetectionPlugin)
        .add_plugins(SchedulePlugin)
        .add_plugins(StatePlugin)
        .run();
}
