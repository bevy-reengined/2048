use bevy::prelude::*;
use ch1_board::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BoardPlugin)
        .add_systems(Startup, greet)
        .add_systems(Startup, assign_example_board.after(BoardInitialized))
        .add_systems(Update, update_when_keypress)
        .run();
}

fn greet(mut commands: Commands) {
    commands.spawn((
        Text2d::new("hello, merge"),
        Transform::from_xyz(0., 150., 0.),
    ));
}

fn assign_example_board(mut record: ResMut<BoardRecord>) {
    record.0 = [[1, 0, 1, 0], [0, 1, 0, 1], [1, 1, 0, 0], [0, 1, 0, 0]];
}

fn update_when_keypress(keyboard: Res<ButtonInput<KeyCode>>, mut record: ResMut<BoardRecord>) {
    let board = &mut record.0;

    // dispatch merge direction
    if keyboard.just_pressed(KeyCode::ArrowLeft) {
        move_left(board);
    }
}

fn move_left(board: &mut [[usize; BOARD_SIZE]; BOARD_SIZE]) {
    compact_left(board);
    merge_left(board);
    compact_left(board);
}

fn compact_left(board: &mut [[usize; BOARD_SIZE]; BOARD_SIZE]) {
    for row in 0..BOARD_SIZE {
        let mut col_new = 0;
        for col in 0..BOARD_SIZE {
            if board[row][col] != 0 {
                board[row][col_new] = board[row][col];
                if col_new != col {
                    board[row][col] = 0;
                }
                col_new += 1;
            }
        }
    }
}

fn merge_left(board: &mut [[usize; BOARD_SIZE]; BOARD_SIZE]) {
    for row in 0..BOARD_SIZE {
        for col in 0..(BOARD_SIZE - 1) {
            let col_next = col + 1;

            if board[row][col] == 0 {
                continue;
            }

            if board[row][col] == board[row][col_next] {
                board[row][col] *= 2;
                board[row][col_next] = 0;
            }
        }
    }
}
