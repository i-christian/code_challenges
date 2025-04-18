use bevy::prelude::*;

/// Velocity component for the spaceship movement
#[derive(Component, Debug)]
pub struct Velocity {
    pub value: Vec3,
}
