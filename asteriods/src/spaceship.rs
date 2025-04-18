use bevy::prelude::*;

use crate::components::Velocity;

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_spaceship);
    }
}

fn spawn_spaceship(mut commands: Commands) {
    commands.spawn((
        Transform::default(),
        Visibility::default(),
        Velocity {
            value: Vec3::new(0.0, 0.0, 0.0),
        },
    ));
}
