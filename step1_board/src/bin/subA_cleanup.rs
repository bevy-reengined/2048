use bevy::prelude::*;
use step1_board::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // add plugin here
        .add_plugins(BoardPlugin)
        .add_systems(Startup, greet)
        .add_systems(Update, update_when_keypress)
        .run();
}

fn greet(mut commands: Commands) {
    // greeting text
    commands.spawn((
        Text2d::new("press space to update i+j+1"),
        Transform::from_xyz(0., 150., 0.),
    ));
}

fn update_when_keypress(
    // detect key input
    keyboard: Res<ButtonInput<KeyCode>>,
    mut record: ResMut<BoardRecord>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                record.0[i][j] += i + j + 1;
            }
        }
    }
}
