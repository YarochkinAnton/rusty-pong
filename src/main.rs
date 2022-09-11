use bevy::{
    prelude::*,
    window::WindowMode,
};

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
enum GameState {
    Menu,
    Play,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(WindowDescriptor {
            title: "Rusty Pong".to_string(),
            resizable: false,
            cursor_visible: true,
            cursor_locked: false,
            mode: WindowMode::BorderlessFullscreen,
            ..default()
        })
        .add_startup_system(setup)
        .add_state(GameState::Menu)
        .add_plugins(DefaultPlugins)
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}
