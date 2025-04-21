use std::ops::Range;

use bevy::prelude::*;

use crate::components::{Acceleration, MovingObjectBundle, Velocity};

const VELOCITY_SCALAR: f32 = 5.0;
const ACCELERATION_SCALAR: f32 = 1.0;
const SPAWN_RANGE_X: Range<f32> = -25.0..25.0;
const SPAWN_RANGE_Z: Range<f32> = 0.0..25.0;
const SPAWN_TIME_SECONDS: f32 = 1.0;

#[derive(Component, Debug)]
pub struct Asteroid;

#[derive(Resource, Debug)]
pub struct SpawnTimer {
    timer: Timer,
}

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer {
            timer: Timer::from_seconds(SPAWN_TIME_SECONDS, TimerMode::Repeating),
        })
        .add_systems(Update, spawn_asteroid);
    }
}

/// a system to spawn asteroids
fn spawn_asteroid(
    mut commands: Commands,
    mut spawn_timer: ResMut<SpawnTimer>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
) {
    spawn_timer.timer.tick(time.delta());
    if !spawn_timer.timer.just_finished() {
        return;
    }

    let translation = Vec3::new(
        rand::random_range(SPAWN_RANGE_X),
        0.0,
        rand::random_range(SPAWN_RANGE_Z),
    );

    // normalise generated vectors to have the same magnitude
    let random_unit_vector = || {
        Vec3::new(
            rand::random_range(-1.0..1.0),
            0.0,
            rand::random_range(-1.0..1.0),
        )
        .normalize_or_zero()
    };
    let velocity = random_unit_vector() * VELOCITY_SCALAR;
    let acceleration = random_unit_vector() * ACCELERATION_SCALAR;

    // spawn the moving object bundle for asteroid
    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity { value: velocity },
            acceleration: Acceleration::new(acceleration),
            transform: Transform::from_translation(translation),
            model: SceneRoot(
                asset_server.load(GltfAssetLabel::Scene(0).from_asset("Asteroid.glb")),
            ),
        },
        Asteroid,
    ));
}
