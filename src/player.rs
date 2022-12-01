use core::time::Duration;

use bevy::{
    input::{keyboard::KeyCode, Input},
    prelude::*,
};

use benimator::*;

use crate::events::{
    MoveEvent,
    MoveLegal
};

const ANIMATION_DURATION: u64 = 200;

#[derive(Component)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Standing,
}

#[derive(Component)]
struct PlayerTag;

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
        let anim_duration = Duration::new(ANIMATION_DURATION, 0);
        DirectionAnimations {
            up: Animation::from_indices(9..=11, FrameRate::from_frame_duration(anim_duration)),
            down: Animation::from_indices(0..=2, FrameRate::from_frame_duration(anim_duration)),
            left: Animation::from_indices(3..=5, FrameRate::from_frame_duration(anim_duration)),
            right: Animation::from_indices(6..=8, FrameRate::from_frame_duration(anim_duration)),
        }
    }
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Default)]
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DirectionAnimations>()
            .add_startup_system(setup)
            .add_system_set_to_stage(
                CoreStage::PreUpdate,
                SystemSet::new()
                    .with_system(try_move_player)   
            )
            .add_system_set_to_stage(
                CoreStage::Update,
                SystemSet::new()
                    .with_system(move_player)   
            )
            .add_system_set_to_stage(
                CoreStage::PostUpdate,
                SystemSet::new()
                    .with_system(animate_player)   
            );
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
    for (animation, mut animation_state, mut texture_atlas) in &mut animation_query {
        animation_state.update(animation, time.delta());
        texture_atlas.index = animation_state.frame_index();
    }
}

fn try_move_player(
    keyboard_input: Res<Input<KeyCode>>,
    animations: Res<DirectionAnimations>,
    time: Res<Time>,
    mut direction_query: Query<(&mut PlayerAnimation, &Transform)>,
    mut move_event: EventWriter<MoveEvent>,
) {
    let (mut animation, transform) = direction_query.single_mut();
    let mut player_move_event = MoveEvent {
        origin: Some(transform.translation),
        destination: None,
    };
    let mut send_event = false;
    let mut destination = transform.translation;

    if keyboard_input.pressed(KeyCode::A) {
        *animation = PlayerAnimation(animations.left.clone());
        destination.x -=  100. * time.delta_seconds();
        send_event = true;
    } else if keyboard_input.pressed(KeyCode::D) {
        *animation = PlayerAnimation(animations.right.clone());
        destination.x += 100. * time.delta_seconds();
        send_event = true;
    } else if keyboard_input.pressed(KeyCode::S) {
        *animation = PlayerAnimation(animations.down.clone());
        destination.y -= 100. * time.delta_seconds();
        send_event = true;
    } else if keyboard_input.pressed(KeyCode::W) {
        *animation = PlayerAnimation(animations.up.clone());
        destination.y += 100. * time.delta_seconds();
        send_event = true;
    } else if keyboard_input.any_just_released([KeyCode::A, KeyCode::S, KeyCode::D, KeyCode::W]) {
        println!("Just released a key... stop animation.")
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
        .insert(PlayerTag);
}

fn move_player(
    mut q: Query<(&mut Transform, With<PlayerTag>)>,
    mut valid_move: EventReader<MoveLegal>,
) {
    for event in valid_move.iter() {
        if event.legal_move {
            for (mut transform, _) in q.iter_mut() {
                println!("event pos.x = {}, event pos.y = {}", event.destination.unwrap().x, event.destination.unwrap().y);
                println!("transform pos.x = {}, transform pos.y = {}", transform.translation.x, transform.translation.y);
                transform.translation = Vec3::new(
                    event.destination.unwrap().x, 
                    event.destination.unwrap().y,
                    10.0,
                );
            }
        }
    }
}