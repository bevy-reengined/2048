use bevy::prelude::*;
use bevy_rand::{
    global::GlobalEntropy,
    prelude::{EntropyPlugin, WyRand},
};
use rand::{Rng, seq::IndexedRandom};
use step1_board::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EntropyPlugin::<WyRand>::default())
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

fn assign_example_board(mut record: ResMut<BoardRecord>, mut rng: GlobalEntropy<WyRand>) {
    let board = &mut record.0;
    spawn(board, &mut rng);
    spawn(board, &mut rng);
}

fn update_when_keypress(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut record: ResMut<BoardRecord>,
    mut rng: GlobalEntropy<WyRand>,
) {
    let board = &mut record.0;

    let mut changed = false;

    // dispatch merge direction
    if keyboard.just_pressed(KeyCode::ArrowLeft) {
        changed = r#move(board, Direction::Left);
    } else if keyboard.just_pressed(KeyCode::ArrowRight) {
        changed = r#move(board, Direction::Right);
    } else if keyboard.just_pressed(KeyCode::ArrowUp) {
        changed = r#move(board, Direction::Up);
    } else if keyboard.just_pressed(KeyCode::ArrowDown) {
        changed = r#move(board, Direction::Down);
    }

    if changed {
        spawn(board, &mut rng);
    }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn r#move(board: &mut [[usize; BOARD_SIZE]; BOARD_SIZE], direction: Direction) -> bool {
    let mut changed = false;
    changed = compact(board, direction) || changed;
    changed = merge(board, direction) || changed;
    changed = compact(board, direction) || changed;
    changed
}

fn compact(board: &mut [[usize; BOARD_SIZE]; BOARD_SIZE], direction: Direction) -> bool {
    use Direction::*;

    let mut changed = false;

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
                    changed = true;
                }
                new_axis_0 = new_axis_0.saturating_add_signed(step);
            }
        }
    }

    changed
}

fn merge(board: &mut [[usize; BOARD_SIZE]; BOARD_SIZE], direction: Direction) -> bool {
    use Direction::*;

    let mut changed = false;

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

            // axis_0 is empty
            if board[row][col] == 0 {
                break;
            }

            // next is equal
            if board[row][col] == board[row_next][col_next] {
                board[row][col] *= 2;
                board[row_next][col_next] = 0;
                changed = true;
            }
        }
    }

    changed
}

fn spawn(board: &mut [[usize; BOARD_SIZE]; BOARD_SIZE], rng: &mut GlobalEntropy<WyRand>) {
    // prepare domain
    let mut domain = vec![];
    for row in 0..BOARD_SIZE {
        for col in 0..BOARD_SIZE {
            if board[row][col] == 0 {
                domain.push((row, col));
            }
        }
    }

    if let Some(chosen) = domain.choose(rng) {
        let (row, col) = chosen;
        let value = if rng.random_bool(0.9) { 2 } else { 4 };
        board[*row][*col] = value;
    }
}

fn axis_to_row_col(direction: Direction, axis_0: usize, axis_1: usize) -> (usize, usize) {
    use Direction::*;
    match direction {
        Up | Down => (axis_0, axis_1),
        Left | Right => (axis_1, axis_0),
    }
}
