use bevy::{
    input::{keyboard::KeyCode, Input},
    prelude::*,
};

use benimator::*;
//use bevy_inspector_egui::Inspectable;

use crate::events::{
    MoveEvent,
    MoveLegal
};

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

#[derive(Default)]
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DirectionAnimations>()
            .add_systems(Startup, setup)
            .add_systems(PreUpdate, try_move_player)
            .add_systems(Update, move_player)
            .add_systems(PostUpdate, animate_player);
    }
}

fn animate_player(
    time: Res<Time>,
    mut animation_query: Query<(
        &PlayerAnimation,
        &mut PlayerAnimationState,
        &mut TextureAtlasSprite,
    )>,
) {
    for (animation, mut animation_state, mut texture_atlas) in animation_query.iter_mut() {
        animation_state.update(animation, time.delta());
        texture_atlas.index = animation_state.frame_index();
    }
}

fn try_move_player(
    keyboard_input: Res<Input<KeyCode>>,
    animations: Res<DirectionAnimations>,
    time: Res<Time>,
    mut player_query: Query<(&mut PlayerAnimation, &mut Transform, &Player)>,
    //wall_query: Query<(&Transform, (With<TileCollider>, Without<Player>))>,
    mut move_event: EventWriter<MoveEvent>,
) {
    let (mut animation, transform, player) = player_query.single_mut();
    let mut player_move_event = MoveEvent {
        origin: Some(transform.translation),
        destination: None,
    };
    let mut send_event = false;
    let mut destination = transform.translation;

    if keyboard_input.pressed(KeyCode::A) {
        *animation = PlayerAnimation(animations.left.clone());
        destination.x -=  player.speed * player.size * time.delta_seconds();
        send_event = true;
    } else if keyboard_input.pressed(KeyCode::D) {
        *animation = PlayerAnimation(animations.right.clone());
        destination.x +=  player.speed * player.size * time.delta_seconds();
        send_event = true;
    } else if keyboard_input.pressed(KeyCode::S) {
        *animation = PlayerAnimation(animations.down.clone());
        destination.y -=  player.speed * player.size * time.delta_seconds();
        send_event = true;
    } else if keyboard_input.pressed(KeyCode::W) {
        *animation = PlayerAnimation(animations.up.clone());
        destination.y +=  player.speed * player.size * time.delta_seconds();
        send_event = true;
    }
    if keyboard_input.any_just_released([KeyCode::A, KeyCode::S, KeyCode::D, KeyCode::W]) {
        send_event = false;
        //info!("Need to create an idle animation!");
        //*animation = PlayerAnimation(animations.idle.clone());
    }

    if send_event {
        player_move_event.destination = Some(destination);
        move_event.send(player_move_event);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    direction_animations: Res<DirectionAnimations>,
) {
    let texture_handle = asset_server.load("Male 01-1.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 3, 4, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let player_position = Transform::from_translation(Vec3::Z * 10.0) * Transform::from_scale(Vec3::splat(1.0));

    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
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

fn move_player(
    mut q: Query<(&mut Transform, With<Player>)>,
    mut valid_move: EventReader<MoveLegal>,
) {
    for event in valid_move.read() {
        if event.destination.is_none() {
            return;
        }
        if event.legal_move {
            for (mut transform, _) in q.iter_mut() {
                transform.translation = Vec3::new(
                    event.destination.unwrap().x, 
                    event.destination.unwrap().y,
                    10.0,
                );
            }
        }
    }
}