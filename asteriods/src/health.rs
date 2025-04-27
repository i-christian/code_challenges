use bevy::prelude::*;

use crate::components::Health;

impl Health {
    pub fn new(value: f32) -> Self {
        Self { value }
    }
}
