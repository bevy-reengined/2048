use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (update_when_keypress, sync_labels))
        .run();
}

const WIDTH: usize = 4;

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
    for row in 0..WIDTH {
        for col in 0..WIDTH {
            // calculate transform of cell
            let transform = calculate_transform(row, col);

            // spawn the cell shape
            commands.spawn((
                Mesh2d(mesh.clone()),
                MeshMaterial2d(material.clone()),
                transform,
            ));

            // spawn cell label
            commands.spawn((
                BoardCellLabel { row, col },
                // no default text any more
                Text2d::default(),
                transform,
                // use black color on white background
                TextColor::BLACK,
            ));
        }
    }

    // insert resource
    commands.insert_resource(BoardRecord([[0; 4]; 4]));

    // greeting text
    commands.spawn((
        Text2d::new("press space to update i+j+1"),
        Transform::from_xyz(0., 150., 0.),
    ));
}

#[derive(Resource)]
struct BoardRecord([[usize; 4]; 4]);

#[derive(Component)]
struct BoardCellLabel {
    row: usize,
    col: usize,
}

fn update_when_keypress(
    // detect key input
    keyboard: Res<ButtonInput<KeyCode>>,
    mut record: ResMut<BoardRecord>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        for i in 0..WIDTH {
            for j in 0..WIDTH {
                record.0[i][j] += i + j + 1;
            }
        }
    }
}

fn sync_labels(record: Res<BoardRecord>, labels: Query<(&BoardCellLabel, &mut Text2d)>) {
    if record.is_changed() {
        for (BoardCellLabel { row, col }, mut text) in labels {
            text.0 = record.0[*row][*col].to_string();
        }
    }
}

fn calculate_transform(row: usize, col: usize) -> Transform {
    // convert
    let row = row as f32;
    let col = col as f32;

    let mean = 1.5; // `1.5` is mean of `[0, 1, 2, 3]`
    let multiplier = 60.; // `50` width + `10` gap

    let x = (col - mean) * multiplier;
    // fix coordinate
    let y = (row - mean) * -multiplier;

    Transform::from_xyz(x, y, 0.)
}
