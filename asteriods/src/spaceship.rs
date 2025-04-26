use crate::{
    asset_loader::SceneAssets,
    components::{
        Acceleration, Collider, MovingObjectBundle, Spaceship, SpaceshipMissile, Velocity,
    },
};
use bevy::prelude::*;

/// Spaceship constants
const SPACESHIP_STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, -20.0);
const SPACESHIP_SPEED: f32 = 25.0;
const SPACESHIP_ROTATION_SPEED: f32 = 2.5;
const SPACESHIP_ROLL_SPEED: f32 = 2.5;
const MISSILE_SPEED: f32 = 50.0;
const MISSILE_FORWARD_SPAWN_SCALAR: f32 = 7.5;
const MISSILE_RADIUS: f32 = 1.0;
const SPACESHIP_RADIUS: f32 = 5.0;

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_spaceship).add_systems(
            Update,
            (spaceship_movement_controls, spaceship_weapon_controls),
        );
    }
}

/// spawns a spaceship entity into the game world
fn spawn_spaceship(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity { value: Vec3::ZERO },
            acceleration: Acceleration::new(Vec3::ZERO),
            collider: Collider::new(SPACESHIP_RADIUS),
            transform: Transform::from_translation(SPACESHIP_STARTING_TRANSLATION),
            model: SceneRoot(scene_assets.spaceship.clone()),
        },
        Spaceship,
    ));
}

/// spaceship movement system
fn spaceship_movement_controls(
    mut query: Query<(&mut Transform, &mut Velocity), With<Spaceship>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let (mut transform, mut velocity) = query
        .single_mut()
        .expect("Error: could not find a single spaceship");

    let mut rotation = 0.0;
    let mut roll = 0.0;
    let mut movement = 0.0;

    // Spaceship rotation controls
    if keyboard_input.pressed(KeyCode::KeyD) {
        rotation = -SPACESHIP_ROTATION_SPEED * time.delta_secs();
    } else if keyboard_input.pressed(KeyCode::KeyA) {
        rotation = SPACESHIP_ROTATION_SPEED * time.delta_secs();
    }

    // spaceship movement controls
    if keyboard_input.pressed(KeyCode::KeyS) {
        movement = -SPACESHIP_SPEED;
    } else if keyboard_input.pressed(KeyCode::KeyW) {
        movement = SPACESHIP_SPEED;
    }

    // spin the spaceship around z-axis
    if keyboard_input.pressed(KeyCode::ShiftLeft) {
        roll = -SPACESHIP_ROLL_SPEED * time.delta_secs();
    } else if keyboard_input.pressed(KeyCode::ShiftRight) {
        roll = SPACESHIP_ROLL_SPEED * time.delta_secs();
    }

    transform.rotate_y(rotation);
    transform.rotate_local_z(roll);

    // update the spaceship's velocity based on new direction
    velocity.value = -transform.forward() * movement;
}

// fire spaceship weapons system
fn spaceship_weapon_controls(
    mut commands: Commands,
    query: Query<&Transform, With<Spaceship>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    scene_assets: Res<SceneAssets>,
) {
    let transform = query
        .single()
        .expect("Error: could not find a single spaceship");
    if keyboard_input.pressed(KeyCode::Space) {
        commands.spawn((
            MovingObjectBundle {
                velocity: Velocity {
                    value: -transform.forward() * MISSILE_SPEED,
                },
                acceleration: Acceleration::new(Vec3::ZERO),
                collider: Collider::new(MISSILE_RADIUS),
                transform: Transform::from_translation(
                    transform.translation + -transform.forward() * MISSILE_FORWARD_SPAWN_SCALAR,
                ),
                model: SceneRoot(scene_assets.missiles.clone()),
            },
            SpaceshipMissile,
        ));
    }
}
