use bevy::prelude::*;

pub const BOARD_SIZE: usize = 4;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup.in_set(BoardInitialized))
            .add_systems(Update, sync_labels);
    }
}

#[derive(SystemSet, Clone, Debug, Eq, Hash, PartialEq)]
pub struct BoardInitialized;

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

            // spawn the cell shape
            commands.spawn((
                Mesh2d(mesh.clone()),
                MeshMaterial2d(material.clone()),
                transform,
            ));

            // spawn cell label
            commands.spawn((
                BoardCellLabel { row, col },
                // no defaults here
                transform,
            ));
        }
    }

    // init instead of insert using default
    commands.init_resource::<BoardRecord>();
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

#[derive(Resource, Default)]
pub struct BoardRecord(pub [[usize; BOARD_SIZE]; BOARD_SIZE]);

impl BoardRecord {
    pub fn new(inner: [[usize; BOARD_SIZE]; BOARD_SIZE]) -> Self {
        Self(inner)
    }
}

#[derive(Component)]
#[require(Text2d, Transform, TextColor::BLACK)]
pub struct BoardCellLabel {
    pub row: usize,
    pub col: usize,
}
