use crate::components::{Acceleration, MovingObjectBundle, Velocity};
use bevy::prelude::*;

/// Spaceship constants
const SPACESHIP_STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, -20.0);
const STARTING_VELOCITY: Vec3 = Vec3::new(0.0, 0.0, 1.0);

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_spaceship);
    }
}

/// spawns a spaceship entity into the game world
fn spawn_spaceship(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(MovingObjectBundle {
        velocity: Velocity {
            value: STARTING_VELOCITY,
        },
        acceleration: Acceleration::new(Vec3::ZERO),
        transform: Transform::from_translation(SPACESHIP_STARTING_TRANSLATION),
        model: SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("Spaceship.glb"))),
    });
}
