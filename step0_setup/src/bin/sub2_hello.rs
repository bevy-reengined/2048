use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_greet_text, spawn_camera))
        .run();
}

// greet text
fn spawn_greet_text(mut commands: Commands) {
    commands.spawn(Text::new("Hello, Bevy!"));
}

// camera
fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
