use bevy::{
    input::{keyboard::KeyCode, ButtonInput},
    prelude::*,
};
use bevy::input::mouse::{MouseWheel, MouseScrollUnit};

use benimator::*;
//use bevy_inspector_egui::Inspectable;

use crate::events::{
    MoveEvent,
    MoveLegal
};

use crate::constants::{BG_COLOR};

const ANIMATION_DURATION: f64 = 8.0;
const MOVE_SPEED: f32 = 3.0;
const PLAYER_TILE_SIZE: f32 = 32.0;

#[derive(Component)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Standing,
}

//#[derive(Component, Inspectable)]
#[derive(Component)]
pub struct Player {
    speed: f32,
    size: f32,
}

#[derive(Component, Deref, Debug)]
struct PlayerAnimation(benimator::Animation);

#[derive(Default, Component, Deref, DerefMut)]
struct PlayerAnimationState(benimator::State);

#[derive(Resource)]
struct DirectionAnimations {
    up: benimator::Animation,
    down: benimator::Animation,
    left: benimator::Animation,
    right: benimator::Animation,
}

impl Default for DirectionAnimations {
    fn default() -> Self {
        DirectionAnimations {
            up: Animation::from_indices(9..=11, FrameRate::from_fps(ANIMATION_DURATION)),
            down: Animation::from_indices(0..=2, FrameRate::from_fps(ANIMATION_DURATION)),
            left: Animation::from_indices(3..=5, FrameRate::from_fps(ANIMATION_DURATION)),
            right: Animation::from_indices(6..=8, FrameRate::from_fps(ANIMATION_DURATION)),
        }
    }
}

// #[derive(Component, Clone, Copy, PartialEq, Eq)]
// struct Position {
//     x: i32,
//     y: i32,
// }
#[derive(Component)]
pub struct PlayerCamera;

#[derive(Default)]
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DirectionAnimations>()
            .add_systems(Startup, setup)
            .add_systems(Startup, setup_camera)
            .add_systems(PreUpdate, try_move_player)
            .add_systems(Update, move_player)
            .add_systems(Update, zoom_map)
            .add_systems(PostUpdate, animate_player);
    }
}

fn animate_player(
    time: Res<Time>,
    mut animation_query: Query<(
        &PlayerAnimation,
        &mut PlayerAnimationState,
        &mut TextureAtlas,
    )>,
) {
    for (animation, mut animation_state, mut texture_atlas) in animation_query.iter_mut() {
        animation_state.update(animation, time.delta());
        texture_atlas.index = animation_state.frame_index();
    }
}

fn try_move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    animations: Res<DirectionAnimations>,
    time: Res<Time>,
    mut player_query: Query<(&mut PlayerAnimation, &mut Transform, &Player), Without<PlayerCamera>>,
    mut camera_query: Query<&mut Transform, With<PlayerCamera>>,
    mut move_event: EventWriter<MoveEvent>,
) {
    for mut camera_transform in camera_query.iter_mut() {
        let mut direction = Vec3::ZERO;

        let (mut animation, transform, player) = player_query.single_mut();
        let mut player_move_event = MoveEvent {
            origin: Some(transform.translation),
            destination: None,
        };
        let mut send_event = false;
        let mut destination = transform.translation;
    
        if keyboard_input.pressed(KeyCode::KeyA) {
            *animation = PlayerAnimation(animations.left.clone());
            destination.x -=  player.speed * player.size * time.delta_seconds();
            send_event = true;
            direction -= Vec3::new(1.0, 0.0, 0.0);
        } else if keyboard_input.pressed(KeyCode::KeyD) {
            *animation = PlayerAnimation(animations.right.clone());
            destination.x +=  player.speed * player.size * time.delta_seconds();
            send_event = true;
            direction += Vec3::new(1.0, 0.0, 0.0);
        } else if keyboard_input.pressed(KeyCode::KeyS) {
            *animation = PlayerAnimation(animations.down.clone());
            destination.y -=  player.speed * player.size * time.delta_seconds();
            send_event = true;
            direction -= Vec3::new(0.0, 1.0, 0.0);
        } else if keyboard_input.pressed(KeyCode::KeyW) {
            *animation = PlayerAnimation(animations.up.clone());
            destination.y +=  player.speed * player.size * time.delta_seconds();
            send_event = true;
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.any_just_released([KeyCode::KeyA, KeyCode::KeyS, KeyCode::KeyD, KeyCode::KeyW]) {
            send_event = false;
            //info!("Need to create an idle animation!");
            //*animation = PlayerAnimation(animations.idle.clone());
        }
    
        if !send_event {
            player_move_event.destination = Some(destination);
            move_event.send(player_move_event);
        }

        let z = camera_transform.translation.z;
        camera_transform.translation += time.delta_seconds() * direction * 500.;
        camera_transform.translation.z = z;
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    direction_animations: Res<DirectionAnimations>,
) {
    let texture: Handle<Image> = asset_server.load("Male 01-1.png");
    let layout= TextureAtlasLayout::from_grid( UVec2::new(32, 32), 3, 4, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    
    let player_position = Transform::from_translation(Vec3::Z * 10.0) * Transform::from_scale(Vec3::splat(1.0));

    commands
        .spawn(SpriteSheetBundle {
            texture,
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: 1
            },
            transform: player_position,
            ..default()
        })
        .insert(PlayerAnimation(direction_animations.left.clone()))
        .insert(PlayerAnimationState::default())
        .insert(Direction::Left)
        .insert(Player {
            speed: MOVE_SPEED,
            size: PLAYER_TILE_SIZE
        });
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        camera: Camera { 
            clear_color: ClearColorConfig::Custom(BG_COLOR),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(PlayerCamera);
}

fn move_player(
    mut q: Query<&mut Transform, With<Player>>,
    mut valid_move: EventReader<MoveLegal>,
) {
    for event in valid_move.read() {
        if event.destination.is_none() {
            return;
        }
        if event.legal_move {
            for mut transform in q.iter_mut() {
                transform.translation = Vec3::new(
                    event.destination.unwrap().x, 
                    event.destination.unwrap().y,
                    10.0,
                );
            }
        }
    }
}

fn zoom_map(
    mut query_camera: Query<&mut OrthographicProjection, With<PlayerCamera>>,
    mut scroll_evr: EventReader<MouseWheel>,
) {
    let mut projection = query_camera.single_mut();
    
    for ev in scroll_evr.read() {
        if ev.unit == MouseScrollUnit::Line {
            if ev.y == -1.0 {
                // zoom in
                projection.scale *= 1.25;
            } else if ev.y == 1.0 {
                // zoom out
                projection.scale /= 1.25;                
            }
        }
    }    
}