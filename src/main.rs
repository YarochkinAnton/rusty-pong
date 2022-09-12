use bevy::{
    prelude::*,
    window::WindowMode,
};

mod game;

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
enum GameState {
    Menu,
    Play,
}

enum ControlEvents {
    SpacePressed,
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Rusty Pong".to_string(),
            resizable: false,
            cursor_visible: true,
            cursor_locked: false,
            mode: WindowMode::BorderlessFullscreen,
            ..default()
        })
        .insert_resource(ClearColor(Color::BLACK))
        .add_startup_system(setup)
        .add_state(GameState::Menu)
        .add_plugins(DefaultPlugins)
        .add_system(bevy::window::close_on_esc)
        .add_plugin(game::GamePlugin)
        .add_event::<ControlEvents>()
        .add_system(main_control)
        .add_system(menu_event_handler)
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn main_control(keyboard_input: Res<Input<KeyCode>>, mut event_writer: EventWriter<ControlEvents>) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        event_writer.send(ControlEvents::SpacePressed);
    }
}

fn menu_event_handler(
    mut event_reader: EventReader<ControlEvents>,
    mut game_state: ResMut<State<GameState>>,
) {
    for event in event_reader.iter() {
        match (event, game_state.current()) {
            (ControlEvents::SpacePressed, GameState::Menu) =>
                game_state.set(GameState::Play).unwrap(),
            (ControlEvents::SpacePressed, GameState::Play) =>
                game_state.set(GameState::Menu).unwrap(),
        };
    }
}
