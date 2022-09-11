use bevy::prelude::*;

use crate::GameState;

#[derive(Debug, PartialEq, Eq)]
#[derive(Component)]
pub enum Player {
    Left,
    Right,
}

pub enum PlayerMovementEvent {
    Up(Player),
    Down(Player),
}

#[derive(Component)]
pub struct Pad;

impl Pad {
    // in % of the screen width
    const WIDTH: f32 = 1.0;
    // in % of the screen height
    const HEIGHT: f32 = 20.0;
    const DEFAUT_LEVEL: f32 = 0.0;
    // in % of the screen width
    const HORIZONTAL_MARGIN: f32 = 10.0;
    const SPEED: f32 = 2.0;
}


pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerMovementEvent>();
        app.add_system_set(
            SystemSet::on_update(GameState::Play)
                .with_system(keyboard_input_system)
                .with_system(pad_movement_system),
        );
        app.add_system_set(SystemSet::on_enter(GameState::Play).with_system(spawn_pad));
    }
}

pub fn spawn_pad(mut commands: Commands, windows: ResMut<Windows>) {
    let primary_window = windows.get_primary().expect("Failed to get primary window");
    let window_width = primary_window.width();
    let window_height = primary_window.height();

    // 10% left or right side of the screen
    let pad_x_margin = window_width / 100.0 * Pad::HORIZONTAL_MARGIN;

    let pad_height = window_height / 100.0 * Pad::HEIGHT;
    let pad_width = window_width / 100.0 * Pad::WIDTH;

    let left_pad_x_offset = 0.0 - window_width / 2.0 + pad_x_margin;
    let right_pad_x_offset = 0.0 + window_width / 2.0 - pad_x_margin;

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::BLUE,
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(pad_width, pad_height, 1.0),
                translation: Vec3::new(left_pad_x_offset, Pad::DEFAUT_LEVEL, 1.0),
                ..default()
            },
            ..default()
        })
        .insert(Player::Left)
        .insert(Pad);

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(pad_width, pad_height, 1.0),
                translation: Vec3::new(right_pad_x_offset, Pad::DEFAUT_LEVEL, 1.0),
                ..default()
            },
            ..default()
        })
        .insert(Player::Right)
        .insert(Pad);
}

fn pad_movement_system(
    mut event_reader: EventReader<PlayerMovementEvent>,
    mut query: Query<(&Pad, &Player, &mut Transform)>,
    windows: ResMut<Windows>,
) {
    let window_height = windows
        .get_primary()
        .expect("Failed to get primary window")
        .height();
    let movement: f32 = window_height / 100.0 * Pad::SPEED;
    for event in event_reader.iter() {
        match event {
            PlayerMovementEvent::Up(event_player) =>
                for q in query.iter_mut() {
                    let (_pad, player, mut transform): (&Pad, &Player, Mut<Transform>) = q;
                    if player == event_player {
                        transform.translation += Vec3::new(0.0, movement, 0.0);
                    }
                },
            PlayerMovementEvent::Down(event_player) =>
                for q in query.iter_mut() {
                    let (_pad, player, mut transform): (&Pad, &Player, Mut<Transform>) = q;
                    if player == event_player {
                        transform.translation += Vec3::new(0.0, -movement, 0.0);
                    }
                },
        }
    }
}

fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut event_writer: EventWriter<PlayerMovementEvent>,
) {
    if keyboard_input.pressed(KeyCode::W) {
        info!("Player 1 - UP");
        event_writer.send(PlayerMovementEvent::Up(Player::Left));
    }

    if keyboard_input.pressed(KeyCode::S) {
        info!("Player 1 - DOWN");
        event_writer.send(PlayerMovementEvent::Down(Player::Left));
    }


    if keyboard_input.pressed(KeyCode::Up) {
        info!("Player 2 - UP");
        event_writer.send(PlayerMovementEvent::Up(Player::Right));
    }

    if keyboard_input.pressed(KeyCode::Down) {
        info!("Player 2 - DOWN");
        event_writer.send(PlayerMovementEvent::Down(Player::Right));
    }
}
