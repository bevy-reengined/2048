use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

const BOARD_SIZE: usize = 4;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);
    // use rectange instead
    let mesh = meshes.add(Rectangle::new(50., 50.));
    let material = materials.add(Color::WHITE);

    // spawn a 4x4 board
    for row in 0..BOARD_SIZE {
        for col in 0..BOARD_SIZE {
            // calculate transform of cell
            let transform = calculate_transform(row, col);

            // spawn the cell
            commands.spawn((
                Mesh2d(mesh.clone()),
                MeshMaterial2d(material.clone()),
                transform,
            ));
        }
    }

    // greeting text
    commands.spawn((
        Text2d::new("hello, board"),
        // move greeting a bit higher
        Transform::from_xyz(0., 150., 0.),
    ));
}

fn calculate_transform(row: usize, col: usize) -> Transform {
    // convert
    let row = row as f32;
    let col = col as f32;

    let mean = 1.5; // `1.5` is mean of `[0, 1, 2, 3]`
    let multiplier = 60.; // `50` width + `10` gap

    let x = (col - mean) * multiplier;
    let y = (row - mean) * multiplier;

    Transform::from_xyz(x, y, 0.)
}
