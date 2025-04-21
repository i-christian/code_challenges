use bevy::prelude::*;

/// Velocity component for the spaceship movement
#[derive(Component, Debug)]
pub struct Velocity {
    pub value: Vec3,
}

// Acceleraration component
#[derive(Component, Debug)]
pub struct Acceleration {
    pub value: Vec3,
}
