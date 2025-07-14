use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, show_greet)
        .run();
}

fn show_greet(mut commands: Commands) {
    // camera
    commands.spawn(Camera2d);

    // greet text
    commands.spawn((
        Text::new("Hello, Bevy!"),
        // 居中显示
        Node {
            margin: UiRect::all(Val::Auto),
            ..default()
        },
        // default is 20.
        TextFont::from_font_size(40.),
    ));
}
