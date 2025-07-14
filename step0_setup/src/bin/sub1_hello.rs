use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_camera, show_greet))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

/// Show greet in the top-left corner.
fn show_greet(
    // commands for spawning entity
    mut commands: Commands,
) {
    commands.spawn(Text::new("Hello, Bevy!"));
}
