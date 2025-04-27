use std::ops::Range;

use bevy::prelude::*;

use crate::{
    asset_loader::SceneAssets,
    components::{
        Acceleration, Asteroid, Collider, CollisionDamage, Health, MovingObjectBundle, Velocity,
    },
    schedule::InGameSet,
};

const VELOCITY_SCALAR: f32 = 5.0;
const ACCELERATION_SCALAR: f32 = 1.0;
const SPAWN_RANGE_X: Range<f32> = -25.0..25.0;
const SPAWN_RANGE_Z: Range<f32> = 0.0..25.0;
const SPAWN_TIME_SECONDS: f32 = 1.0;
const ROTATE_SPEED: f32 = 2.5;
const RADIUS: f32 = 2.5;
const HEALTH: f32 = 80.0;
const COLLISION_DAMAGE: f32 = 20.0;

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
        .add_systems(
            Update,
            (spawn_asteroid, rotate_asteroids).in_set(InGameSet::EntityUpdates),
        );
    }
}

/// a system to spawn asteroids
fn spawn_asteroid(
    mut commands: Commands,
    mut spawn_timer: ResMut<SpawnTimer>,
    time: Res<Time>,
    scene_assets: Res<SceneAssets>,
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
            collider: Collider::new(RADIUS),
            transform: Transform::from_translation(translation),
            model: SceneRoot(scene_assets.asteroid.clone()),
        },
        Asteroid,
        Health::new(HEALTH),
        CollisionDamage::new(COLLISION_DAMAGE),
    ));
}

/// asteroids rotation system
fn rotate_asteroids(mut query: Query<&mut Transform, With<Asteroid>>, time: Res<Time>) {
    for mut transform in query.iter_mut() {
        transform.rotate_local_z(ROTATE_SPEED * time.delta_secs());
    }
}
