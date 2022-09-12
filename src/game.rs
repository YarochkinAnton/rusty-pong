use std::f32::consts::PI;

use bevy::{
    prelude::*,
    sprite::collide_aabb::{
        collide,
        Collision,
    },
    time::FixedTimestep,
};

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

#[derive(Component)]
pub struct Collider;

// In % of screen height
const WALL_THICKNESS: f32 = 3.0;

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

#[derive(Component)]
pub struct Ball {
    angle: Vec2,
}

impl Default for Ball {
    fn default() -> Self {
        Self {
            angle: Vec2::from_angle(PI * rand::random::<f32>()),
        }
    }
}

impl Ball {
    const SPEED: f32 = 1.25;

    // In % of screen width
    const SIZE: f32 = 2.0;
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerMovementEvent>();
        app.add_system_set(
            SystemSet::on_enter(GameState::Play)
                .with_system(spawn_pad)
                .with_system(spawn_walls)
                .with_system(spawn_ball),
        );
        app.add_system_set(
            SystemSet::on_update(GameState::Play).with_system(keyboard_input_system),
        );
        app.add_system_set(
            SystemSet::on_update(GameState::Play)
                .with_run_criteria(FixedTimestep::step(1.0 / 60.0))
                .with_system(pad_movement_system)
                .with_system(ball_movement_system)
                .with_system(collision_system.after(ball_movement_system)),
        );
        app.add_system_set(
            SystemSet::on_exit(GameState::Play)
                .with_system(clear::<Pad>)
                .with_system(clear::<Ball>),
        );
    }
}

pub fn spawn_walls(mut commands: Commands, windows: ResMut<Windows>) {
    let primary_window = windows.get_primary().expect("Failed to get primary window");
    let window_width = primary_window.width();
    let window_height = primary_window.height();

    let wall_height = window_height / 100.0 * WALL_THICKNESS;

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(window_width, wall_height)),
            color: Color::WHITE,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, window_height / 2.0 - wall_height / 2.0, 1.0),
            ..default()
        },
        ..default()
    });

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(window_width, wall_height)),
            color: Color::WHITE,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, -window_height / 2.0 + wall_height / 2.0, 1.0),
            ..default()
        },
        ..default()
    });
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
                custom_size: Some(Vec2::new(pad_width, pad_height)),
                color: Color::BLUE,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(left_pad_x_offset, Pad::DEFAUT_LEVEL, 1.0),
                ..default()
            },
            ..default()
        })
        .insert(Player::Left)
        .insert(Pad)
        .insert(Collider);

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(pad_width, pad_height)),
                color: Color::RED,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(right_pad_x_offset, Pad::DEFAUT_LEVEL, 1.0),
                ..default()
            },
            ..default()
        })
        .insert(Player::Right)
        .insert(Pad)
        .insert(Collider);
}

fn spawn_ball(mut commands: Commands, windows: ResMut<Windows>) {
    let window_width = windows
        .get_primary()
        .expect("Failed to get primary window")
        .width();

    let ball_size = window_width / 100.0 * Ball::SIZE;

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(ball_size, ball_size)),
                color: Color::WHITE,
                ..default()
            },
            ..default()
        })
        .insert(Ball::default());
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

    let pad_height = window_height / 100.0 * Pad::HEIGHT;

    let wall_height = window_height / 100.0 * WALL_THICKNESS;

    let top_y_limit = window_height / 2.0 - wall_height - pad_height / 2.0;
    let bottom_y_limit = -(window_height / 2.0) + wall_height + pad_height / 2.0;

    for event in event_reader.iter() {
        match event {
            PlayerMovementEvent::Up(event_player) =>
                for q in query.iter_mut() {
                    let (_pad, player, mut transform): (&Pad, &Player, Mut<Transform>) = q;
                    if player == event_player {
                        let new_pad_y = transform.translation.y + movement;

                        transform.translation.y = new_pad_y.clamp(bottom_y_limit, top_y_limit);
                    }
                },
            PlayerMovementEvent::Down(event_player) =>
                for q in query.iter_mut() {
                    let (_pad, player, mut transform): (&Pad, &Player, Mut<Transform>) = q;
                    if player == event_player {
                        let new_pad_y = transform.translation.y - movement;
                        transform.translation.y = new_pad_y.clamp(bottom_y_limit, top_y_limit);
                    }
                },
        }
    }
}

fn ball_movement_system(mut ball_query: Query<(&Ball, &mut Transform)>, windows: Res<Windows>) {
    let primary_window = windows.get_primary().expect("Faile to get primary window");

    let window_height = primary_window.height();
    let window_width = primary_window.width();

    let x_movement = window_width / 100.0 * Ball::SPEED;
    let y_movement = window_height / 100.0 * Ball::SPEED;

    if let Ok((ball, mut transform)) = ball_query.get_single_mut() {
        let ball: &Ball = ball;

        let movement_vector = ball.angle * Vec2::new(x_movement, y_movement);

        transform.translation += movement_vector.extend(0.0);
    }
}

fn collision_system(
    mut ball_query: Query<(&mut Ball, &mut Transform, &Sprite), Without<Collider>>,
    pad_query: Query<(&Transform, &Sprite), With<Collider>>,
    windows: Res<Windows>,
) {
    let primary_window = windows.get_primary().expect("Failed to get primary window");
    let window_height = primary_window.height();
    let window_width = primary_window.width();

    let wall_height = window_height / 100.0 * WALL_THICKNESS;

    let top_wall_y = window_height / 2.0 - wall_height / 2.0;
    let bottom_wall_y = -(window_height / 2.0) + wall_height / 2.0;

    let top_wall_center = Vec2::new(0.0, top_wall_y).extend(0.0);
    let bottom_wall_center = Vec2::new(0.0, bottom_wall_y).extend(0.0);
    let wall_size = Vec2::new(window_width, wall_height);

    let left_border = Vec2::new(-(window_width / 2.0), 0.0).extend(0.0);
    let right_border = Vec2::new(window_width / 2.0, 0.0).extend(0.0);
    let border_size = Vec2::new(1.0, window_height);

    if let Ok((ball, ball_transform, ball_sprite)) = ball_query.get_single_mut() {
        let mut ball: Mut<Ball> = ball;
        let mut ball_transform: Mut<Transform> = ball_transform;
        let ball_sprite: &Sprite = ball_sprite;

        let ball_size = ball_sprite.custom_size.expect("WTF ball doesn't have size");

        let top_wall_collision = collide(
            ball_transform.translation,
            ball_size,
            top_wall_center,
            wall_size,
        );

        let bottom_wall_collision = collide(
            ball_transform.translation,
            ball_size,
            bottom_wall_center,
            wall_size,
        );

        if top_wall_collision.is_some() || bottom_wall_collision.is_some() {
            ball.angle.y = -ball.angle.y;
            info!("{:?}", ball.angle);
        }

        let left_border_collision = collide(
            ball_transform.translation,
            ball_size,
            left_border,
            border_size,
        );

        let right_border_collision = collide(
            ball_transform.translation,
            ball_size,
            right_border,
            border_size,
        );

        if left_border_collision.is_some() || right_border_collision.is_some() {
            *ball.as_mut() = Ball::default();
            ball_transform.translation = Vec2::splat(0.0).extend(0.0);
        }


        for (pad_transform, pad_sprite) in pad_query.iter() {
            let pad_transform: &Transform = pad_transform;
            let pad_sprite: &Sprite = pad_sprite;

            let collision = collide(
                ball_transform.translation,
                ball_size,
                pad_transform.translation,
                pad_sprite
                    .custom_size
                    .expect("WTF Pad doesn't have custom size!"),
            );

            match collision {
                Some(Collision::Left) | Some(Collision::Right) => ball.angle.x = -ball.angle.x,
                _ => (),
            };
        }
    }
}


fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut event_writer: EventWriter<PlayerMovementEvent>,
) {
    if keyboard_input.pressed(KeyCode::W) {
        event_writer.send(PlayerMovementEvent::Up(Player::Left));
    }

    if keyboard_input.pressed(KeyCode::S) {
        event_writer.send(PlayerMovementEvent::Down(Player::Left));
    }


    if keyboard_input.pressed(KeyCode::Up) {
        event_writer.send(PlayerMovementEvent::Up(Player::Right));
    }

    if keyboard_input.pressed(KeyCode::Down) {
        event_writer.send(PlayerMovementEvent::Down(Player::Right));
    }
}

fn clear<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
