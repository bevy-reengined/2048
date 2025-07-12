use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // we need camera camera
    commands.spawn(Camera2d);

    // spawn a white, centered rectangle
    let mesh = meshes.add(Rectangle::new(50., 100.));
    let material = materials.add(Color::WHITE);
    commands.spawn((
        Mesh2d(mesh),
        MeshMaterial2d(material),
        Transform::from_xyz(0., 0., 0.),
    ));

    // greeting text
    commands.spawn((
        Text2d::new("hello, bevy"),
        // put text a little higher than reactangle
        Transform::from_xyz(0., 75., 0.),
    ));
}
