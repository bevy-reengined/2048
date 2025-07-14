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
        r#move(board, Direction::Left);
    } else if keyboard.just_pressed(KeyCode::ArrowRight) {
        r#move(board, Direction::Right);
    } else if keyboard.just_pressed(KeyCode::ArrowUp) {
        r#move(board, Direction::Up);
    } else if keyboard.just_pressed(KeyCode::ArrowDown) {
        r#move(board, Direction::Down);
    }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn r#move(board: &mut [[usize; BOARD_SIZE]; BOARD_SIZE], direction: Direction) {
    compact(board, direction);
    merge(board, direction);
    compact(board, direction);
}

fn compact(board: &mut [[usize; BOARD_SIZE]; BOARD_SIZE], direction: Direction) {
    use Direction::*;

    for axis_1 in 0..BOARD_SIZE {
        let (mut new_axis_0, step) = match direction {
            Up | Left => (0, 1),
            Right | Down => (BOARD_SIZE - 1, -1),
        };

        let axis_0_iter: &mut dyn Iterator<Item = usize> = match direction {
            Up | Left => &mut (0..BOARD_SIZE),
            Right | Down => &mut (0..BOARD_SIZE).rev(),
        };
        for axis_0 in axis_0_iter {
            let (row, col) = axis_to_row_col(direction, axis_0, axis_1);
            let (new_row, new_col) = axis_to_row_col(direction, new_axis_0, axis_1);

            if board[row][col] != 0 {
                board[new_row][new_col] = board[row][col];
                if new_axis_0 != axis_0 {
                    board[row][col] = 0;
                }
                new_axis_0 = new_axis_0.saturating_add_signed(step);
            }
        }
    }
}

fn merge(board: &mut [[usize; BOARD_SIZE]; BOARD_SIZE], direction: Direction) {
    use Direction::*;

    let step = match direction {
        Up | Left => 1,
        Right | Down => -1,
    };

    for axis_1 in 0..BOARD_SIZE {
        let axis_0_iter: &mut dyn Iterator<Item = usize> = match direction {
            Up | Left => &mut (0..(BOARD_SIZE - 1)),
            Right | Down => &mut (1..BOARD_SIZE).rev(),
        };
        for axis_0 in axis_0_iter {
            let axis_0_next = axis_0.saturating_add_signed(step);

            let (row, col) = axis_to_row_col(direction, axis_0, axis_1);
            let (row_next, col_next) = axis_to_row_col(direction, axis_0_next, axis_1);

            if board[row][col] == 0 {
                continue;
            }

            if board[row][col] == board[row_next][col_next] {
                board[row][col] *= 2;
                board[row_next][col_next] = 0;
            }
        }
    }
}

fn axis_to_row_col(direction: Direction, axis_0: usize, axis_1: usize) -> (usize, usize) {
    use Direction::*;
    match direction {
        Up | Down => (axis_0, axis_1),
        Left | Right => (axis_1, axis_0),
    }
}
