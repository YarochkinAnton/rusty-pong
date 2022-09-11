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

enum MenuEvents {
    PlayPressed,
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
        .add_state(GameState::Menu)
        .add_startup_system(setup)
        .add_plugins(DefaultPlugins)
        .add_system(bevy::window::close_on_esc)
        .add_plugin(game::GamePlugin)
        .add_event::<MenuEvents>()
        .add_system_set(SystemSet::on_update(GameState::Menu).with_system(enter_the_game))
        .add_system(menu_event_handler)
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn enter_the_game(keyboard_input: Res<Input<KeyCode>>, mut event_writer: EventWriter<MenuEvents>) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        event_writer.send(MenuEvents::PlayPressed);
    }
}

fn menu_event_handler(
    mut event_reader: EventReader<MenuEvents>,
    mut game_state: ResMut<State<GameState>>,
) {
    for event in event_reader.iter() {
        match event {
            MenuEvents::PlayPressed => game_state.set(GameState::Play).unwrap(),
        };
    }
}
