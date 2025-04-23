use bevy::prelude::*;

/// Velocity component for the spaceship movement
#[derive(Component, Debug)]
pub struct Velocity {
    pub value: Vec3,
}

/// Acceleraration component
#[derive(Component, Debug)]
pub struct Acceleration {
    pub value: Vec3,
}

/// A bundler for moving objects
#[derive(Bundle)]
pub struct MovingObjectBundle {
    pub velocity: Velocity,
    pub acceleration: Acceleration,
    pub collider: Collider,
    pub transform: Transform,
    pub model: SceneRoot,
}

/// spaceship marker component
#[derive(Component, Debug)]
pub struct Spaceship;

/// spaceship missile marker component
#[derive(Component, Debug)]
pub struct SpaceshipMissile;

// Asteroid marker component
#[derive(Component, Debug)]
pub struct Asteroid;

#[derive(Component, Debug)]
pub struct Collider {
    pub radius: f32,
    pub colliding_entities: Vec<Entity>,
}
