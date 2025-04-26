use bevy::prelude::*;

use crate::schedule::InGameSet;

const DESPAWN_DISTANCE: f32 = 100.0;

pub struct DespawnPlugin;

impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            despawn_far_away_entities.in_set(InGameSet::DespawnEntities),
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
