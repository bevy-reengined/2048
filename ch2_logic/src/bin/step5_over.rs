use bevy::prelude::*;
use bevy_rand::{
    global::GlobalEntropy,
    prelude::{EntropyPlugin, WyRand},
};
use ch1_board::*;
use rand::{Rng, seq::IndexedRandom};

#[derive(States, Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
enum GameState {
    #[default]
    Game,
    Menu,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EntropyPlugin::<WyRand>::default())
        .add_plugins(BoardPlugin)
        .add_plugins(ScorePlugin)
        .add_plugins(menu::menu_plugin)
        .init_state::<GameState>()
        .add_systems(Startup, assign_example_board.after(BoardInitialized))
        .add_systems(
            Update,
            update_when_keypress.run_if(in_state(GameState::Game)),
        )
        .run();
}

mod menu {
    use bevy::prelude::*;

    use crate::GameState;

    #[derive(Component)]
    struct OnMenu;

    pub fn menu_plugin(app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), menu_setup)
            .add_systems(OnExit(GameState::Menu), menu_cleanup);
    }

    fn menu_setup(mut commands: Commands) {
        commands.spawn((
            OnMenu,
            Node {
                width: Val::Percent(80.),
                height: Val::Percent(80.),
                align_self: AlignSelf::Center,
                justify_self: JustifySelf::Center,
                ..default()
            },
            BackgroundColor(Color::WHITE),
            children![(OnMenu, Text::new("Game Over"), TextColor::BLACK),],
        ));
    }

    fn menu_cleanup(mut commands: Commands, query: Query<Entity, With<OnMenu>>) {
        for entity in query {
            commands.entity(entity).despawn();
        }
    }
}

#[derive(Resource, Default)]
struct Score(usize);

#[derive(Component)]
#[require(Text2d, Transform::from_xyz(0., 150., 0.))]
struct ScoreLabel;

struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .add_systems(Startup, init_score_label)
            .add_systems(Update, sync_score_label);
    }
}

fn init_score_label(mut commands: Commands) {
    commands.spawn((ScoreLabel,));
}

fn sync_score_label(mut label: Single<&mut Text2d, With<ScoreLabel>>, score: Res<Score>) {
    if score.is_changed() {
        label.0 = format!("hello, score = {}", score.0);
    }
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
    mut score: ResMut<Score>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let board = &mut record.0;

    let mut changed = false;

    // dispatch merge direction
    if keyboard.just_pressed(KeyCode::ArrowLeft) {
        changed = r#move(board, Direction::Left, &mut score.0);
    } else if keyboard.just_pressed(KeyCode::ArrowRight) {
        changed = r#move(board, Direction::Right, &mut score.0);
    } else if keyboard.just_pressed(KeyCode::ArrowUp) {
        changed = r#move(board, Direction::Up, &mut score.0);
    } else if keyboard.just_pressed(KeyCode::ArrowDown) {
        changed = r#move(board, Direction::Down, &mut score.0);
    } else if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Menu);
    }

    if changed {
        spawn(board, &mut rng);
    }

    if is_game_over(board) {
        next_state.set(GameState::Menu);
    }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn r#move(
    board: &mut [[usize; BOARD_SIZE]; BOARD_SIZE],
    direction: Direction,
    score: &mut usize,
) -> bool {
    let mut changed = false;
    changed = compact(board, direction) || changed;
    changed = merge(board, direction, score) || changed;
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

fn merge(
    board: &mut [[usize; BOARD_SIZE]; BOARD_SIZE],
    direction: Direction,
    score: &mut usize,
) -> bool {
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
                *score += board[row][col];
                board[row_next][col_next] = 0;
                changed = true;
            }
        }
    }

    changed
}

fn spawn(board: &mut [[usize; BOARD_SIZE]; BOARD_SIZE], rng: &mut GlobalEntropy<WyRand>) -> bool {
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
        true
    } else {
        false
    }
}

fn is_game_over(board: &[[usize; BOARD_SIZE]; BOARD_SIZE]) -> bool {
    // empty
    for row in board {
        for &val in row {
            if val == 0 {
                return false;
            }
        }
    }

    // row mergable
    for row in 0..BOARD_SIZE {
        for col in 0..(BOARD_SIZE - 1) {
            if board[row][col] == board[row][col + 1] {
                return false;
            }
        }
    }

    // col mergable
    for col in 0..BOARD_SIZE {
        for row in 0..(BOARD_SIZE - 1) {
            if board[row][col] == board[row + 1][col] {
                return false;
            }
        }
    }

    true
}

fn axis_to_row_col(direction: Direction, axis_0: usize, axis_1: usize) -> (usize, usize) {
    use Direction::*;
    match direction {
        Up | Down => (axis_0, axis_1),
        Left | Right => (axis_1, axis_0),
    }
}
