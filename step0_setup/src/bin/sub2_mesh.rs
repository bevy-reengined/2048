use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(
            Startup,
            // order does not matter
            (show_greet, show_rectangle),
        )
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

/// Show rectangle in the center
fn show_rectangle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // mesh and material handle
    let mesh = meshes.add(Rectangle::new(50., 100.));
    let material = materials.add(Color::WHITE);

    // mesh in center
    commands.spawn((Mesh2d(mesh), MeshMaterial2d(material)));
}
