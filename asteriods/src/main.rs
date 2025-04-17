use bevy::prelude::*;

#[derive(Component, Debug)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component, Debug)]
struct Velocity {
    x: f32,
    y: f32,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.04, 0.04, 0.04)))
        .add_systems(Startup, spawn_spaceship)
        .add_systems(Update, (update_position, print_position))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (840.0, 480.0).into(),
                title: "Asteriods".to_string(),
                ..default()
            }),
            ..default()
        }))
        .run();
}

fn spawn_spaceship(mut commands: Commands) {
    commands.spawn((Position { x: 0.0, y: 0.0 }, Velocity { x: 1.0, y: 1.0 }));
}

fn update_position(mut query: Query<(&mut Velocity, &mut Position)>) {
    for (velocity, mut position) in query.iter_mut() {
        position.x += velocity.x;
        position.y += velocity.y;
    }
}

fn print_position(query: Query<(Entity, &Position)>) {
    for (entity, position) in query.iter() {
        info!("Entity {:?} is at position {:?},", entity, position);
    }
}
