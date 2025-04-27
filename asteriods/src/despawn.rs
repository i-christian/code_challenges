use bevy::prelude::*;

use crate::{components::Health, schedule::InGameSet};

const DESPAWN_DISTANCE: f32 = 100.0;

pub struct DespawnPlugin;

impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (despawn_far_away_entities, despawn_dead_entities).in_set(InGameSet::DespawnEntities),
        );
    }
}

/// a system to despawns out of view entities
fn despawn_far_away_entities(mut commands: Commands, query: Query<(Entity, &GlobalTransform)>) {
    for (entity, transform) in query.iter() {
        let distance = transform.translation().distance(Vec3::ZERO);

        // entity far away from camera's viewport
        if distance > DESPAWN_DISTANCE {
            commands.entity(entity).despawn();
        }
    }
}

/// a system to despawn death entities
fn despawn_dead_entities(mut commands: Commands, query: Query<(Entity, &Health)>) {
    for (entity, health) in query.iter() {
        if health.value <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}
