use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.04, 0.04, 0.04)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (840.0, 480.0).into(),
                title: "Space Invaders".to_string(),
                ..default()
            }),
            ..default()
        }))
        .run();
}
